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

GitHub userid: Kenzpie

#### Contributions

Revised the superscript specification and completed review rounds 6–8.

#### Build and run test

Prompt: "Verify that you can build and run the project in the current git repo."

Summarized output from Amy's own computer: **[Amy must add her OS, architecture, Rust/Cargo versions, commands, and results.]**

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 6 | 9 | Cleaned Markdown; clarified footnote scope, parsing rules, compatibility decisions, and test requirements. |
| 7 | 5 | Resolved newline contradictions; separated renderer and parser expectations; clarified hard-break and metadata preservation. |
| 8 | 4 | Defined rewrite safety, bounded project scope, specified fallback regions, and separated requirements from implementation guidance. |

#### Last specification review

LLM: lowest paid tier of ChatGPT; the specific model name was not recorded.

Prompt: "Please conduct the next review round focusing on bigger issues, logic flaws, and test erros."

Output: "The spec’s detailed syntax coverage is now substantial. I would prioritize a concrete safety strategy and a bounded scope moving forward."

### Alisher Arginbekov

GitHub userid: aarginbekov

#### Contributions

Added nested superscript/strikethrough examples and fixture requirements in commits `74429c8` and reviewed the spec for missing HTML-context fixtures in round 9; the HTML additions were removed in Dias's round 10 rewrite after the instructor requested a narrower scope.

#### Build and run test

Prompt: "Verify that you can build and run the project in the current git repo."

Environment: Windows 11 Pro 10.0.26200, x64. Neither `cargo` nor `rustc` is on PATH, and neither exists at `C:\Users\sem1c\.cargo\bin`; therefore I could not build or run the project in this environment. No build or test command was run.

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 9 | 1 | Identified missing HTML-context fixtures; these were later removed in round 10 to follow the instructor's request to narrow the specification. |

#### Last specification review

LLM: OpenAI Codex, GPT-6.

Prompt: "Review the superscript spec for claims that have no required acceptance fixtures. Focus on inline HTML: distinguish carets in HTML attributes, text between inline tags, and raw HTML blocks. Report only concrete coverage gaps and propose the smallest spec change."

Output: The spec described how superscript markers behave in HTML attributes, inline text between tags, and HTML blocks, but did not require fixtures for these cases. I added examples and requirements to check the renderer output, parser AST, and formatter behavior. These additions were later removed in the round 10 scope reduction.

### Aslan Tasmagambetov

GitHub userid: tasma-07

#### Contributions

Supplied the specification version used as the basis for the PR and completed review rounds 2–3.

#### Build and run test

Prompt: "Verify that you can build and run the project in the current git repo."

Summarized output from Aslan's own computer: **[Aslan must add his OS, architecture, Rust/Cargo versions, commands, and results.]**

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 2–3 | 3 main | Made caret runs count as pairable; rewrote footnote handling and added wikilink, heading-attribute, and formatter-configuration cases. |

#### Last specification review

The supplied PR material does not identify Aslan's LLM, last-round prompt, or output. **[Aslan must add these details.]**

Final submission
----------------

(To be filled in for the final submission.)
