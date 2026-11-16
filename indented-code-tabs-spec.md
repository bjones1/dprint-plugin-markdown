# Indented Code Block Tab Support Specification

## Overview

This specification defines a new configuration option `indentedCode.useTabs` that controls whether indented code blocks (4-space style without fence markers) use tab characters or spaces for their structural indentation.

## Motivation

- **User preference**: Some projects mandate tab indentation for all code, including markdown
- **Consistency**: Projects using tabs elsewhere need consistency in indented code blocks
- **CommonMark compliance**: The [CommonMark spec](https://spec.commonmark.org/0.31.2/#tabs) explicitly allows tabs for indented code blocks at 4-column boundaries
- **Existing demand**: [Issue #242](https://github.com/dprint/dprint-plugin-markdown/issues/242) and [PR #253](https://github.com/dprint/dprint-plugin-markdown/pull/253) demonstrate user need for this feature

## Configuration

### New Option

Add to your `dprint.json` configuration:

```json
{
  "markdown": {
    "indentedCode.useTabs": true
  }
}
```

**Type**: `boolean`  
**Default**: `false`

When `true`, the formatter writes indented code blocks using a single tab character (U+0009) for the 4-column structural indentation instead of four space characters (U+0020).

When `false` (default), the formatter maintains current behavior and uses four spaces.

### Scope

This option applies exclusively to indented code blocks - blocks marked by 4-column indentation without fence delimiters (``` or ~~~).

**Not affected by this option:**
- **Fenced code blocks** (``` or ~~~) - these use existing `codeBlock.*` options
- **List item continuation indentation** - remains spaces per `list.indentKind`
- **Block quote marker indentation** - remains spaces
- **Content within code blocks** - only the structural 4-column indent marking the block as code is affected

**Rationale for scope limitations:**

Lists are excluded because list continuation indentation depends on marker width (e.g., `-` vs `10.`). Content does not always start at a 4-column boundary, making tab conversion ambiguous. Supporting this would require separate design work to handle variable-width markers. See design decision below for details.

Footnote definitions are excluded for the same reason - the body indentation depends on the footnote label width and wrapping behavior. The printer measures a tab as one column, which would cause wrapped footnote text to overflow the line width. This was discovered during [PR #253](https://github.com/dprint/dprint-plugin-markdown/pull/253) review.

### Interaction with Other Options

This option is independent and does not inherit from or affect other configuration:

- Does not inherit from any global `useTabs` setting
- `codeBlock.useTabs`: controls tabs *within* fenced code block content (passed to the code formatter), not markdown structural indentation
- `html.useTabs`: controls HTML block content indentation, unrelated to code blocks
- `list.indentKind`: controls list continuation indentation strategy (CommonMark vs PythonMarkdown), does not affect code block indentation

## Algorithm

### Core Logic

When formatting an indented code block:

1. **Identify the block type**: Determine whether the current code block is indented (4-column structural indent, no fence markers) or fenced (delimited by ``` or ~~~).

2. **Check conversion eligibility**: If the block is fenced, skip tab conversion regardless of `indentedCode.useTabs` value. Only indented code blocks are eligible for tab conversion.

3. **Apply indentation strategy**:
   - If `indentedCode.useTabs` is `false`: Write the 4-column structural indentation as four space characters (U+0020). This is the default behavior.
   - If `indentedCode.useTabs` is `true`: Write the 4-column structural indentation as a single tab character (U+0009).

4. **Preserve internal structure**: Content within the code block retains its original whitespace exactly as written. Tabs within the content remain tabs; spaces within the content remain spaces. The formatter only modifies the structural 4-column indent that marks the block as code, never the content itself.

5. **Handle blank lines**: A blank line is defined as a line containing only whitespace (spaces, tabs) or no characters. Blank lines within indented code blocks are written as completely empty lines (no whitespace characters). This matches existing formatter behavior and CommonMark semantics, which do not require blank lines in code blocks to maintain any indentation.

### Container Contexts

When an indented code block appears within a container (list item or block quote):

1. **List items**: The list item's continuation indentation is written first using spaces according to `list.indentKind`. Then the 4-column code block indentation is added using the strategy from step 3 above (tab if `useTabs` is true, spaces otherwise).

   Example: For a code block in a list with `useTabs: true`, a line would be: `<list continuation spaces><tab><code content>`. If the list continuation is 2 spaces, the output is: `  \t<code content>`.

2. **Block quotes**: The block quote marker (`>`) and its trailing space are written first. Then the 4-column code block indentation is added using the strategy from step 3 above.

   Example: With `useTabs: true`, a line would be: `> <tab><code content>`.

3. **Nested containers**: For code blocks in lists within block quotes (or other nested containers), each container's indentation is applied in order from outer to inner, with the code block's 4-column indent applied last according to the `useTabs` setting.

   Example: For code in a list in a blockquote with `useTabs: true`: `> <list continuation spaces><tab><code content>`.

### Conversion to Fenced Blocks

In certain contexts, dprint's formatter converts indented code blocks to fenced blocks to avoid parsing ambiguity. This is existing formatter behavior, not introduced by this specification.

**Known conversion cases:**
- **Immediately following a list**: Prevents the code from being parsed as part of the last list item
- **After certain block structures**: When the indentation would create structural ambiguity in how the block is parsed

When this conversion occurs, `indentedCode.useTabs` has no effect because the block is no longer an indented code block. The resulting fenced block follows standard fenced code block formatting rules.

**Note**: This behavior should be verified during implementation by examining existing dprint formatter logic for indented code blocks.

### Input Normalization

The formatter normalizes input regardless of how it was originally written:

- Input using 4 spaces for code block indentation is converted according to the `useTabs` setting
- Input using 1 tab for code block indentation is converted according to the `useTabs` setting
- Mixed input (some lines with tabs, some with spaces) is normalized to the configured style

**Measuring indentation with tabs**: When input contains tabs, each tab character counts as advancing to the next multiple of 4 columns (standard tab stop behavior per CommonMark). For structural indentation measurement:
- A line starting with one tab is treated as having 4 columns of indentation
- A line starting with two tabs is treated as having 8 columns of indentation
- Mixed tabs and spaces: calculate column position by treating each tab as advancing to the next tab stop (multiples of 4)

**Converting multi-column indentation**: For input with more than 4 columns of structural indentation:
- The first 4 columns of indentation are converted per the `useTabs` setting (to either one tab or four spaces)
- Any indentation beyond 4 columns is treated as code content and preserved exactly as written

Example: Input line `\t\tcode` (8 columns) with `useTabs: true` → Output `\tcode` (the 4-column structural indent becomes one tab, the remaining 4-column indent is preserved as part of the code content, so the second tab remains).

This ensures consistent output regardless of input variation.

## CommonMark Compliance

The CommonMark specification [Section 4.4](https://spec.commonmark.org/0.31.2/#indented-code-blocks) states:

> An indented code block is composed of one or more indented chunks separated by blank lines. An indented chunk is a sequence of non-blank lines, each preceded by four or more spaces of indentation.

And [Section 2.2 (Tabs)](https://spec.commonmark.org/0.31.2/#tabs) states:

> Tabs in lines are not expanded to spaces. However, in contexts where whitespace helps to define block structure, tabs behave as if they were replaced by spaces with a tab stop of 4 characters.

This means a tab character at the start of a line counts as 4 columns of indentation for parsing purposes. When `useTabs` is `true`, the formatter writes one tab (which parsers interpret as 4 columns), maintaining semantic equivalence with the 4-space version. Any CommonMark-compliant parser will parse both representations as semantically equivalent (producing the same AST structure and content).

**Compatibility note**: While CommonMark explicitly supports tabs in indented code blocks, some markdown processors outside the CommonMark ecosystem may not handle them correctly. Users working with non-CommonMark tools should verify compatibility before enabling this option.

## Design Decisions

### 1. Input Normalization Strategy

**Decision**: Always normalize input to the configured style, never preserve input formatting.

**Rationale**: dprint is an opinionated formatter designed to enforce consistent style. Auto-detecting and preserving input style would defeat this purpose and create unpredictable output. Users configure `useTabs` once, and all indented code blocks are formatted consistently regardless of how they were originally written.

### 2. List Indentation Exclusion

**Decision**: List continuation indentation uses spaces; only the 4-column code block marker can use tabs.

**Rationale**: List continuation indentation depends on marker width. The content column varies:
- `-` marker: content starts at column 2
- `1.` marker: content starts at column 3  
- `10.` marker: content starts at column 4

Since content doesn't consistently start at 4-column boundaries, tab conversion is ambiguous. A code block in a list combines list continuation indent (variable) plus code block indent (always 4 columns). We convert only the code block portion to tabs when `useTabs` is true. Full list tab support would require separate design work to handle variable marker widths, which is beyond this specification's scope.

### 3. Footnote Definition Exclusion

**Decision**: Footnote definition bodies are excluded from tab conversion.

**Rationale**: During [PR #253](https://github.com/dprint/dprint-plugin-markdown/pull/253) review, it was discovered that tab conversion in footnote bodies causes wrapping issues. The dprint printer's internal column tracking treats a tab as occupying one column width, not four visual columns. When the formatter wraps long footnote text, it calculates wrap points based on this one-column measurement, causing the wrapped text to overflow the configured line width by approximately 3 columns per tab. Since indented code blocks are never wrapped by the formatter, this issue doesn't affect them. Including footnotes would require modifying the printer's tab measurement behavior, which is beyond this specification's scope and could affect other formatting.

**Note**: This printer behavior claim should be verified during implementation by examining dprint-core's printer logic.

### 4. No Global useTabs Inheritance

**Decision**: The option must be explicitly set in the markdown plugin configuration and does not inherit from global settings.

**Rationale**: Different parts of markdown (lists, code blocks, HTML blocks, block quotes) have different indentation semantics. A global `useTabs` option would be ambiguous about which structures it applies to. Making the option explicit and scoped to indented code blocks avoids confusion.

### 5. Content Indentation Preservation

**Decision**: Only the structural 4-column indent is converted; content within code blocks retains its original indentation.

**Rationale**: Code block content is literal text that should not be reformatted. If the content contains tabs, those tabs are preserved. If it contains spaces, those spaces are preserved. The formatter only modifies the markdown structural indentation that marks the block as code, not the code's internal formatting.

## Terminology

**Indented code blocks**: Code blocks marked by 4 spaces or 1 tab at the line start, without fence delimiters (``` or ~~~). Includes code at document level, in block quotes, and in list items.

**Structural indentation**: The 4-column indentation that marks a block as code in the CommonMark specification. This is distinct from:
- Container indentation (list continuation, block quote markers)
- Internal code indentation (spacing within the code content itself)

**Fenced code blocks**: Code blocks delimited by ``` or ~~~ fence markers. Not affected by this specification.

## Implementation Notes

Primary changes required in `src/generation/generate.rs`, function `gen_code_block`:

1. Check if the code block is fenced (`code_block.is_fenced()`)
2. If not fenced, check `context.configuration.indented_code_use_tabs`
3. Modify indentation output to emit `"\t"` when `useTabs` is true, `"    "` when false
4. Ensure the printer infrastructure correctly handles tab characters in indentation

Configuration changes:
- Add `indented_code_use_tabs: bool` (default `false`) to `src/configuration/types.rs`
- Add configuration parsing in `src/configuration/builder.rs`
- Add configuration resolution in `src/configuration/resolve_config.rs`

## Testing Strategy

### 1. Oracle Test

Format the CommonMark spec corpus with `useTabs: true`, then format the output with `useTabs: false`. Parse both outputs using `pulldown-cmark` (the CommonMark parser used by dprint-plugin-markdown) and compare AST structure:

**Comparison criteria:**
- Compare `pulldown-cmark::Event` enum variants (Text, Code, CodeBlock, etc.)
- CodeBlock content strings must be byte-identical (after normalizing line endings to LF)
- Ignore parser-internal metadata such as source byte offsets and span positions
- The AST structure (nesting and sequence of Events) must match exactly

This verifies that tab and space representations are semantically equivalent.

### 2. Specification Tests

Add tests in `tests/specs/CodeBlocks/`:
- Basic indented code blocks (top level)
- Code blocks with internal indentation (tabs within content preserved)
- Code blocks in lists (list continuation + code indent)
- Code blocks in block quotes
- Nested containers (code in lists in block quotes)
- Mixed tab/space input (normalization)
- Blank lines within code blocks
- Conversion to fenced blocks (verify `useTabs` has no effect)
- Multiple indented code blocks in same document

### 3. Round-Trip Test

Format with `useTabs: true`, parse, format with `useTabs: false`, parse again. Verify AST equivalence using same criteria as oracle test. This validates that conversion is reversible and semantics are preserved.

### 4. Regression Test

Run the full existing test suite with `useTabs: false` (default) and verify all tests pass. This ensures the new option does not break existing behavior when disabled.

## Success Criteria

Implementation is complete when:

1. Setting `indentedCode.useTabs: true` converts all indented code blocks to use tabs for 4-column structural indentation
2. Setting `indentedCode.useTabs: false` (default) maintains current behavior (4 spaces)
3. Oracle test passes (tabs ↔ spaces produce equivalent ASTs)
4. All existing tests continue to pass
5. New specification tests cover all scenarios listed above

## References

- [CommonMark Spec - Indented Code Blocks](https://spec.commonmark.org/0.31.2/#indented-code-blocks)
- [CommonMark Spec - Tabs](https://spec.commonmark.org/0.31.2/#tabs)
- [Issue #242](https://github.com/dprint/dprint-plugin-markdown/issues/242) - Original feature request
- [PR #253](https://github.com/dprint/dprint-plugin-markdown/pull/253) - Prior implementation attempt
