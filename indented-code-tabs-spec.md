# Indented Code Block Tab Support Specification

## Overview

A new option, `indentedCode.useTabs`, lets the formatter write the indentation
that marks an indented code block (a code block without ``` or ~~~ fences) as a
tab instead of four spaces.

This addresses [Issue #242](https://github.com/dprint/dprint-plugin-markdown/issues/242).
An earlier attempt, [PR #253](https://github.com/dprint/dprint-plugin-markdown/pull/253),
also converted list and footnote indentation to tabs; this specification
deliberately limits itself to indented code blocks (see Design Decisions).

## Configuration

```json
{
  "markdown": {
    "indentedCode.useTabs": true
  }
}
```

- **Type**: `boolean`
- **Default**: `false` (current behavior: four spaces)
- Not inherited from the global `useTabs` setting.

Unrelated options keep their meaning: `codeBlock.useTabs` and
`codeBlock.indentWidth` only affect what an external code formatter writes
*inside* fenced code blocks; `html.useTabs` only affects HTML blocks;
`list.indentKind` only affects list continuation indentation.

## Definitions

- **Code indent**: the four columns of indentation that make a line part of an
  indented code block, as defined by CommonMark.
- **Container prefix**: everything the formatter writes on a line before the
  code indent: list and definition continuation spaces, footnote body
  indentation, and block quote markers. Nested block quotes are written as one
  `>` each with a single space after the last (`>> `), plus a space before any
  indentation that separates two of them (`>   > `). A top-level code block has
  an empty container prefix.
- **Start column**: the column where the code indent begins, i.e. the width of
  the container prefix. Columns are counted from 0.
- **Empty line**: a line of the code text with no characters at all.

## Algorithm

The option changes only how the code indent is written. Which blocks are
indented code blocks, the container prefix, and the code text itself are
produced exactly as they are today.

For each code block the formatter is about to write:

1. **Fenced output.** If the block will be written fenced, the option has no
   effect. This covers blocks that were fenced in the input and indented blocks
   the formatter already converts to fenced ones: a block directly after a list,
   and a block inside a list item whose marker is narrower than its
   continuation indentation (`list.indentKind: "pythonMarkdown"`).

2. **Choose the code indent.** If `indentedCode.useTabs` is `true` **and** the
   start column is a multiple of 4, the code indent is one tab (U+0009).
   Otherwise it is four spaces (U+0020).

3. **Write each line** of the code block:
   - Empty line: no code indent; the rest of the line is written as it is
     today.
   - Any other line, including one holding only whitespace: container prefix,
     then the code indent, then the code text. This matches how the four-space
     indent is written today.

All lines of one code block use the same code indent. This holds for the first
line of a list item too: step 1 already sends every item whose marker is not
exactly as wide as its continuation indentation to fenced output, so the marker
line and the following lines share the start column.

The code text is not affected by this option. Whatever `codeBlock.preserveIndentation`
leaves of the text's own leading whitespace (by default the common leading
whitespace is removed) is written after the code indent unchanged, including
any tabs it contains.

The input form is irrelevant: a block indented with spaces, tabs, or a mix of
both is parsed the same way, and the output is decided only by steps 1–3.

### Why the start column matters

CommonMark ([Tabs](https://spec.commonmark.org/0.31.2/#tabs)) expands a tab to
the next multiple of 4 columns, counted from the start of the line, not from
the start of the container. A tab therefore supplies four columns of
indentation only when it begins at a multiple of 4. With the default
`list.indentKind`:

| Container prefix           | Start column | Code indent with `useTabs: true` |
| -------------------------- | ------------ | -------------------------------- |
| none (top level)           | 0            | tab                              |
| `- ` list item             | 2            | four spaces                      |
| `1. ` list item            | 3            | four spaces                      |
| `10. ` list item           | 4            | tab                              |
| `> ` block quote           | 2            | four spaces                      |
| `>> ` nested block quote   | 3            | four spaces                      |
| `>>> ` block quote, 3 deep | 4            | tab                              |
| `- > ` quote in list item  | 4            | tab                              |
| `- ` inside `- ` (nested)  | 4            | tab                              |
| `: ` definition            | 2            | four spaces                      |
| `[^1]: ` footnote body     | 4            | tab                              |

Writing a tab at column 2 would reach only column 4, two columns short of a code
indent, and the block would turn into a paragraph. CommonMark example 6
(`>\t\tfoo`) shows this partial-tab behavior. Falling back to spaces keeps the
document's meaning in every container.

## Design Decisions

1. **Only the code indent changes.** List continuation and footnote body
   indentation stay spaces. Their width depends on the marker or label
   (`-`, `1.`, `10.`, `[^note]:`) and usually does not start or end at a tab
   stop, so a tab cannot represent it exactly. In addition, the printer is
   configured with `indent_width: 1` and measures a tab as one column, so tabs
   inside wrapped paragraph text (footnote bodies) would make lines overflow
   `lineWidth`. Code blocks are never wrapped, so the code indent is unaffected.
   This was the problem found in PR #253.

2. **Fenced blocks and code text are not changed.** A fenced block has no code
   indent: every leading space or tab on its lines (after the container
   prefix) belongs to the code itself, and the same is true of the code text
   after an indented block's code indent. Rewriting that whitespace would
   change the code, which matters for whitespace-sensitive languages such as
   Python or Makefiles. Re-indenting code is the job of the embedded code
   formatter, which `codeBlock.useTabs` already controls.

3. **Fall back to spaces instead of failing or padding.** When the start column
   is not a multiple of 4, writing spaces preserves meaning. Padding the prefix
   to the next tab stop would change list and block quote output that this
   option is not supposed to touch.

4. **Always normalize.** Like the rest of dprint, the output depends only on the
   configuration, never on whether the input used tabs or spaces.

5. **Opt-in only.** The option is not inherited from global `useTabs`, because
   some non-CommonMark renderers mishandle tab-indented code. Users who share
   a global `useTabs: true` with other plugins should not get this change
   silently.

## Testing

1. **CommonMark oracle.** `tests/commonmark_spec_check.rs` formats every
   CommonMark spec example and checks that the formatted output still renders
   to the spec's HTML. Setting `SPEC_INDENTED_CODE_TABS=1` turns the option on;
   the check must pass with the same known-failure list as with the option off.
   This proves the tab output means the same as the input.

2. **Spec tests** in `tests/specs/CodeBlocks/`, each with the option on:
   - top-level block written with spaces, with tabs, and mixed → one tab;
   - code text containing its own tabs, deeper indentation, and a line of
     only spaces;
   - empty lines inside the block → stay empty;
   - every row of the container table above, plus block quotes separated by a
     list (`> - > `, `> 10. > `) and a block quote in a footnote body;
   - a list item that starts with the code block (tab on the marker line);
   - block directly after a list → fenced, no tab;
   - fenced input → indentation inside it unchanged.

   The spec test runner formats each expected output a second time
   (`format_twice`), which checks idempotence.

3. **Default unchanged.** With the option off, all existing tests pass
   unmodified.

## Implementation Notes

- Configuration: add `indented_code_use_tabs: bool` to
  `src/configuration/types.rs`, a builder method in `builder.rs`, and resolve
  `"indentedCode.useTabs"` with default `false` in `resolve_config.rs`.
- Generation: `gen_code_block` in `src/generation/generate.rs` currently writes
  the code indent through `with_indent_times(items, 4)`, and the printer always
  indents with spaces (`use_tabs: false` in `format_text.rs`). For the tab case
  the indent must instead be emitted as a `Signal::Tab` at the start of each
  non-empty line (including the first line of a list item, which is currently
  written as an explicit `"    "`).
- Start column: the generator's `indent_level` covers list and definition
  indentation but not footnote bodies or block quote markers, so the context
  also tracks footnote indentation and adds the marker widths described under
  Definitions, mirroring `gen_block_quote_markers`.
