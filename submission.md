# Submission: heading attributes

Team: Aibar, Assem, Ethan and Nurislam. The specification is [heading-attributes-spec.md](heading-attributes-spec.md).

## Contributions

- **Aibar** wrote the most complete first draft setting the recognition, output, and edge-case rules, and contributed to the final review and verification of the pull request.
- **Assem** wrote the draft defining the `attr_list` syntax and line-wrapping behavior, revised `heading-attributes-spec.md` to address instructor feedback (referencing `Options::ENABLE_HEADING_ATTRIBUTES`, raw block parsing before inline parsing, CRLF normalization, and escaped braces), and ran the final specification review rounds.
- **Ethan** wrote the draft covering multi-line and no-space CommonMark heading edge cases, and contributed to the final review and test verification of the pull request.
- **Nurislam** merged the three drafts into one specification and implemented the feature and its tests with Claude, then for the final submission fixed the regressions that implementation review found in the updated parser, brought the specification in line with the code, and ran specification review rounds 5–7 and implementation review rounds 7–11 with Claude Opus 5.5.

## Verifying that the project compiles and runs

Each member ran the prompt "Verify that you can compile and run the project in the current git repo." in their own clone on their own computer.

### Nurislam

Claude Opus 5.5 (Claude Code) produced the following output.

> The project compiles and runs. The native build, the full test suite, the release WebAssembly plugin build, and the Node test CI runs on that plugin all pass, with no compiler warnings.
>
> **Setup:** Rust was not installed on this Mac (no `cargo`, `rustc` or `rustup` anywhere). I installed rustup and the Rust 1.98.0 toolchain the repo pins into the session scratchpad only, without changing PATH, the shell profile or `~/.cargo`. It goes away when the scratchpad is cleared; for lasting work the user should install rustup normally.
>
> **Results:**
>
> - **`cargo build` (debug):** succeeded in about 1m27s with no warnings.
> - **`cargo test`:** every suite passed, 0 failures: 160 unit tests; commonmark_spec_check: 10; corpus_check: 1; nesting_test: 2; newline_test: 9; formatter spec files: 192; parser spec files: 39; HTML parser spec files: 8; doc-tests: 2. The three spec counts match the number of spec files on disk exactly.
> - **Release wasm build** (`cargo build --target wasm32-unknown-unknown --features wasm --release`): succeeded and produced `target/wasm32-unknown-unknown/release/dprint_plugin_markdown.wasm` (about 801 KB).
> - **Node test of the plugin** (`deployment/npm`: `npm ci`, `node setup.js`, `npm run test`): passed with exit code 0. It loads the wasm through `@dprint/formatter` 0.4.1 and checks that `#   Hello` becomes `# Hello`.
> - **Run on messy Markdown:** I also loaded the plugin on a sample with a 40-column line width and wrapping turned on. It reported itself as `dprint-plugin-markdown` 0.29.0 and fixed the heading, changed `*em*` to `_em_` and `__strong__` to `**strong**`, wrapped the paragraph at 40 columns, kept the `*` and `+` items as two separate lists, which is the correct CommonMark behaviour, and aligned the table. Running it again on its own output changed nothing.
>
> **Not run:** the CI step that generates the config schema needs Deno, which isn't installed; it only produces `deployment/schema.json` for publishing. The `dprint` CLI isn't installed, so I ran the plugin through the official Node host instead.
>
> **Repo state:** `git status` is the same as at the start: only the three untracked `*-spec.md` files that were already there. The new `target/`, `deployment/npm/node_modules` and `deployment/npm/plugin.wasm` are all gitignored. No commits or edits were made.

### Aibar

The LLM produced the following output.

```text
Verified: the repository compiles and its test suite runs successfully on your PC.
Installed the repository’s pinned Rust 1.98.0 toolchain.
cargo build --locked — passed.
cargo test --locked — passed, including formatter, parser, CommonMark, and documentation tests.
Git working tree remains clean.
This project is a library plugin, so execution was verified through its tests rather than a standalone application.
```

### Assem

The LLM produced the following output.

```text
Verified successfully using the installed Rust 1.98.0 toolchain.
cargo test: passed, 184 tests total; 0 failed.
CI release WebAssembly build: passed; produced target/wasm32-unknown-unknown/release/dprint_plugin_markdown.wasm.
This project is a plugin/library, not a standalone executable; the tests ran the project’s code. The worktree remains unchanged.
Cargo wasn’t on PATH, so I invoked it by its installed path.
```

### Ethan

The LLM produced the following output.

```text
Verification complete: the repository builds clean and all tests run successfully on the pinned Rust 1.98.0 toolchain using cargo build --locked and cargo test --locked (covering formatting, parsing, CommonMark, and documentation). The Git working tree is clean. Since this is a library plugin, functionality was verified directly through the test suite.
```

## Specification review

Rounds 1–4 were run by Assem with Copilot Fusion (`mai-code-1.1-flash`) and rounds 5–7 by Nurislam with Claude Opus 5.5.

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | 2 | Clarified CRLF (`\r\n`) normalization prior to block recognition; specified that inline element boundary checks rely on the formatter's existing `pulldown-cmark` inline parser |
| 2 | 1 | Explicitly noted in Recognition rule 3 that backslash-escaped braces (`\{...}`) are rejected because no space precedes the opening brace |
| 3 | 2 | Clarified that block matching spans from the opening `{` to trailing whitespace while tracking quote state for inner braces; added an example clarifying literal backslashes before closing quotes |
| 4 | 2 | No changes made; findings were trivial/contradictory (EBNF already excludes tabs outside quotes, and tying inline checks to the formatter's `pulldown-cmark` parser avoids overspecification) |
| 5 | 8 | Rule 2 names exactly which elements can take the `{` in (a link's destination or title too, not autolinks; link definitions matter); the kept-spaces exception says how the block is read and why; a heading whose text opens a backtick fence stays ATX; the introduction says the formatter doesn't use `pulldown-cmark`. Not applied: emphasis meaning that changes when a block on its own line is joined and recognized (the feature itself), and an existing wrapping bug a long last word triggers |
| 6 | 5 | Text delimiters are decided with the block read on its own, so a block joined from its own line keeps the text the same; the wrapping protections heading text with a block gets are described; Parser integration matches rule 2; wording of rule 2 and the code-fence exception |
| 7 | 3 | Wording only: the wrapping protections also name keeping a line's first word with the next one; the "written the same" sentence excepts emphasis that pairs across; any line the protections keep together may exceed `lineWidth` |

### Last specification review

Prompting Claude Opus 5.5 (a Claude Code subagent, default effort) with the prompt below produced the following output; strikethrough indicates LLM output that I deemed incorrect or irrelevant. All three findings were wording fixes and were applied, so nothing is struck through. `[scratchpad]` stands for the session's temporary folder.

> Review the specification in heading-attributes-spec.md in the git repo at /Users/fine/Desktop/ASP-assignment/dprint-plugin-markdown (a dprint Markdown formatter plugin with its own Markdown parser in src/). Review only that file. Read the code only to check what the spec claims about existing behavior or about the formatter. Write numbered findings to /Users/fine/Desktop/ASP-assignment/work/spec-review-final/round-07.md, each with a one-line title, a severity (high/medium/low) and a short explanation. Look for contradictions, ambiguities, wrong examples, cases an implementer would have to guess, and statements that conflict with the existing formatter. Do not edit any other file and do not rewrite the spec. End your reply with the number of findings and a one-line list of their titles.
>
> The previous rounds' findings and what was done about them are in round-05.md, round-06.md and log.md in /Users/fine/Desktop/ASP-assignment/work/spec-review-final/; `git diff fa6c6dd -- heading-attributes-spec.md` shows the changes since round 6 reviewed it. Don't raise a finding that the log lists as not applied unless you have a new reason. Keep this round to about 15 minutes of work.
>
> If you want to check what the current build does, `[scratchpad]/fmtcli/target/release/fmtcli '<config json>' < input.md` formats stdin with the current code (it prints `UNSTABLE` to stderr and exits 2 when a second pass differs), and `[scratchpad]/oldcli/target/release/oldcli` is the same CLI built from the code before the feature. Example config: `'{"heading.kind":"setext","textWrap":"always","lineWidth":12}'`. Don't build or install anything; put scratch files in `[scratchpad]/sr7/`.

#### Spec review, round 7: heading-attributes-spec.md (as committed in 7c84e56)

Every example in the spec gives the stated output with the current build (fmtcli), and none of them is unstable. Round 6's two instabilities (`` `c` a`` / ``{x=`}`` / `===` and `**e*` / `{x=*}` / `===`, with `atx`) are now stable. I checked the cases below with fmtcli and oldcli. I also ran about 2,500 random headings with blocks (ATX, and Setext with the block on its own line) through both builds, under `atx`, `setext` + `always` + `lineWidth: 8`, and `setext` + `maintain`. None was unstable in fmtcli where oldcli was stable. In ATX output, the text never differed from oldcli's except where emphasis pairs across the text and the block.

##### 1. The wrapping protections leave out the rule that keeps a line's first word with the next word

Severity: medium

Output bullet 3 lists three ways that wrapped text with a block is kept from starting a block. None of them covers a first word that would start a block if it were left alone on its line. "A word that would start one is kept off the start of a line" can't apply to the text's first word, because that word always starts a line. The build keeps such a word with the word after it, on the text's first line and on any line after a hard break:

- `# --- alpha beta {#id}`, with `setext`, `always` and `lineWidth: 5` (or 8), gives `--- alpha` / `beta {#id}` / `==========`.
- `# *** alpha beta {#id}` gives `*** alpha` / `beta {#id}`.
- `alpha\` / `*** beta gamma {#id}` / `===` gives `alpha\` / `*** beta` / `gamma {#id}`.

An implementer who follows the three listed rules writes `---` / `alpha` / `beta {#id}`. There `---` is a thematic break and the heading loses its first word. This isn't today's behavior either: both builds write `---` alone for `# --- alpha beta` (no block), and the second pass differs. The leading-tag item is one case of this rule. The bullet should state the general rule ("a line's first word that would start a block on a line of its own stays with the word after it") and say that it applies to the first line and to each line after a hard break.

##### 2. "The text is written the same whether or not the block is recognized" is not true where emphasis pairs across

Severity: low

Output bullet 1 now states this as a result of the delimiter rule. It fails where a delimiter in the text pairs with one in the block when the block isn't recognized:

- `# *a {x=*}` is recognized and gives `# *a {x=*}`.
- `*a` / `{x=*}` / `===` (the block on its own line, not recognized), with `atx`, gives `# _a {x=_}`. oldcli gives the same for `# *a {x=*}`.

The text is `*a` in one case and `_a` in the other. The log lists the change in meaning as the feature itself and not to be fixed, and this finding doesn't reopen that. The problem is the round-6 sentence, which states a general guarantee that this case breaks. It should be limited, for example "so where nothing in the text pairs with the block, the text is written the same whether or not the block is recognized".

##### 3. "The final line may exceed `lineWidth`" no longer covers every line that can

Severity: low

The rules that round 6 added keep two words together on lines other than the last, so those lines can also run past `lineWidth`. Example: `# <span> alpha beta gamma {#id}`, with `setext`, `always` and `lineWidth: 8`, gives `<span> alpha` (12 columns) / `beta` / `gamma {#id}`. With finding 1's rule, `--- alpha` behaves the same way. Read next to bullet 3's three rules, the sentence suggests that only the line holding the block may be too long. An implementer could then break `<span>` away from `alpha` to honor `lineWidth`, which is what the tag rule forbids. The sentence should say that a line holding the block, or holding a word kept with the one after it, may exceed `lineWidth`.

## Implementation review

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | 8 | No bugs found (including a 120k-heading differential fuzz); added tests for every gap it listed: decorations before a block, the file-text fallback, spaces inside emphasis and HTML, rule 2 with links and emphasis, the Setext conversion check, closing-hash variants, `textWrap: sentence`, and parser unit tests for recognition and the re-parsed text |
| 2 | 6 | Fixed a regression: the block made Setext lines longer, so wrapping could leave a first word that starts a block (`***`, `<img>`) alone and lose the heading; heading text with a block now gets the line-start protection paragraphs get and is not wrapped after a leading block-starting tag; added tests for list markers before the block, containers, links/math/strikethrough in the block and non-ASCII spaces; replaced an unreachable branch with a debug assertion |
| 3 | 5 | Line-start protection now also covers lines a hard break starts; only the space after a leading block-starting tag is unbreakable instead of turning off wrapping for the whole heading; the text no longer gets a paragraph's block bound, which had doubled a literal backslash; added a `maintainAndWrap` test. Not applied: thematic-break words split across a soft break, a bug paragraphs already have |
| 4 | 4 | A leading tag stays with any element after it, not only plain text; the text's last words are read together with the block on their line (kept apart from the paragraph bound that doubled backslashes), and a footnote reference followed by a colon is kept off a line start; added a test for duplicate attributes, order and unquoted values. Not applied: hard breaks nested inside emphasis or links, the same gap paragraphs have |
| 5 | 1 | The footnote-reference rule from round 4 also changed how paragraphs wrap, outside the spec's scope; it now applies only to heading text followed by a block |
| 6 | 2 | Trivial: corrected the comment on the footnote rule (it applies anywhere in heading text with a block) and added a test for it; added rule 2 tests for inline HTML and strikethrough. No behavior changes |
| 7 | 4 | Reviewed Aibar's parser change for the final submission (the block is found before inline parsing): fixed a regression where any unclosed `[`, `![` or `[^` before the block rejected it, by having the parser and the formatter's read-back share one function; spaces inside Markdown-like text in the block now followed the updated spec; added a code-span-only rejection test; removed an unused parameter |
| 8 | 3 | Collapsing every space in the block made a block on its own Setext line change on the second pass once joined, so the kept-spaces exception was restored in the code and the spec; added tests for images, reference links, links within emphasis and definitions. Not applied: a link label that only matches its definition after emphasis is rewritten |
| 9 | 3 | A heading whose text opens a backtick code fence stays ATX when a backtick in its block hides the fence; fixed a test whose block was invalid. Not applied: a `+*a` / `{k=a*}` heading that switches from ATX to Setext once its block is joined, which needs emphasis pairing across lines |
| 10 | 2 | The block is read on its own when the heading is parsed and its delimiter runs count when the text's delimiters are chosen, so the text is written as today whether or not the block is recognized (a lone backtick in a joined block had changed `` `make` ``); added tests for both join directions and the preserved-decorations retry |
| 11 | 2 | Trivial, no behavior bugs, panics or unstable output: added a test with a code span in the block for the preserved-decorations retry and one showing that the kept spaces depend on the document's link definitions |

### Last implementation review

Prompting Claude Opus 5.5 (a Claude Code subagent, default effort) with the prompt below produced the following output; strikethrough indicates LLM output that I deemed incorrect or irrelevant. Both findings were missing tests and were added, so nothing is struck through. `[scratchpad]` stands for the session's temporary folder.

> Review the implementation of heading-attributes-spec.md in the git repo at /Users/fine/Desktop/ASP-assignment/dprint-plugin-markdown (a dprint Markdown formatter plugin with its own Markdown parser in src/). The implementation is `git diff 631ceea -- src tests` on the current branch. Check that it does what the spec says, and look for bugs, panics, non-idempotent output, behavior that differs from the spec, and gaps in the tests. Treat the spec as correct; don't review the spec itself. This round focuses on commit 2ba1a40, which fixes implementation review round 10 (/Users/fine/Desktop/ASP-assignment/work/impl-review/round-10.md): the block is now read on its own when the heading is parsed (`HeadingAttributes::block_children`), its delimiter runs count when the text's delimiters are chosen, and the same reading decides where the block's spaces are kept. /Users/fine/Desktop/ASP-assignment/work/impl-review/log.md lists what earlier rounds changed and what was deliberately not applied; don't raise those again unless the fix is wrong or incomplete.
>
> Tools (all outside the repo; don't install anything else):
> - Rust 1.98.0: run `export S=[scratchpad] RUSTUP_HOME=$S/rust/rustup CARGO_HOME=$S/rust/cargo PATH=$S/rust/cargo/bin:$PATH` first. Use `CARGO_TARGET_DIR=$S/target` for `cargo test` or `cargo clippy --all-targets` in the repo.
> - `$S/fmtcli/target/release/fmtcli '<config json>' < input.md` formats stdin with the current repo code, already built. It prints the output, and if a second pass differs it prints `UNSTABLE` to stderr and exits 2. Example config: `'{"heading.kind":"setext","textWrap":"always","lineWidth":12}'`. Don't rebuild it.
> - `$S/oldcli/target/release/oldcli` is the same CLI built from 631ceea, the code before the feature, for comparing with today's behavior.
> - Put any scratch files in `$S/ir11/`.
>
> Keep the review to about 15 minutes of work: read the code and try targeted inputs rather than running large fuzzing campaigns. Write numbered findings to /Users/fine/Desktop/ASP-assignment/work/impl-review/round-11.md, each with a one-line title, a severity (high/medium/low), a short explanation and, where possible, an input that shows the problem with its actual and expected output. Do not edit any file in the repo. End your reply with the number of findings and a one-line list of their titles.

#### Heading attributes implementation review, round 11

Scope: `git diff 631ceea -- src tests` on branch `heading-attributes`, checked against `heading-attributes-spec.md`. This round focuses on 2ba1a40, which fixes implementation review round 10:

- `parse_heading_content` now reads the block on its own as inline Markdown at parse time, into the new `HeadingAttributes::block_children`. Its spans are absolute.
- `gen_heading_content` gathers its delimiter runs with `heading_delimiter_runs`, which takes the text's runs plus the block's runs read on its own. It does this on both the first write and the preserved-decorations retry.
- `gen_attribute_block` decides which spaces to keep from `block_children` and no longer parses the block again. The position is now made absolute (`attributes.span.start + start`).

##### Summary

`cargo test` passes: 166, 16, 1, 2, 9 and 2 tests, plus the spec runners (203 spec files). `cargo clippy --all-targets` reports no warnings.

The fix for round 10 holds, and I found no behavior bugs in 2ba1a40:

- Round 10 finding 1 is fixed:
  - `` Run `make` `` / ``{title="press ` to open"}`` / `===` is stable under `atx`, `setext`, and `setext` with `always`.
  - So are `` `c` a `` / ``{t=`}`` / `===` and `*a 1.*** <div>` / `{x=*a}` / `===`.
  - `` # The `run` command {data-cmd="`run`"} `` and `` # `c` {`a`} `` still keep a single backtick.
- A recognized ATX block now gets the same text as `oldcli` gives for the whole heading:
  - `ir11/diffold.py` checked 3,100 random headings in two emphasis configurations.
  - The only differences were headings where `oldcli` pairs a run in the block with a leftover or unclosed run in the text (`# **a* {a=b*}`, `# ***x_y* {x=***}`). Rule 1 says that pairing doesn't happen.
- Backticks in the block that sit inside an element read on its own give the same result as `oldcli`. This covers link text and destinations, math, footnote references and inline HTML.
- The kept-spaces exception still works:
  - in a block quote and a list item;
  - after a CJK character;
  - with a reference link whose definition comes before the heading, after it, inside a block quote, or inside a list item.
- All of the spec's examples give the spec's output.

Two findings remain, both test gaps:

1. low: the preserved-decorations retry has no test with a code span in the block.
2. low: no test checks that the kept spaces depend on the document's link definitions.

##### How it was checked

- Read the 2ba1a40 diff and the code around it:
  - `parse_heading_content`, `parse_heading_inlines`, `parse_inlines_before_attributes`;
  - `heading_delimiter_runs`, `push_delimiter_runs`, `push_pairable_runs`, `push_backtick_runs`;
  - `decorations_need_checking`, `gen_heading_content`, `gen_attribute_block`;
  - `gen_code`/`get_backtick_count`, and how `decoration_delimiter`/`Beside::allows` use `pairable` and `decorations`.
- Ran `ir11/stab.py` with seeds 1, 2, 3, 22 and 43, about 2,700 headings in total, and `ir11/stab_k.py` with `emphasisKind: asterisks` and `strongKind: underscores`, 500 headings. Each heading was tried in two forms: with the block on a Setext line of its own, and as an ATX heading. Each form was formatted under `atx`, `setext`, and `setext`/`always` (`stab_k.py` also uses width 14).
  - Every instability had an emphasis run in the text that pairs with a run in the block across the lines when the block isn't recognized. Examples: `**`` / `{a=b*}`, `**[x](u)a*` / `{a=b*}`, `***x_y <b> *b _` / `{x=***}`. That's the round 9 case that was deliberately not applied.
  - Some of these switch from Setext to ATX on the second pass instead of from ATX to Setext.
- Ran `ir11/diff2.py`, about 1,800 headings. It compares the text part of `# T B` (recognized) with `T` / `B` / `===` under `atx` (block not recognized on the first pass, then joined). The only differences were:
  - the same cross-line emphasis pairing;
  - a text backtick that pairs with a block backtick, which rule 2 rejects;
  - the block itself being formatted as today while it isn't recognized.
- Mutation on a copy (`ir11/mut`, built as `ir11/mutcli`): see finding 1.

Seen but not counted:

- When a code span runs from the text into a block on the next line, and conversion to ATX joins the lines, the space at the line break is lost: `` `*** `` / ``{t="a ` b"}`` / `===` gives ``# `***{t="a ` b"}``. `oldcli` does the same. The block isn't recognized here, because rule 2 rejects it.
- In `parse_heading_content`, `block_line.column` is `last.column + start`, where `start` is a byte offset, not a column. The column is wrong after a tab or a multi-byte character in the text. Nothing in the inline parser reads `column` (it only uses `virtual_spaces`), so the output isn't affected.

---

##### 1. The preserved-decorations retry has no test with a code span in the block

**Severity:** low

2ba1a40 changed both `gen_heading_content` calls to `heading_delimiter_runs(attributes, context)`, at line 817 for the first write and line 831 for the `with_preserved_decorations` retry. The only retry test, `` # foo******bar*********baz `c` {t=`} ``, has a lone backtick in the block. Every reading counts that backtick, so the test can't tell `heading_delimiter_runs` apart from the whole content with the block as plain text, which is what the code used before 249ed33.

On a copy of the tree, I changed line 831 to `delimiter_runs(heading.children, context)`. All 203 spec files still pass. The mutant counts the backticks of the block's own code span:

Input (`heading.kind: atx`):
```markdown
# foo******bar*********baz `c` {`a`}
```
Current output (correct, and what `oldcli` gives):
```markdown
# foo******bar*********baz `c` {`a`}
```
Mutant output:
```markdown
# foo******bar*********baz ``c`` {`a`}
```

With the mutant, the joined form `` foo******bar*********baz `c` `` / ``{`a`}`` / `===` (`atx`) is also unstable. The first pass gives `` # foo******bar*********baz `c` {`a`} `` and the second gives ``` ``c`` ```.

Suggested fix: add `` # foo******bar*********baz `c` {`a`} `` to the "should not count the backticks of a code span within the block" test in `Headers_Attributes.txt`, with its current output.

---

##### 2. No test checks that the kept spaces depend on the document's link definitions

**Severity:** low

Output bullet 2 says the block is read on its own "with the document's link and footnote definitions" to decide which spaces are kept. Before 2ba1a40, `gen_attribute_block` parsed the block with the formatter's `context.link_labels()` and `context.footnote_labels()`. Now the parser reads it, with the parser's inline context. So whether every definition is known when the heading is parsed now matters, including one that comes after the heading or sits in a container.

It works today:

- `# Title {x=[a   b][r]   .c}` with `[r]: /v` after the heading gives `# Title {x=[a   b][r] .c}`.
- Without the definition, it gives `# Title {x=[a b][r] .c}`.
- A definition before the heading, inside a block quote, or inside a list item behaves the same way.

No spec file covers this, though. Every kept-spaces test uses an element that doesn't need a definition: a code span, emphasis, inline HTML, an inline link, math, or strikethrough. A change that left the definitions out of the block's reading would pass every test.

Input (`heading.kind: atx`):
```markdown
# Title {x=[a   b][r]   .c}

# Title {x=[c   d][s]   .c}

[r]: /v
```
Expected (and current) output:
```markdown
# Title {x=[a   b][r] .c}

# Title {x=[c d][s] .c}

[r]: /v
```

Suggested fix: add this to `Headers_Attributes.txt` next to "should keep the spaces within a link, math and strikethrough in the block". It covers both a defined label, whose spaces are kept, and an undefined one, whose spaces collapse.

---

2 findings: (1) the preserved-decorations retry has no test with a code span in the block; (2) no test checks that the kept spaces depend on the document's link definitions.
