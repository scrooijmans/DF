---
description: Research implementations using Context7 MCP and create an implementation plan for a given issue. Use for complex features that benefit from studying existing open-source solutions.
allowed-tools: Read, Grep, Glob, Task, WebSearch, WebFetch, mcp__plugin_context7_context7__resolve-library-id, mcp__plugin_context7_context7__get-library-docs
---

# Context7 Research and Plan

Use this command to research how other projects implement a feature, then create a detailed implementation plan based on those findings.

## When to Use

- Implementing a new feature where existing open-source implementations exist
- Solving a complex architectural problem that others have solved before
- Needing to understand best practices for a specific library or pattern
- Planning major refactoring based on industry patterns

## Instructions

### Phase 1: Understand the Issue

1. Parse the user's issue/feature request from `$ARGUMENTS`
2. Identify:
   - What capability is needed
   - What technologies/libraries are involved
   - What constraints exist (our stack: Svelte 5, SvelteKit, Tauri 2, TypeScript, Rust)

### Phase 2: Context7 Research

1. **Identify relevant libraries** to research:
   - Use `mcp__plugin_context7_context7__resolve-library-id` to find library IDs
   - Focus on libraries in our stack or comparable projects

2. **Fetch documentation** for each relevant library:
   - Use `mcp__plugin_context7_context7__get-library-docs` with specific topics
   - Look for implementation patterns, best practices, and examples

3. **Search for comparable implementations**:
   - Use WebSearch to find open-source projects that implement similar features
   - Look for projects using similar tech stacks (Tauri, Svelte, Rust)
   - Consider well-known applications: Grafana, GitButler, Zed, Obsidian

4. **Analyze implementations**:
   - Use WebFetch to read relevant documentation or code explanations
   - Identify common patterns across multiple implementations
   - Note trade-offs and alternatives

### Phase 3: Create Implementation Plan

Based on research findings, create a plan with this structure:

```markdown
## Implementation Plan: [Feature Name]

### Research Summary
- **Libraries researched**: [list]
- **Projects analyzed**: [list with links]
- **Key patterns identified**: [list]

### Recommended Approach
[Describe the chosen approach and why]

### Implementation Steps

1. **[Step name]**
   - Files to modify/create: [list]
   - Key changes: [description]
   - Based on: [which research informed this]

2. **[Step name]**
   ...

### File Changes Summary

| Action | File Path | Purpose |
|--------|-----------|---------|
| CREATE | path/to/file | description |
| MODIFY | path/to/file | description |

### Dependencies
- New packages needed: [list or "None"]
- Breaking changes: [list or "None"]

### Testing Strategy
- Unit tests: [what to test]
- Integration tests: [what to test]
- Manual verification: [what to check]

### Alternatives Considered
1. **[Alternative name]**: [why not chosen]
2. ...

### References
- [Link to relevant docs]
- [Link to example implementation]
```

### Phase 4: Save Documentation

If significant research was done, offer to save the findings to `docs/context7/[topic]/` for future reference.

## Example Usage

```
/context7-research-and-plan Implement drag-and-drop file upload with progress tracking
```

```
/context7-research-and-plan Add real-time cursor synchronization between chart panels
```

```
/context7-research-and-plan Implement undo/redo for data editing operations
```

## User Input

$ARGUMENTS
