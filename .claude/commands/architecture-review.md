---
description: Evaluate feature/refactoring proposals against Principal Engineer checklist and local-first guidelines. Use before implementing new features, APIs, data models, or making refactoring decisions.
allowed-tools: Read, Grep, Glob, Task
---

# Architecture Review

Use this command when:
- Implementing a new feature that affects data flow, APIs, or system boundaries
- Making refactoring decisions that change component responsibilities
- Designing new APIs or modifying existing contracts
- Introducing new data storage patterns or sync mechanisms

**Do NOT use for**: small UI changes, bug fixes, styling, or isolated component updates.

## Instructions

First, read the evaluation criteria:
1. Read `/Users/sc/DF/Principal_Engineer_Critical_Evaluation_Checklist.md`
2. For any data/sync architecture, also read `/Users/sc/DF/LOCAL_FIRST_GUIDELINES.md`

## Workflow

1. **Understand the scope**: What is being proposed/implemented? Ask the user if not clear from the arguments below.

2. **Identify relevant sections**: Not all checklist sections apply to every task. Select only the relevant BLOCKER and non-BLOCKER sections based on what's being changed:
   - New API? -> Sections 5, 6
   - New data model? -> Sections 4, 7
   - New component/service? -> Sections 1, 2, 3
   - Refactoring? -> Sections 9, 11
   - Data/sync changes? -> Local-first guidelines

3. **Evaluate against criteria**: For each relevant section, assess:
   - Which criteria are met
   - Which criteria are at risk
   - Which red flags are present

4. **Report findings**: Provide a brief assessment in this format:

```
## Architecture Review: [Feature/Change Name]

### Scope
[What's being evaluated]

### Relevant Criteria
[Which sections apply]

### Assessment
- BLOCKER issues: [list or "None"]
- Warnings: [list or "None"]
- Recommendations: [list]

### Decision
[Proceed / Proceed with changes / Block until resolved]
```

5. **For local-first concerns**: If the change involves data storage, sync, or offline behavior, also evaluate against the 7 ideals in LOCAL_FIRST_GUIDELINES.md.

## User Input

$ARGUMENTS
