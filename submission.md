Project 2 submission
====================

Team name: dprint-plugin-markdown subscript

The specification consists of the file
[subscript-spec.md](src/parser/subscript-spec.md).

Draft submission
----------------

The build and run test summarizes LLM output, on each person's own computer,
for the prompt "Verify that you can compile and run the project in the current
git repo", the wording of the assignment when the draft was submitted.

### Jacob Timmons

GitHub userid: jacobtimmons03-cyber

#### Contributions

Independently developed and reviewed a Pandoc-style Markdown subscript specification, investigated existing parser/formatter behavior, and corrected a whitespace-related semantic-preservation issue through three review rounds.

#### Build and run test

In my Windows Project 2 checkout, Codex verified that cargo build --locked and cargo test --locked passed, including parser, formatter, and documentation tests. Execution was verified through library tests; the WebAssembly release build was not tested. This verification was on my individual branch, not the team branch.

#### Specification review rounds

Dias ran the draft review rounds for the whole team on one shared
specification; see Dias Lesbay's section below for the table and the last
review.

#### Last specification review

Dias ran the draft review rounds for the whole team on one shared
specification; see Dias Lesbay's section below for the table and the last
review.

### Amir Adilov

GitHub userid: amir-kbtu-pge

#### Contributions

ran the compiler test on macOS, settled open question 2 by
  building pulldown-cmark at the pinned commit `1c1a312` and measuring how
  subscript and emphasis resolve when they overlap, which supplied parsing
  rule 10 and its eight test rows, and argued for two options by measuring
  that `ENABLE_SUPERSCRIPT` alone leaves `~` untouched.

#### Build and run test

Rust was not installed, so it was installed with rustup; the toolchain is
pinned to 1.98.0 by `rust-toolchain.toml`, which rustup fetched on the first
build. `cargo build` succeeded and `cargo test` passed 192 tests plus 2
doc-tests, with no failures. The project is a library with no binary, so
`format_text` was called from a scratch crate to check that it runs: it
collapsed the extra spaces in `Chemistry:   H~2~O  and   ~~gone~~` and
rewrote `*` bullets as `-`.

#### Specification review rounds

Dias ran the draft review rounds for the whole team on one shared
specification; see Dias Lesbay's section below for the table and the last
review.

#### Last specification review

Dias ran the draft review rounds for the whole team on one shared
specification; see Dias Lesbay's section below for the table and the last
review.

### Anuar Kurmanov

GitHub userid: FIFSAK

#### Contributions

ran the compiler test on macOS and argued for keeping
  `subscript` and `superscript` as separate options, matching pulldown-cmark's
  separate `ENABLE_SUBSCRIPT` and `ENABLE_SUPERSCRIPT` flags.

#### Build and run test

Rust was not installed, so it was installed with rustup (`rustc` 1.98.0). Then
`cargo build` succeeded and `cargo test` passed 182 tests across the unit and
integration test binaries, all spec-file suites, and 2 doc-tests, with no
failures. The project is a library, so there is no binary to run.

#### Specification review rounds

Dias ran the draft review rounds for the whole team on one shared
specification; see Dias Lesbay's section below for the table and the last
review.

#### Last specification review

Dias ran the draft review rounds for the whole team on one shared
specification; see Dias Lesbay's section below for the table and the last
review.

### Dias Lesbay

GitHub userid: diaslesbay2003kbtu

#### Contributions

drafted the "Subscript support" specification, verified its
  parser expectations against pulldown-cmark, and ran and triaged the ten LLM
  review rounds.

#### Build and run test

`cargo build` succeeded and `cargo test` passed all 192 tests plus 2 doc-tests;
the project is a library, so there is no binary to run, and it was not run
through the dprint CLI. `cargo` was not on `PATH` and had to be added from the
rustup install.

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1     | 13       | Rules for empty content, nesting and escaping; formatter examples; "first wins" wording fixed; pulldown-cmark `master` and 0.13.4 differ on `H~2~O` |
| 2     | 17       | Commit `1c1a312` pinned; openers inside a pair are dropped; `\\~`; soft break; 13 to 27 parser rows |
| 3     | 13       | Unreachable empty-content rule replaced by nesting; Unicode whitespace at line edges; escape only if the tree would change; `subscript` off leaves output unchanged |
| 4     | 12       | Rule tags fixed; closes-before-opens order; fence indentation and containers; punctuation beside a delimiter matters (`a~“b”~`) |
| 5     | 17       | Dropped openers are strictly between the pair; links, autolinks and inline HTML are not scanned; off-mode behavior; wrapping applies in both modes |
| 6     | 15       | A closer without a same-length opener disables earlier openers (`~a~~b~` is text); a wrapped `~~~` break is refused, not escaped |
| 7     | 9        | Punctuation and run-length rules derived from pulldown-cmark source; configuration split into two steps; more formatter rows |
| 8     | 14       | Wrong claim fixed: this formatter writes `~a~` as `~~a~~`; Parsing applies only with `subscript` on |
| 9     | 22       | Open question on overlap with emphasis added (later settled by Amir as rule 10); the rest, mostly requests for more test rows, not applied |
| 10    | 18       | `~~` flanking stated explicitly; formatting rule 2 decided by re-parsing and comparing trees; configuration step 2 tied to open question 1; one rule tag fixed; formatter escaping of `*` beside a delimiter noted as open |

Findings were counted from each review's numbered and bulleted items. Rounds 9
and 10 mostly ask for more test rows; most of those were not applied, to keep
the specification short.

#### Last specification review

Prompting Claude Sonnet 5.5 (`claude-sonnet-5-5`), default effort, in a fresh
session with `Review the section named "Subscript support" in
src/parser/subscript-spec.md. Do not edit any files; return the review as your
final message.` produced the following output.

> The spec is mostly consistent and its references check out. I did not rebuild pulldown-cmark at 1c1a312, so the HTML expectations are unverified.
>
> **Substantive issues**
>
> 1. Rule 10 has no formatter coverage, though it is called "settled". Formatting rule 2 only covers escaping `~`, but `~a *b~ c*` parses as `<sub>a *b</sub> c*`, so the formatter must also decide whether the literal `*` needs escaping. Add formatter rows for all eight rule-10 cases and state the `*` escaping rule.
> 2. The rule 5 `flanking` sentence is ambiguous; state the `~~` rule explicitly.
> 3. The formatting-rule-2 "would change the tree" test is underspecified; a stated algorithm such as re-parse and compare is needed, and rule 5 should share one definition.
> 4. Open question 1 blocks configuration step 2 but isn't visible there; a shared sub/superscript flag would also change the configuration table and the `NH~4~^+^` row.
>
> **Rule and table inconsistencies**
>
> * The `~a~b~` row cites rule 7, but the behavior is in rule 5.
> * Rows citing "8, 10" show nesting, not interleaving, so "eight rows citing rule 10" needs rewording.
> * Add the hard-break backslash to rule 9's text.
> * Say what `~~~` does adjacent to a pair (`~a ~~~ b~`); not in the test cases.
> * Add a `- ~~~` fence row for list items.
> * State how the newline notation in the tables is encoded.
>
> **Missing test cases**
>
> * Subscript inside a table cell with an escaped pipe.
> * Reference links with subscript in the label (`[~x~][r]`).
> * Unicode punctuation openers other than quotes.
> * `subscript` off with `~x~` adjacent to `~~`.
> * Formatter rows for headings, lists, blockquotes and tables.
> * A line-wrap case where a break would put `~` at the next line start (`aaaa ~x~`).
>
> **Formatting nits**
>
> * Some table rows are not aligned and one line is very long.
> * State once that `U+00A0` is a literal no-break space.
>
> **Overall:** the parser half is solid and well sourced. The formatter half and the interleaving cases need work before implementation starts.

The review above is shortened from the full output. After it, the following were applied: items 2, 3 and 4 of the substantive issues, the `~a~b~` rule tag, and a note on the `*` escaping question. The requests for more test rows were not applied.

Final submission
----------------

### Anuar Kurmanov

GitHub userid: FIFSAK

#### Contributions

Rewrote the specification around the formatter, following the instructor's
review that dprint formats Markdown rather than rendering it, implemented the
`subscript` option with an LLM (configuration, parser, formatter, spec files and
a formatting fuzz test), and ran its five specification and two implementation
review rounds.

#### Specification review rounds

LLM: Claude Sonnet 5.5 (`claude-sonnet-5-5`), default effort, a fresh session
each round.

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1     | 8        | Goal reworded to "writes nothing that parses differently"; configuration files named; schema warns that `10~20 and 30~40` becomes subscript; reference-link text writes `~`; unequal-run and link rows added |
| 2     | 7        | `Subscript` arm in `written_delimiter` stated; off rows moved to `TextDecoration_All.txt`; idempotence check named; `~a~~b~` and `~~a ~b~ c~~` rows replace a row that tested nothing |
| 3     | 9        | CommonMark flanking rules stated to still apply; reference-link row added; the existing `~x~` off row reused; source of the pulldown-cmark behavior named |
| 4     | 10       | `parse` and `parse_inline_content` signatures and callers stated; off mode keeps `Strikethrough`; wrap case moved to its own file, since it needs `textWrap: always` |
| 5     | 8        | Wording only: the callers that pass `false` named correctly; the rule of three is off only with `subscript` on; why `~a ~~b~` is text, and its test name |

Implementing the specification found one rule it missed: the CommonMark "rule
of three" made `~a~~b~` a subscript, so parser change 3 was added between rounds
4 and 5. Findings asking for more test rows of the same kind, or for wording
changes, were mostly not applied, to keep the specification short.

#### Last specification review

Prompting Claude Sonnet 5.5 (`claude-sonnet-5-5`), default effort, in a fresh
session with `Review the section named "Subscript support" in
src/parser/subscript-spec.md. Do not edit any files; return the review as your
final message.` produced the following output; strikethrough marks findings I
judged incorrect or irrelevant.

> I checked the spec against inline.rs, generate.rs, mod.rs, format_range.rs, format_text.rs, the configuration, schema.json and the specs. I did not run the tests or check the pulldown-cmark claims at commit 1c1a312.
>
> 1. ~~The spec is written in the future tense ("Add the option", "gains") although the work is already in the tree; it should say it records the implemented design.~~
> 2. "The HTML tests pass false" is wrong: the `html/` parser has its own `parse` with no subscript argument. The callers that pass `false` are `src/parser/spec_test.rs`, `tests/parser_spec_test.rs` and `tests/commonmark_spec_check.rs`; `collect_labels` takes the flag too.
> 3. Parser change 3 says the rule of three "doesn't apply to `~`", but the code disables it only with `subscript` on; say so.
> 4. The "nearest opener" paragraph is half right: `find_opener` returns the nearest `~` opener of any length and `wrap_emphasis` gives up on a mismatch, which also blocks a closer behind it (`~a ~~b~ c~`). State the mechanism.
> 5. The test name "should leave a double tilde in a subscript as text" for `~a ~~b~` hides that this is a known divergence from pulldown-cmark; the name and table row should say so.
> 6. ~~A one-line pointer from each table row to its spec case name would make drift easy to catch.~~
> 7. ~~The spec could say more strongly that users with prose like "approx 10~20" should leave the option off.~~
> 8. ~~The goal says the formatter "writes nothing that parses differently", but the fuzz test only fails when the option-off run passes; the spec should state the weaker guarantee.~~

The review above is shortened from the full output. Items 2 to 5 were applied.
Item 1 is struck because a specification states what to build; item 6 because
the table and the spec files are short enough to compare by eye; item 7 because
the schema description already warns about it; item 8 because the spec files
check the goal directly, and the fuzz test only adds coverage.

#### Implementation review rounds

LLM: Claude Sonnet 5.5 (`claude-sonnet-5-5`), default effort, a fresh session
each round. The implementation itself was written with Claude Opus 5.5
(`claude-opus-5-5`).

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1     | 7        | No bugs found; parser fuzz now runs with `subscript` on and off; formatting fuzz test added; rows for nested subscript, subscript against emphasis and `~a ~~b~` added; stale comment fixed |
| 2     | 6        | No bugs or spec violations found; schema section description mentions subscript; one spec sentence that overstated how tildes pair fixed |

The formatting fuzz test also found an existing idempotence bug unrelated to
subscript: `__ ` then a line `_` is written as `__ _`, which then becomes a
thematic break `---`. It happens with the option off too, so it was left alone.

#### Last implementation review

Prompting Claude Sonnet 5.5 (`claude-sonnet-5-5`), default effort, in a fresh
session with `Review the implementation of the "Subscript support" section of
src/parser/subscript-spec.md. The implementation is the uncommitted diff (git
diff, plus untracked files in tests/specs/TextDecoration/). Check that it does
what the spec says, and look for bugs. Do not edit any files; return the review
as your final message.` produced the following output; strikethrough marks
findings I judged incorrect or irrelevant.

> No spec violations or functional bugs; the findings are minor gaps. `cargo test` passes in full, every row of the spec's test table is present and passes, and about 80 extra inputs at line widths 80, 6 and 10 kept their subscripts.
>
> 1. **Closer with an unequal-length opener.** `find_opener` returns the nearest opener whatever its length, so `~~a ~b~~` and `~~This ~is~~ stricken.~` aren't strikethrough where pulldown-cmark reads `<del>`. The behavior predates this diff and the output text is unchanged, but it conflicts with the spec's phrase "tildes pair only with a run of the same length".
> 2. ~~**The formatting fuzz test is weaker than the spec's wording.** It excuses a bug that also happens with the option off, compares only the flat list of decoration kinds, never runs with `textWrap` maintain, and uses a fixed seed.~~
> 3. ~~**The parser unit test checks only top-level children.** Nested cases are covered only by the spec files.~~
> 4. **The schema puts `subscript` in the "Emphasis and line breaks" section**, whose description doesn't mention it.
> 5. ~~**The spec file itself is rewritten in the diff**, and cases of the draft such as `a~(b)~` and `~a~b~` no longer have tests; probes show they come out unchanged.~~
> 6. ~~**Minor test gaps:** no test that `subscript` defaults to false, no `format_range` test with the option on, and the builder test only bumps the key count.~~

The review above is shortened from the full output. Items 1 (the spec sentence)
and 4 were applied. Item 2 is struck because a fixed seed keeps the test
reproducible and a bug shared with the off mode isn't subscript's; items 3, 5
and 6 ask for tests of what the spec files and fuzz test already cover.

### Jacob Timmons

GitHub userid: jacobtimmons03-cyber

#### Contributions

Added a subscript-enabled range-formatting regression fixture that checks selected-paragraph formatting, preservation of single-tilde subscript syntax, unchanged surrounding paragraphs, and repeated-format stability.

Commit: `05705c03cda0f809f35b495c3f4b1e8234f109f7` (`tests/specs/Range/Range_Subscript_On.txt`). The fixture passed independent review with zero findings and was pushed to team PR #3.

#### Specification review rounds

Model: `gpt-6-astra`. All four final-stage reviews examined commit `05705c03cda0f809f35b495c3f4b1e8234f109f7`; they are separate from the draft reviews and the focused pre-commit fixture review.

| Round | Critical | Major | Minor | Verdict | Main changes made |
| ----- | -------- | ----- | ----- | ------- | ----------------- |
| 1 | 0 | 0 | 0 | READY | No changes required. |
| 2 | 0 | 0 | 0 | READY | No changes required. |

#### Last specification review

Model: `gpt-6-astra`. Exact Round 2 prompt:

```text
Literate Programming Project 2 Final — Jacob Timmons
Final Specification Review, Round 2

Repository:
C:\Users\jacob\Documents\Codex\dprint-plugin-markdown-project2-team

Branch: subscript-spec
Reviewed HEAD: 05705c03cda0f809f35b495c3f4b1e8234f109f7
Specification: src/parser/subscript-spec.md
Team PR: https://github.com/bjones1/dprint-plugin-markdown/pull/3

Perform a second independent READ-ONLY specification review.

Round 1 returned READY with 0/0/0 findings. Do not simply repeat that review or invent findings to justify another round.

Focus this pass on whether the written specification is complete, unambiguous, internally consistent, and independently testable, especially for:
- Enabled versus disabled subscript behavior
- Tilde-run interactions and strikethrough compatibility
- Delimiter boundaries, escaping, whitespace, and nesting
- Wrapping, formatting stability, and configuration examples
- Any mismatches between normative rules and examples

Use the relevant existing source and fixtures to verify concrete claims, but do not assume implementation behavior is automatically correct when judging the specification. Respect documented intentional deviations from pulldown-cmark.

Report only genuine, actionable defects, with severity, exact location, evidence, and minimal suggested correction. Exclude cosmetic suggestions, speculative cases, and unrelated functionality.

Include:
- Reviewed HEAD and working-tree state
- Critical / Major / Minor finding counts
- Verdict: READY or NEEDS_CORRECTION
- Exact model identifier
- Complete review response suitable for submission.md

STRICTLY READ-ONLY. No edits, tests that change files, commits, pushes, merges, or PR modifications.
```

Complete Round 2 response:

**Literate Programming Project 2 Final — Jacob Timmons**
**Final Specification Review, Round 2**

- **Branch:** `subscript-spec`
- **Reviewed HEAD:** `05705c03cda0f809f35b495c3f4b1e8234f109f7`
- **Working tree:** Clean before and after review.
- **Exact model identifier:** `gpt-6-astra`

**No substantive actionable defects found.**

This pass checked the written requirements and examples for ambiguity and contradictions, using source and fixtures as supporting evidence:

- Enabled and disabled behavior is distinguishable and testable. The default-off compatibility requirement and single-/double-tilde rules agree with the corresponding examples.
- The specification explicitly retains existing delimiter, escape, code-span, and link rules except for its listed changes. The unequal-run behavior at lines 60–62 is a documented intentional deviation, not an unresolved contradiction.
- Nested formatting, escaped tildes, literal code, reference links, and whitespace examples agree with the stated rules.
- Formatting stability and semantic preservation are separate requirements at lines 71–76. The wrapping example identifies a fixture that explicitly supplies `textWrap: always` and `lineWidth: 6`; emphasis examples use the documented underscore default.

I found no material ambiguity requiring a new design decision or additional specification content. No tests were run during this round, and no files or PR state were changed. This is the complete review response.

- **Critical / Major / Minor finding counts: 0 / 0 / 0**
- **Unresolved blocking decisions: None identified**
- **Verdict: READY**

#### Implementation review rounds

Model: `gpt-6-astra`.

| Round | Critical | Major | Minor | Verdict | Main changes made |
| ----- | -------- | ----- | ----- | ------- | ----------------- |
| 1 | 0 | 0 | 0 | READY | No changes required. |
| 2 | 0 | 0 | 0 | READY | No changes required. |

#### Last implementation review

Model: `gpt-6-astra`. Exact Round 2 prompt:

```text
Literate Programming Project 2 Final — Jacob Timmons
Final Implementation Review, Round 2

Repository:
C:\Users\jacob\Documents\Codex\dprint-plugin-markdown-project2-team

Branch: subscript-spec
Expected HEAD: 05705c03cda0f809f35b495c3f4b1e8234f109f7
Team PR: https://github.com/bjones1/dprint-plugin-markdown/pull/3
Specification: src/parser/subscript-spec.md

Perform an independent, READ-ONLY second final-stage implementation review.

Implementation Review Round 1 returned READY with zero findings and passed the complete Rust test suite. Specification Reviews 1 and 2 also returned READY. Do not repeat earlier conclusions without checking them or manufacture new issues.

Focus specifically on remaining integration and regression risks:
1. Confirm subscript disabled by default does not unintentionally change existing single-tilde or double-tilde behavior.
2. Inspect enabled-mode handling at delimiter boundaries: unequal tilde runs, punctuation, escaping, whitespace, nested inline formatting, code spans, and links.
3. Verify parser-to-formatter consistency, including semantic preservation and repeated formatting.
4. Examine range-formatting integration and the new Range_Subscript_On.txt regression for meaningful isolation.
5. Check configuration propagation through relevant parsing, formatting, and reparsing paths.
6. Identify concrete gaps only if supported by the specification and implementation evidence.

Use focused tests or minimal reproducible inputs when needed. The full suite passed in Round 1; rerun it only if useful to validate a particular finding. Report actual test execution accurately.

Report ONLY substantive, actionable defects. For each provide severity, exact source location, evidence, observable impact, and the smallest necessary fix. Do not request unrelated refactoring, speculative coverage, or new features.

Finish with:
- Reviewed HEAD and working-tree state
- Critical / Major / Minor finding counts
- Tests executed and results, or explicit verification limits
- Verdict: READY or NEEDS_CORRECTION
- Exact model identifier

Return the complete review response for preservation in submission.md.

STRICTLY READ-ONLY: No edits, staging, commits, pushes, merges, configuration changes, or PR modifications.
```

Complete Round 2 response:

**Literate Programming Project 2 Final — Jacob Timmons**
**Final Implementation Review, Round 2**

**No substantive actionable defects found.**

Rechecked configuration propagation through initial parsing, formatter reparsing, and range replacement validation. The flag defaults to `false`; enabled-only parser changes remain gated appropriately. Delimiter matching, escapes, code spans, nesting, and link handling are consistent with the approved specification, including its deliberate unequal-run behavior.

The range regression meaningfully checks isolation: the selected paragraph changes from `*~x~*` to `_~x~_`, while surrounding emphasis remains untouched. Its expected-range markers also enable repeated-format validation.

Focused tests independently executed this round:

| Command                                               | Result                                                              |
| ----------------------------------------------------- | ------------------------------------------------------------------- |
| `cargo test --locked --lib subscript`                 | PASS — 2 tests, including generated preservation/idempotence checks |
| `cargo test --locked --test specs Subscript`          | PASS — 3 fixture files, including wrapping and range isolation      |
| `cargo test --locked --test specs TextDecoration_All` | PASS — default-off decoration compatibility fixture                 |

The full suite was not rerun this round. These focused results and source inspection revealed no reproducible integration regression; they do not establish correctness for every possible input.

No source, fixture, configuration, Git, or PR changes were made. This is the complete review response.

- **Reviewed branch:** `subscript-spec`
- **Reviewed HEAD:** `05705c03cda0f809f35b495c3f4b1e8234f109f7`
- **Working-tree state:** Clean before and after review
- **Critical / Major / Minor:** **0 / 0 / 0**
- **Tests executed:** All three focused commands above passed
- **Verdict:** **READY**
- **Exact model identifier:** `gpt-6-astra`

### Amir Adilov

GitHub userid: amir-kbtu-pge

(Amir's final contributions, review tables and last reviews)

### Dias Lesbay

GitHub userid: diaslesbay2003kbtu

#### Contributions

Added `TextDecoration_Subscript_Emphasis.txt`, seven fixture cases that check
the formatter reads interleaved subscript and emphasis, and punctuation beside a
`~`, the way pulldown-cmark does (each result measured at commit `1c1a312`),
and named the file in the specification's test plan.

The cases were written with Claude Sonnet 5.5 (`claude-sonnet-5-5`) through
the `common:implement` workflow, one case at a time. Every case was already
green, so no production code changed. Before this, I wrote a rewrite of the
specification around the formatter goal with two review rounds (11 and 9
findings); the team's rewrite replaced it, so those rounds are not counted here.

#### Specification review rounds

LLM: Claude Sonnet 5.5 (`claude-sonnet-5-5`), default effort, a fresh session
each round, on the team's specification.

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1     | 23       | Test plan names the new fixture file; a paragraph on interleaving and punctuation cases added; most requests for more test rows not applied |
| 2     | 14       | Goal no longer promises every subscript pulldown-cmark reads, and points to the known difference; the rest was wording or tests already covered elsewhere |

Findings were counted from each review's numbered items and bullets.

#### Last specification review

Prompting Claude Sonnet 5.5 (`claude-sonnet-5-5`), default effort, in a fresh
session with `Review the section named "Subscript support" in
src/parser/subscript-spec.md. Do not edit any files; return the review as your
final message.` produced the following output; strikethrough marks findings I
judged incorrect or irrelevant.

> I read the file only. I did not check the claims about `inline.rs`, `generate.rs` or pulldown-cmark against the code.
>
> 1. The Goal says every subscript stays a subscript, but lines 60-62 admit that `~a ~~b~` is a subscript in pulldown-cmark and text here. Weaken the Goal and list the deviation.
> 2. The rule-of-three change affects `~~` too; say so and add mixed rows (`~~a~b~~`, `~a~~b~~`).
> 3. ~~Intraword flanking has a large blast radius (`10~20 and 30~40`); the schema text must name the example.~~
> 4. ~~The test table lacks a "pulldown-cmark reads as" column; the wording "measured ... for the draft" will go stale.~~
> 5. ~~Missing test cases: unmatched runs, subscript in strong or headings, soft breaks, `format_range` with the option on, config tests.~~
> 6. ~~Clarity: define "a pair of single tildes"; make the existing `~` behavior explicit in one place; reference-link wording; hard wrapping; the H1 id; a README or changelog entry.~~

The review above is shortened from the full output. Item 1 was applied. Item 2
was not applied, because the mixed rows were not measured. Item 3 is struck
because the schema description already names the example; item 4 because the
spec files, not a table column, hold the expected output; item 5 because the
cases it lists are covered by the existing fixtures, and `format_range` with the
option on is already tested by `Range_Subscript_On.txt`; item 6 because it asks
for wording and wrapping changes only.

#### Implementation review rounds

LLM: Claude Sonnet 5.5 (`claude-sonnet-5-5`), default effort, a fresh session
each round. The implementation was written with the same model.

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1     | 7        | Expectations traced by hand (the reviewer could not run `cargo`): none wrong; two cases holding two inputs each split into separate tests; every input measured at `1c1a312`; the file named in the spec |
| 2     | 6        | No wrong expectations; one test title that misdescribed which delimiter is left as text renamed; the spec paragraph now describes both kinds of case |

Cases 1, 4 and 5 of the file would give the same output with `subscript` off
(emphasis wins anyway), so they guard against a subscript over-matching, not
against the option being inactive. The other four depend on the option.

#### Last implementation review

Prompting Claude Sonnet 5.5 (`claude-sonnet-5-5`), default effort, in a fresh
session with `Review the implementation of the "Subscript support" section of
src/parser/subscript-spec.md. The implementation is the new untracked file
tests/specs/TextDecoration/TextDecoration_Subscript_Emphasis.txt and the changed
paragraph under "Test cases" in the spec. Check that its cases agree with what
the spec says and look for wrong expectations or tests that would pass for the
wrong reason. Rust is available at /opt/homebrew/opt/rustup/bin (add it to
PATH); you may run `cargo test --test specs Subscript`. Do not edit any files;
return the review as your final message.` produced the following output;
strikethrough marks findings I judged incorrect or irrelevant.

> The new spec file matches the spec paragraph and pulldown-cmark, and I found no wrong expectations. `cargo test --test specs Subscript` passes all 4 spec files, and all 7 cases run and pass with `subscript: true`. I ran all 7 inputs through pulldown-cmark 0.13.6 (the registry version, not the pinned commit); every expectation agrees.
>
> 1. The first, fourth and fifth cases would pass with the option off; they guard against over-matching, not against the option being inactive.
> 2. The new spec paragraph only half describes the punctuation cases.
> 3. ~~The punctuation coverage is one-sided: nothing covers a closing `~` with punctuation before it and a word after it.~~
> 4. Cases 5, 6 and 7 form a good contrast set; a wrong parse would produce `_`.
> 5. ~~Minor gaps: no interleaving with `~~` or `_`, no `emphasisKind: asterisks` variant, no rows in the spec table.~~
> 6. In the first case, "take the tilde" is slightly misleading.

The review above is shortened from the full output. Items 2 and 6 were applied;
item 1 is recorded above as a limit of those cases. Item 3 is struck because
its inputs were not measured against pulldown-cmark, and item 5 because it asks
for more rows of the same kind.
