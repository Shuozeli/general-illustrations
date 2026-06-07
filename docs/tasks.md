# Tasks

## Done

- Create recipe-router Codex skill.
- Add style packs for technical, review, article, and cartoon explainer visuals.
- Add provider-neutral Rust core API.
- Add MiniMax Rust provider adapter.
- Add CLI for provider-backed generation.
- Add GitHub Actions CI.

## Next

- Add prompt adapter crate for recipe + style -> provider-specific prompt.
- Add MiniMax prompt shortening rules for the 1500 character limit.
- Add fixture prompts for LSM Tree and cloud structured editing.
- Add integration test gated behind `MINIMAX_API_KEY`.
- Add provider contract tests with mocked MiniMax responses.
- Add formal Codex/CodeIce adapter only if a stable API is available.
- Add Gemini adapter when an official image API path is selected.

## Open Questions

- Should generated assets be stored in a separate examples repo or kept out of
  git entirely?
- Should the public package expose async providers or keep blocking providers
  for CLI simplicity?
- Should recipes live in Markdown only, Rust data structures only, or both?
