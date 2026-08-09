<!-- agent-updated: 2026-08-09T00:00:00Z -->
# general-illustrations

Provider-neutral illustration recipes and image generation adapters for agents.

Gallery:

```text
https://shuozeli.github.io/general-illustrations/
```

`general-illustrations` separates four concerns:

- Categories: which use-context a style/recipe belongs to (pick a category first,
  then a concrete recipe inside it).
- Recipes: a first-class, selectable unit that binds a category, a style, a default
  composition, a prompt template, and a per-provider prompt set.
- Prompt adapters: how a recipe becomes a provider-specific prompt.
- Providers: how an image request is sent to MiniMax, Ark/Seedream, Codex/CodeIce,
  Gemini (web/CDP), ChatGPT (web/CDP), or another backend.

The first implemented provider adapter was MiniMax. Ark/Seedream is also
available through the Volcengine Ark Agent Plan image endpoint. Codex/CodeIce
currently remains a tool-backed provider used by Codex itself, so the Rust API
treats it as a provider target to support without pretending it has the same
HTTP API.

## What This Improves

This project started from studying `ian-xiaohei-illustrations`, but it is not a
single-character or single-style clone.

The important improvement is that illustration knowledge is now broken into
structured, reusable pieces:

- `SkillSpec`: the skill definition, workflow, references, QA, and metadata.
- `CategorySpec`: one use-context bucket (`article-docs`, `video-light`,
  `video-character-comic`, `video-meme-motion`, `literary-period`). Every style
  and recipe references a category so they are discoverable, not a flat list.
- `StyleSpec`: one selectable visual style, such as `simple-doodle` or
  `technical-minimal`.
- `CompositionPatternSpec`: one visual structure, such as workflow, system
  slice, before/after, or comic panels.
- `PromptTemplateSpec`: reusable prompt shapes for generation and editing.
- `RecipeSpec`: a first-class recipe binding `category + style + composition +
  prompt template`, plus `recommended_providers`, `default_variables`, and a
  `providers` prompt set (recipe-faithful Gemini prompt + optional ChatGPT
  override). Select one `recipe.id` instead of hand-pairing style/composition.
- Provider adapters: MiniMax, Ark/Seedream, Codex/CodeIce, Gemini, ChatGPT, or
  future image backends.

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
│   ├── general-illustrations-ark/      Ark/Seedream image adapter
│   ├── general-illustrations-skill-spec/      JSON skill schema and validation
│   ├── general-illustrations-skill-renderer/  JSON-to-skill renderer (+ golden test)
│   └── general-illustrations-cli/             CLI for generation and skills
├── specs/general-illustrations.json    Source of truth (categories, styles, recipes)
├── skill/general-illustrations/        Generated Codex skill, references, provider sets
├── gallery/                            Data-driven demo page (build_gallery.py + gallery.json)
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

Generate an image with Ark / Seedream:

```bash
export DOUBAO_ARK_AGENT_PLAN_API_KEY=ark-...
cargo run --release -p general-illustrations-cli -- generate \
  --provider ark \
  --prompt-file prompt.txt \
  --aspect-ratio 16:9 \
  --output-format png \
  --output-dir out \
  --output-prefix seedream-sample
```

List provider adapters:

```bash
cargo run --release -p general-illustrations-cli -- providers
```

### Recipe-driven prompt rendering

Validate recipes and generate concrete prompts from JSON data:

```bash
cargo run --release -p general-illustrations-cli -- recipe prompt \
  --spec specs/general-illustrations.json \
  --recipe cartoon-explainer-lsm \
  --data /tmp/cartoon-explainer-lsm-data.json
```

Generate a JSON Schema for a recipe's required template keys:

```bash
cargo run --release -p general-illustrations-cli -- recipe schema \
  --spec specs/general-illustrations.json \
  --recipe cartoon-explainer-lsm \
  --out /tmp/cartoon-explainer-lsm.schema.json
```

The schema is derived from the selected recipe's prompt template placeholders
(curly-brace keys), so you can safely validate your JSON payload before rendering.

Example `/tmp/cartoon-explainer-lsm-data.json`:

```json
{
  "选择的预置风格：clean-docs / technical-minimal / review-minimal / editorial-sketch / product-explainer / simple-doodle / cartoon-explainer / honglou-retro-flat-anime / honglou-period-drama-painterly / coupon-worker-comic / yellow-worker-comic / mambo-meme-comic / story-infra-kafka-mambo-technical / whiteboard-comic / sticker-layer / soft-clay-cartoon": "cartoon-explainer",
  "正文配图主题": "LSM树的写入路径",
  "Workflow / 系统局部 / 前后对比 / 角色状态 / 概念隐喻 / 方法分层 / 地图路线 / 小漫画分镜 / 动作循环场景": "概念隐喻",
  "这张图要表达的核心意思": "先写入 MemTable，再 flush 到 L0，异步合并形成更稳健的查询路径",
  "具体画面：主体在哪里、正在发生什么、主要物件是什么、信息如何流动": "左边角色把新记录塞进圆筒，箭头引导到内存层与磁盘层",
  "元素1": "MemTable",
  "元素2": "SSTable",
  "元素3": "Compaction",
  "元素4": "读路径",
  "标注词1": "flush",
  "标注词2": "compact",
  "标注词3": "SSTable",
  "标注词4": "write path",
  "可选标注词5": ""
}
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
- `references/styles.md`: prebuilt visual styles, grouped by category.
- `references/composition-patterns.md`: structure types and originality rules.
- `references/prompt-template.md`: prompt templates.
- `references/recipes.md`: the recipe catalog (category -> style + composition +
  template + provider prompts).
- `references/providers/gemini.md`, `references/providers/chatgpt.md`: per-recipe
  provider prompt sets and CDP usage contracts.
- `references/qa-checklist.md`: generation QA and iteration rules.
- `agents/openai.yaml`, `agents/gemini.yaml`, `agents/chatgpt.yaml`: per-agent
  interface metadata.
- `assets/examples/`: calibration images only, not templates to copy.

The main difference from the Xiaohei skill is that this skill does not bind to a
single IP, character, composition, model, or visual style. It adds multiple
prebuilt styles such as `simple-doodle`, `whiteboard-comic`, `sticker-layer`,
`technical-minimal`, `yellow-worker-comic`, `mambo-meme-comic`,
`story-infra-kafka-mambo-technical`, and `soft-clay-cartoon`. Those styles are
data in `specs/general-illustrations.json` rather than hand-written Markdown.

## Provider Boundary

The Rust API intentionally does not bake provider quirks into recipes.

- MiniMax needs short prompts and uses `image_base64` responses.
- Ark/Seedream works best with shorter, direct visual prompts. Long recipe
  prompts can intermittently fail at the transport layer, so the adapter
  requests URL responses and downloads the image bytes instead of moving large
  base64 payloads through the JSON response.
- Codex/CodeIce follows structured prompts well but is currently exposed as a
  Codex tool, not as this repo's HTTP provider.
- Gemini Web and ChatGPT Web are driven over Chrome DevTools Protocol (CDP) today
  rather than an HTTP adapter. Each recipe ships a recipe-faithful prompt in its
  `providers` set (Gemini required, ChatGPT optional override falling back to
  Gemini) with a `{scene}` placeholder. These prompts MUST NOT reintroduce a
  global `photorealistic, cinematic` wrapper -- that overrides the recipe style
  and produces wrong-style output. See `references/providers/{gemini,chatgpt}.md`.
  Reference CDP drivers live in the dragb monorepo at
  `playground/yuanchenxi/meitou_weekly/scripts/{gemini,chatgpt}_image_gen_cdp.*`
  (`--recipe <id>` reads the recipe prompt from this spec).

Recipes should stay provider-neutral. Provider adapters should translate request
shape, prompt length, response format, auth, and errors.

## Gallery

The demo gallery is generated from the spec, not hand-authored, so it cannot
drift from the canonical recipes:

```bash
python3 gallery/build_gallery.py          # spec -> gallery/gallery.json
```

`gallery/index.html` fetches `gallery.json` at runtime and renders one card per
recipe, grouped by category, with a per-provider example strip
(Codex / Ark / Gemini / ChatGPT) so provider outputs can be compared in place.
Do not hand-edit `index.html`/`gallery.json`; edit the spec and re-run
`build_gallery.py`.

## Docs

- [Architecture](docs/architecture.md)
- [Gallery Design](docs/gallery-design.md)
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
