Project 2 submission
====================

Team name: dprint tabs

The specification consists of the file [indented-code-tabs-spec.md](indented-code-tabs-spec.md).

Draft submission
----------------

### Baltabayev Timur

GitHub userid: timikxd

#### Contributions

Created specification document with behavior descriptions, examples, and implementation notes; conducted two rounds of LLM review and applied all findings; shortened final specification for clarity.

#### Build and run test

Prompting Claude Opus 5.5 with "Verify that you can build and run the project in the current git repo." produced: the repo pins Rust 1.98.0 in `rust-toolchain.toml`, which rustup installed (MSVC toolchain, Windows 11). `cargo build` finished successfully in 35 s with one harmless `linker_messages` warning (the MSVC linker reporting the generated `.lib`/`.exp` files). `cargo test` passed with 0 failures: 160 unit tests, 22 integration tests (including the CommonMark spec check), 2 doc tests, and 239 spec-file tests (192 formatter spec files under `tests/specs` and 47 parser/HTML parser spec files).

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ---------------------------------------------------------------- |
| 1     | 5        | Removed contradictory list examples; removed meta-commentary ("Actually, on review:", "Wait, this is getting complex"); resolved 3 open questions into Design Decisions section; added Terminology section defining "top-level"; replaced hypothetical TAB_INDENT_MARKER code with implementation discussion |
| 2     | 3        | Replaced duplicate Example 4 with mixed indentation example; enhanced oracle test with explicit AST comparison criteria (parser choice, fields to compare/ignore); added nested container test case |

#### Last specification review

Prompting Claude 4 Sonnet (model ID: kr/claude-sonnet-4.5), medium effort, with code-review skill on `indented-code-tabs-spec.md` produced the following output:

**Round 2 Verified Findings:**

1. **CONFIRMED**: Example 3 and Example 4 were identical duplicates (both "Code Block After Paragraph"). *Fixed by replacing duplicate with mixed indentation example.*

2. **PLAUSIBLE**: Oracle test mentioned "equivalent ASTs" without specifying comparison criteria (which parser, which fields to compare, what to ignore). *Fixed by adding explicit criteria: use pulldown-cmark parser, compare node types and content, ignore positions/metadata.*

3. **PLAUSIBLE**: No example showed mixed tab/space input behavior despite Design Decisions claiming normalization. *Fixed by adding Example 4 with mixed input.*

**Refuted findings:**
- Blank line behavior is consistent with CommonMark
- "4-column" vs "4-space" terminology is correctly distinguished
- "Top-level" definition matches scope usage

All findings were applied.

### Kurmanbek Beksultan

GitHub userid: Bexultan-KBTU

#### Contributions

Reviewed specification structure and CommonMark compliance; contributed to design decisions and terminology definitions; verified behavior specifications against CommonMark standard.

#### Build and run test

Prompting Claude Opus 5.5 with "Verify that you can build and run the project in the current git repo." produced: the repo pins Rust 1.98.0 in `rust-toolchain.toml`, which rustup installed (MSVC toolchain, Windows 11). `cargo build` finished successfully in 35 s with one harmless `linker_messages` warning (the MSVC linker reporting the generated `.lib`/`.exp` files). `cargo test` passed with 0 failures: 160 unit tests, 22 integration tests (including the CommonMark spec check), 2 doc tests, and 239 spec-file tests (192 formatter spec files under `tests/specs` and 47 parser/HTML parser spec files).

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ---------------------------------------------------------------- |
| 1     | 5        | Removed contradictory list examples; removed meta-commentary ("Actually, on review:", "Wait, this is getting complex"); resolved 3 open questions into Design Decisions section; added Terminology section defining "top-level"; replaced hypothetical TAB_INDENT_MARKER code with implementation discussion |
| 2     | 3        | Replaced duplicate Example 4 with mixed indentation example; enhanced oracle test with explicit AST comparison criteria (parser choice, fields to compare/ignore); added nested container test case |

#### Last specification review

Prompting Claude 4 Sonnet (model ID: kr/claude-sonnet-4.5), medium effort, with code-review skill on `indented-code-tabs-spec.md` produced the following output:

**Round 2 Verified Findings:**

1. **CONFIRMED**: Example 3 and Example 4 were identical duplicates (both "Code Block After Paragraph"). *Fixed by replacing duplicate with mixed indentation example.*

2. **PLAUSIBLE**: Oracle test mentioned "equivalent ASTs" without specifying comparison criteria (which parser, which fields to compare, what to ignore). *Fixed by adding explicit criteria: use pulldown-cmark parser, compare node types and content, ignore positions/metadata.*

3. **PLAUSIBLE**: No example showed mixed tab/space input behavior despite Design Decisions claiming normalization. *Fixed by adding Example 4 with mixed input.*

**Refuted findings:**
- Blank line behavior is consistent with CommonMark
- "4-column" vs "4-space" terminology is correctly distinguished
- "Top-level" definition matches scope usage

All findings were applied.

### Sagynbayev Alisher

GitHub userid: a-sagynbayev

#### Contributions

Developed testing strategy and success criteria; contributed to configuration schema and scope definitions; verified implementation approach and test case coverage.

#### Build and run test

Prompting Claude Opus 5.5 with "Verify that you can build and run the project in the current git repo." produced: the repo pins Rust 1.98.0 in `rust-toolchain.toml`, which rustup installed (MSVC toolchain, Windows 11). `cargo build` finished successfully in 35 s with one harmless `linker_messages` warning (the MSVC linker reporting the generated `.lib`/`.exp` files). `cargo test` passed with 0 failures: 160 unit tests, 22 integration tests (including the CommonMark spec check), 2 doc tests, and 239 spec-file tests (192 formatter spec files under `tests/specs` and 47 parser/HTML parser spec files).

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ---------------------------------------------------------------- |
| 1     | 5        | Removed contradictory list examples; removed meta-commentary ("Actually, on review:", "Wait, this is getting complex"); resolved 3 open questions into Design Decisions section; added Terminology section defining "top-level"; replaced hypothetical TAB_INDENT_MARKER code with implementation discussion |
| 2     | 3        | Replaced duplicate Example 4 with mixed indentation example; enhanced oracle test with explicit AST comparison criteria (parser choice, fields to compare/ignore); added nested container test case |

#### Last specification review

Prompting Claude 4 Sonnet (model ID: kr/claude-sonnet-4.5), medium effort, with code-review skill on `indented-code-tabs-spec.md` produced the following output:

**Round 2 Verified Findings:**

1. **CONFIRMED**: Example 3 and Example 4 were identical duplicates (both "Code Block After Paragraph"). *Fixed by replacing duplicate with mixed indentation example.*

2. **PLAUSIBLE**: Oracle test mentioned "equivalent ASTs" without specifying comparison criteria (which parser, which fields to compare, what to ignore). *Fixed by adding explicit criteria: use pulldown-cmark parser, compare node types and content, ignore positions/metadata.*

3. **PLAUSIBLE**: No example showed mixed tab/space input behavior despite Design Decisions claiming normalization. *Fixed by adding Example 4 with mixed input.*

**Refuted findings:**
- Blank line behavior is consistent with CommonMark
- "4-column" vs "4-space" terminology is correctly distinguished
- "Top-level" definition matches scope usage

All findings were applied.

Final submission
----------------

### Baltabayev Timur

GitHub userid: timikxd

#### Contributions

Implemented the `indentedCode.useTabs` option (configuration, schema, start-column tracking and tab output in `gen_code_block`) and updated `submission.md`.

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 3     | 7        | Rewrote the spec as an algorithm per instructor feedback: removed behavior-by-example, added design decisions with rationale, links to Issue #242 / PR #253, and footnote coverage |
| 4     | 9        | Tab only where the code indent starts at a column that is a multiple of 4 (a tab after `- `/`> ` broke CommonMark); oracle moved from pulldown-cmark to the project's `commonmark_spec_check.rs`; code text defined by existing `codeBlock.preserveIndentation`; spec shortened from 254 to ~165 lines; added rationale for excluding fenced blocks |
| 5     | 6        | Nested block quotes are written `>> ` (column 3), not `> > `; only empty lines skip the code indent; definition lists added as containers; oracle switch `SPEC_INDENTED_CODE_TABS` named |
| 6     | 3        | Only trivial findings: container table labeled as using the default `list.indentKind`; two rejected |

#### Last specification review

Prompting Claude Opus 5.5, high effort, with `Review indented-code-tabs-spec.md against the dprint-plugin-markdown source code and the CommonMark spec. Report logical contradictions, ambiguities, and claims the code does not support. Keep the spec short; do not add new features.` produced the following output (full text in [spec-review-round-6.md](spec-review-round-6.md)); strikethrough indicates LLM output that we deemed incorrect or irrelevant.

1. **LOW**: The container table's start columns assume the default `list.indentKind`; with `pythonMarkdown` a definition starts at column 4. Fixed: the table is labeled as using the default.
2. ~~**LOW**: "the rest of the line is written as it is today" for empty lines is vague.~~ The option does not change empty lines, which is what the sentence says.
3. ~~**LOW**: Add a rule for configurable tab widths.~~ CommonMark fixes tab stops at 4.

#### Implementation review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1     | 5        | Start column fixed for nested block quotes (`>>` is one column per marker) and for quotes separated by a list (extra space before the indentation); tests added for definition lists, a quote in a footnote, and a whitespace-only line; clippy `is_multiple_of` and rustfmt fixes |
| 2     | 3        | Only trivial or refuted findings (footnote starting with code is parsed as a paragraph; defensive `\r` strip kept; doc comment wording) |

#### Last implementation review

Prompting Claude Opus 5.5, high effort, with `Review the implementation of indentedCode.useTabs (git diff) against indented-code-tabs-spec.md. Look for wrong start columns, broken CommonMark meaning, missing tests, and lint or formatting problems. Run the tests, the CommonMark check with SPEC_INDENTED_CODE_TABS=1, clippy and rustfmt.` produced the following output (full text in [impl-review-round-2.md](impl-review-round-2.md)); strikethrough indicates LLM output that we deemed incorrect or irrelevant.

1. ~~**MEDIUM**: A footnote whose body starts with an indented code block would get a tab right after `[^1]: `, which spaces mode does not write.~~ Refuted: the parser reads `[^1]:     code` as a paragraph, so no code block is generated (output is `[^1]: code` in both modes).
2. **LOW**: `strip_suffix('\r')` in the tab branch is defensive, since the code text is split on `\n` like `gen_from_string`, which also drops `\r`. Kept, no change.
3. **LOW**: The doc comment on `footnote_indent_level` could say why it is separate from `indent_level`. Trivial; no change.

All tests pass (`cargo test`), including 24 new cases in `tests/specs/CodeBlocks/CodeBlocks_IndentedCodeUseTabs.txt`, and the CommonMark check passes with `SPEC_INDENTED_CODE_TABS=1`.

### Kurmanbek Beksultan

GitHub userid: Bexultan-KBTU

#### Contributions

Wrote the spec tests in `tests/specs/CodeBlocks/CodeBlocks_IndentedCodeUseTabs.txt` covering every container case, and recorded the implementation review rounds.

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 3     | 7        | Rewrote the spec as an algorithm per instructor feedback: removed behavior-by-example, added design decisions with rationale, links to Issue #242 / PR #253, and footnote coverage |
| 4     | 9        | Tab only where the code indent starts at a column that is a multiple of 4 (a tab after `- `/`> ` broke CommonMark); oracle moved from pulldown-cmark to the project's `commonmark_spec_check.rs`; code text defined by existing `codeBlock.preserveIndentation`; spec shortened from 254 to ~165 lines; added rationale for excluding fenced blocks |
| 5     | 6        | Nested block quotes are written `>> ` (column 3), not `> > `; only empty lines skip the code indent; definition lists added as containers; oracle switch `SPEC_INDENTED_CODE_TABS` named |
| 6     | 3        | Only trivial findings: container table labeled as using the default `list.indentKind`; two rejected |

#### Last specification review

Prompting Claude Opus 5.5, high effort, with `Review indented-code-tabs-spec.md against the dprint-plugin-markdown source code and the CommonMark spec. Report logical contradictions, ambiguities, and claims the code does not support. Keep the spec short; do not add new features.` produced the following output (full text in [spec-review-round-6.md](spec-review-round-6.md)); strikethrough indicates LLM output that we deemed incorrect or irrelevant.

1. **LOW**: The container table's start columns assume the default `list.indentKind`; with `pythonMarkdown` a definition starts at column 4. Fixed: the table is labeled as using the default.
2. ~~**LOW**: "the rest of the line is written as it is today" for empty lines is vague.~~ The option does not change empty lines, which is what the sentence says.
3. ~~**LOW**: Add a rule for configurable tab widths.~~ CommonMark fixes tab stops at 4.

#### Implementation review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1     | 5        | Start column fixed for nested block quotes (`>>` is one column per marker) and for quotes separated by a list (extra space before the indentation); tests added for definition lists, a quote in a footnote, and a whitespace-only line; clippy `is_multiple_of` and rustfmt fixes |
| 2     | 3        | Only trivial or refuted findings (footnote starting with code is parsed as a paragraph; defensive `\r` strip kept; doc comment wording) |

#### Last implementation review

Prompting Claude Opus 5.5, high effort, with `Review the implementation of indentedCode.useTabs (git diff) against indented-code-tabs-spec.md. Look for wrong start columns, broken CommonMark meaning, missing tests, and lint or formatting problems. Run the tests, the CommonMark check with SPEC_INDENTED_CODE_TABS=1, clippy and rustfmt.` produced the following output (full text in [impl-review-round-2.md](impl-review-round-2.md)); strikethrough indicates LLM output that we deemed incorrect or irrelevant.

1. ~~**MEDIUM**: A footnote whose body starts with an indented code block would get a tab right after `[^1]: `, which spaces mode does not write.~~ Refuted: the parser reads `[^1]:     code` as a paragraph, so no code block is generated (output is `[^1]: code` in both modes).
2. **LOW**: `strip_suffix('\r')` in the tab branch is defensive, since the code text is split on `\n` like `gen_from_string`, which also drops `\r`. Kept, no change.
3. **LOW**: The doc comment on `footnote_indent_level` could say why it is separate from `indent_level`. Trivial; no change.

All tests pass (`cargo test`), including 24 new cases in `tests/specs/CodeBlocks/CodeBlocks_IndentedCodeUseTabs.txt`, and the CommonMark check passes with `SPEC_INDENTED_CODE_TABS=1`.

### Sagynbayev Alisher

GitHub userid: a-sagynbayev

#### Contributions

Finalized the specification after instructor feedback (start-column rule, block quote markers, definition lists) and recorded the specification review rounds.

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 3     | 7        | Rewrote the spec as an algorithm per instructor feedback: removed behavior-by-example, added design decisions with rationale, links to Issue #242 / PR #253, and footnote coverage |
| 4     | 9        | Tab only where the code indent starts at a column that is a multiple of 4 (a tab after `- `/`> ` broke CommonMark); oracle moved from pulldown-cmark to the project's `commonmark_spec_check.rs`; code text defined by existing `codeBlock.preserveIndentation`; spec shortened from 254 to ~165 lines; added rationale for excluding fenced blocks |
| 5     | 6        | Nested block quotes are written `>> ` (column 3), not `> > `; only empty lines skip the code indent; definition lists added as containers; oracle switch `SPEC_INDENTED_CODE_TABS` named |
| 6     | 3        | Only trivial findings: container table labeled as using the default `list.indentKind`; two rejected |

#### Last specification review

Prompting Claude Opus 5.5, high effort, with `Review indented-code-tabs-spec.md against the dprint-plugin-markdown source code and the CommonMark spec. Report logical contradictions, ambiguities, and claims the code does not support. Keep the spec short; do not add new features.` produced the following output (full text in [spec-review-round-6.md](spec-review-round-6.md)); strikethrough indicates LLM output that we deemed incorrect or irrelevant.

1. **LOW**: The container table's start columns assume the default `list.indentKind`; with `pythonMarkdown` a definition starts at column 4. Fixed: the table is labeled as using the default.
2. ~~**LOW**: "the rest of the line is written as it is today" for empty lines is vague.~~ The option does not change empty lines, which is what the sentence says.
3. ~~**LOW**: Add a rule for configurable tab widths.~~ CommonMark fixes tab stops at 4.

#### Implementation review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1     | 5        | Start column fixed for nested block quotes (`>>` is one column per marker) and for quotes separated by a list (extra space before the indentation); tests added for definition lists, a quote in a footnote, and a whitespace-only line; clippy `is_multiple_of` and rustfmt fixes |
| 2     | 3        | Only trivial or refuted findings (footnote starting with code is parsed as a paragraph; defensive `\r` strip kept; doc comment wording) |

#### Last implementation review

Prompting Claude Opus 5.5, high effort, with `Review the implementation of indentedCode.useTabs (git diff) against indented-code-tabs-spec.md. Look for wrong start columns, broken CommonMark meaning, missing tests, and lint or formatting problems. Run the tests, the CommonMark check with SPEC_INDENTED_CODE_TABS=1, clippy and rustfmt.` produced the following output (full text in [impl-review-round-2.md](impl-review-round-2.md)); strikethrough indicates LLM output that we deemed incorrect or irrelevant.

1. ~~**MEDIUM**: A footnote whose body starts with an indented code block would get a tab right after `[^1]: `, which spaces mode does not write.~~ Refuted: the parser reads `[^1]:     code` as a paragraph, so no code block is generated (output is `[^1]: code` in both modes).
2. **LOW**: `strip_suffix('\r')` in the tab branch is defensive, since the code text is split on `\n` like `gen_from_string`, which also drops `\r`. Kept, no change.
3. **LOW**: The doc comment on `footnote_indent_level` could say why it is separate from `indent_level`. Trivial; no change.

All tests pass (`cargo test`), including 24 new cases in `tests/specs/CodeBlocks/CodeBlocks_IndentedCodeUseTabs.txt`, and the CommonMark check passes with `SPEC_INDENTED_CODE_TABS=1`.
