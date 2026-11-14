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

Output. All five findings were reproduced by hand and applied. Finding 1 was
fixed another way: instead of reading `[^label]` as text, a `^` right after
`[` cannot open. One verified-correct claim turned out wrong and is struck.

<details>
<summary>Full review</summary>

##### Problems, most severe first

1. **"A footnote reference `[^1]` is read as it is today" is false, and it leads to rendering changes.** The parser has no footnote-reference node, so the `^` in `[^1]` becomes an opener. The prototype parses `[^1] text^` as `[` followed by Superscript `1] text` (AST dumped with UPDATE=1 in a scratch copy). Because of this, the prototype still gets two of the Goal-table cases wrong, in maintain mode at width 80:
   - `^^[^1]^␣␣⏎b` → `^^[^1]^\⏎b`: pulldown loses `<sup>`. Same with `^^x [^1]^␣␣⏎b`.
   - `x ^␣␣⏎[^1] b^` → `x ^\⏎[^1] b^`: pulldown gains `<sup>`.
   - The prototype also stops rewriting `*`→`_` in `[^1] *a^ b*`. That one is harmless.
   
   **Fix:** have the inline parser read `[^label]` as text before emphasis (pulldown does this whenever it is not followed by `(`, so `[^a](u)` is a link). Add these cases to both test files.

2. **The known-difference paragraph has two errors.**
   - "The formatter writes such input back unchanged" is false when a hard break follows. `^a .^^. b.^␣␣⏎(c^` → `…b.^\⏎(c^` creates `<sup><br/>(c</sup>`, and `.^. .^^.^␣␣⏎.^.` does the same.
   - "One bottom per character" misdescribes the parser. It keys the bottom by `(byte, len%3, can_open)` (`inline.rs:805`). pulldown keeps one slot for all `^` and `~` runs (`parse.rs:1648-1671`). The `^a .^^. b^` case comes from pulldown not splitting by length or can-open. Sharing the slot with `~` causes a second case: `^a ~~b~ c^` has no superscript in pulldown, but this parser reads one.
   
   **Fix:** correct the wording, and either stop claiming the output is unchanged or handle the hard-break case.

3. **"The read-back check catches a rewrite that would create or remove a superscript" overstates it.** The check compares each decoration's kind and text with backslashes and whitespace removed (`generate.rs:165, 198`). A hard-break rewrite can move a superscript and still pass: `^␣␣␣⏎.^.^ !` → `^\⏎.^.^ !` turns `<sup>.</sup>` into `<sup><br/>.</sup>.^`. `_` has the same flaw today. **Fix:** list it as a limitation, or count hard breaks in the comparison.

4. **Goal row 4's config note is incomplete.** The CJK join also happens with `textWrap: never`. Rows 1–3 happen with maintain, always and never alike. In pulldown's HTML, row 4 differs only by `⏎` versus a space. **Fix:** say "always or never", and say rows 1–3 occur under every textWrap.

5. **Minor:** for the test row `a ^b c d e f g h i j^ k`, the input and output HTML differ by space versus `⏎`. "Render the same way" should say "apart from whitespace".

Every case in 1–3 behaves the same on today's formatter, so none of them is a regression. They contradict what the spec claims the change achieves.

##### Verified correct
- **Goal table:** all four rows on today's formatter (`probe`), including the stated HTML changes.
- **Syntax table:** all 10 rows match pulldown 0.13.6, and `superscript.txt` has matching ASTs. `[^1]` on its own is Text.
- **Known difference:** `^a .^^. b^` has no superscript in pulldown and one in the prototype, and that exact input formats unchanged.
- **Code:** every named function exists. `flanking()` puts `^` in the `_` branch. `wrap_emphasis` uses 2 carets when both runs have at least 2. `written_delimiter` uses the offset rule, and `resolve`, `is_delimiter`, the three run helpers and the `commonmark_spec_check` "sup" arm all match the spec. The prototype diff matches the spec exactly, with nothing missing or extra.
- **Tests:** every formatter test row gives exactly the stated output with the stated config, and the test files match the tables.
- **Test suite:** the full `cargo test --offline` in proto passes.
- ~~**Own fuzzing:** about 10,000 of my own inputs across maintain, always, never, sentence and maintainAndWrap, at widths 3–80. Ignoring `~` and whitespace-only HTML differences, I found no other superscript changes beyond items 1–3.~~ (wrong: a later review found `[^a *b^ c*` and `[^␣\⏎b^`, which change the same way on today's formatter)
- **Not checked:** the claim about `x^2^` in pulldown master and Pandoc (no network).

</details>

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

Summarized output on commit `f0d4544`:

- Environment: Windows 11 Pro 10.0.26200, x64; rustc 1.98.0 and cargo 1.98.0 (pinned by `rust-toolchain.toml`); Visual Studio Build Tools 2022 17.14.45 with the MSVC x64 toolchain and Windows 11 SDK 10.0.26100.
- `cargo build --locked`: finished, exit code 0.
- `cargo test --locked`: exit code 0; 160 unit tests, 10 CommonMark checks, 1 corpus check, 8 HTML parser tests, 2 nesting tests, 9 newline tests, 39 parser spec tests, 192 formatter spec tests, and 2 doc tests passed; 0 failed.

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
