## Summary

**Goal:** Build wc clone in Rust

Replicate core wc functionality: word count and line count (-l) for regular files. Keep it dependency-minimal. Will later compile to WASM for web UI.


## Key Decisions

### Choose argument parsing approach

**Observations:**

- WASM target constrains dependency choices

### Use std::env::args for CLI, separate core logic

**Observations:**

- WASM target constrains dependency choices

### Structure: lib.rs for core logic, main.rs for CLI

## Implementation

- Initialize Rust project with cargo
- Implement line count feature
- Create PR for rwc line count feature

## Outcomes

- Rust project initialized successfully (95% confidence)
- Line count and word count implemented successfully (95% confidence)
- PR #1 created successfully (95% confidence)

## Decision Graph

```dot
digraph DecisionGraph {
  rankdir=TB;
  node [fontname="Arial" fontsize=10];
  edge [fontname="Arial" fontsize=9];
  label="feat: implement rwc - a Rust wc clone";
  labelloc=t;
  fontsize=14;

  1 [label="[1] Build wc clone in Rust\\n(95%)" shape="house" fillcolor="#FFE4B5" style="filled"];
  2 [label="[2] Choose argument parsing approach\\n(80%)" shape="diamond" fillcolor="#E6E6FA" style="filled"];
  3 [label="[3] WASM target constrains dependency cho...\\n(90%)" shape="note" fillcolor="#DDA0DD" style="filled"];
  4 [label="[4] Use std::env::args for CLI, separate ...\\n(85%)" shape="diamond" fillcolor="#E6E6FA" style="filled"];
  5 [label="[5] Initialize Rust project with cargo\\n(95%)" shape="box" fillcolor="#90EE90" style="filled"];
  6 [label="[6] Rust project initialized successfully\\n(95%)" shape="ellipse" fillcolor="#87CEEB" style="filled"];
  7 [label="[7] Implement line count feature\\n(90%)" shape="box" fillcolor="#90EE90" style="filled"];
  8 [label="[8] Structure: lib.rs for core logic, mai...\\n(90%)" shape="diamond" fillcolor="#E6E6FA" style="filled"];
  9 [label="[9] Line count and word count implemented...\\n(95%)" shape="ellipse" fillcolor="#87CEEB" style="filled"];
  10 [label="[10] Create PR for rwc line count feature\\n(90%)" shape="box" fillcolor="#90EE90" style="filled"];
  11 [label="[11] PR #1 created successfully\\n(95%)" shape="ellipse" fillcolor="#87CEEB" style="filled"];

  1 -> 2 [style="solid" color="#333333"];
  2 -> 3 [style="solid" color="#333333"];
  3 -> 4 [style="solid" color="#333333"];
  2 -> 4 [style="solid" color="#333333"];
  4 -> 5 [style="solid" color="#333333"];
  5 -> 6 [style="solid" color="#333333"];
  6 -> 7 [style="solid" color="#333333"];
  1 -> 7 [style="solid" color="#333333"];
  7 -> 8 [style="solid" color="#333333"];
  4 -> 8 [style="solid" color="#333333"];
  7 -> 9 [style="solid" color="#333333"];
  8 -> 9 [style="solid" color="#333333"];
  9 -> 10 [style="solid" color="#333333"];
  10 -> 11 [style="solid" color="#333333"];
}
```

*Render with: `dot -Tpng graph.dot -o graph.png`*

## Test Plan

- [ ] Verify implementation
- [ ] Run test suite

## Decision Graph Reference

This PR corresponds to deciduous nodes: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11

