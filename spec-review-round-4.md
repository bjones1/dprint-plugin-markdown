# Specification Review - Round 4

**Reviewer**: Claude Opus 5.5, high effort
**Prompt**: `Review indented-code-tabs-spec.md against the dprint-plugin-markdown source code and the CommonMark spec. Report logical contradictions, ambiguities, and claims the code does not support. Keep the spec short; do not add new features.`

Strikethrough marks findings we judged incorrect or irrelevant and did not apply.

## Findings

1. **CRITICAL**: The container rule (`<list spaces><tab>`, `> <tab>`) breaks
   CommonMark. Tabs expand to the next multiple of 4 counted from the line
   start, so a tab after `- ` or `> ` (column 2) supplies only 2 columns and the
   code block becomes a paragraph (see CommonMark example 6, `>\t\tfoo`).
   *Fixed: a tab is used only when the start column is a multiple of 4,
   otherwise four spaces; added a table of container cases.*

2. **CRITICAL**: The testing section relied on `pulldown-cmark`, which the
   project does not use (it has its own parser in `src/parser`). *Fixed: the
   oracle is now the existing `tests/commonmark_spec_check.rs`, run with the
   option on.*

3. **HIGH**: "Content is preserved exactly" and the example `\t\tcode` →
   `\tcode` contradict the default `codeBlock.preserveIndentation: false`,
   which removes common leading whitespace. *Fixed: the code text is defined
   as whatever existing behavior produces; the option only writes the code
   indent. The Input Normalization section was removed.*

4. **MEDIUM**: The fenced-conversion cases were vague ("after certain block
   structures") and marked "should be verified". *Fixed: listed the exact cases
   from `gen_code_block`: after a list, and list markers that do not line up.*

5. **MEDIUM**: The footnote rationale ("printer measures a tab as one column")
   was unverified. *Fixed: confirmed `indent_width: 1` in `format_text.rs`;
   removed the "should be verified" note.*

6. **MEDIUM**: Blank lines were described as "container prefix only, with no
   trailing whitespace", which contradicts a `> ` prefix. *Fixed: blank lines
   get no code indent and are otherwise written as today.*

7. **LOW**: Footnote bodies were named as a container but missing from the
   container table. *Fixed: added `[^1]: ` (start column 4 → tab) and a test.*

8. **LOW**: "All lines share a start column" was unjustified for the first line
   of a list item, which follows the marker. *Fixed: explained that step 1
   already fences items whose marker does not line up.*

9. **LOW**: The implementation note did not say block quote markers are
   written outside the generator's indent level. *Fixed.*

10. ~~**MEDIUM**: Tabs inside the code text will render at different columns
    after a tab code indent than after four spaces.~~ Incorrect: the code indent
    ends at the same column (a multiple of 4) in both forms, so the code text
    starts and expands identically.

11. ~~**LOW**: Allow a configurable tab width.~~ Irrelevant: CommonMark fixes tab
    stops at 4, and this would be a new feature.

## Result

9 findings applied, 2 rejected. The spec was also shortened from 254 to about
165 lines by removing duplicated motivation, interaction, and normalization text.
