# Architecture

## Goal

`general-illustrations` is a provider-neutral image generation layer for
agent-authored visual assets. It should let one recipe target several providers
without rewriting product logic for each model.

It also makes skill authoring structured. The generated Codex skill is still a
Markdown directory, but the source is JSON data that can be validated, stored,
served through an API, and rendered deterministically.

## Layers

```text
User request / Codex skill
  -> recipe selection
  -> style selection
  -> composition pattern selection
  -> provider-specific prompt adapter
  -> ImageGenerationRequest
  -> ImageProvider
  -> GeneratedImage
```

## Crates

### `general-illustrations-core`

Defines the stable API:

- `ImageGenerationRequest`
- `GeneratedImage`
- `ImageProvider`
- provider ids
- aspect ratio and output format types
- provider-neutral errors

This crate should not know about MiniMax, Codex, Gemini, or any provider's JSON
shape.

### `general-illustrations-minimax`

Implements `ImageProvider` for MiniMax:

- sends requests to `/v1/image_generation`
- maps `aspect_ratio`
- requests base64 output
- decodes `data.image_base64`
- normalizes `base_resp.status_code` errors

### `general-illustrations-cli`

Provides a small CLI for testing provider adapters and generating images from
prompt files.

It also provides:

- `skill validate`: validate a JSON `SkillSpec`
- `skill render`: render a JSON `SkillSpec` into a Codex skill directory

### `general-illustrations-skill-spec`

Defines the JSON authoring schema for generated skills:

- skill metadata
- style DNA
- references
- workflow
- styles
- composition patterns
- prompt templates
- QA rules
- example assets

### `general-illustrations-skill-renderer`

Turns a validated `SkillSpec` into:

- `SKILL.md`
- `agents/openai.yaml`
- `references/style-dna.md`
- `references/styles.md`
- `references/composition-patterns.md`
- `references/prompt-template.md`
- `references/qa-checklist.md`

The renderer keeps generated output deterministic so generated diffs can be
reviewed reliably.

## Provider Rules

- Keep auth inside provider adapters.
- Do not log API keys.
- Keep recipes provider-neutral.
- Let provider adapters own quirks such as prompt limits, response formats,
  model names, and error codes.
- Prefer explicit provider ids over stringly-typed branching in callers.

## Codex / CodeIce

Codex image generation is currently tool-backed in the Codex runtime rather than
a normal HTTP provider in this repo. Treat it as a first-class provider target in
the design, but do not fake an adapter until there is a callable API surface.

The skill can still use Codex directly because Codex has access to `image_gen`.
The Rust library is responsible for external providers and reusable API shape.
