# Codelabs

## Structured Skill Generation

See [Codelab: Generate A Skill From JSON](codelabs/skill-generation.md).

## Generate With MiniMax

Create `prompt.txt`:

```text
16:9 simple doodle explainer for a short tech video.
Pure white background, black marker-like lines, one orange accent and one blue
accent.

Topic: LSM Tree.
Scene: write cards fall into MemTable, flush into SSTables, then compaction
merges levels.
Labels only: write, MemTable, flush, SSTable, compact.
```

Run:

```bash
export MINIMAX_API_KEY=...
cargo run --release -p general-illustrations-cli -- generate \
  --provider minimax \
  --prompt-file prompt.txt \
  --output-format jpeg \
  --output-dir out \
  --output-prefix lsm-doodle
```

The CLI prints the generated file path.
