# Svelte 5 TypeScript Best Practices

## Context API Type Safety

When using Svelte's Context API with TypeScript, follow these patterns:

### Problem

```typescript
// ❌ This causes TypeScript errors
let sidebarState = getSidebarMainState(); // Type: unknown
```

### Solution

```typescript
// ✅ Explicit typing with type assertion
const sidebarState: SidebarMainState | undefined = getSidebarMainState() as
  | SidebarMainState
  | undefined;

// ✅ Use optional chaining for safe access
let currentSelectedOption = $derived(
  sidebarState?.currentSelectedOption || "default",
);
```

### Key Points

- Always type context values explicitly
- Use type assertions (`as Type`) when context might return `unknown`
- Use optional chaining (`?.`) for safe property access
- Provide fallback values for reactive expressions
- Use `const` for context values that don't change, `let` for reactive derived values
