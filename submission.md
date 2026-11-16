# Submission: heading attributes

Team: Aibar, Assem, Ethan and Nurislam. The specification is [heading-attributes-spec.md](heading-attributes-spec.md).

## Contributions

- **Aibar** wrote the most complete first draft setting the recognition, output, and edge-case rules, and contributed to the final review and verification of the pull request.
- **Assem** wrote the draft defining the `attr_list` syntax and line-wrapping behavior, revised `heading-attributes-spec.md` to address instructor feedback (referencing `Options::ENABLE_HEADING_ATTRIBUTES`, raw block parsing before inline parsing, CRLF normalization, and escaped braces), and ran the final specification review rounds.
- **Ethan** wrote the draft covering multi-line and no-space CommonMark heading edge cases, and contributed to the final review and test verification of the pull request.
- **Nurislam** merged the three drafts into one specification, implemented the feature and test suite in `src/` and `tests/`, and ran the implementation review rounds.

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

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | 2 | Clarified CRLF (`\r\n`) normalization prior to block recognition; specified that inline element boundary checks rely on the formatter's existing `pulldown-cmark` inline parser |
| 2 | 1 | Explicitly noted in Recognition rule 3 that backslash-escaped braces (`\{...}`) are rejected because no space precedes the opening brace |
| 3 | 2 | Clarified that block matching spans from the opening `{` to trailing whitespace while tracking quote state for inner braces; added an example clarifying literal backslashes before closing quotes |
| 4 | 2 | No changes made; findings were trivial/contradictory (EBNF already excludes tabs outside quotes, and tying inline checks to the formatter's `pulldown-cmark` parser avoids overspecification) |

### Last specification review

Prompting Copilot Fusion (`mai-code-1.1-flash`) with `Review the section named "Heading attributes" in heading-attributes-spec.md` produced the following output; strikethrough indicates LLM output that I deemed incorrect or irrelevant. Finding 1 contradicts round 1 and would overspecify `pulldown-cmark` internals, while Finding 2 is already enforced by the EBNF grammar (`SP` and `char` exclude tabs).

> I reviewed `heading-attributes-spec.md`. Overall the section is quite strong, but I’d tighten two points:
>
> - ~~High confidence: Recognition rule 2 is underspecified in a way that can make the behavior version-dependent.~~
>   - ~~It says “no inline construct remains unclosed at the boundary where the block starts,” but whether that is true depends on the exact pulldown-cmark configuration and delimiter state.~~
>   - ~~This means the same source could be recognized or rejected depending on the formatter’s extension configuration or parser version, even if the block grammar itself is unchanged.~~
>   - ~~I’d recommend either enumerating the exact parser config used for this check or defining the boundary test more concretely so the spec is stable across formatter updates.~~
>
> - ~~Medium confidence: The tab-invalid rule is only in prose, not in the EBNF.~~
>   - ~~The grammar uses `SP` as a space, while the prose says “Outside quoted values, a tab makes the block invalid.”~~
>   - ~~That means a literal parser built from the grammar alone could accept `{\t#id}` or `{#id\t}` unless the implementation also adds a separate tab check.~~
>   - ~~I’d fold the “no tabs outside quoted values” rule into the grammar or add an explicit normative sentence that must be enforced in addition to the EBNF.~~
>
> Those are the main things I’d fix before treating this as a final, implementation-ready spec.

## Implementation review

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | 8 | No bugs found (including a 120k-heading differential fuzz); added tests for every gap it listed: decorations before a block, the file-text fallback, spaces inside emphasis and HTML, rule 2 with links and emphasis, the Setext conversion check, closing-hash variants, `textWrap: sentence`, and parser unit tests for recognition and the re-parsed text |
| 2 | 6 | Fixed a regression: the block made Setext lines longer, so wrapping could leave a first word that starts a block (`***`, `<img>`) alone and lose the heading; heading text with a block now gets the line-start protection paragraphs get and is not wrapped after a leading block-starting tag; added tests for list markers before the block, containers, links/math/strikethrough in the block and non-ASCII spaces; replaced an unreachable branch with a debug assertion |
| 3 | 5 | Line-start protection now also covers lines a hard break starts; only the space after a leading block-starting tag is unbreakable instead of turning off wrapping for the whole heading; the text no longer gets a paragraph's block bound, which had doubled a literal backslash; added a `maintainAndWrap` test. Not applied: thematic-break words split across a soft break, a bug paragraphs already have |
| 4 | 4 | A leading tag stays with any element after it, not only plain text; the text's last words are read together with the block on their line (kept apart from the paragraph bound that doubled backslashes), and a footnote reference followed by a colon is kept off a line start; added a test for duplicate attributes, order and unquoted values. Not applied: hard breaks nested inside emphasis or links, the same gap paragraphs have |
| 5 | 1 | The footnote-reference rule from round 4 also changed how paragraphs wrap, outside the spec's scope; it now applies only to heading text followed by a block |
| 6 | 2 | Trivial: corrected the comment on the footnote rule (it applies anywhere in heading text with a block) and added a test for it; added rule 2 tests for inline HTML and strikethrough. No behavior changes |

### Last implementation review

Prompting Claude Opus 5.5 (a Claude Code subagent, default effort) with the prompt below produced the following output. Both findings were applied, so nothing is struck through.

> Review the implementation of heading-attributes-spec.md in the git repo at /Users/fine/Desktop/ASP-assignment/dprint-plugin-markdown (a dprint Markdown formatter plugin with its own Markdown parser in src/). The implementation is `git diff 631ceea -- src tests` on the current branch. Check that it does what the spec says, and look for bugs, panics, non-idempotent output, behavior that differs from the spec, and gaps in the tests. Treat the spec as correct; don't review the spec itself. [Instructions for using the Rust toolchain and a small formatting CLI in the scratchpad.] Keep the review to about 30 minutes of work: read the code and try targeted inputs rather than running large fuzzing campaigns. Write numbered findings to /Users/fine/Desktop/ASP-assignment/work/impl-review/round-06.md, each with a one-line title, a severity (high/medium/low), a short explanation and, where possible, an input that shows the problem with its actual and expected output. Do not edit any file in the repo. End your reply with the number of findings and a one-line list of their titles.

#### Heading attributes implementation review, round 6

Scope: `git diff 631ceea -- src tests` on branch `heading-attributes` (commits 910ae93 through 8f13512), checked against `heading-attributes-spec.md`. This round re-reads the whole change, with extra attention on 8f13512 (the round 5 fix) and on areas earlier rounds did not probe.

##### Summary

`cargo test` passes (all six suites: 165, 15, 1, 2, 9 and 2 tests). I found no panics, no unstable output caused by the change, and no case where a recognized block is lost, split or changed beyond the spec's space collapsing. 8f13512 works: the footnote-reference rule now fires only inside `with_line_starts`, so paragraphs and headings without a block format exactly as in the base.

There are two low-severity findings:

1. The footnote-reference rule is wider than its own comment says, which affects how the middle of the heading text wraps.
2. A test gap in rule 2.

##### How it was checked

- Re-read the full diff:
  - parser: `attribute_block_start`, `read_block`, `attribute_len`, `heading_attributes` in `block.rs`;
  - generator: `gen_heading`, `gen_heading_content`, `gen_text_and_attribute_block`, `gen_attribute_block`, `with_line_starts`, `text_from`, `starts_block_at_line_start`;
  - surrounding code: `gen_nodes`, `gen_checked_decorations`, `reads_back_decorations`, `delimiter_runs`, `TextBuilder::space_items`, `utils::wrapping_word_starts_block` and `leading_block_words_end`.
- Checked the grammar by hand against the spec's invalid examples and edge forms: `{: }`, `{ :#id}`, `{a= b}`, `{=a}`, empty quoted values, non-ASCII names. Checked slicing for char-boundary panics.
- Checked the claim that reparsing the text without the block can only differ from the full parse when the `{` is not plain text:
  - Emphasis: closers are matched left to right and look back.
  - Code spans: a run in the block can only pair with one in the text across the `{`.
  - Links and brackets.
  - HTML and autolinks: no inline element can end with `}`.
  - The inline parser keeps no shared state between the two parses.
- Checked that `debug_assert!(context.delimiter_runs().is_none())` in `gen_heading_content` can't fire. `DisplayMath`, the one inline-kind node that could start a container's children, is only ever created inside a paragraph. Also tried tight lists, task items, definition lists, footnote definitions and tables next to a heading.
- Ran about 70 hand-made inputs through fmtcli and the pre-change `oldcli`. They covered:
  - tabs in container prefixes (`-\t#`, `>\t`, `1.\t` with virtual spaces);
  - a tab after a leading tag;
  - a heading whose text is only a tag (`<img src=x> {#id}`, `<div> {#id}`, `</span> {.c}`);
  - a literal `\` before the block;
  - `##`, `<div>`, `<!-- -->`, `>`, `+++`, `$$`, `=`, `===`, `---`, `***` and ` : ` before the block or mid-text with wrapping;
  - code span delimiter choice with backticks in the block;
  - CRLF with multi-line Setext and hard breaks;
  - `textWrap: never`;
  - rule 2 with inline HTML, autolinks, strikethrough, images and strong emphasis.
- A small random sweep (`/private/tmp/claude-501/-Users-fine-Desktop-ASP-assignment-dprint-plugin-markdown/3ab15907-ec21-4256-8db0-805b2ce33b6f/scratchpad/ir6/sweep.py`, seeds 1 to 4):
  - Inputs: 1,900 single-line headings whose text and block both mix delimiter characters (`*`, `_`, `` ` ``, `~~`, `$`, brackets, tags, `[^1]`, `\`). Blocks used both quote kinds with spaces and braces inside.
  - Configurations: `atx`, plus `setext` with `always`/12, `maintain` and `sentence`/12.
  - Result: 3 unstable outputs. All 3 are byte-identical to the base formatter's output. Two have no recognized block, because rule 2 rejects a `{` inside a code span or emphasis. The third is the pre-existing footnote problem listed below.

Problems seen that already exist before the change and are not counted:

- The Setext conversion check doesn't treat a leading defined footnote reference followed by `:` as a block start.
  - Example, `heading.kind: setext`: `# [^1]: x` becomes `[^1]: x` / `=======`, which is a footnote definition. The second pass gives `[^1]: x =======`.
  - `# [^1]: {#id}` does the same. That is correct for this feature, since the spec says to use the existing check.
- `- [x]` / `  # Title {#id}` is written as `- [x] # Title {#id}`, where the heading becomes paragraph text after the task marker. The base does the same without the block.
- The file-text fallback for a multi-line Setext heading written as ATX loses the hard break. `__ \` / `'x___   {#id   .c}` / `===` becomes `# __ \'x___   {#id   .c}`, which is unstable. The base produces the same output for the same input, so this is the "already unstable" case the spec excludes.

---

##### 1. The footnote-reference rule keeps any footnote reference in the text off a line start, not only one before the block, so the middle of the text wraps differently from today

**Severity:** low

8f13512 limits the rule to heading text with a block, using `context.is_within_line_text()`. The code comment gives the reason: the block "after the last words can't be wrapped away from them, so a footnote reference with a colon after it *there* is followed on its line by more than the colon". That reason holds only for a reference among the last words, as in `[^1]: {#id}`. Here the block itself is what turns the line into a definition, and that case is the one the existing test covers.

The code does not check where the reference is. `following_text` runs from the reference to the end of the block, so `starts_block_in_paragraph` is true for a reference followed by `:` anywhere in the text. As a result, a reference in the middle of the text, far from the block, is kept off a line start when a block follows, and is not when there is none.

The spec says "The heading text is formatted as it is today" and limits the wrapping change to the block joining the last word. This changes mid-text wrapping instead. The change is for the better, because today's output makes a definition and is unstable. But it is untested, and it is not what the comment describes. Adding `{#id}` to a heading also changes where its middle lines break.

Input (`heading.kind: setext`, `textWrap: always`, `lineWidth: 12`):
```markdown
# Alpha beta [^1]: gamma delta epsilon {#id}

# Alpha beta [^1]: gamma delta epsilon

[^1]: n
```
Actual:
```markdown
Alpha
beta [^1]:
gamma delta
epsilon {#id}
=============

Alpha beta
[^1]: gamma
delta
epsilon
===========

[^1]: n
```
The second heading is today's behavior and is unstable: the second pass turns `[^1]: gamma …` into a footnote definition. The first heading is wrapped differently only because a block follows it.

Expected: per the spec's wording, the first heading wraps like the second, with only `epsilon {#id}` joined. Two options:
- Keep the wider rule as a deliberate fix. Update the comment to say it covers every reference in heading text with a block, add a test with a mid-text reference such as the input above, and mention it in the PR.
- Or narrow the rule to the case the block causes, where only spaces separate the colon from the block. Fix mid-text references separately for all headings and paragraphs.

---

##### 2. Rule 2 is untested for inline HTML and strikethrough

**Severity:** low

The tests cover rule 2 ("the `{` is not inside any inline element that starts before it") for a link (`# Title [a {b](u) c}`), emphasis (`# *a {*}`) and math (`# a   $b   {x=$}`). The code span example `# Title `{#id` }` is also rejected by rule 3, because the `{` touches the backtick.

No test isolates rule 2 for inline HTML or strikethrough.

Inline HTML matters most. Its closing `>` is a valid `char` and can fall inside a quoted value, so the tag can close inside what reads as a valid block. Its content is written verbatim. If the "inside an element" check ever broke for `Html` nodes, the formatter would collapse spaces inside a raw HTML attribute value.

Some elements need no test of their own:
- Autolinks and footnote references can't contain a space before the `{`, so rule 3 already covers them.
- An image is written the same whether or not its `{` is accepted, so a test would not show the difference.

The current output is correct for both cases.

Input (`heading.kind: atx`):
```markdown
# a <b c=" {x   y=">"}

# a ~~b {x="1   2" y=~~}
```
Actual and expected (unchanged, no block recognized):
```markdown
# a <b c=" {x   y=">"}

# a ~~b {x="1 2" y=~~}
```
If either block were wrongly recognized:
- the HTML case would print `# a <b c=" {x y=">"}`, changing the attribute value;
- the strikethrough case would keep `"1   2"` with two spaces.

Suggested fix: add both to the "should not recognize a block whose brace is within a link or emphasis" test in `tests/specs/headers/Headers_Attributes.txt`.

---

2 findings: (1) the footnote-reference rule keeps any footnote reference in the text off a line start, not only one before the block, so the middle of the text wraps differently from today; (2) rule 2 is untested for inline HTML and strikethrough.