# Skill Generation Design

## Goal

Generate a Codex skill directory from structured data instead of hand-writing all
Markdown files.

The generated skill should look like the hand-authored
`skill/general-illustrations/` directory:

```text
skill-name/
├── SKILL.md
├── agents/openai.yaml
├── references/
│   ├── style-dna.md
│   ├── styles.md
│   ├── composition-patterns.md
│   ├── prompt-template.md
│   └── qa-checklist.md
└── assets/examples/
```

The source of truth should be a typed spec that can be sent over a REST API,
stored as JSON, or encoded with Protobuf.

## Improvement Over Hand-Written Skills

The reference illustration skill is hand-authored around one strong identity.
That is useful for a fixed visual brand, but it makes each new style or new
provider feel like another Markdown fork.

This project changes that boundary:

- The authoring unit is a JSON object, not a Markdown file.
- A style is a structured record, not a paragraph buried in instructions.
- A composition pattern is a structured record, independent from style.
- A provider is an adapter, independent from recipes and skill prose.
- The Codex skill directory is generated output.

This makes the project general. It is not bound to one character, one visual
style, one composition type, one prompt shape, or one image generation provider.
Future tools can create a `SkillSpec` from a REST request, database row, UI form,
or protobuf message and render the same skill layout deterministically.

## Non-Goals

- Do not replace Codex skills with a new runtime format.
- Do not make agents read raw JSON at invocation time.
- Do not generate images during skill generation.
- Do not couple skill generation to MiniMax, Codex/CodeIce, Gemini, or any image
  provider.
- Do not make one universal schema for every possible skill type on day one.

## Why Structured Specs

Markdown is good for Codex to read, but poor as the source of truth:

- hard to validate
- hard to diff semantically
- hard to update with UI forms
- hard to generate provider-specific variants
- easy to forget required sections

A structured spec gives us:

- validation before generation
- reusable style and recipe libraries
- deterministic Markdown output
- REST API support for tools and UI
- versioned skill definitions

## Architecture

```text
JSON / Protobuf SkillSpec
  -> validate
  -> normalize
  -> render Markdown files
  -> render agents/openai.yaml
  -> copy or link assets
  -> validate generated skill
```

The generated Markdown remains the artifact that Codex consumes. The structured
spec is the authoring and API layer.

## Data Model

### SkillSpec

Top-level definition.

```json
{
  "schema_version": 1,
  "name": "general-illustrations",
  "display_name": "General Illustrations",
  "short_description": "Recipe-based article and technical illustrations",
  "description": "生成通用中文正文配图...",
  "default_prompt": "Use $general-illustrations ...",
  "positioning": {
    "title": "核心定位",
    "body": [
      "为中文文章、技术文档和短视频知识内容设计并生成 16:9 横版配图。"
    ]
  },
  "style_dna": {
    "one_liner": "清爽、可读、解释性强...",
    "must": ["16:9 横版。"],
    "colors": ["橙色：主路径。"],
    "text_rules": ["每张图最多 3-8 个短标注。"],
    "never": ["不要 PPT 信息图。"],
    "aesthetic_direction": ["要清楚、有趣、轻、有空间感。"]
  },
  "references": {
    "intro": "按任务需要读取，不要一次塞满上下文：",
    "items": [
      {
        "path": "references/styles.md",
        "description": "预置风格列表和选择规则。"
      }
    ]
  },
  "workflow": [],
  "styles": [],
  "composition_patterns": [],
  "prompt_templates": [],
  "qa": {},
  "examples": []
}
```

### StyleSpec

Defines one visual style.

```json
{
  "id": "whiteboard-comic",
  "tags": ["engineering", "teaching", "comic", "whiteboard"],
  "name": "whiteboard-comic",
  "use_when": "工程故事、bug 解释、before/after。",
  "drawing_rule": "白板线稿，2-4 格分镜，少量箭头和动作线。",
  "avoid": "长段手写字、太多格、太可爱。"
}
```

### CompositionPatternSpec

Defines one structure type.

```json
{
  "id": "workflow",
  "tags": ["flow", "process", "pipeline"],
  "name": "Workflow 流程",
  "use_when": "输入 -> 处理 -> 输出，AI 工作流。",
  "drawing_rule": "左侧输入，中间处理动作，右侧输出，橙色箭头表达主流向。"
}
```

### PromptTemplateSpec

Defines reusable prompt templates.

```json
{
  "id": "default-image",
  "tags": ["image", "general"],
  "name": "Default Image Prompt",
  "body": "Generate one standalone 16:9 horizontal..."
}
```

### QaSpec

Defines checks and iteration rules.

```json
{
  "must_pass": ["是 16:9 横版", "背景干净"],
  "failure_signals": ["画面像 PPT", "文字变成大段解释"],
  "iteration_rules": [
    {
      "problem": "太可爱",
      "fix": "强调 clean technical explainer、not cute、not mascot"
    }
  ]
}
```

### ExampleAssetSpec

Defines calibration examples.

```json
{
  "id": "lsm-whiteboard-comic",
  "source_path": "../skill/general-illustrations/assets/examples/02-whiteboard-comic-lsm.png",
  "target_path": "assets/examples/02-whiteboard-comic-lsm.png",
  "caption": "LSM Tree whiteboard comic calibration"
}
```

## REST API

### Validate Spec

```http
POST /v1/skills:validate
Content-Type: application/json
```

Request:

```json
{ "spec": { "...": "..." } }
```

Response:

```json
{
  "valid": true,
  "errors": []
}
```

### Render Skill

```http
POST /v1/skills:render
Content-Type: application/json
```

Request:

```json
{
  "spec": { "...": "..." },
  "format": "tar"
}
```

Response options:

- `application/gzip`: tarball of the generated skill directory
- JSON manifest with generated file paths and contents for editor use

### Generate Skill In Workspace

```http
POST /v1/workspaces/{workspace_id}/skills
Content-Type: application/json
```

Request:

```json
{
  "spec": { "...": "..." },
  "target_path": "skill/general-illustrations"
}
```

Response:

```json
{
  "skill_path": "skill/general-illustrations",
  "files": [
    "SKILL.md",
    "references/styles.md",
    "references/qa-checklist.md"
  ]
}
```

## Protobuf Sketch

```proto
syntax = "proto3";

package general_illustrations.skills.v1;

message SkillSpec {
  string schema_version = 1;
  string name = 2;
  string display_name = 3;
  string description = 4;
  string default_prompt = 5;
  Positioning positioning = 6;
  repeated WorkflowStep workflow = 7;
  repeated StyleSpec styles = 8;
  repeated CompositionPatternSpec composition_patterns = 9;
  repeated PromptTemplateSpec prompt_templates = 10;
  QaSpec qa = 11;
  repeated ExampleAssetSpec examples = 12;
}

message Positioning {
  string one_liner = 1;
  string core_purpose = 2;
  repeated string differences = 3;
}

message WorkflowStep {
  string title = 1;
  repeated string bullets = 2;
}

message StyleSpec {
  string id = 1;
  string name = 2;
  repeated string use_when = 3;
  repeated string visual_rules = 4;
  repeated string avoid = 5;
  string prompt_fragment = 6;
}

message CompositionPatternSpec {
  string id = 1;
  string name = 2;
  repeated string use_when = 3;
  string drawing_rule = 4;
}

message PromptTemplateSpec {
  string id = 1;
  string name = 2;
  string template = 3;
  repeated string variables = 4;
}

message QaSpec {
  repeated string must_pass = 1;
  repeated string failure_signals = 2;
  repeated IterationRule iteration_rules = 3;
}

message IterationRule {
  string problem = 1;
  string fix = 2;
}

message ExampleAssetSpec {
  string id = 1;
  string source_path = 2;
  string caption = 3;
  string style_id = 4;
}

message RenderSkillRequest {
  SkillSpec spec = 1;
  string output_format = 2; // "files", "tar", "zip"
}

message RenderedFile {
  string path = 1;
  bytes content = 2;
}

message RenderSkillResponse {
  repeated RenderedFile files = 1;
}
```

## Rendering Rules

The renderer should be deterministic:

- stable file order
- stable Markdown headings
- no timestamps in generated files
- no random example ordering
- no provider-specific text unless the spec includes it

Generated files:

- `SKILL.md` from positioning, workflow, delivery, and reference navigation
- `references/style-dna.md` from style DNA fields
- `references/styles.md` from `StyleSpec[]`
- `references/composition-patterns.md` from `CompositionPatternSpec[]`
- `references/prompt-template.md` from `PromptTemplateSpec[]`
- `references/qa-checklist.md` from `QaSpec`
- `agents/openai.yaml` from display metadata

## Validation Rules

- `name` must be lowercase kebab-case.
- `description` must include when to use the skill.
- `default_prompt` must mention `$skill-name`.
- At least one style is required.
- At least one prompt template is required.
- QA must include must-pass checks and failure signals.
- Example assets must exist if `copy_assets = true`.
- Generated skill must pass Codex skill validation.

## Implementation Plan

1. Add `general-illustrations-skill-spec` crate with serde/protobuf-compatible
   data types.
2. Add `general-illustrations-skill-renderer` crate that renders Markdown files.
3. Add CLI commands:
   - `skill validate --spec spec.json`
   - `skill render --spec spec.json --out skill/general-illustrations`
4. Add REST server only after the CLI renderer is stable.
5. Convert the current hand-authored `skill/general-illustrations` into a
   `specs/general-illustrations.json` fixture.
6. Add golden tests that render the fixture and compare generated Markdown.

## Open Questions

- Should Protobuf be the primary schema, with JSON generated from it, or should
  JSON Schema be primary?
- Should generated Markdown be committed, or should specs be committed and
  Markdown generated during install?
- Should assets be copied into generated skills or referenced by URI?
- How much provider-specific prompt data belongs in skill specs versus provider
  adapters?
