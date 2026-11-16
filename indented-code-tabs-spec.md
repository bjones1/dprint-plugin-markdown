# Indented Code Block Tab Support Specification

## Overview

This specification defines a new configuration option `indentedCode.useTabs` that allows indented code blocks (4-space style) to be written using tab characters instead of spaces.

## Motivation

- **User preference**: Some projects mandate tab indentation for all code, including markdown
- **Consistency**: Projects using tabs elsewhere need consistency in indented code blocks
- **CommonMark compliance**: The spec explicitly allows tabs for indented code blocks at 4-column boundaries
- **Existing demand**: Issue #242 and PR #253 demonstrate user need

## Configuration

### New Option

```typescript
{
  "indentedCode.useTabs": boolean  // default: false
}
```

When `true`, indented code blocks use a single tab character (U+0009) for the 4-column structural indentation instead of four spaces (U+0020).

### Scope

Applies **only** to indented code blocks (4-space/1-tab style without fence markers). Does **not** affect:
- Fenced code blocks (``` or ~~~) — controlled by `codeBlock.*` options
- List item continuation indentation — remains spaces per `list.indentKind`
- Block quote indentation — remains spaces
- Content within code blocks — only the structural 4-column indent is converted

### Interaction with Other Options

- Independent of global configuration (does not inherit)
- `codeBlock.useTabs`: controls tabs *within* fenced code content, not markdown indentation
- `html.useTabs`: controls HTML block content, unrelated
- `list.indentKind`: does not affect list indentation

## Behavior

### Basic Conversion

**Input:**
```markdown
Text before.

    const x = 1;
    const y = 2;
```

**Output (useTabs: true):**
```markdown
Text before.

	const x = 1;
	const y = 2;
```

### Internal Indentation Preserved

Content within the code block preserves relative indentation using spaces:

**Input:**
```markdown
    function example() {
      return true;
    }
```

**Output (useTabs: true):**
```markdown
	function example() {
	  return true;
	}
```

The 4-column code block indent becomes a tab. The additional 2-space internal indent remains spaces.

### Blank Lines

Blank lines within code blocks have no indentation (matching current behavior):

**Input:**
```markdown
    line 1

    line 3
```

**Output (useTabs: true):**
```markdown
	line 1

	line 3
```

### Code Blocks in Lists

The 4-column code block indentation converts to a tab, added to list continuation indentation (which remains spaces):

**Input:**
```markdown
- Item text

      code line 1
      code line 2
```

**Output (useTabs: true):**
```markdown
- Item text

  	code line 1
  	code line 2
```

The 2 spaces provide list continuation indentation. The tab provides the 4-column code block indentation.

### Mixed Indentation Input

**Input:**
```markdown
Text.

    line 1 (4 spaces)
	line 2 (1 tab)
      line 3 (6 spaces)
```

**Output (useTabs: true, normalized):**
```markdown
Text.

	line 1
	line 2
	  line 3
```

All lines normalize to the configured style. Line 3's 6 spaces become 1 tab + 2 spaces (4-column code indent + 2-space internal indent).

### Edge Case: Conversion to Fenced Blocks

When dprint converts indented code blocks to fenced blocks to avoid ambiguity (e.g., after a list), `indentedCode.useTabs` has no effect:

**Input:**
```markdown
- list item

    code here
```

**Output (both useTabs values):**
```markdown
- list item

```
code here
```
```

## Terminology

**Indented code blocks**: Code blocks marked by 4 spaces or 1 tab at the line start (no fence markers). Includes code at document level, in block quotes, and in list items.

**4-column structural indentation**: The indentation that marks a block as code (distinct from internal code indentation or container indentation).

## Design Decisions

1. **Input normalization**: Input is always normalized to the configured style (4 spaces → 1 tab when `useTabs: true`, 1 tab → 4 spaces when `useTabs: false`)

2. **List interaction**: Tab replaces only the 4-column code block indentation. List continuation indentation remains spaces.

3. **CommonMark compatibility**: While CommonMark supports tabs in indented code blocks, users should verify compatibility with tools outside the CommonMark ecosystem.

## Implementation

Primary changes in `src/generation/generate.rs`, function `gen_code_block`. The function uses `indent_level` (currently 4 for indented code blocks) and applies it via `with_indent_times`. Implementation should check `context.configuration.indented_code_use_tabs` and output `"\t"` instead of `"    "` when appropriate.

Configuration: Add `indented_code_use_tabs: bool` (default `false`) to `src/configuration/types.rs`, `builder.rs`, and `resolve_config.rs`.

## Testing

1. **Oracle test**: Format CommonMark corpus with `useTabs: true`, then with `useTabs: false`. Parse both with `pulldown-cmark` and compare AST structure (node types, content). Ignore metadata (positions, offsets).

2. **Spec tests** in `tests/specs/CodeBlocks/`:
   - Basic indented code blocks
   - Code with internal indentation
   - Code in lists
   - Code in block quotes
   - Nested containers (code in lists in block quotes)
   - Mixed tab/space input
   - Blank lines
   - Conversion to fenced blocks

3. **Round-trip test**: Format with `useTabs: true` → parse → format with `useTabs: false` → verify AST equivalence

## Success Criteria

1. `indentedCode.useTabs: true` converts all indented code blocks to use tabs for 4-column structural indentation
2. `indentedCode.useTabs: false` (default) maintains current behavior (4 spaces)
3. Oracle test passes (tabs ↔ spaces produce equivalent ASTs)
4. All existing tests pass
5. New spec tests cover listed cases

## References

- [CommonMark Spec - Indented Code Blocks](https://spec.commonmark.org/0.31.2/#indented-code-blocks)
- [CommonMark Spec - Tabs](https://spec.commonmark.org/0.31.2/#tabs)
- [Issue #242](https://github.com/dprint/dprint-plugin-markdown/issues/242)
- [PR #253](https://github.com/dprint/dprint-plugin-markdown/pull/253)
