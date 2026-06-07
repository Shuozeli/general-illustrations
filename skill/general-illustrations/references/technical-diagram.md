# Recipe: Technical Diagram

## Purpose

Explain a system, boundary, data flow, state machine, API, or architecture
decision without requiring the reader to parse prose first.

## Shot List Fields

- Scope: subsystem or concept covered.
- Entities: services, stores, clients, queues, workers, data structures.
- Flow: data/control direction and important sync/async boundaries.
- Ownership: who owns state, validation, writes, and retries.
- Risk or constraint: optional highlighted limitation.
- Style: usually `technical-minimal` or `clean-docs`.

## Diagram Types

- Boundary diagram: boxes and ownership zones.
- Dataflow diagram: arrows, payload names, stores.
- State diagram: states, transitions, invalid paths.
- RPC contract visual: caller, service, method, request, response, errors.
- Old/new diagram: local files/compile vs cloud struct/RPC editing.

## Prompt Pattern

```text
Generate one clean 16:9 technical diagram.

System concept:
{concept}

Entities:
{entities}

Flow:
{flow}

Highlight:
{boundary, risk, state owner, or key change}

Style:
{chosen style pack}

Constraints:
Make it visually precise and readable. Use sparse labels. Do not invent
components not present in the source. Do not make it a decorative illustration
when the goal is architecture clarity.
```

## Failure Modes

- It invents architecture.
- It hides the key boundary.
- It uses vague labels like "system" everywhere.
- It looks like a marketing graphic instead of a technical explanation.
- It crams multiple unrelated diagrams into one image.
