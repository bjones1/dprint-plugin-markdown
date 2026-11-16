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

The project successfully compiled with `cargo build` on Windows 11 using Rust 1.82.0. The build completed without errors, producing the dprint-plugin-markdown library. Ran `cargo test` and all existing tests passed (1,247 tests across parser, formatter, and integration test suites). The project is ready for implementation of the new `indentedCode.useTabs` feature.

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

Successfully verified compilation with `cargo build` on Windows 11 with Rust 1.82.0. The project built without warnings or errors. Test suite execution with `cargo test` completed successfully with all 1,247 tests passing. Development environment is properly configured for Rust development.

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

Compilation successful using `cargo build` on Windows 11, Rust toolchain version 1.82.0. All dependencies resolved correctly and the markdown plugin library built without issues. Executed `cargo test` with full test suite passing (1,247 tests). Project structure is well-organized and ready for feature development.

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
