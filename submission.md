Project 2 submission
====================

Team name: dprint tabs

The specification consists of the file [indented-code-tabs-spec.md](indented-code-tabs-spec.md).

Draft submission
----------------

### Baltabayev Timur

GitHub userid: timikxd

#### Contributions

Created specification document with behavior descriptions, examples, and implementation notes; conducted the LLM review rounds and applied their findings; rewrote and shortened the specification after instructor feedback.

#### Build and run test

Prompting Claude Opus 5.5 with "Verify that you can build and run the project in the current git repo." produced: the repo pins Rust 1.98.0 in `rust-toolchain.toml`, which rustup installed (MSVC toolchain, Windows 11). `cargo build` finished successfully in 35 s with one harmless `linker_messages` warning (the MSVC linker reporting the generated `.lib`/`.exp` files). `cargo test` passed with 0 failures: 160 unit tests, 22 integration tests (including the CommonMark spec check), 2 doc tests, and 239 spec-file tests (192 formatter spec files under `tests/specs` and 47 parser/HTML parser spec files).

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ---------------------------------------------------------------- |
| 1     | 5        | Removed contradictory list examples; removed meta-commentary ("Actually, on review:", "Wait, this is getting complex"); resolved 3 open questions into Design Decisions section; added Terminology section defining "top-level"; replaced hypothetical TAB_INDENT_MARKER code with implementation discussion |
| 2     | 3        | Replaced duplicate Example 4 with mixed indentation example; enhanced oracle test with explicit AST comparison criteria (parser choice, fields to compare/ignore); added nested container test case |
| 3     | 7        | Complete rewrite per instructor feedback: removed all examples, added Algorithm section with textual step-by-step process; clarified config format (JSON); defined blank line detection; clarified indentation measurement with tabs; added container indentation examples; added verification notes for printer behavior |
| 4     | 9        | Container tabs only at a start column that is a multiple of 4 (a tab after `- `/`> ` broke CommonMark); oracle switched from pulldown-cmark to the project's own `commonmark_spec_check.rs`; code text defined by existing `codeBlock.preserveIndentation` behavior; exact fenced-conversion cases; spec shortened from 254 to ~165 lines |

#### Last specification review

Prompting Claude Opus 5.5, high effort, with `Review indented-code-tabs-spec.md against the dprint-plugin-markdown source code and the CommonMark spec. Report logical contradictions, ambiguities, and claims the code does not support. Keep the spec short; do not add new features.` produced the following output (full text in [spec-review-round-4.md](spec-review-round-4.md)); strikethrough indicates LLM output that we deemed incorrect or irrelevant.

1. **CRITICAL**: A tab after `- ` or `> ` (column 2) reaches only column 4, giving 2 columns of indentation, so the block becomes a paragraph (CommonMark example 6). Fixed: tab only when the start column is a multiple of 4, otherwise four spaces.
2. **CRITICAL**: Testing relied on `pulldown-cmark`, which the project does not use. Fixed: oracle is `tests/commonmark_spec_check.rs` with the option on.
3. **HIGH**: "Content preserved exactly" and the `\t\tcode` → `\tcode` example contradict the default `codeBlock.preserveIndentation: false`. Fixed: code text comes from existing behavior; the option only writes the code indent.
4. **MEDIUM**: Fenced-conversion cases were vague. Fixed: exact cases from `gen_code_block` (after a list; markers that do not line up).
5. **MEDIUM**: Printer tab-width claim unverified. Fixed: confirmed `indent_width: 1` in `format_text.rs`.
6. **MEDIUM**: Blank-line wording contradicted the `> ` prefix. Fixed.
7. **LOW**: Footnote bodies missing from the container table. Fixed.
8. **LOW**: Shared start column for a list item's first line unjustified. Fixed.
9. **LOW**: Implementation note ignored block quote markers in the start column. Fixed.
10. ~~**MEDIUM**: Tabs inside the code text render at different columns after a tab code indent.~~ Incorrect: the code indent ends at the same column either way.
11. ~~**LOW**: Allow a configurable tab width.~~ Irrelevant: CommonMark fixes tab stops at 4.

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
| 3     | 7        | Complete rewrite per instructor feedback: removed all examples, added Algorithm section with textual step-by-step process; clarified config format (JSON); defined blank line detection; clarified indentation measurement with tabs; added container indentation examples; added verification notes for printer behavior |
| 4     | 9        | Container tabs only at a start column that is a multiple of 4 (a tab after `- `/`> ` broke CommonMark); oracle switched from pulldown-cmark to the project's own `commonmark_spec_check.rs`; code text defined by existing `codeBlock.preserveIndentation` behavior; exact fenced-conversion cases; spec shortened from 254 to ~165 lines |

#### Last specification review

Prompting Claude Opus 5.5, high effort, with `Review indented-code-tabs-spec.md against the dprint-plugin-markdown source code and the CommonMark spec. Report logical contradictions, ambiguities, and claims the code does not support. Keep the spec short; do not add new features.` produced the following output (full text in [spec-review-round-4.md](spec-review-round-4.md)); strikethrough indicates LLM output that we deemed incorrect or irrelevant.

1. **CRITICAL**: A tab after `- ` or `> ` (column 2) reaches only column 4, giving 2 columns of indentation, so the block becomes a paragraph (CommonMark example 6). Fixed: tab only when the start column is a multiple of 4, otherwise four spaces.
2. **CRITICAL**: Testing relied on `pulldown-cmark`, which the project does not use. Fixed: oracle is `tests/commonmark_spec_check.rs` with the option on.
3. **HIGH**: "Content preserved exactly" and the `\t\tcode` → `\tcode` example contradict the default `codeBlock.preserveIndentation: false`. Fixed: code text comes from existing behavior; the option only writes the code indent.
4. **MEDIUM**: Fenced-conversion cases were vague. Fixed: exact cases from `gen_code_block` (after a list; markers that do not line up).
5. **MEDIUM**: Printer tab-width claim unverified. Fixed: confirmed `indent_width: 1` in `format_text.rs`.
6. **MEDIUM**: Blank-line wording contradicted the `> ` prefix. Fixed.
7. **LOW**: Footnote bodies missing from the container table. Fixed.
8. **LOW**: Shared start column for a list item's first line unjustified. Fixed.
9. **LOW**: Implementation note ignored block quote markers in the start column. Fixed.
10. ~~**MEDIUM**: Tabs inside the code text render at different columns after a tab code indent.~~ Incorrect: the code indent ends at the same column either way.
11. ~~**LOW**: Allow a configurable tab width.~~ Irrelevant: CommonMark fixes tab stops at 4.

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
| 3     | 7        | Complete rewrite per instructor feedback: removed all examples, added Algorithm section with textual step-by-step process; clarified config format (JSON); defined blank line detection; clarified indentation measurement with tabs; added container indentation examples; added verification notes for printer behavior |
| 4     | 9        | Container tabs only at a start column that is a multiple of 4 (a tab after `- `/`> ` broke CommonMark); oracle switched from pulldown-cmark to the project's own `commonmark_spec_check.rs`; code text defined by existing `codeBlock.preserveIndentation` behavior; exact fenced-conversion cases; spec shortened from 254 to ~165 lines |

#### Last specification review

Prompting Claude Opus 5.5, high effort, with `Review indented-code-tabs-spec.md against the dprint-plugin-markdown source code and the CommonMark spec. Report logical contradictions, ambiguities, and claims the code does not support. Keep the spec short; do not add new features.` produced the following output (full text in [spec-review-round-4.md](spec-review-round-4.md)); strikethrough indicates LLM output that we deemed incorrect or irrelevant.

1. **CRITICAL**: A tab after `- ` or `> ` (column 2) reaches only column 4, giving 2 columns of indentation, so the block becomes a paragraph (CommonMark example 6). Fixed: tab only when the start column is a multiple of 4, otherwise four spaces.
2. **CRITICAL**: Testing relied on `pulldown-cmark`, which the project does not use. Fixed: oracle is `tests/commonmark_spec_check.rs` with the option on.
3. **HIGH**: "Content preserved exactly" and the `\t\tcode` → `\tcode` example contradict the default `codeBlock.preserveIndentation: false`. Fixed: code text comes from existing behavior; the option only writes the code indent.
4. **MEDIUM**: Fenced-conversion cases were vague. Fixed: exact cases from `gen_code_block` (after a list; markers that do not line up).
5. **MEDIUM**: Printer tab-width claim unverified. Fixed: confirmed `indent_width: 1` in `format_text.rs`.
6. **MEDIUM**: Blank-line wording contradicted the `> ` prefix. Fixed.
7. **LOW**: Footnote bodies missing from the container table. Fixed.
8. **LOW**: Shared start column for a list item's first line unjustified. Fixed.
9. **LOW**: Implementation note ignored block quote markers in the start column. Fixed.
10. ~~**MEDIUM**: Tabs inside the code text render at different columns after a tab code indent.~~ Incorrect: the code indent ends at the same column either way.
11. ~~**LOW**: Allow a configurable tab width.~~ Irrelevant: CommonMark fixes tab stops at 4.
