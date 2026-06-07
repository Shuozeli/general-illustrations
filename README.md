# general-illustrations

Provider-neutral illustration recipes and image generation adapters for agents.

`general-illustrations` separates three concerns:

- Recipes: what kind of image should be made.
- Prompt adapters: how a recipe becomes a provider-specific prompt.
- Providers: how an image request is sent to MiniMax, Codex/CodeIce, Gemini, or
  another backend.

The first implemented provider adapter is MiniMax. Codex/CodeIce currently
remains a tool-backed provider used by Codex itself, so the Rust API treats it
as a provider target to support without pretending it has the same HTTP API.

## What This Improves

This project started from studying `ian-xiaohei-illustrations`, but it is not a
single-character or single-style clone.

The important improvement is that illustration knowledge is now broken into
structured, reusable pieces:

- `SkillSpec`: the skill definition, workflow, references, QA, and metadata.
- `StyleSpec`: one selectable visual style, such as `simple-doodle` or
  `technical-minimal`.
- `CompositionPatternSpec`: one visual structure, such as workflow, system
  slice, before/after, or comic panels.
- `PromptTemplateSpec`: reusable prompt shapes for generation and editing.
- Provider adapters: MiniMax, Codex/CodeIce, Gemini, or future image backends.

So the project is no longer bound to one fixed image identity, one character, one
composition, or one model. A user request can choose the right recipe, style,
composition pattern, and provider independently.

Markdown skills still exist because Codex consumes `SKILL.md`, but Markdown is
now generated from JSON. The source of truth is structured data that can later be
edited by a CLI, REST API, UI, database record, or protobuf service.

## Workspace

```text
general-illustrations/
├── crates/
│   ├── general-illustrations-core/     Provider-neutral request/response API
│   ├── general-illustrations-minimax/  MiniMax image_generation adapter
│   ├── general-illustrations-skill-spec/      JSON skill schema and validation
│   ├── general-illustrations-skill-renderer/  JSON-to-skill renderer
│   └── general-illustrations-cli/             CLI for generation and skills
├── specs/general-illustrations.json    Source of truth for the Codex skill
├── skill/general-illustrations/        Generated Codex skill and references
└── docs/                               Design notes and roadmap
```

## CLI

Generate an image with MiniMax:

```bash
export MINIMAX_API_KEY=...
cargo run --release -p general-illustrations-cli -- generate \
  --provider minimax \
  --prompt-file prompt.txt \
  --aspect-ratio 16:9 \
  --output-format jpeg \
  --output-dir out \
  --output-prefix lsm-tree
```

List provider adapters:

```bash
cargo run --release -p general-illustrations-cli -- providers
```

## Skill

The skill authoring format is JSON:

```text
specs/general-illustrations.json
```

Validate it:

```bash
cargo run --release -p general-illustrations-cli -- skill validate \
  --spec specs/general-illustrations.json
```

Render the generated Codex skill:

```bash
cargo run --release -p general-illustrations-cli -- skill render \
  --spec specs/general-illustrations.json \
  --out skill/general-illustrations
```

The generated Codex skill lives at:

```text
skill/general-illustrations/
```

Do not hand-edit generated Markdown for durable changes. Edit the JSON spec and
render again.

The skill intentionally mirrors the structure of `ian-xiaohei-illustrations`:

- `SKILL.md`: activation, workflow, shot list, generation, QA, delivery.
- `references/style-dna.md`: shared visual DNA.
- `references/styles.md`: prebuilt visual styles.
- `references/composition-patterns.md`: structure types and originality rules.
- `references/prompt-template.md`: prompt templates.
- `references/qa-checklist.md`: generation QA and iteration rules.
- `assets/examples/`: calibration images only, not templates to copy.

The main difference from the Xiaohei skill is that this skill does not bind to a
single IP, character, composition, model, or visual style. It adds multiple
prebuilt styles such as `simple-doodle`, `whiteboard-comic`, `sticker-layer`,
`technical-minimal`, and `soft-clay-cartoon`, and those styles are data in
`specs/general-illustrations.json` rather than hand-written Markdown.

## Provider Boundary

The Rust API intentionally does not bake provider quirks into recipes.

- MiniMax needs short prompts and uses `image_base64` responses.
- Codex/CodeIce follows structured prompts well but is currently exposed as a
  Codex tool, not as this repo's HTTP provider.
- Gemini Web can reuse recipe prompts manually today; a formal adapter should be
  added only when there is a stable API path.

Recipes should stay provider-neutral. Provider adapters should translate request
shape, prompt length, response format, auth, and errors.

## Docs

- [Architecture](docs/architecture.md)
- [Skill Generation Design](docs/skill-generation-design.md)
- [Skill Generation Codelab](docs/codelabs/skill-generation.md)
- [Tasks](docs/tasks.md)

## Shuozeli Open Source Management

Local project states:

- `thirdparty/`: upstream clones. Do not push changes there.
- `shuozeli/_wip/`: local-only experiments without public remotes.
- `shuozeli/<category>/<repo>`: active Shuozeli-owned repos.
- `~/.pidx/pidx.toml`: source of truth for public index categories and repo
  descriptions.
- `shuozeli/meta/Shuozeli/README.md`: generated by `pidx`; do not hand-edit.

Promotion path:

1. Start in `_wip`.
2. Define scope, README, docs, license, and repo structure.
3. Create the public `Shuozeli/<repo>` remote.
4. Move the local clone into the matching category directory.
5. Add the repo to `~/.pidx/pidx.toml`.
6. Regenerate the public index with `pidx sync` and `pidx index`.

## Boundaries

- Do not copy Ian's "小黑" IP as Shuozeli's own visual identity.
- If adapting MIT-licensed text or examples, preserve attribution.
- The Shuozeli version should define its own visual language before becoming a
  public skill.
