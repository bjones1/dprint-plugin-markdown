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

(Jacob's final contributions, review tables and last reviews)

### Amir Adilov

GitHub userid: amir-kbtu-pge

(Amir's final contributions, review tables and last reviews)

### Dias Lesbay

GitHub userid: diaslesbay2003kbtu

(Dias's final contributions, review tables and last reviews)
