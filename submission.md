Project 2 submission
====================

Team name: dprint superscript

The specification consists of the file [superscript-spec.md](superscript-spec.md).

Draft submission
----------------

### Dias Jakupov

GitHub userid: diasjakupovkbtu

#### Contributions

Wrote the first spec draft and opened the PR, ran review rounds 1, 4 and 5,
and rewrote the spec after the instructor's review (round 10), checking it
against a prototype implementation.

#### Build and run test

Prompt: "Verify that you can build and run the project in the current git
repo." Summarized output, on commit `54bcb94` (code identical to `main`
`7428123`):

- Environment: macOS 26.6.2 (arm64), rustc 1.98.0 and cargo 1.98.0 (pinned by
  `rust-toolchain.toml`).
- `cargo build --locked`: finished, exit code 0.
- `cargo test --locked`: exit code 0. Unit tests 160 passed; integration
  tests 10 + 1 + 2 + 9 + 2 passed; spec runners 192 (formatter), 39 (parser)
  and 8 (HTML parser) passed; 0 failed.

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1     | 7        | A line break next to a superscript is never dropped (CJK text); listed where pulldown-cmark's shared `^`/`~` lower bound differs from `_`; named the `generate.rs` changes; more tests |
| 4     | 12       | Setext heading could still drop a break next to a superscript; footnote inside link text; `[^a^][x]` is not a footnote; formatter test configs fixed |
| 5     | 10       | A hard break right after a `^` run is written as it was; footnotes also stop earlier image brackets; math and `*`/`_` in footnote names added as known differences |
| 10    | 5        | Rewrite after instructor review (663 → 162 lines, formatter only); fixed: `^` after `[` no longer opens, hard-break rule restored, openers-bottom wording, Goal config note, whitespace in the test rule |

#### Last specification review

LLM: Claude Opus 5.5, run as a fresh reviewer that had not seen the rewrite
being written. It had the repo, a prototype implementation of the spec, and a
probe that formats input and renders input and output with pulldown-cmark
0.13.6.

<details>
<summary>Prompt</summary>

> Check `superscript-spec.md` for factual accuracy. Do not edit any file.
> Check, with evidence (command output or file:line): (1) every row of the
> Goal table on today's formatter; (2) every row of the Syntax table against
> pulldown-cmark 0.13.6 and the prototype parser; (3) the known-difference
> claim; (4) every function name, file and behavior claim in "Data
> structures" and "Changes", and whether the prototype diff matches the spec;
> (5) every row of the Tests tables, and whether the full test suite passes;
> (6) anything else the spec claims that is wrong, or a case where the
> prototype formatter still creates, removes or changes a superscript.
> Report concrete problems (claim, evidence, fix) ranked by severity, then
> what you verified as correct.

</details>

Output (summarized). All five findings were reproduced by hand and applied;
none was wrong, so none is struck.

1. "A footnote reference `[^1]` is read as it is today" is false: the `^` in
   `[^1]` became an opener, so `x ^␣␣⏎[^1] b^` still gained a superscript.
   Fix: a `^` right after `[` cannot open.
2. The known-difference paragraph misdescribed the parser's openers bottom
   (it is keyed by character, length mod 3 and can-open) and claimed such
   input always formats unchanged, which failed with a hard break after it.
3. The read-back check misses a hard break that moves a superscript without
   changing its text (`^␣␣␣⏎.^.^ !`). Fix: write a hard break after `^` as it
   was.
4. Goal row 4 also happens with `textWrap: never`; rows 1-3 with any
   `textWrap`.
5. The test rule must allow whitespace-only differences in rendering.

Verified correct: all Goal and Syntax rows, every named function, the
prototype diff matching the spec, every test row, the full test suite, and
about 10,000 fuzzed inputs with no other superscript change.

### Amy Smith

(Same sections as Dias Jakupov)

### Alisher Arginbekov

(Same sections as Dias Jakupov)

### Aslan Tasmagambetov

(Same sections as Dias Jakupov)

Final submission
----------------

(To be filled in for the final submission.)
