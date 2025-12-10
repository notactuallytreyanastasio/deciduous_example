---
description: Recover context from decision graph and recent activity - USE THIS ON SESSION START
allowed-tools: Bash(deciduous:*, git:*)
argument-hint: [focus-area]
---

# Context Recovery

**RUN THIS AT SESSION START.** The decision graph is your persistent memory.

## Step 1: Query the Graph

```bash
# See all decisions (look for recent ones and pending status)
deciduous nodes

# See how decisions connect
deciduous edges

# What commands were recently run?
deciduous commands
```

## Step 2: Check Git State

```bash
git status
git log --oneline -10
git diff --stat
```

## After Gathering Context, Report:

1. **Current branch** and pending changes
2. **Recent decisions** (especially pending/active ones)
3. **Last actions** from git log and command log
4. **Open questions** or unresolved observations
5. **Suggested next steps**

---

## REMEMBER: Real-Time Logging Required

After recovering context, you MUST follow the logging workflow:

```
EVERY USER REQUEST -> Log goal/decision first
BEFORE CODE CHANGES -> Log action
AFTER CHANGES -> Log outcome, link nodes
BEFORE GIT PUSH -> deciduous sync
```

### Quick Logging Commands

```bash
deciduous add goal "What we're trying to do" -c 90
deciduous add action "What I'm about to implement" -c 85
deciduous add outcome "What happened" -c 95
deciduous link FROM TO -r "Connection reason"
deciduous sync  # Do this frequently!
```

---

## The Memory Loop

```
SESSION START
    |
Run /context -> See past decisions
    |
DO WORK -> Log BEFORE each action
    |
AFTER CHANGES -> Log outcomes, observations
    |
BEFORE PUSH -> deciduous sync
    |
PUSH -> Graph persists
    |
SESSION END -> Graph survives
    |
(repeat)
```

---

## Why This Matters

- Context loss during compaction loses your reasoning
- The graph survives - query it early, query it often
- Retroactive logging misses details - log in the moment
