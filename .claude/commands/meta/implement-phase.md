---
allowed-tools: Read, TodoWrite
argument-hint: [phase-number]
description: Guide implementation of a specific roadmap phase
---

## Roadmap Phase $1
- Phase details: !`grep -A 20 "## Phase $1" ROADMAP.md || echo "Phase $1 not found"`
- Completed items: !`grep -A 20 "## Phase $1" ROADMAP.md | grep -E "^- \[x\]" | wc -l`
- Pending items: !`grep -A 20 "## Phase $1" ROADMAP.md | grep -E "^- \[ \]" | wc -l`

## Task
Create implementation plan for Phase $1:

1. **Analyze phase items**: Extract all pending items from @ROADMAP.md
2. **Identify dependencies**: Order items by prerequisites
3. **Create todo list**: Break down items into actionable tasks
4. **Estimate effort**: Rough time estimates for planning
5. **Suggest starting point**: Best first item to tackle

## Implementation Strategy
- **Quick wins first**: High-impact, low-effort items
- **Foundation before features**: Core infrastructure before advanced features
- **Test as you go**: Don't let testing debt accumulate
- **Document incrementally**: Keep docs current with implementation

## Output
- **Ordered task list**: Items in dependency order
- **Effort estimates**: Rough sizing for each item
- **Starting recommendation**: Which item to begin with
- **Success criteria**: How to know phase is complete