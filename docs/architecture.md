# Architecture

## Goal

`general-illustrations` is a provider-neutral image generation layer for
agent-authored visual assets. It should let one recipe target several providers
without rewriting product logic for each model.

## Layers

```text
User request / Codex skill
  -> recipe selection
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
