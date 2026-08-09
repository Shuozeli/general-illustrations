<!-- agent-updated: 2026-08-09T00:00:00Z -->
# Tasks

## Done

- Add `CategorySpec` and give every style a required `category`; group `styles.md`
  by category (5 categories: article-docs, video-light, video-character-comic,
  video-meme-motion, literary-period).
- Add first-class `RecipeSpec` (recipe = category + style + default composition +
  prompt template + provider prompts) with referential-integrity validation; 14
  recipes rendered to `references/recipes.md`.
- Add Gemini provider set: per-recipe Gemini (Imagen web/CDP) prompts that stay
  faithful to the recipe style (no global photorealistic/cinematic wrapper),
  `agents/gemini.yaml`, and `references/providers/gemini.md` with the CDP contract.
- Add golden test that renders the committed spec and diffs against
  `skill/general-illustrations`, plus a check that no Gemini prompt reintroduces a
  photoreal wrapper (Berkshire regression guard).
- Wire the canonical CDP callers (`meitou_weekly/scripts/gemini_image_gen_cdp.py`
  and `.mjs`) to `--recipe <id>` / per-item `recipe`: they read the recipe's Gemini
  prompt from the spec and fill `{scene}`; the hardcoded photoreal wrapper is
  deleted and no-recipe prompts are sent verbatim.
- Regenerate the demo gallery from the spec: `gallery/build_gallery.py` writes
  `gallery/gallery.json` (5 categories, 14 recipes, provider chips, mapped example
  images) and `gallery/index.html` now fetches it and renders grouped-by-category
  cards -- no more hand-written HTML that drifts from the recipes. finance-news
  showcases a real ChatGPT-generated recipe-faithful image.
- Add ChatGPT as a second provider set: optional per-recipe `providers.chatgpt`
  (falls back to `gemini`), `agents/chatgpt.yaml`, `references/providers/chatgpt.md`,
  and a CDP driver `meitou_weekly/scripts/chatgpt_image_gen_cdp.py` that drives the
  logged-in ChatGPT web image-gen on the SAME alienware Chrome. Verified e2e: the
  `finance-news` recipe produced a recipe-faithful yellow-worker-comic image
  (1672x941). ChatGPT images come from `chatgpt.com/backend-api/estuary/content`
  (same-origin, canvas capture works), with an element-screenshot fallback.

- Create recipe-router Codex skill.
- Add style packs for technical, review, article, and cartoon explainer visuals.
- Add provider-neutral Rust core API.
- Add MiniMax Rust provider adapter.
- Add CLI for provider-backed generation.
- Add JSON skill spec, validator, renderer, and codelab.
- Add GitHub Actions CI.
- Add coupon-worker-comic recipe for action-first coupon and app-deal video frames.
- Add mambo-meme-comic recipe for original Chinese short-video meme hooks in technical explainers.
- Add honglou-retro-flat-anime recipe for Hongloumeng retro TV anime video keyframes.
- Document that all image recipes should be authored in `specs/general-illustrations.json` first.

## Next

- Add prompt adapter crate for recipe + style -> provider-specific prompt.
- Add MiniMax prompt shortening rules for the 1500 character limit.
- Add fixture prompts for LSM Tree and cloud structured editing.
- Add integration test gated behind `MINIMAX_API_KEY`.
- Add provider contract tests with mocked MiniMax responses.
- Add formal Codex/CodeIce adapter only if a stable API is available.
- Add a formal HTTP Gemini adapter when an official image API path is selected
  (the current set targets the CDP web surface).
- Regenerate/delete the stale `libcli/earnings_call_cli/_*_gemimg.mjs` one-off
  copies that still embed the old hardcoded photoreal wrapper.

## Open Questions

- Should generated assets be stored in a separate examples repo or kept out of
  git entirely?
- Should the public package expose async providers or keep blocking providers
  for CLI simplicity?
- Should the skill generation API accept only JSON, or also expose a Protobuf
  endpoint after the schema stabilizes?
