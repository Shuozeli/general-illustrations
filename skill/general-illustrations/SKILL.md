---
name: general-illustrations
description: Recipe-based image and illustration generation for articles, technical docs, architecture explanations, product docs, code reviews, workflows, comparisons, and visual planning. Use when Codex should choose an illustration recipe, produce a shot list, generate or edit images, define a visual style, or turn text/code/design material into article illustrations, technical diagrams, review visuals, product explainers, workflow maps, or comparison charts.
---

# General Illustrations

Create useful visuals by routing each request through a recipe and a style pack.
This skill is intentionally general: do not force one fixed IP, mascot, or art
style on every task.

## Workflow

1. Read the user's source material: article, design doc, README, code review,
   architecture notes, screenshot, or concept.
2. Identify the visual job: explain, compare, warn, summarize, teach, route,
   or make a concept memorable.
3. Choose one recipe from `references/selection-guide.md`.
4. Choose a style from `references/style-packs.md`, or define a short ad hoc
   style if the user already gave one.
5. If the user asks for planning, output a concise shot list only.
6. If the user asks to generate, create each image separately with `image_gen`;
   do not combine multiple shots into one image unless explicitly requested.
7. Check the output against `references/qa-checklist.md`; regenerate or edit
   if the visual misses the recipe's purpose.

## Recipes

- **Article illustration:** use `references/article-illustration.md` for blog,
  essay, newsletter, Notion, and long-form writing visuals.
- **Technical diagram:** use `references/technical-diagram.md` for systems,
  APIs, RPC, database, dataflow, architecture, and infrastructure explanations.
- **Code review visual:** use `references/code-review-visual.md` for bugs,
  regressions, risk surfaces, before/after behavior, and test gaps.

## Output Modes

- **Shot list:** recommend 1-8 images with placement, purpose, recipe, style,
  composition, labels, and generation notes.
- **Single image prompt:** produce one complete prompt ready for image
  generation.
- **Generated images:** call `image_gen` once per image.
- **Edit prompt:** keep the original meaning and ask for focused edits only.

## Defaults

- Default aspect ratio: 16:9 for article/docs visuals.
- Default background: clean and readable, not decorative.
- Default text on image: sparse, short labels only.
- Default visual density: one core idea per image.
- Default style: `clean-docs` unless the recipe or user suggests otherwise.

## Boundaries

- Do not copy Ian's "小黑" IP or any third-party recurring character as a
  Shuozeli identity.
- Do not treat example images as templates to copy.
- Do not make dense PPT slides when the user asked for illustrations.
- Do not make decorative art when the user needs an explanatory diagram.
- Do not invent precise UI, architecture, or code facts that are not present in
  the source material.
