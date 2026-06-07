# Recipe: Code Review Visual

## Purpose

Make a review finding easier to understand: bug path, regression mechanism,
risk area, missing test, or before/after behavior.

## Shot List Fields

- Finding: concise bug/risk statement.
- Path: how the issue occurs.
- Impact: user-visible or system-visible consequence.
- Missing guard: check, validation, fallback, test, or invariant.
- Fix direction: optional, only if known.
- Style: usually `review-minimal` or `technical-minimal`.

## Visual Types

- Bug path: input -> branch -> bad state -> impact.
- Risk surface: changed module in center, affected callers around it.
- Before/after: old behavior vs new behavior.
- Test gap map: behavior matrix with uncovered cell.
- Invariant break: expected guard removed or bypassed.

## Prompt Pattern

```text
Generate one 16:9 code review visual.

Finding:
{finding}

Mechanism:
{bug path or risk surface}

Impact:
{impact}

Missing guard/test:
{guard or test gap}

Style:
{chosen style pack}

Constraints:
Keep labels short and concrete. Do not include large code blocks. Do not add
facts not present in the review. Make the risky transition obvious.
```

## Failure Modes

- It repeats the prose finding without adding visual clarity.
- It shows code text too small to read.
- It omits the bad transition.
- It overstates severity beyond the review evidence.
- It becomes a generic warning icon.
