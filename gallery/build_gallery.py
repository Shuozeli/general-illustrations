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

# English translations for the i18n-friendly gallery. The spec (source of truth)
# is authored in Chinese; these provide the `en` label. Missing entries fall back
# to the Chinese text at render time, so the page never breaks on a gap.
CATEGORY_EN = {
    "article-docs": ("Article / Docs",
                     "Static explainer art for articles, READMEs, tech blogs, and docs."),
    "video-light": ("Light Video",
                    "Lightweight short-video knowledge frames: doodle, sticker, soft clay."),
    "video-character-comic": ("Character Comic",
                              "Yellow/blue worker character action comics for finance, business, consumer, tech, and software news."),
    "video-meme-motion": ("Meme Motion",
                          "Meme-rhythm loop frames for system mechanics and engineering stories."),
    "literary-period": ("Literary / Period",
                        "Retro TV-anime or period-drama keyframes for Hongloumeng and classical literature."),
}

RECIPE_DESC_EN = {
    "clean-doc": "Articles, READMEs, product explainers, general article art.",
    "tech-diagram": "System structure, RPC, databases, LSM trees, architecture boundaries, data flow.",
    "code-review": "Code review, bug paths, risk propagation, test gaps, before/after.",
    "article-sketch": "Article arguments, methodology, abstract metaphors, cognitive turns.",
    "product-explainer": "Product capabilities, feature intros, tool explainers, docs intros.",
    "whiteboard-story": "Engineering stories, bug explanations, before/after, fail-to-fix, teaching panels.",
    "doodle-light": "Short-video knowledge frames, lightweight tech explainers, doodles.",
    "sticker-layers": "Short-video animation layers, component intros, process breakdowns.",
    "clay-metaphor": "Concept metaphors, video illustrations, light tech explainers.",
    "software-news": "Software news, AI engineering, platforms, developer-tool explainer videos.",
    "consumer-coupon": "Coupons, subscriptions, delivery platforms, membership pricing, dynamic pricing, consumer-psychology shorts.",
    "finance-news": "Finance, company news, earnings explainers, AI infrastructure, supply chain, regulation, valuation, business-model videos.",
    "infra-meme": "System mechanics: Kafka backlog, stalled order paths, consumers chasing offset, Redis hot keys, slow queries.",
    "story-infra": "Kafka / message-queue / distributed-systems series: incident openers, message flow, partitions, replicas, offset, consumer groups, retries, idempotency, ISR.",
    "honglou-keyframe": "Hongloumeng / classical-literature retro TV-anime keyframes and sequential storyboards.",
    "honglou-painterly": "For Hongloumeng videos needing live-actor-like repainting, a TV-drama-still feel, mature portraits, and serious family narratives.",
}


def i18n(zh: str, en: str | None) -> dict[str, str]:
    """A localized label: Chinese source plus optional English."""
    return {"zh": zh, "en": en or zh}

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
    categories = []
    for c in spec.get("categories", []):
        en_name, en_summary = CATEGORY_EN.get(c["id"], (None, None))
        categories.append(
            {
                "id": c["id"],
                "name": i18n(c["name"], en_name),
                "summary": i18n(c["summary"], en_summary),
            }
        )

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
                "description": i18n(r["description"], RECIPE_DESC_EN.get(r["id"])),
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
        "languages": ["en", "zh"],
        "categories": categories,
        "recipes": recipes,
    }
    OUT.write_text(json.dumps(gallery, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    have = sum(1 for r in recipes if r["image"])
    print(f"wrote {OUT} | {len(categories)} categories | {len(recipes)} recipes | {have} with images")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
