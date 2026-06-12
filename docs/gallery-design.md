# Gallery Design

## Goal

Make recipes discoverable by humans before they read JSON or generated skill
Markdown.

The gallery should answer:

- What can this recipe make?
- What does it look like?
- Which tags describe its use case and visual family?
- Which prompt template, style, and composition pattern does it use?
- How can a contributor copy it and make a variant?

## Tags

Tags are governance metadata. They are not prose decorations.

Use tags to group and filter gallery cards:

- Use case: `article`, `docs`, `video`, `product`, `technical`,
  `code-review`, `business`, `consumer`, `finance`, `tech-news`
- Visual family: `minimal`, `doodle`, `cartoon`, `comic`, `whiteboard`,
  `sticker`, `clay`, `sketch`
- Structure: `flow`, `systems`, `before-after`, `action-loop`, `layers`,
  `route`, `metaphor`
- Workflow role: `image`, `edit`, `cleanup`, `style-adjustment`
- Domain: `coupon`, `saas`, `engineering`, `risk`

Rules:

- Tags must be lowercase kebab-case.
- Tags should be broad enough to group several recipes.
- Do not create near-duplicates such as `biz`, `business-video`, and
  `business`.
- Prefer 3-6 tags per item.
- Every future `RecipeSpec` should have tags.
- A style, composition pattern, or prompt template may have tags even before it
  is attached to a first-class recipe.

## Current Tagged Objects

The current schema tags:

- `StyleSpec`
- `CompositionPatternSpec`
- `PromptTemplateSpec`

These are enough to power an early gallery that groups by style and template.
The next step is a first-class `RecipeSpec`.

## Future RecipeSpec

A recipe should combine one style, one composition pattern, one prompt template,
sample briefs, and example outputs.

```json
{
  "id": "yellow-worker-finance-video",
  "tags": ["video", "business", "finance", "comic", "action-loop"],
  "name": "Yellow Worker Finance Video",
  "description": "财经、商业、科技新闻短视频动作漫画画面。",
  "style_id": "yellow-worker-comic",
  "composition_pattern_id": "action-loop-scenes",
  "prompt_template_id": "yellow-worker-video-frame",
  "recommended_providers": ["codex", "minimax"],
  "sample_briefs": [
    {
      "id": "ai-capex",
      "title": "AI Capex",
      "brief": "Explain why AI companies keep spending cash on data centers."
    }
  ],
  "examples": [
    {
      "brief_id": "ai-capex",
      "provider": "codex",
      "image_path": "gallery/assets/yellow-worker-finance-video/ai-capex-codex.png",
      "prompt_path": "gallery/assets/yellow-worker-finance-video/ai-capex.prompt.txt"
    }
  ]
}
```

## Static Gallery Shape

First version should be static output:

```text
gallery/
├── index.html
├── gallery.json
├── recipes/
│   └── yellow-worker-finance-video.html
└── assets/
    └── yellow-worker-finance-video/
        ├── ai-capex-codex.png
        └── ai-capex.prompt.txt
```

Group cards by:

- use case tags: `video`, `docs`, `technical`, `business`
- visual family tags: `comic`, `minimal`, `doodle`, `sticker`
- domain tags: `finance`, `consumer`, `engineering`

## Contributor Flow

1. Copy an existing recipe or style.
2. Pick 3-6 existing tags.
3. Add one new tag only if no existing tag fits.
4. Add 2-3 sample briefs.
5. Generate local examples.
6. Store images and prompts under `gallery/assets/<recipe-id>/`.
7. Run gallery build.
8. Submit the JSON, generated gallery, prompts, and images together.

Do not contribute a recipe without tags, sample briefs, and example images.

## Deployment

The first gallery is deployed through GitHub Pages from the checked-in
`gallery/` directory:

```text
https://shuozeli.github.io/general-illustrations/
```

The Pages workflow uploads the directory as a static artifact. No Node or Rust
build step is required for the first version.

The checked-in gallery keeps at least one card for every current `StyleSpec`.
Cards should be marked `sample` only after a project-local image has been
generated specifically for that recipe.

When adding a new style such as `mambo-meme-comic` or
`honglou-retro-flat-anime`, update the JSON tags, render the skill references,
add a Gallery card, and include at least one demo image.
