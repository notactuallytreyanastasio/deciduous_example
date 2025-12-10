# Deciduous Roadmap

> Features and improvements planned for deciduous based on real-world usage.

---

## Git Integration

### Branch-Based Decision Groups
- Associate decision nodes with git branches
- Automatically group work/decisions by branch for cleaner viewing
- Enable better PR storytelling by filtering graph to branch-relevant nodes
- `deciduous branch link <branch-name>` to tag current nodes with branch context
- PR writeups auto-filter to show only decisions from that branch's work

### Git Log as Queryable Data
- Store git log entries as a data structure alongside decision nodes
- Cross-reference commits with decisions/actions
- Query: "what decisions led to commit X?"
- Query: "what commits resulted from decision Y?"
- Interleave git history with decision history for complete audit trail

---

## Context & Memory

### Persistent Context Recovery
- Make `/context` (or `context.md` generation) run automatically after memory compaction
- Ensure decision graph context is ALWAYS available, not just on-demand
- Hook into Claude Code's compaction events to auto-refresh context
- Never lose thread of work even across long sessions

### File & Line Queries
- Query decisions related to specific files: `deciduous query --file src/lib.rs`
- Query decisions related to line ranges: `deciduous query --file src/lib.rs --lines 10-50`
- Link decisions to code locations, not just concepts
- "Why does this code exist?" answerable from the graph

---

## Configuration & Setup

### CLAUDE.md Additive Mode
- Don't overwrite existing CLAUDE.md - append/merge deciduous instructions
- Detect existing content and add deciduous section if missing
- Preserve user's custom instructions while ensuring deciduous workflow is included
- `deciduous init --additive` flag (or make it default)

### Rules File Merging
- Same additive behavior for `.claude/settings.json` rules
- Check if rule names already exist before adding
- Merge rather than replace to preserve user customizations
- Warn if conflicts detected, let user resolve

---

## Model Comparison & Benchmarking

### Multi-Model Decision Runs
- Record which model made each decision (already have model info?)
- Re-run same prompts/goals with different models
- Compare decision paths: "How did Opus approach this vs Sonnet?"
- Benchmark metrics:
  - Decision count (more granular vs fewer big jumps)
  - Confidence distributions
  - Time to completion
  - Token usage
- Visualize decision trees side-by-side

### Reproducible Scenarios
- Save a "scenario" (starting state + goal) as replayable
- Run scenario against multiple models
- Compare outcomes and paths taken
- Useful for evaluating model upgrades or fine-tuning

---

## Visualization & UX

### Enhanced Graph Viewer
- Filter by branch/group
- Filter by date range
- Filter by node type
- Search within node titles/descriptions
- Zoom to specific subgraphs
- Timeline view (decisions over time)

### PR Integration
- GitHub Action to auto-generate decision graph for PRs
- Embed graph visualization in PR description
- Link nodes to specific commits/files changed

---

## CLI Improvements

### Query Language
```bash
# Find decisions about a topic
deciduous query "WASM"

# Find by type and confidence
deciduous query --type decision --confidence ">80"

# Find by file
deciduous query --file "src/lib.rs"

# Find by date
deciduous query --since "2025-12-01"
```

### Better Sync
- Auto-sync on every add/link (optional)
- Sync to multiple destinations (local JSON, remote, etc.)
- Real-time sync for live graph viewing during sessions

---

## Future Ideas (Brainstorm)

- [ ] Team collaboration - multiple contributors to same graph
- [ ] Decision templates - common patterns (feature, bugfix, refactor)
- [ ] Integration with issue trackers (link nodes to GitHub issues)
- [ ] Decision "replay" - step through historical decisions
- [ ] AI-generated summaries of decision paths
- [ ] Export to other formats (Mermaid, PlantUML, etc.)
- [ ] Slack/Discord notifications for key decisions
- [ ] Decision metrics dashboard

---

## Priority Order (Suggested)

1. **Branch-based groups** - Immediate value for PR workflow
2. **Context persistence** - Critical for long sessions
3. **CLAUDE.md additive mode** - Better onboarding experience
4. **File/line queries** - Connect decisions to code
5. **Git log integration** - Complete audit trail
6. **Multi-model comparison** - Evaluation & benchmarking

---

*This roadmap is itself tracked in deciduous. Meta.*
