# Specification Review - Round 5

**Reviewer**: Claude Opus 5.5, high effort
**Prompt**: `Review indented-code-tabs-spec.md against the dprint-plugin-markdown source code and the CommonMark spec. Report logical contradictions, ambiguities, and claims the code does not support. Keep the spec short; do not add new features.`

This round ran while implementing the spec, so findings were checked against
the formatter's real output. Strikethrough marks findings we judged incorrect
or irrelevant.

## Findings

1. **HIGH**: The container table claimed a nested block quote is written
   `> > ` (start column 4 → tab). dprint writes nested quotes as `>> `, start
   column 3, so a tab there supplies 1 column and breaks the code block.
   *Fixed: Definitions now state how quote markers are written (`>> `, and
   `>   > ` when a list separates two quotes); the table lists `>> ` → spaces
   and `>>> ` → tab.*

2. **MEDIUM**: "Blank line" was defined as a whitespace-only line with no code
   indent, but today a whitespace-only line keeps the four-space indent before
   its whitespace. *Fixed: only an empty line has no code indent; a
   whitespace-only line is written like any other line.*

3. **MEDIUM**: Definition lists (`: `) were missing from the container
   prefix, although they are containers with their own indentation. *Fixed:
   added to Definitions and the table.*

4. **LOW**: The oracle said "run it with `indentedCode.useTabs: true`" without
   saying how; the test builds its own configuration. *Fixed: names the
   `SPEC_INDENTED_CODE_TABS=1` switch.*

5. **LOW**: The separate idempotence test duplicated what the spec test runner
   already does (`format_twice: true`). *Fixed: merged into the spec test item.*

6. **LOW**: The implementation note about the start column did not mention
   footnote indentation, which the generator's `indent_level` does not count.
   *Fixed.*

7. ~~**MEDIUM**: The table of containers is behavior by example, which the
   instructor asked to remove.~~ Rejected: the rule is stated in text (step 2);
   the table only explains why the rule needs the start column.

## Result

6 findings applied, 1 rejected.
