# Project 2 submission

Team name: dprint `<br>`

The specification consists of the file
[src/generation/br-new-line-spec.md](src/generation/br-new-line-spec.md).

## Draft submission

### Askar Oralkhan

GitHub userid: askaroralkhan

#### Contributions

Wrote the first draft of the specification, ran review rounds 1–6 and applied
their findings, including the opt-in `lineBreakAfterBr` option and the Rule's
exceptions (with Claude Opus 5.5).

#### Build and run test

Prompting Claude Opus 5.5 (high effort) in Claude Code, in a clean checkout of
7c8b2ef, with "Verify that you can build and run the project in the current git
repo." produced the following output, summarized:

- `cargo build` (native, with the Rust 1.98.0 the repo pins) passes.
- The release WebAssembly plugin build
  (`cargo build --target wasm32-unknown-unknown --features wasm --release`)
  passes and produces `dprint_plugin_markdown.wasm` (about 786 KB), after
  adding the `wasm32-unknown-unknown` target as CI does.
- `cargo test` passes: 160 unit tests, `br_render_check` (2),
  `commonmark_spec_check` (10), `corpus_check` (1), `nesting_test` (2),
  `newline_test` (9), the doc-tests (2), and the `specs`, `parser_specs` and
  `html_parser_specs` suites.
- Not run: the Node smoke test in `deployment/npm`, which needs `npm ci`, and
  loading the plugin in dprint, whose CLI isn't installed.

#### Specification review rounds

Each round was a new, read-only Claude Code session, so the reviewer could read
the code and run `cargo` but not change files. Between rounds, the accepted
findings were applied as small, targeted edits (made with Claude Opus 5.5 in a
separate session and checked against the code), never as an LLM rewrite.

| Round | Findings | Main changes made                                                                                                                                                                                                                                                                                                  |
| ----- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1     | 9        | Rule made opt-in (`lineBreakAfterBr`, off by default); no break when the tag begins a block (type-7 HTML block), in shortcut/collapsed link text, or in a setext heading; "what follows" limited to the tag's parent; concrete examples for every `textWrap` mode; `is_break_tag` moved beside `starts_html_block` |
| 2     | 8        | Nested-tag example fixed (`*a<br>*b` isn't emphasis); setext headings follow the rule; no break before `www.` text, which only links at a line start; image alt text excluded; "begins a block" tightened to "begins a line of its paragraph"; link-label check gets its own flag                                  |
| 3     | 11       | A `<br>` that begins a setext heading and a footnote reference followed by `:` are left alone; examples fixed for the setext underline width, the `_…_` default and missing link definitions; test files for setext and option-off; option renamed `lineBreakAfterBr`                                              |
| 4     | 6        | Block-start check reads to the end of the source line (setext list marker, two-line code span); "begins a line" made the same in Rule and Implementation; list-marker exception for modes that move text; `*`/`_` re-pairing falls back to the read-back check                                                     |
| 5     | 9        | Simplified: the break is written only before text that begins with a letter, replacing the block-start exceptions (`never` mode could still make a list); no break above a table delimiter row or inside inline `<pre>`; setext handling only when the option is on                                                |
| 6     | 4        | Findings below, all applied: the table exception is decided by the block below the paragraph rather than the next source line, which wasn't stable on a second pass; `<pre>`-like elements at any nesting level; a known limitation for `[a<br>b]`; setext line starts kept in their own field                     |

#### Last specification review

Prompting Claude Opus 5.5 (`claude-opus-5-5`), extra high effort (`xhigh`), in
Claude Code 2.1.280, with `Review the spec in src/generation/br-new-line-spec.md.`
produced the following output on commit 4fc597b; strikethrough indicates LLM
output that we deemed incorrect or irrelevant. There is none: all four findings
were applied in 3102232.

> Round 6 found one real bug: the table delimiter-row exception (Rule section, "next source line"; example 40) breaks the "format again, no change" guarantee. The spec's own example 40 would fail the test runner, which formats every spec twice. I also found one gap that needs a decision and two minor points. Everything else I checked against the code holds.
>
> I didn't run the formatter. Writes outside the repo and into `target/` were both blocked, so these findings come from reading the code. I didn't edit the spec.
>
> ## 1. The delimiter-row exception changes on a second pass and misses the case it's for (implicit context)
>
> The check reads the source line after the tag's line, but the formatter doesn't keep that line where it was:
>
> - **Example 40 changes on the second pass.** At top level, the formatter always writes a blank line between a paragraph and the next block (`generate.rs:661`). In example 40, `a | b<br>c | d⏎- | -` is a paragraph followed by a list: the cell counts differ, so it isn't a table, and `- | -` interrupts the paragraph.
>   - Pass 1 sees `- | -` below, holds back the break and writes `a | b<br>c | d⏎⏎- | -`.
>   - Pass 2 sees a blank line below and adds the break: `a | b<br>⏎c | d⏎⏎- | -`.
>
>   So the stated reason ("the new line would sit directly above it") doesn't hold here.
> - **A delimiter-shaped line inside the paragraph flips the same way.** In `a | b<br>c | d⏎| - | - |`, the formatter pulls the last line up onto the line above in every mode (`generate.rs:468–475`, `block.rs:910`; `Text_BlockStarts.txt:15–25` shows this in `maintain`).
> - **The real risk is missed.** It exists only where the next block is written directly below with no blank line, which happens only inside a list. Take `> - a | b<br>c | d⏎>   - | -`. The next source line starts with `>`, so the shape check is false and the break is written. Then `c | d` (2 cells) sits over `- | -` (2 cells) and becomes a table (`block.rs:532`).
>
> **Fix:** decide this by block structure, not by source lines. Hold back the break for every tag in a paragraph that is followed, with no blank line between, by a block whose first line (after its container prefix) is shaped like a delimiter row. That decision is the same on every pass.
>
> - Replace example 40 with `- a | b<br>c | d⏎  - | -` and add the block-quote case above.
> - `is_table_delimiter_shape` is private (`block.rs:1000`), so the spec should say to make it `pub` or to use `starts_block_in_paragraph` instead.
>
> ## 2. The `<pre>`/`<textarea>` check is unclear (ambiguity, omission)
>
> - **Where it looks.** The spec puts the check in `gen_nodes_within_breaks`, which only sees the tag's siblings. In `x <pre>*a<br>b*</pre>`, the tag's parent is the emphasis while `<pre>` belongs to the paragraph. A sibling scan misses it, and a visible extra line appears inside the `<pre>`. The spec should say the scan covers the whole paragraph or setext heading at every nesting level, and add this as an example.
> - **Which tags.** `<listing>`, `<xmp>` and `<plaintext>` also show source line breaks. Tag names should match in any case (`<PRE>`), as the break-tag definition already does.
> - "In the paragraph" should also say "or setext heading".
>
> ## 3. A break can turn bracketed text into a link (omission, low)
>
> Link labels are matched with runs of whitespace collapsed to one space (`links.rs:258`). Take `[a<br>b]⏎⏎[a<br> b]: /u`:
>
> - `[a<br>b]` isn't a link, because `a<br>b` doesn't match `a<br> b`, so the shortcut-link exception doesn't apply.
> - The break writes `[a<br>⏎b]`, which now matches and becomes a link.
>
> It's contrived. Either hold back the break inside an unmatched `[…]`, or list it as a known limitation so the intro's "can't change the document" stays true.
>
> ## 4. Recording setext line starts affects more than `<br>` (interface contract, low)
>
> Other code also reads `is_line_start`:
>
> - `gen_hard_break` uses it to choose two spaces or `\` (`generate.rs:2982`).
> - `gen_text` uses it in its leading-block-words check (`generate.rs:1577`).
>
> The obvious way to record the line starts, `with_paragraph_escapes`, also sets `block_end`, which `text_from` and `is_within_block` read. So with the option on, setext headings that contain no `<br>` can come out differently. Either store these line starts in a field only the new check reads, or state the side effects.
>
> ## Checked and holds
>
> - Text nodes are trimmed of spaces and tabs (`inline.rs:367`), so example 3 works with the "first character" test.
> - Image alt text is written as raw text (`generate.rs:2280`), so `![a<br>b](/i)` contains no tag node.
> - ATX headings, table cells and definition-list terms are all written with newlines forced off (`generate.rs:773, 2864, 3113, 3178`).
> - The emphasis fallback sets `is_preserving_decorations()` for the whole paragraph (`generate.rs:131`), so the new flag is needed. Collapsed references pass `is_collapsed` to `gen_link_text` (`generate.rs:2515`).
> - Paragraph line starts are only the first child and children after a hard break (`generate.rs:812–828`), which matches the spec's "begins a line". Setext headings record no line starts today.
> - Example 14 (`<br><br>⏎b`) is safe, because a type 7 HTML block needs a single tag alone on its line.
> - Footnote content is indented by 4 (`generate.rs:2243`), matching example 21. `check_all_values_set` counts 31 options today (`builder.rs:317`).

### Daniil Chegodayev

GitHub userid: Daniil-Chegodayev

#### Contributions

Reviewed the first draft and reworded its introduction, Problem and Rule
sections (f045976).

#### Build and run test

Prompting GPT-6 Luna with "Verify that you can build and run the project in the
current git repo." produced the following output, summarized:

- `cargo build` succeeds with the repository's pinned Rust 1.98.0.
- `cargo test` passes: 160 unit tests, `br_render_check` (2),
  `commonmark_spec_check` (10), `corpus_check` (1), `html_parser_specs` (8),
  `nesting_test` (2), `newline_test` (9), `parser_specs` (39), `specs` (199),
  and 2 documentation tests.

#### Specification review rounds

My individual reviews are listed below:

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| Manual draft review (`f045976`) | 3 | Reworded the introduction, Problem and Rule sections to clarify the source-level behavior and the purpose of the exceptions. |
| Final LLM-assisted review (after round 12) | 0 | Reviewed the final spec's consistency, exceptions and examples; no changes were needed. See the review below. |

#### Last specification review

Prompting GPT-6 Luna with the prompt:

> Review `src/generation/br-new-line-spec.md` for internal consistency,
> completeness of the rule and exceptions, and gaps in the examples. Report
> concrete correctness or clarity findings, and identify renderer-specific
> claims you cannot verify from the specification alone.

produced the following review on commit 7986cd0. No changes were required:

> I found no concrete contradictions or correctness omissions in the rule,
> exceptions, examples, or implementation and test-plan descriptions. The
> examples cover the main branches of the rule, including wrapping, headings,
> links, list and table hazards, and preformatted tags. The known limitations
> are called out explicitly.

### Zhan Dzholdybayev

GitHub userid: zhanapan

#### Contributions

Rewrote the Examples table and the test plan, with a concrete output for each
spelling of `<br>`, each `textWrap` mode and the round 1 edge cases, and moved
`is_break_tag` beside `starts_html_block` (5f61af7, with Gemini).

#### Build and run test

Prompting an LLM with "Verify that you can build and run the project in the current git repo." produced the following summary:

- Native build via `cargo build` succeeds cleanly with zero warnings on Rust 1.98.0.
- `cargo test` executes and passes all test suites: 160 unit tests, `br_render_check` (2), `commonmark_spec_check` (10), `corpus_check` (1), `nesting_test` (2), `newline_test` (9), documentation tests, and all integration suites (`specs`, `parser_specs`, `html_parser_specs`).

#### Specification review rounds

Review rounds focused specifically on the "Examples" and "Test plan" sections assigned to me.

| Round | Findings | Main changes made                                                                                                                                                                                                                                                                                                                                               |
| ----- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1     | 4        | Replaced prose description in Example 4 with concrete Markdown outputs for each `<br>` variant; added test examples covering `textWrap: never`, `sentence`, and `maintainAndWrap`; added edge cases for HTML block type-7 start (`<br>b`) and shortcut links (`[a<br>b]`); relocated `is_break_tag` to `src/generation/generate.rs` beside `starts_html_block`. |

#### Last specification review

Prompting Gemini 3.8 Flash with:
`"Review the 'Examples' table and test plan in src/generation/br-new-line-spec.md for coverage of all textWrap modes and CommonMark edge cases where a newline breaks AST parsing."`
produced the following output:

> 1. **Example 4 is underspecified:** The output says "A line break after the tag; the tag unchanged". This must be expanded into distinct test cases showing `a<BR>⏎b`, `a<br/>⏎b`, `a<br />⏎b`, and `a<br class="x">⏎b` so automated tests can assert exact strings.
> 2. **Missing `textWrap` modes:** The test plan only specifies `maintain` and `always`. The modes `never`, `sentence`, and `maintainAndWrap` need explicit test cases to prove they insert a line break without breaking mode-specific semantics.
> 3. **Block-start HTML parsing hazard:** In CommonMark, `<br>b` at the beginning of a line starts a type-7 HTML block. The examples must explicitly demonstrate that `<br>` at block start remains unchanged to avoid breaking Markdown formatting.
> 4. **Reference links:** Shortcut links `[a<br>b]` must not be broken across lines because whitespace normalization would break label resolution. Inline links `[a<br>b](/u)` can be safely broken.

## Final submission

The instructor reviewed the draft on 2026-11-14 and requested changes. In
response:

- A hard break can also be two trailing spaces; a break tag is defined by the
  parser's `Node::Html`, so the list of non-cases went.
- Checking the implementation: it adds about 240 lines to `src/generation` (220
  net, with rustfmt's layout) and 28 lines of option plumbing. With the option
  off, every existing test passes unchanged; with it on, every example gives
  its expected output in each `textWrap` mode. Writing it showed several
  exceptions could be stated more simply, and the spec now does.
- `tests/br_render_check.rs` formats every spec case holding a `<br>` with the
  option on, renders it with pulldown-cmark and checks the HTML is the same as
  the input's, or as today's output's. Removing the line-start or `<pre>`
  exception makes it fail.
- Reviews that run code: the draft's rounds were read-only, so the reviewer
  couldn't run anything. From round 7 on, every specification and
  implementation review ran in a throwaway worktree of the implementation, where
  the reviewer could write scratch tests and run `cargo`; from specification
  round 9 and implementation round 2 on, the reviewers also fuzzed the
  formatter with up to about 120,000 random documents.

### Askar Oralkhan

GitHub userid: askaroralkhan

#### Contributions

Applied the instructor's review, wrote the implementation and its tests (the
`Html_BrNewLine*` spec files and the pulldown-cmark rendering check), and ran
specification review rounds 7–12 and implementation review rounds 1–4 with the
reviewer able to run code (with Claude Opus 5.5).

#### Specification review rounds

These continue the draft's numbering.

| Round | Findings | Main changes made                                                                                                                                                                                                                                                                                                                                                |
| ----- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 7     | 5        | Reviewer runs code from here on. A tag first on its line is left alone, which covers a callout's `[!NOTE]` line; `<script>`/`<style>` join the `<pre>`-like tags; the rendering check runs over every spec case holding a `<br>` and keeps `<pre>` whitespace; CSS limitations stated; an unverified mdBook claim struck                                         |
| 8     | 5        | Wrapping could move a tag to a line start, which a second pass read differently, so the space before a tag that gets a break is a plain space; the table check reads a `*`/`+` bullet as `-`; `<code>` joins the `<pre>`-like tags; a `<pre>` opened earlier in the file is a known limitation                                                                   |
| 9     | 5        | A paragraph that begins like a link reference definition (`[x]: <br>b`) gets no break; the table check made stable when lines are joined or wrapped; `<tt>` joins the `<pre>`-like tags; the render check keeps `<code>`/`<tt>` whitespace                                                                                                                       |
| 10    | 11       | A list whose first item begins with a table or list counts for the table check; a `<br>` that begins a line keeps the next word beside it, so wrapping can't leave it alone where a block begins; the render check compares with the input or today's output; definition lists on GitHub and `>` after a task box listed as known limitations; six wording fixes |
| 11    | 4        | "Begins a line" decided from the parsed tree (a text `>` on an indented line was unstable); a footnote read as a link reference definition listed as a known limitation; example 42's underline fixed; the table-or-list check limited to bullet lists                                                                                                           |
| 12    | 6        | Findings below, all applied: the table check reads the parsed list (a `>` prefix inside a block quote was unstable) and ignores a `\` hard break on a delimiter cell; two older formatter hazards the option can reach listed as known limitations; two wording fixes                                                                                            |

#### Last specification review

Prompting Claude Opus 5.5 (`claude-opus-5-5`), extra high effort (`xhigh`), in
Claude Code 2.1.280, running in a throwaway worktree of the implementation
where it could write scratch tests and run `cargo`, with
`Review the spec in src/generation/br-new-line-spec.md.` produced the following
output on 801e69a; strikethrough indicates LLM output that we deemed incorrect
or irrelevant. There is none: findings 1, 2 and the two wording points were
applied, and findings 3 and 4 were added to the known limitations, as the
reviewer suggested.

> Spec review, round 12: I found two real bugs. Both break the spec's promise that "Formatting any output again leaves it unchanged" (line 223), and both are in the table exception's text checks. I also found two smaller gaps that should probably be listed as known limitations, plus two wording points.
>
> I ran the implementation to confirm everything below; `cargo test` passes. My scratch test file is deleted, the tree is clean, and I didn't edit the spec.
>
> ## 1. The bullet-list check doesn't skip a `>` prefix (medium)
>
> The Implementation section (lines 255–256) says the check passes when the list's first item "begins with `|`, after any whitespace and line endings". The code does exactly that (`generate.rs:819`). Inside a block quote, though, a `>` sits between the line ending and the `|`, so the check misses it. The formatter then pulls the `| -` up beside the marker, and the second pass sees `- | -`.
>
> `> a<br>b⏎>⏎> -⏎>   | -` with `textWrap: always`, `never` or `sentence`:
>
> - Pass 1: `> a<br>⏎> b⏎>⏎> - | -`
> - Pass 2: `> a<br> b⏎>⏎> - | -`
>
> The same thing happens with `*` as the marker, and with the list inside an item in a block quote. With a blank line between the paragraph and the list, no table can form, so the problem is stability only.
>
> **Fix:** decide this from the parsed tree, not the source text. The test would be: the first item's first child is a paragraph whose text starts with `|`. A paragraph's span already starts after its container prefix. Then reword lines 255–256 to match and add this input as an example.
>
> ## 2. The bare delimiter-cell check flips when a hard break is rewritten (medium-low)
>
> `is_bare_delimiter_cell` runs on the paragraph's source split at whitespace (`generate.rs:782`). The formatter rewrites a two-space hard break as `\`, which sticks to the word before it.
>
> `a<br>b -:␣␣⏎c` (two trailing spaces), in any mode:
>
> - Pass 1 holds the break back because of `-:`, and writes `a<br>b -:\⏎c`.
> - Pass 2 sees the word `-:\`, which isn't a bare cell, and writes `a<br>⏎b -:\⏎c`.
>
> It also fails the other way round: with `hardBreakKind: doubleSpace` and `textWrap: always`, `a<br>b -:\⏎c` gives `a<br>⏎b -:␣␣⏎c` on pass 1, then `a<br> b -:␣␣⏎c` on pass 2.
>
> So the claim at lines 118–120 ("These tests don't change when the formatter rewrites a marker or joins or wraps lines") doesn't cover hard breaks.
>
> **Fix:** drop a trailing `\` from each word before the check. Add "or rewrites a hard break" to that sentence, and add this input as an example.
>
> ## 3. Rule 4's plain space can move a wrap onto a hazard the formatter already has (low; the page changes)
>
> `- </span> aaaa <br>b` with `textWrap: always`, `lineWidth: 16`:
>
> - **Option off:** `- </span> aaaa⏎  <br>b`, which renders like the input.
> - **Option on:** `aaaa <br>` can't be split, so the line wraps earlier: `- </span>⏎  aaaa <br>⏎  b`.
>
> `</span>` is now alone on the item's first line. That starts an HTML block (type 7), so the rest of the item becomes raw HTML.
>
> Today's formatter already does this at other widths. `- </span> aaaaaaaaaaaaaaaa` at width 16 gives a lone `</span>` with the option off. Rule 4 only protects `<br>` tags, so the option just makes this existing hazard reachable for more inputs. The fuzz run below hit one more case of this kind, which I didn't reduce: a `| -` fragment wrapped to the start of a line, which the second pass pulls back up.
>
> **Suggestion:** list it as a known limitation. Fixing it means changing how the formatter wraps after any inline tag at the start of a block, which is out of scope.
>
> ## 4. The formatter's parser reads a link reference definition after a task box (low; stability only)
>
> `- [ ] [r]: /u Lorem<br>Lorem` with `textWrap: always`, `lineWidth: 20`:
>
> - **Pass 1:** the `[`…`]:` exception holds the break back. The line wraps to `- [ ] [r]: /u⏎      Lorem<br>Lorem`.
> - **Pass 2:** the formatter's parser now reads `[r]: /u` as a definition. CommonMark and GFM don't, and pulldown-cmark renders it as text. So the rest of the paragraph gets the break.
>
> Without the task box, the formatter avoids that wrap. This is the same kind of problem as the existing `>`-after-task-box limitation (line 145) and could go next to it.
>
> ## Minor
>
> - **Line 89, "the node before it":** this should say "with the same parent", as the definition of "What follows" does. Take `a⏎_<br>b_ c`: in document order, the node before the tag is `a`, with a line ending between them, yet the implementation writes the break (`a⏎_<br>⏎b_ c`). Its line begins with `_`, so the break is correct; only the wording is off.
> - **Wikilinks (line 133):** it could say plainly that the break *is* written there (`[[a<br>⏎b]]`). "Out of scope" can read as "left alone".
>
> ## What checks out
>
> - **Implementation section:** it matches the code function by function. The size estimate is accurate: 236 lines added and 213 net, against "about 240 (210 net)". The option count goes from 31 to 32, and the schema entry is there.
> - **Test files:** the configurations and example placement match the test plan. Every example that names no mode is in all five `textWrap` files; 16a, 37 and 42a–c are in the setext file, and 29 is in the option-off file.
> - **Fuzz run:** I generated about 120,000 random documents mixing block quotes, lists, task boxes, footnotes, definitions, table fragments, link text, `www.` and delimiter cells. Each ran in all five modes, at widths 80, 20 and 12, and with setext headings: about 970,000 format runs. I only counted cases where today's formatter keeps the page and is stable. Apart from findings 2–4 and the known unclosed-`<pre>` limitation, there were no option-only instabilities or page changes.
>   - The page changes it did report were all of one kind. pulldown-cmark reads a `:` with no space after it as a definition marker (`b c⏎:-:x` becomes a `<dl>`); the formatter doesn't. That mismatch exists today, and the spec-file render check passes, but it would make a fuzzed render check noisy.
> - **Hand checks:** these all came out stable and render the same:
>   - a callout with the tag on the marker line
>   - `[a⏎<br>b](/u)` in every mode
>   - a delimiter row with pipes inside the paragraph (it gets pulled up, so no table)
>   - definitions and footnotes followed by `- | -`
>   - 3-space nested markers and `*` markers
> - **Not verified:** the GitHub-specific claims (`www.` linking, heading anchors, a bare `:-:` read as a delimiter row). They match what I know of cmark-gfm, but I didn't run anything against GitHub.

#### Implementation review rounds

Each round was a new Claude Code session in a throwaway worktree of the branch,
where the reviewer could build the formatter, write scratch tests and run
`cargo`; rounds 2–4 also ran tens of thousands of random inputs. Nothing it
wrote reached the repo; the accepted findings were applied as small, targeted
edits.

| Round | Findings | Main changes made                                                                                                                                                                                                                        |
| ----- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1     | 5        | Headings get no break, since a line break can change a GitHub heading anchor (`a<br> b`), which removed the setext code; the rule is decided once per tag; the render check leaves wikilinks out; narrowing the table exception declined |
| 2     | 4        | Task-list items get the table check before their nested list; the table check narrowed to a bullet list whose first item begins with `                                                                                                   | `; the render check uses GFM footnotes; rustfmt |
| 3     | 4        | A paragraph holding a delimiter cell without `                                                                                                                                                                                           | `` (``:-:``) gets no break, since GitHub would make a table (found with comrak, a port of GitHub's parser); the list check skips line breaks after the marker; a redundant check removed; ``list.indentKind: pythonMarkdown` definitions listed as a known limitation |
| 4     | 3        | Findings below: no correctness bugs; a wording contradiction in the spec and a stale comment fixed; an optional refactor declined                                                                                                        |

#### Last implementation review

Prompting Claude Opus 5.5 (`claude-opus-5-5`), extra high effort (`xhigh`), in
Claude Code 2.1.280, running in a throwaway worktree of the branch where it
could build the formatter, write scratch tests and run `cargo`, with
`Review the implementation of the spec in src/generation/br-new-line-spec.md (the changes on this branch since upstream/main).`
produced the following output on 801e69a; strikethrough indicates LLM output
that we deemed incorrect or irrelevant. Finding 3 is struck through: it is an
optional refactor, and the reviewer itself recommends keeping the local tag
list, so we kept the code as it is. Findings 1 and 2 were applied.

> I found no correctness bugs. The implementation matches the spec, all 199 tests pass, and `cargo fmt --check` and `cargo clippy --all-targets` are clean.
>
> **How I checked:** I read the full diff against `upstream/main`. Then I ran about 70 extra inputs through a temporary test, with the option on and off, and formatted each output a second time to check it doesn't change again. The inputs covered:
>
> - definition lists and footnotes where the tag comes first
> - images, and shortcut and collapsed links
> - tables that interrupt a paragraph
> - bare delimiter cells, and list-item delimiter rows (indented, `*`, task boxes, escaped `|`)
> - wrapping around emphasis, links and code spans, including nested containers
> - CJK text, tags that span lines, CRLF line endings, two-space hard breaks
> - ignore comments and every `textWrap` mode
>
> Two things held up particularly well:
>
> - **Tables:** the parser already keeps any delimiter-shaped line containing a `|` off a line start. `is_bare_delimiter_cell` and `may_begin_with_delimiter_row` cover the two cases it misses.
> - **Block quotes:** the newline that sits inside the `if_true_or` still gets its `>` prefix, because the text after it is outside the condition.
>
> The temporary test file is deleted and the worktree is clean.
>
> **Minor findings:**
>
> 1. **The spec contradicts itself.** Rule 4's last sentence and example 55 say a tag that already begins a line keeps the word after it on the same line. The exceptions list says that for "The tag begins a line" the formatter "writes today's separator". In `always` and `maintainAndWrap`, today's separator lets wrapping split them (`<br>⏎https://…`). The code follows rule 4 and example 55, which is the right choice. The exceptions list (`br-new-line-spec.md:81-96`) should say the formatter "writes no line break after the tag" instead of "writes today's separator".
>
> 2. **One misleading comment.** `generate.rs:758` says "`last_node` is a break tag, which the caller checks", but `breaks_line_after_tag` has no `last_node`. Something like "the caller checks that the node before `next` is a break tag" would be clearer.
>
> 3. ~~**Optional tidy-up.** `is_break_tag` and `holds_preformatted_tag` each extract the tag name by hand. One small helper that reads the name once would shorten both. `crate::html::tags` already has `is_preformatted` and `is_raw_text`, but their lists don't match the spec's (no `xmp`, `code` or `tt`, and an extra `title`), so I'd keep the local list.~~
>
> **Unrelated to this branch:** `a<br>b⏎+⏎  | -` changes again on a second format with the option off too. The formatter moves `| -` up beside the `+`, and `+ | -` then reads as a list item. That's an upstream bug, not something this branch introduced.
>
> My probe inputs are still in `/tmp/brprobe/` because deleting files there needed approval; you can remove that folder.

### Daniil Chegodayev

GitHub userid: Daniil-Chegodayev

#### Contributions

Reviewed the first draft and revised the introduction, Problem and Rule sections
to clarify that the change concerns formatted source, explain the original
wrapping problem, and make the rule and its exceptions easier to follow
(f045976).

At the end, prompted GPT-6 Luna to review the completed specification for
consistency, exception coverage and gaps in the examples. The review found no
concrete issues requiring changes.

### Zhan Dzholdybayev

GitHub userid: zhanapan

#### Contributions

Checked the finalized 49-case examples table against CommonMark rendering behavior, and verified the trial implementation and full test suite locally.


#### Build and run test

Prompting an LLM with "Verify that you can build and run the project in the current git repo." on the final commit produced the following summary:

- `cargo build` succeeds without issues.
- `cargo test` passes 100% of tests, including `br_render_check` (verifying HTML identity with and without the option) and all newly added `Html_BrNewLine*` specs across all wrap modes.

#### Specification review rounds

| Round | Findings | Main changes made                                                                                                                                                                               |
| ----- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 2     | 2        | Verified idempotency across the 49 examples; confirmed table delimiter row checks with `*` and `-` list bullets; verified that plain space before `<br>` prevents unwanted line-start wrapping. |

#### Last specification review

Prompting Gemini 3.8 Flash with:
`"Review the updated 49 examples and the pulldown-cmark rendering verification test plan in src/generation/br-new-line-spec.md."`
produced the following output:

> 1. All 49 examples now specify exact inputs and outputs and cover all wrap modes, edge cases (block start, tables, `<pre>`, links), and configuration flags.
> 2. The `pulldown-cmark` test plan ensures no semantic HTML changes occur between formatted and unformatted markdown. No further specification changes needed.
