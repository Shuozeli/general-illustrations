# Codelab: Generate A Skill From JSON

This repo treats Markdown skills as generated artifacts. Agents should edit the
JSON spec first, then use the Rust CLI to validate and render the Codex skill
directory.

## Source Of Truth

The current skill spec is:

```text
specs/general-illustrations.json
```

The generated skill directory is:

```text
skill/general-illustrations/
```

Do not hand-edit `skill/general-illustrations/SKILL.md` or `references/*.md`
for durable changes. Those files are overwritten by `skill render`.

## 1. Edit The JSON Spec

Use the existing spec shape:

- `name`, `display_name`, `short_description`, `description`, `default_prompt`
- `positioning`
- `style_dna`
- `references`
- `workflow`
- `styles`
- `composition_patterns`
- `prompt_templates`
- `qa`
- `examples`

Rules:

- `name` and all ids must be lowercase kebab-case.
- `tags` must be lowercase kebab-case and should use existing gallery tags
  where possible.
- `default_prompt` must include `$<skill-name>`.
- Keep visual content in JSON fields, not in Markdown files.
- Keep examples as assets. The JSON should point to them by path.

## 2. Validate

Run:

```bash
CARGO_INCREMENTAL=0 cargo run --release -p general-illustrations-cli -- \
  skill validate \
  --spec specs/general-illustrations.json
```

Expected output:

```text
valid: general-illustrations
```

Validation catches bad schema versions, invalid ids, missing required sections,
duplicate ids, and a default prompt that does not mention the skill.

## 3. Render To A Scratch Directory

Render outside the repo first:

```bash
rm -rf /tmp/general-illustrations-skill
CARGO_INCREMENTAL=0 cargo run --release -p general-illustrations-cli -- \
  skill render \
  --spec specs/general-illustrations.json \
  --out /tmp/general-illustrations-skill \
  --copy-assets
```

Inspect:

```bash
find /tmp/general-illustrations-skill -maxdepth 3 -type f | sort
sed -n '1,180p' /tmp/general-illustrations-skill/SKILL.md
```

The renderer writes:

- `SKILL.md`
- `agents/openai.yaml`
- `references/style-dna.md`
- `references/styles.md`
- `references/composition-patterns.md`
- `references/prompt-template.md`
- `references/qa-checklist.md`
- `assets/examples/*` when `--copy-assets` is set

## 4. Render Into The Repo

When the scratch output looks right:

```bash
CARGO_INCREMENTAL=0 cargo run --release -p general-illustrations-cli -- \
  skill render \
  --spec specs/general-illustrations.json \
  --out skill/general-illustrations
```

This updates Markdown and YAML generated files. It does not copy assets unless
`--copy-assets` is set.

## 5. Review And Test

Run:

```bash
cargo fmt --all
CARGO_INCREMENTAL=0 cargo test --workspace --release
CARGO_INCREMENTAL=0 cargo clippy --workspace --all-targets -- -D warnings
```

Review generated diffs:

```bash
git diff -- specs/general-illustrations.json skill/general-illustrations
```

The acceptable diff should show the JSON source change and the corresponding
generated Markdown/YAML change. If only Markdown changed, move that change back
into JSON and render again.

## Agent Contract

When another agent changes this skill:

1. Update `specs/general-illustrations.json`.
2. Run `skill validate`.
3. Render to `/tmp` and inspect.
4. Render to `skill/general-illustrations`.
5. Run Rust tests and clippy.

Markdown skills still exist because Codex consumes `SKILL.md`; they are not the
authoring format for this project.
