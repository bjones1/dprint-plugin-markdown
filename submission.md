Project 2 submission
====================

Team members are Jacob Timmons, Amir Adilov, Anuar Kurmanov, and Dias Lesbay.
The specification consists of the file
[subscript-spec.md](src/parser/subscript-spec.md).

Draft submission
----------------

### Contributions

* Jacob Timmons: (Jacob's contributions)
* Amir Adilov: ran the compiler test on macOS, settled open question 2 by
  building pulldown-cmark at the pinned commit `1c1a312` and measuring how
  subscript and emphasis resolve when they overlap, which supplied parsing
  rule 10 and its eight test rows, and argued for two options by measuring
  that `ENABLE_SUPERSCRIPT` alone leaves `~` untouched.
* Anuar Kurmanov: ran the compiler test on macOS and argued for keeping
  `subscript` and `superscript` as separate options, matching pulldown-cmark's
  separate `ENABLE_SUBSCRIPT` and `ENABLE_SUPERSCRIPT` flags.
* Dias Lesbay: drafted the "Subscript support" specification, verified its
  parser expectations against pulldown-cmark, and ran and triaged the ten LLM
  review rounds.

### Compiler test

The following statements summarize LLM output on each team member's computer,
based on the prompt "Verify that you can compile and run the project in the
current git repo."

#### Jacob Timmons

(Summarized LLM output)

#### Amir Adilov

Rust was not installed, so it was installed with rustup; the toolchain is
pinned to 1.98.0 by `rust-toolchain.toml`, which rustup fetched on the first
build. `cargo build` succeeded and `cargo test` passed 192 tests plus 2
doc-tests, with no failures. The project is a library with no binary, so
`format_text` was called from a scratch crate to check that it runs: it
collapsed the extra spaces in `Chemistry:   H~2~O  and   ~~gone~~` and
rewrote `*` bullets as `-`.

#### Anuar Kurmanov

Rust was not installed, so it was installed with rustup (`rustc` 1.98.0). Then
`cargo build` succeeded and `cargo test` passed 182 tests across the unit and
integration test binaries, all spec-file suites, and 2 doc-tests, with no
failures. The project is a library, so there is no binary to run.

#### Dias Lesbay

`cargo build` succeeded and `cargo test` passed all 192 tests plus 2 doc-tests;
the project is a library, so there is no binary to run, and it was not run
through the dprint CLI. `cargo` was not on `PATH` and had to be added from the
rustup install.

### Spec review rounds

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

### Last LLM review

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
