#!/usr/bin/env python3
"""Regenerate gallery/gallery.json from specs/general-illustrations.json.

The gallery is the human-facing layer over the recipe spec. It is generated
(not hand-authored) so it cannot drift from the canonical recipes -- the same
anti-drift rule that governs the skill Markdown. index.html fetches this JSON at
runtime and renders one card per recipe, grouped by category.

Run: python3 gallery/build_gallery.py   (from the repo root)
"""
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SPEC = ROOT / "specs" / "general-illustrations.json"
OUT = ROOT / "gallery" / "gallery.json"
ASSETS = ROOT / "gallery" / "assets"

# Deterministic build stamp (no wall-clock time in generated files).
UPDATED = "2026-08-09"

# recipe id -> example image (relative to gallery/). Missing entries render a
# placeholder card. Existing example images are reused from the prior gallery;
# finance-news showcases a real ChatGPT-generated recipe-faithful image.
IMAGE_MAP = {
    "clean-doc": "assets/clean-docs-overview/reference.png",
    "tech-diagram": "assets/technical-lsm-tree/codex.png",
    "code-review": "assets/code-review-risk-path/reference.png",
    "article-sketch": "assets/motion-editorial-lsm-tree/codex.png",
    "product-explainer": "assets/product-explainer-api/reference.png",
    "whiteboard-story": "assets/whiteboard-lsm-tree/codex.png",
    "doodle-light": "assets/cartoon-lsm-tree/codex.png",
    "sticker-layers": "assets/sticker-layer-lsm-tree/codex.png",
    "clay-metaphor": "assets/soft-clay-lsm-tree/codex.png",
    "software-news": "assets/cartoon-explainer-lsm/codex.png",
    "consumer-coupon": "assets/coupon-worker-consumer-video/reference.png",
    "finance-news": "assets/finance-news/chatgpt.png",
    "infra-meme": "assets/mambo-meme-kafka/codex.png",
    # honglou-keyframe: no example image yet -> placeholder.
}


def main() -> int:
    spec = json.loads(SPEC.read_text(encoding="utf-8"))
    categories = [
        {"id": c["id"], "name": c["name"], "summary": c["summary"]}
        for c in spec.get("categories", [])
    ]

    recipes = []
    for r in spec.get("recipes", []):
        providers = ["gemini"]
        if (r.get("providers") or {}).get("chatgpt"):
            providers.append("chatgpt-override")
        else:
            providers.append("chatgpt")
        image = IMAGE_MAP.get(r["id"])
        if image and not (ROOT / "gallery" / image).exists():
            image = None
        recipes.append(
            {
                "id": r["id"],
                "category": r["category"],
                "name": r["name"],
                "description": r["use_when"],
                "style_id": r["style_id"],
                "composition_pattern_id": r["default_composition_id"],
                "prompt_template_id": r["prompt_template_id"],
                "providers": providers,
                "tags": r.get("tags", []),
                "image": image,
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
