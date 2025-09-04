---
allowed-tools: Read, Edit
argument-hint: [completed-item]
description: Update roadmap progress and suggest next priorities
---

## Current Roadmap Status
- Current roadmap: @ROADMAP.md
- Completed features: !`grep -E "^- \[x\]" ROADMAP.md | wc -l || echo "No completed items found"`
- Pending features: !`grep -E "^- \[ \]" ROADMAP.md | wc -l || echo "No pending items found"`

## Task
Update the ROADMAP.md file to reflect current development progress:

### If a specific item was provided ($1):
1. **Mark item as completed**: Find "$1" in ROADMAP.md and change `[ ]` to `[x]`
2. **Update dependencies**: Check if completing this item unlocks other items
3. **Suggest next steps**: Recommend what should be tackled next in the same phase

### General roadmap maintenance:
1. **Review current phase**: Are we on track? Any blockers?
2. **Update priorities**: Have any items become more or less important?
3. **Add new items**: Based on recent development, what's missing?
4. **Remove obsolete items**: Any items that are no longer relevant?

## Roadmap Analysis

### Current Phase Assessment:
- **Phase 1** (Core Foundation): How complete are we?
- **Phase 2** (Developer Experience): What's ready to start?
- **Blockers**: Any items preventing progress?
- **Quick wins**: Items that could be completed soon?

### Priority Suggestions:
Based on:
- **Current codebase state**: What exists now
- **Developer feedback**: Issues or requests received
- **Integration needs**: Dependencies between features
- **Time investment**: High-impact, low-effort items first

### Success Metrics Update:
Review the success metrics at the bottom of ROADMAP.md:
- Are they still relevant?
- How are we tracking against them?
- Do we need additional metrics?

## Updates to Make

1. **Mark completed items** with `[x]`
2. **Update phase descriptions** if scope has changed
3. **Reorganize priorities** based on current needs
4. **Add implementation notes** for complex items
5. **Update success metrics** with current measurements

## Output
Provide:
- **Summary of changes made** to the roadmap
- **Recommended next 3 items** to focus on
- **Updated timeline estimate** for current phase
- **Dependency analysis** - what's blocking what
- **Resource allocation** - time estimates for upcoming items