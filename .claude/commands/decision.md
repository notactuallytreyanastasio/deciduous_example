---
description: Manage decision graph - track algorithm choices and reasoning
allowed-tools: Bash(deciduous:*)
argument-hint: <action> [args...]
---

# Decision Graph Management

**Log decisions IN REAL-TIME as you work, not retroactively.**

## When to Use This

| You're doing this... | Log this type | Command |
|---------------------|---------------|---------|
| Starting a new feature | `goal` | `/decision add goal "Add user auth"` |
| Choosing between approaches | `decision` | `/decision add decision "Choose auth method"` |
| Considering an option | `option` | `/decision add option "JWT tokens"` |
| About to write code | `action` | `/decision add action "Implementing JWT"` |
| Noticing something | `observation` | `/decision add obs "Found existing auth code"` |
| Finished something | `outcome` | `/decision add outcome "JWT working"` |

## Quick Commands

Based on $ARGUMENTS:

### View Commands
- `nodes` or `list` -> `deciduous nodes`
- `edges` -> `deciduous edges`
- `graph` -> `deciduous graph`
- `commands` -> `deciduous commands`

### Create Nodes (with optional confidence)
- `add goal <title>` -> `deciduous add goal "<title>" -c 90`
- `add decision <title>` -> `deciduous add decision "<title>" -c 75`
- `add option <title>` -> `deciduous add option "<title>" -c 70`
- `add action <title>` -> `deciduous add action "<title>" -c 85`
- `add obs <title>` -> `deciduous add observation "<title>" -c 80`
- `add outcome <title>` -> `deciduous add outcome "<title>" -c 90`

### Create Edges
- `link <from> <to> [reason]` -> `deciduous link <from> <to> -r "<reason>"`

### Sync Graph
- `sync` -> `deciduous sync`

## Node Types

| Type | Purpose | Example |
|------|---------|---------|
| `goal` | High-level objective | "Add user authentication" |
| `decision` | Choice point with options | "Choose auth method" |
| `option` | Possible approach | "Use JWT tokens" |
| `action` | Something implemented | "Added JWT middleware" |
| `outcome` | Result of action | "JWT auth working" |
| `observation` | Finding or data point | "Existing code uses sessions" |

## Edge Types

| Type | Meaning |
|------|---------|
| `leads_to` | Natural progression |
| `chosen` | Selected option |
| `rejected` | Not selected (include reason!) |
| `requires` | Dependency |
| `blocks` | Preventing progress |
| `enables` | Makes something possible |

## The Rule

```
LOG BEFORE YOU CODE, NOT AFTER.
SYNC BEFORE YOU PUSH.
```
