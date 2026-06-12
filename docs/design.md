# Design

## Principle

Recipes decide what to draw. Providers decide how to ask a model to draw it.

This is the main difference from the reference `ian-xiaohei-illustrations`
project. That project is intentionally one strong style. This project is a
general illustration system that can route the same conceptual request through
multiple styles and multiple image providers.

## Improvement Over The Reference

The reference project is valuable because it shows a complete skill pattern:
activation text, references, examples, QA, and a strong visual identity. This
project keeps that skill shape, but changes the authoring model.

The improvement is not “more styles” by itself. The real improvement is that the
system is no longer pinned to one visual identity or one image shape:

- Skill content is authored as JSON data, then rendered into Codex Markdown.
- Styles are independent records, not prose hidden inside a hand-written skill.
- Composition patterns are independent from styles.
- Prompt templates are independent from providers.
- Providers are adapters under a unified request/response API.
- Examples are calibration assets, not templates to copy.
- Tags make styles, prompt templates, composition patterns, and future recipes
  groupable in a Gallery.

That means a future agent or service can create, validate, store, and render a
new illustration skill through structured data. It can add a style without
rewriting the workflow, add a provider without changing the recipe, or generate a
new skill from a REST/API payload without editing local Markdown files.

## First Recipes

- Article illustration
- Technical diagram
- Code review visual

## Style Families

- Clean docs
- Technical minimal
- Review minimal
- Editorial sketch
- Product explainer
- Simple doodle
- Cartoon explainer
- Whiteboard comic
- Sticker layer
- Soft clay cartoon

## Provider Strategy

MiniMax is useful for broad availability but needs short, concrete prompts.
Codex/CodeIce currently produces stronger structured diagrams and better short
text, but it is a tool surface rather than this repo's HTTP API. Gemini Web can
be tested manually with the same recipe prompts until a stable API adapter is
chosen.
