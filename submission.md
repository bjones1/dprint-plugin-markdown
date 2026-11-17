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

- Environment: Windows; VS Code PowerShell terminal; Cargo 1.98.0.
- `cargo build`: completed successfully.
- `cargo test`: the provided output showed 192 specification tests
  and 2 documentation tests passed, with no failures shown.
- Execution was verified through the test suite; the plugin was
  not separately tested inside dprint.

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

Prompt: "Inspect the OS, architecture, shell and installed toolchain versions
relevant to this repository, and check its configuration and CI files to
identify the intended build and test setup. Then build the project and run its
tests."

- Environment: Windows 11 Home Single Language 10.0.26200 (build
  26200.9711), x64; Command Prompt (`cmd.exe`); rustc 1.98.0 and cargo
  1.98.0, toolchain `1.98.0-x86_64-pc-windows-msvc` (pinned by
  `rust-toolchain.toml`; rustup installed it on first use).
- CI (`.github/workflows/ci.generated.yml`): Ubuntu with the toolchain from
  `rust-toolchain.toml`; builds the Wasm plugin and runs `cargo test`.
- `cargo build --locked`: finished in 33 s, exit code 0.
- `cargo test --locked`: exit code 0; 160 unit tests, 10 CommonMark checks,
  1 corpus check, 8 HTML parser tests, 2 nesting tests, 9 newline tests,
  39 parser spec tests, 192 formatter spec tests and 2 doc tests passed;
  0 failed.

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 2     | 8        | A hard-break rewrite could create a superscript, so `^` runs now count as pairable; rule 6 rewritten (a footnote only when no link forms, and its label is text); `written_delimiter()` uses the measured length; wikilinks added as a known difference; one formatter test file per config |
| 3     | 7        | Rule 6 covers `![^…]` and only valid labels (`match_label`); heading attributes added as a known difference; subscript PR #1 noted as following Pandoc; Configuration lists the hard-break change; "same delimiters" instead of "exactly as written" |

Both rounds reviewed the draft that came before the round 10 rewrite, so most
of what they changed (rule 6, footnotes, Configuration) is no longer in the
spec; counting `^` runs as pairable and measuring the delimiter in
`written_delimiter()` are.

#### Last specification review

LLM: Claude Opus 5.5 (claude.ai). Prompt (round 3, on the spec with the
round 2 fixes applied): "Review this specification again and list your
findings."

Output, shortened (`␣` is a space, `⏎` a line break). All seven findings were
applied; none was rejected.

1. Rule 6 misses image brackets: `![^a^]` is `!` followed by footnote `a^`.
2. Rule 6 doesn't say what a footnote name may be: `[^␣]` and `[^a [b] c^]`
   are not footnotes; apply it only where `match_label()` matches.
3. Heading attributes are another known difference: `# ^a {#b^}` gives
   `<h1 id="b^">^a</h1>`.
4. The subscript spec (PR #1) follows Pandoc, this one pulldown-cmark 0.13;
   both teams should pick one target.
5. Configuration misses an output change: `x ^␣␣⏎b^` now keeps its two
   spaces.
6. Goal contradicts formatting rule 1: say "with the same delimiters", not
   "exactly as written".
7. The file isn't formatted with the repo's own dprint config; run
   `dprint fmt`.

Final submission
----------------

### Dias Jakupov

GitHub userid: diasjakupovkbtu

#### Contributions

Rewrote the spec after the instructor's review, ran spec review rounds 10-12
and implementation review rounds 3-4 on Amy's implementation, and fixed what
they found in the spec, the code and the tests.

#### Specification review rounds

Round 10 is also in the draft section; it ran after the instructor's review,
so it belongs here.

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 10    | 5        | Rewrite after the instructor's review (663 → 162 lines, formatter only), checked against a prototype; fixed: `^` after `[` no longer opens, hard-break rule restored, openers-bottom wording, Goal config note, whitespace in the test rule |
| 11    | 7        | Checked against the real implementation: existing `_` bugs that now also hit `^` listed, a `^` after an escaped `\[` can open, the known-differences sentence and the hard-break rationale corrected, the render check tool named |
| 12    | 6        | Footnote-space example given its definition; tests for `^^a^^`, `x ^␣\⏎b^` and a setext heading; the `TextWrap_Always` test, not the render check, guards Goal row 4; known bug 1 says the superscript is lost; 2 findings rejected |

#### Last specification review

LLM: Claude Opus 5.5, run as a fresh reviewer that had not seen the earlier
rounds. It had the repo with the implementation and the round 11 fixes,
`main` to compare with, and `tools/superscript-render-check` to render with
pulldown-cmark 0.13.6. The prompt also had environment notes (repo path,
toolchain, scratch folder), left out here.

<details>
<summary>Prompt</summary>

> Review `superscript-spec.md` for factual accuracy against the code and the
> tests. Do not edit any tracked file. The implementation is now in the
> branch. Check with evidence (command output or file:line): (1) every row of
> the Goal and Syntax tables on the current formatter, and against
> pulldown-cmark 0.13.6; (2) every name, file and behavior claim in "Data
> structures" and "Changes" against the implementation; (3) every row of the
> Tests tables against the test files; (4) anything the spec claims that is
> wrong, unclear or missing for a reviewer who knows the existing parser.
> Report concrete problems (claim, evidence, fix) ranked by severity, and mark
> a finding "trivial" if it does not change behavior or meaning. Then list
> what you verified as correct.
>
> Output format: a heading "Problems, most severe first" with a numbered
> list, then "Verified correct" as a bullet list. Keep it under 700 words.
> Use ␣ for a significant space and ⏎ for a line break in inline examples.

Sent while it was running:

> Addition to the review task: focus on substance, not nitpicks. Report a
> problem only if it is a wrong or missing fact, a logic flaw, or a gap in
> the tests. Also flag anything in the spec that is overengineered or more
> than the change needs. Put wording or formatting remarks under one short
> "Trivial" line at the end, not as numbered findings.

</details>

Output. Findings 1, 2 and 4 were reproduced by hand and applied. The tests
from finding 2 are in `TextDecoration_Superscript.txt`. The setext one is
`^中^⏎字⏎===`, not `^^中^⏎字⏎===`: implementation round 4 removed the `^`
from `is_delimiter` that finding 2 aimed at, and this test covers the rule
that replaced it.
Finding 3 was applied as one sentence in Tests. Findings 5 and 6 are rejected
and struck. The trivial line was left as it is.

<details>
<summary>Full review</summary>

##### Problems, most severe first

1. **The example for known bug 2 (footnote space) doesn't reproduce as written.** On the branch, `x ^a^ [^1] y` comes back unchanged. On `7428123`, `x _a_ [^1] y` also comes back unchanged. The space is only dropped when the footnote has a definition:
   - `x ^a^ [^1] y⏎⏎[^1]: n` → `x ^a^[^1] y` on the branch.
   - `x _a_ [^1] y⏎⏎[^1]: n` → `x _a_[^1] y` on main.

   Fix: add `⏎⏎[^1]: n` to both examples.

2. **Test gap: three items in Changes have no test that fails without them.** I reverted each one in a scratch copy and ran `specs`, `parser_specs` and `commonmark_spec_check`:
   - **`^` in `unescaped_delimiter_runs()`**: all tests pass without it. Every test that needs the read-back check puts the break right after `^`, so the hard-break rule alone keeps them. The read-back path does matter: the branch keeps `x ^␣\⏎b^` unchanged in every `textWrap`. Without the `^` it becomes `x ^\⏎b^` and a superscript appears, which is what main does today. Add this test; it is also a missing Goal row.
   - **`^` in `decoration_keeps_its_delimiters()`**: all tests pass without it. It matters for `^^中^⏎字⏎===`. The branch writes `# ^^中^␣字`; without the `^` it writes `# ^^中^字` and the superscript is lost. Add this test.
   - **`^^` in `written_delimiter()`**: forcing `^` still passes. The only `^^` test ("runs") has three superscripts in one paragraph, so the read-back check fails and the paragraph is written verbatim. On its own line, `^^a^^` → `^a^`. Add `^^a^^` as a separate test.

   Removing the `[` rule, its backslash count, or the hard-break rule does make a test fail.

3. **Goal row 4 doesn't match the pass bar in Tests.** The render check collapses whitespace, so main's `^中^␣字` passes it: the HTML differs only by a newline vs a space. The spec counts row 4 as a rendering change, yet sets "same apart from whitespace" as the bar. Fix: say why the space is visible (browsers can drop a line break between CJK characters, not a space). Also say that the expected output in `TextDecoration_Superscript_TextWrap_Always.txt` guards row 4, not the render check.

4. **Known bug 1 understates the effect.** In `# ^中^字` the closing `^` now sits inside a word, so the superscript disappears: pulldown renders `<h1>^中^字</h1>`. The `_` version loses its emphasis the same way. The bullet only mentions the lost line break.

5. ~~**No test pins the known differences.** The claims hold today (checked below), but nothing guards them:~~
   - ~~`^a .^^. b^`, `^a ~~b~ c^`, `[^a^ b` and `[^a^](/u)` format without a rendering change.~~
   - ~~`[^a *b^ c*` → `[^a _b^ c_`.~~

   ~~Also untested: `\\[^1] b^` (an even number of backslashes, so a footnote), and rows 1-3 under `always`/`never`.~~

   (rejected: the known differences are limits of this change, not behavior to keep, so there is nothing to pin)

6. ~~**Over-built: the render-check tool adds little.** It re-checks only the 13 inputs whose outputs the spec tests already pin. CI doesn't run it, and it can't see row 4. Either run it as a test (pulldown-cmark as a dev-dependency) or drop it from the test plan.~~ (rejected: it shows that each expected output keeps pulldown-cmark's rendering, which the spec tests alone can't show)

Trivial: the Data structures formula `children.first().span().start - span.start` is `content_span(children).map_or(1, …)` in the code. Bug 3 lacks the `_` "today" example the other bullets give (on main, `_a␣␣⏎*⏎b_` also turns into a list).

##### Verified correct

- **Goal table.** `7428123` = `origin/main`. Rows 1-3 change rendering on main in maintain, always and never. Row 4 changes only in always/never. The branch keeps all four, and the pulldown-cmark 0.13.6 HTML is as described.
- **Known bugs 1, 3, 4.** Branch outputs are as stated, and main writes the `^` forms back safely. The `_` forms already break on main.
- **Regression sweep.** I generated 200k short random inputs across 5 configs. Every case that renders correctly on main but not on the branch falls under bugs 1, 3 or 4. (The inputs had no footnote definitions, so bug 2 couldn't show up.)
- **Syntax table.** All 11 rows match both the parser ASTs in `tests/parser_specs/superscript.txt` and the pulldown-cmark 0.13.6 HTML.
- **Known differences.**
  - pulldown keeps one bound for both `^` and `~` (`lower_bounds[TILDES]`, parse.rs:1634/1670).
  - This parser's key is `(byte, original_len % 3, can_open)` (inline.rs:812).
  - From AST dumps: `^a .^^. b^` and `^a ~~b~ c^` each give one superscript here, none in pulldown.
  - `[^a^ b` and `[^a^](/u)` get a `<sup>` in pulldown and stay text here.
  - `[^a *b^ c*` → `[^a _b^ c_` on both the branch and main.
- **Data structures and Changes, at the cited places.**
  - Parser: ast.rs:133; inline.rs:336, 738-741, 780 (`^` takes the `_` branch), 880, 893.
  - Generator: generate.rs:1597-1598, 1637, 1882-1890, 1958, 1997, 2009, 2989, 3581.
  - Test renderer: commonmark_spec_check.rs:493.
  - The JSON writes `"decoration": "Superscript"`.
  - pulldown also nests `^^^a^^^` as a one-caret superscript around a two-caret one.
- **Hard-break rule.** Without it, only "moving superscript" fails (`^\⏎.^.^ !`), and `x ^␣␣⏎b^` is still kept by the read-back check, as the spec says.
- **Tests tables.** They match the files: the 10 rows give 11 cases, and `TextWrap_Always` has the stated config and 2 cases. Full `cargo test` passes, and the render-check tool reports 13 cases.

</details>

#### Implementation review rounds

Rounds 1-2 are Amy's (see her section).

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 3     | 6        | A `^` after an escaped `\[` can open (code and tests); the `unwrap()` in `written_delimiter()` removed; existing `_` bugs that now also hit `^` listed in the spec; comments and the render check tool's formatting fixed |
| 4     | 3        | With `wrapUnspacedScripts` a line break next to a superscript is no longer dropped (`decoration_keeps_its_delimiters()` is false for a superscript); tests for `^^a^^`, `x ^␣\⏎b^` and a setext heading; the `[` check reads the delimiter stack instead of counting backslashes; one sub-point rejected |

#### Last Implementation review

LLM: Claude Opus 5.5, run as a fresh reviewer, with the same tools as the
spec review. Environment notes are left out of the prompt here as well.

<details>
<summary>Prompt</summary>

> Review the superscript implementation (`git diff 82d6034 -- src tests tools
> .gitignore`) against `superscript-spec.md`. Do not edit any tracked file;
> scratch copies are fine. Check with evidence (command output or
> file:line): (1) does the code do what the spec's "Changes" section says,
> with nothing missing and nothing extra; (2) bugs: inputs where the formatter
> now creates, removes or moves a superscript, changes output it did not
> change before, or panics. Compare the rendering of input and output with
> pulldown-cmark, and compare with `main` where useful; (3) do the tests cover
> every row of the spec's Tests tables, and does `cargo test --locked` pass;
> (4) does the code follow the style of the surrounding code. Report concrete
> problems (location, failing input, evidence, fix) ranked by severity, and
> mark a finding "trivial" if it does not change behavior. Then list what you
> verified as correct.
>
> Output format: a heading "Problems, most severe first" with a numbered
> list, then "Verified correct" as a bullet list. Keep it under 700 words.
> Use ␣ for a significant space and ⏎ for a line break in inline examples.

Sent while it was running:

> Addition to the review task: focus on substance, not nitpicks. Report a
> problem only if it is a logic bug, a real code-style problem (not matching
> the surrounding code), or a gap in the tests. Also flag overengineering:
> code that could be simpler, duplicates an existing helper, or does more
> than the spec needs. Put minor remarks under one short "Trivial" line at
> the end, not as numbered findings.

</details>

Output. All three findings were reproduced and applied. For finding 1, the
setext heading test fails without the new rule, so no separate
`wrapUnspacedScripts` test was added. With that rule, `^` is no longer needed
in `is_delimiter` and was removed; on the reviewer's fuzz inputs this adds no
rendering change against `main`. The `run_can_open()`/`run_can_close()` part
of finding 2 is rejected and struck. The trivial line was left; its setext
case is a `_` bug that `main` has too.

<details>
<summary>Full review</summary>

##### Problems, most severe first

1. **wrapUnspacedScripts removes a superscript (logic bug, not in the spec's known list).** `src/generation/generate.rs`, `decoration_keeps_its_delimiters` (~3571), called by `drops_break_between` (line 351) through `can_be_written_beside`.
   - Input with `wrapUnspacedScripts: true` and textWrap always or never: `^中^⏎字` → `^中^字`. Same for `字⏎^中^` → `字^中^` and `^a 中^⏎字` → `^a 中^字`.
   - Evidence: pulldown-cmark renders the output as `<p>^中^字</p>`, so the superscript is gone. Main writes `^中^ 字`, which keeps it. `_中_⏎字` stays safe because `resolve()` rewrites an in-word `_` as `*`. Superscript returns early from `resolve()`, so `^` has no such fallback, yet the function still lets the line break be dropped.
   - Fix, checked in a scratch copy (`cargo test --locked` passes): start `decoration_keeps_its_delimiters` with `decoration.kind != TextDecorationKind::Superscript &&`. The output becomes `^中^ 字`. The same fix also clears known bug 1 under the default config (`^中^⏎字⏎===` → `# ^中^ 字`). The textWrap always variant of that bug stays broken.
   - Add a spec test with `wrapUnspacedScripts: true`.

2. **Test gaps (mutation testing).** I reverted each of these changes on its own, and `cargo test` still passed:
   - `^` in `unescaped_delimiter_runs`: without it, `x ^␣\⏎b^` → `x ^\⏎b^`, which creates a superscript. The spec row `x ^␣␣⏎b^` is also caught by the hard-break rule, so it doesn't test this change on its own. Add `x ^␣\⏎b^` (expected unchanged).
   - The `"^^"` arm in `gen_text_decoration` (removing it falls through to `~~`), and `resolve`/`written_delimiter` always returning `"^"`: `^^a^^` is only tested inside a paragraph where the read-back check fails and the file text is written as-is. Add a test with `^^a^^` on its own.
   - `^` in `is_delimiter` (in `decoration_keeps_its_delimiters`) ~~and `^` in `run_can_open`/`run_can_close`~~: no test fails without them. The first only matters under wrapUnspacedScripts, so the test from problem 1 is the natural place to cover it. (`run_can_open`/`run_can_close` part rejected: reverting it changed no output on the round's fuzz inputs, so no test can pin it)
   - These were caught: the hard-break rule, both parts of `after_bracket`, and `written_delimiter` reading the file text.

3. **`after_bracket` works out escape state again (simplification).** `src/parser/inline.rs:735-738` counts the backslashes before `[`. But `parse_backslash` has already consumed `\[`, and an unescaped `[` pushes a Link/Image delimiter ending at `start` (`parse_open_bracket`, ~581). This checks the same thing from parser state, like `last_bracket()` does: `self.delimiters.last().is_some_and(|d| matches!(d.kind, DelimiterKind::Link | DelimiterKind::Image) && d.end == start)`. I swapped it in a scratch copy and the full `cargo test --locked` passes. The current code is correct (`\\[^a^` does not open a superscript, `\[^a^` does).

Trivial: The hard-break rule also applies after an escaped `\^` or a closing caret: `^a^␣␣⏎b` keeps its two spaces where main wrote `\`. That is harmless. `map_or(1, …)` in `written_delimiter` handles an empty superscript, which can't happen. rustfmt is not installed, so formatting is unchecked. The spec's list of shared `_` bugs misses one: with textWrap always and width 20, `~> \^^)\⏎=== _a_ab^^*^^^^!!(^^` wraps so `===` ends up alone on a line and becomes a setext heading. Main does the same with `__`.

##### Verified correct

- `cargo test --locked` passes: 194 spec, 160 unit, 40 parser-spec, 10, 9, 8, 2 and doc tests. `tools/superscript-render-check` passes 13/13. Clippy `--all-targets` is clean.
- Every item in the spec's "Changes" section is present: the parser arm, `after_bracket`, the `wrap_emphasis` kind and run length, the ast variant, the `gen_text_decoration` arms, the `resolve` early return, `written_delimiter`, `unescaped_delimiter_runs`, `run_can_open`/`run_can_close`, the hard-break rule, `is_delimiter`, and the test helper's `"sup"`. Nothing extra beyond the tool and `.gitignore` line the spec mentions.
- Tests tables: all 11 Syntax rows have a parser test, and all formatter rows have a spec test. One row is split in two, and the CJK and wrapping rows are in `TextDecoration_Superscript_TextWrap_Always.txt`.
- Differential fuzzing against main (82d6034): about 95k random inputs × 11 configs.
  - No panics or errors, and no output change for any input without `^`.
  - Every rendering regression falls into the spec's four known bugs shared with `_`, each reproduced on main with `_`. Variants seen: a tab before `[^1]`, and `^a␣␣⏎\⏎b^` under doubleSpace. The only exceptions are problem 1 and the setext case in the Trivial line.
  - The only other output change seen is a different wrap point under wrapUnspacedScripts, which only adds a space between CJK characters, as that option already does. No superscript is lost.
- The known pulldown-cmark differences behave as the spec documents: `^a .^^. b^`, `^a ~~b~ c^`, `[^a^ b` and `[^a^](/u)` render the same before and after formatting. `[^a *b^ c*` changes, as the spec says.
- The goal inputs are now formatted safely: `^a *b^ c*`, `x ^␣␣⏎b^`, `^^a .^␣␣⏎b`, and `^中^⏎字` under always/never with wrapUnspacedScripts off. `^^^^a^^^^` and `^^^^^a^^^^^` come back unchanged.
- The style matches the surrounding code: lowercase `//` comments, `--` dashes, lines ≤120, the same match-arm layout.

</details>

### Amy Smith

GitHub userid: Kenzpie

#### Contributions

Added review round 11 to the updated Superscript Spec.
Created first implementation of the Superscript Spec and ran the first two rounds of test-oriented LLM review.

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 11 | 2 | Clarifies AST types/fields aren't allowed and specifies both LF and CRLF test-file line endings should be accepted. |

#### Last specification review

LLM: GPT-6.1 Sol Light.

Prompt: "Please conduct another review round to further polish our spec."

Output: "The spec is already detailed enough to support an implementation. However, some wording can be clarified to prevent future confusion."

#### Implementation review rounds

Note: Initial creation not reflected in this chart, and the first review rounds focus on test errors, not overall improvement.

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | 2 | Removed extra blank lines in formatter output and after comparison when comparing fixture conventions. |
| 2 | 1 | Fixed a Windows line-ending compatability issue in rendering verification tool. |

#### Last Implementation review

LLM: GPT-6.1 Sol Light.

Prompt: "Please conduct the next review round focusing on bigger test issues."

Output: "No additional defects were identified in the superscript implementation during this review. The implementation appears consistent with the superscript specification and fits the project’s existing parser and formatter architecture. "

### Alisher Arginbekov

GitHub userid: aarginbekov

#### Contributions

*
To be added
*

#### Specification review rounds

*
Table showing spec review rounds
*

#### Last specification review

*
LLM and prompt used
LLM output, possibly with strikethrough
*

#### Implementation review rounds

*
Table showing implementation review rounds
*

#### Last Implementation review

*
LLM and prompt used
LLM output, possibly with strikethrough
*

### Aslan Tasmagambetov

GitHub userid: tasma-07

#### Contributions

*
To be added
*

#### Specification review rounds

*
Table showing spec review rounds
*

#### Last specification review

*
LLM and prompt used
LLM output, possibly with strikethrough
*

#### Implementation review rounds

*
Table showing implementation review rounds
*

#### Last Implementation review

*
LLM and prompt used
LLM output, possibly with strikethrough
*

