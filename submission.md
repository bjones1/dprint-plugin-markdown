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
| 3     | 7        | Complete rewrite per instructor feedback: removed all examples, added Algorithm section with textual step-by-step process; clarified config format (JSON); defined blank line detection; clarified indentation measurement with tabs; added container indentation examples; added verification notes for printer behavior |

#### Last specification review

Prompting Claude Opus 5.5, high effort, with code-review skill on `indented-code-tabs-spec.md` produced the following output:

**Round 3 Verified Findings (after instructor feedback):**

1. **CRITICAL**: Config format showed TypeScript instead of actual dprint.json format. *Fixed by showing exact JSON: `{"markdown": {"indentedCode.useTabs": true}}`*

2. **CRITICAL**: Algorithm didn't specify how to count "4 columns" when input has tabs. *Fixed by adding tab stop calculation rules (each tab = next multiple of 4) with example*

3. **HIGH**: Blank line detection ambiguous between empty and whitespace-only. *Fixed by defining: "line containing only whitespace (spaces, tabs) or no characters"*

4. **MEDIUM**: "Relative indentation" preservation unclear. *Fixed by clarifying content whitespace is preserved exactly as written*

5. **MEDIUM**: Container indentation order needed concrete examples. *Fixed by adding exact output patterns for list/blockquote/nested*

6. **MEDIUM**: Printer behavior claim unverified. *Fixed by adding implementation verification note*

7. **LOW**: Fenced conversion behavior should be verified. *Fixed by noting this is existing behavior to verify*

All instructor feedback addressed:
- ✅ Algorithm stated in text (not examples)
- ✅ Design decisions explained with rationale
- ✅ Links to Issue #242 and PR #253 added
- ✅ Footnotes addressed
- ✅ Contradictions fixed

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
| 3     | 7        | Complete rewrite per instructor feedback: removed all examples, added Algorithm section with textual step-by-step process; clarified config format (JSON); defined blank line detection; clarified indentation measurement with tabs; added container indentation examples; added verification notes for printer behavior |

#### Last specification review

Prompting Claude Opus 5.5, high effort, with code-review skill on `indented-code-tabs-spec.md` produced the following output:

**Round 3 Verified Findings (after instructor feedback):**

1. **CRITICAL**: Config format showed TypeScript instead of actual dprint.json format. *Fixed by showing exact JSON: `{"markdown": {"indentedCode.useTabs": true}}`*

2. **CRITICAL**: Algorithm didn't specify how to count "4 columns" when input has tabs. *Fixed by adding tab stop calculation rules (each tab = next multiple of 4) with example*

3. **HIGH**: Blank line detection ambiguous between empty and whitespace-only. *Fixed by defining: "line containing only whitespace (spaces, tabs) or no characters"*

4. **MEDIUM**: "Relative indentation" preservation unclear. *Fixed by clarifying content whitespace is preserved exactly as written*

5. **MEDIUM**: Container indentation order needed concrete examples. *Fixed by adding exact output patterns for list/blockquote/nested*

6. **MEDIUM**: Printer behavior claim unverified. *Fixed by adding implementation verification note*

7. **LOW**: Fenced conversion behavior should be verified. *Fixed by noting this is existing behavior to verify*

All instructor feedback addressed:
- ✅ Algorithm stated in text (not examples)
- ✅ Design decisions explained with rationale
- ✅ Links to Issue #242 and PR #253 added
- ✅ Footnotes addressed
- ✅ Contradictions fixed

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
| 3     | 7        | Complete rewrite per instructor feedback: removed all examples, added Algorithm section with textual step-by-step process; clarified config format (JSON); defined blank line detection; clarified indentation measurement with tabs; added container indentation examples; added verification notes for printer behavior |

#### Last specification review

Prompting Claude Opus 5.5, high effort, with code-review skill on `indented-code-tabs-spec.md` produced the following output:

**Round 3 Verified Findings (after instructor feedback):**

1. **CRITICAL**: Config format showed TypeScript instead of actual dprint.json format. *Fixed by showing exact JSON: `{"markdown": {"indentedCode.useTabs": true}}`*

2. **CRITICAL**: Algorithm didn't specify how to count "4 columns" when input has tabs. *Fixed by adding tab stop calculation rules (each tab = next multiple of 4) with example*

3. **HIGH**: Blank line detection ambiguous between empty and whitespace-only. *Fixed by defining: "line containing only whitespace (spaces, tabs) or no characters"*

4. **MEDIUM**: "Relative indentation" preservation unclear. *Fixed by clarifying content whitespace is preserved exactly as written*

5. **MEDIUM**: Container indentation order needed concrete examples. *Fixed by adding exact output patterns for list/blockquote/nested*

6. **MEDIUM**: Printer behavior claim unverified. *Fixed by adding implementation verification note*

7. **LOW**: Fenced conversion behavior should be verified. *Fixed by noting this is existing behavior to verify*

All instructor feedback addressed:
- ✅ Algorithm stated in text (not examples)
- ✅ Design decisions explained with rationale
- ✅ Links to Issue #242 and PR #253 added
- ✅ Footnotes addressed
- ✅ Contradictions fixed
