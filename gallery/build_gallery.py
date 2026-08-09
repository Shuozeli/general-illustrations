#!/usr/bin/env python3
"""Regenerate gallery/gallery.json from specs/general-illustrations.json.

The gallery is the human-facing layer over the recipe spec. It is generated
(not hand-authored) so it cannot drift from the canonical recipes -- the same
anti-drift rule that governs the skill Markdown. index.html fetches this JSON at
runtime and renders one card per recipe, grouped by category, with a per-provider
image strip (Codex / Ark / Gemini / ChatGPT) so provider outputs can be compared
inside the card.

Run: python3 gallery/build_gallery.py   (from the repo root)
"""
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SPEC = ROOT / "specs" / "general-illustrations.json"
OUT = ROOT / "gallery" / "gallery.json"
GALLERY = ROOT / "gallery"

# Deterministic build stamp (no wall-clock time in generated files).
UPDATED = "2026-08-09"

# recipe id -> list of (provider, image path relative to gallery/). The first
# existing image is the card's primary; the rest render as a comparison strip.
# Existing example images are reused from the prior gallery (their Codex/Ark
# comparison assets); finance-news showcases a real ChatGPT-generated image.
PROVIDER_IMAGES = {
    "clean-doc": [("codex", "assets/clean-docs-overview/reference.png"),
                  ("ark", "assets/clean-docs-overview/ark.png")],
    "tech-diagram": [("codex", "assets/technical-lsm-tree/codex.png"),
                     ("ark", "assets/technical-lsm-tree/ark.png")],
    "code-review": [("codex", "assets/code-review-risk-path/reference.png")],
    "article-sketch": [("codex", "assets/motion-editorial-lsm-tree/codex.png")],
    "product-explainer": [("codex", "assets/product-explainer-api/reference.png")],
    "whiteboard-story": [("codex", "assets/whiteboard-lsm-tree/codex.png")],
    "doodle-light": [("codex", "assets/cartoon-lsm-tree/codex.png")],
    "sticker-layers": [("codex", "assets/sticker-layer-lsm-tree/codex.png")],
    "clay-metaphor": [("codex", "assets/soft-clay-lsm-tree/codex.png")],
    "software-news": [("codex", "assets/cartoon-explainer-lsm/codex.png")],
    "consumer-coupon": [("codex", "assets/coupon-worker-consumer-video/reference.png")],
    "finance-news": [("chatgpt", "assets/finance-news/chatgpt.png"),
                     ("codex", "assets/yellow-worker-finance-video/reference.png"),
                     ("ark", "assets/yellow-worker-finance-video/ark.png")],
    "infra-meme": [("codex", "assets/mambo-meme-kafka/codex.png")],
    "honglou-keyframe": [("codex", "assets/honglou-retro-banquet/codex.png"),
                         ("ark", "assets/honglou-retro-banquet/ark.png")],
    # story-infra, honglou-painterly: no example images yet -> placeholder.
}


def main() -> int:
    spec = json.loads(SPEC.read_text(encoding="utf-8"))
    categories = [
        {"id": c["id"], "name": c["name"], "summary": c["summary"]}
        for c in spec.get("categories", [])
    ]

    recipes = []
    for r in spec.get("recipes", []):
        images = []
        for provider, rel in PROVIDER_IMAGES.get(r["id"], []):
            if (GALLERY / rel).exists():
                images.append({"provider": provider, "src": rel})
        recipes.append(
            {
                "id": r["id"],
                "category": r["category"],
                "name": r["name"],
                "description": r["description"],
                "style_id": r["style_id"],
                "composition_pattern_id": r["composition_pattern_id"],
                "prompt_template_id": r["prompt_template_id"],
                "recommended_providers": r.get("recommended_providers", []),
                "tags": r.get("tags", []),
                "image": images[0]["src"] if images else None,
                "images": images,
            }
        )

    gallery = {
        "title": "General Illustrations Gallery",
        "updated": UPDATED,
        "categories": categories,
        "recipes": recipes,
    }
    OUT.write_text(json.dumps(gallery, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    have = sum(1 for r in recipes if r["image"])
    print(f"wrote {OUT} | {len(categories)} categories | {len(recipes)} recipes | {have} with images")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
