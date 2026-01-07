---
description: Commit all staged/unstaged changes and push directly to master. Use for quick iterations when not using feature branches.
allowed-tools: Bash
---

# Commit and Push to Master

Quick commit and push workflow for direct master branch development.

## Instructions

1. **Check current state**:
   - Run `git status` to see all changes
   - Run `git diff` to review unstaged changes
   - Run `git diff --cached` to review staged changes

2. **Stage all changes**:
   - Run `git add -A` to stage all changes

3. **Create commit**:
   - Analyze the changes to understand what was modified
   - Create a concise commit message (1-2 sentences) focusing on the "why"
   - Use this format:
   ```
   git commit -m "$(cat <<'EOF'
   [Commit message here]

   🤖 Generated with [Claude Code](https://claude.com/claude-code)

   Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>
   EOF
   )"
   ```

4. **Push to master**:
   - Run `git push origin master`

5. **Report result**:
   - Show the commit hash and message
   - Confirm push succeeded

## Safety Checks

- Do NOT commit files that likely contain secrets (.env, credentials.json, etc.)
- Warn if committing more than 20 files at once
- If there are no changes to commit, report that and exit

## User Input

$ARGUMENTS
