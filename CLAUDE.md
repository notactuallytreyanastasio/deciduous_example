# Project Instructions

## Decision Graph Workflow

**THIS IS MANDATORY. Log decisions IN REAL-TIME, not retroactively.**

### The Core Rule

```
BEFORE you do something -> Log what you're ABOUT to do
AFTER it succeeds/fails -> Log the outcome
ALWAYS -> Sync frequently so the graph updates
```

### Behavioral Triggers - MUST LOG WHEN:

| Trigger | Log Type | Example |
|---------|----------|---------|
| User asks for a new feature | `goal` | "Add dark mode" |
| Choosing between approaches | `decision` | "Choose state management" |
| About to write/edit code | `action` | "Implementing Redux store" |
| Something worked or failed | `outcome` | "Redux integration successful" |
| Notice something interesting | `observation` | "Existing code uses hooks" |

### Quick Commands

```bash
deciduous add goal "Title" -c 90
deciduous add decision "Title" -c 75
deciduous add action "Title" -c 85
deciduous link FROM TO -r "reason"
deciduous serve   # View live
deciduous sync    # Export for static hosting
```

### Session Start Checklist

Every new session, run `/context` or:

```bash
deciduous nodes    # What decisions exist?
deciduous edges    # How are they connected?
git status         # Current state
git log -10        # Recent commits
```
