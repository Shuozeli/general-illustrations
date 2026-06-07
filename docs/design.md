# Design

## Principle

Recipes decide what to draw. Providers decide how to ask a model to draw it.

This is the main difference from the reference `ian-xiaohei-illustrations`
project. That project is intentionally one strong style. This project is a
general illustration system that can route the same conceptual request through
multiple styles and multiple image providers.

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
