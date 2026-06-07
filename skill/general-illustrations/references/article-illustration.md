# Recipe: Article Illustration

## Purpose

Turn a key idea in an article into a memorable visual. Choose cognitive anchors,
not evenly spaced paragraphs.

## Shot List Fields

- Placement: where the image belongs.
- Theme: short name for the shot.
- Core idea: the one thought the image expresses.
- Visual metaphor: concrete physical scene or object system.
- Style: one style pack or user-provided style.
- Labels: 0-6 short labels.
- Generation notes: composition and constraints.

## Good Anchors

- Core thesis.
- A contrast or reversal.
- A recurring failure mode.
- A handoff between people, tools, or phases.
- A small mechanism that explains the larger argument.
- A metaphor already implied by the text.

## Prompt Pattern

```text
Generate one standalone 16:9 article illustration.

Source idea:
{core idea}

Visual metaphor:
{fresh metaphor or scene}

Composition:
{main subject, spatial layout, motion, emphasis, empty space}

Style:
{chosen style pack}

Labels:
{short labels, if any}

Constraints:
One image explains one idea. Keep it readable at article width. Avoid dense
infographics, slide decks, stock-photo feeling, generic mascots, and copied
third-party IP.
```

## Failure Modes

- It summarizes the whole article instead of one idea.
- It becomes a generic stock illustration.
- It contains too much text.
- It copies an example composition.
- It is decorative but not explanatory.
