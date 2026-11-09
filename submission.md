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
  parser expectations against pulldown-cmark, and ran and triaged the nine LLM
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
| 9     | 22       | Open question on overlap with emphasis added; the rest, mostly requests for more test rows, not applied |

Findings were counted from each review's numbered and bulleted items.

### Last LLM review

Prompting Claude Sonnet 5.5 (`claude-sonnet-5-5`), default effort, in a fresh
session with `Review the section named "Subscript support" in
src/parser/subscript-spec.md. Do not edit any files; return the review as your
final message.` produced the following output.

> The tilde rules and test tables are mostly consistent.
>
> **Gaps and ambiguities that would block implementation**
>
> 1. Interaction with emphasis is unspecified (`~a *b~ c*`, `*a ~b* c~`, `**a ~b** c~`). Add a rule and rows.
> 2. Rule 5 leaves "punctuation" undefined; the `a~“b”~` row depends on Unicode punctuation, not only ASCII.
> 3. `~~` intraword behavior is deferred to `flanking` in `inline.rs`; the `a~~x~~b` row cites rule 4 but depends on rule 5.
> 4. The "off" behavior is asymmetric and under-tested; no off-mode rows for nesting or `~a~b~`.
> 5. Rule 4's "makes every opener before it unavailable" would be clearer as a short stack description.
> 6. Rule 2's fence definition does not cover tabs, list-item indentation or nested containers such as `- ~~~`.
>
> **Missing test cases**
>
> * A formatter row for the wrapping refusal before a `~~~` run.
> * Strikethrough inside subscript and the reverse.
> * Table cells with `~` next to pipes.
> * Hard-break and soft-break formatting when the next line starts with `~`.
> * Escaped-tilde round trips with the option off.
> * Run lengths above two inside nesting (`~a ~~~ b~`).
> * Reference-style links and footnote labels containing `~`.
> * A formatter row for `[~x~](u)`.
>
> **Internal inconsistencies and wording**
>
> * Some "text" rows cite only one rule where rule 5 also applies; cite all or drop the column.
> * Say which rules were verified by running the commit and which by reading source.
> * Make clear that configuration step 2 is blocked on the open question.
> * Mark the `NH~4~^+^` row as temporary.
> * State formatting rule 2 as a decision procedure (re-parse the candidate and compare).
>
> **Presentation**
>
> * The raw `<h1 id=...>` heading versus setext headings.
> * Some lines run past the wrap width and break table alignment.
> * Spell out "run length" consistently.
>
> **Verdict:** careful and mostly self-consistent, with strong parser tables. It needs a defined interaction with emphasis, a defined punctuation set and intraword `~~` rule, and the missing formatter wrapping and escape rows.

Only the open question on emphasis overlap was applied after this review.
