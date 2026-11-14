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

_To be added by Daniil: run "Verify that you can build and run the project in the current git repo." in your own clone and paste a summary of the output here._

#### Specification review rounds

_To be added by Daniil: the review rounds you ran, if any, with the same columns as Askar's table; the team's shared rounds are in Askar's section._

#### Last specification review

_To be added by Daniil: your last review, the prompt and the LLM, if you ran one._

### Zhan Dzholdybayev

GitHub userid: zhanapan

#### Contributions

Rewrote the Examples table and the test plan, with a concrete output for each
spelling of `<br>`, each `textWrap` mode and the round 1 edge cases, and moved
`is_break_tag` beside `starts_html_block` (5f61af7, with Gemini).

#### Build and run test

_To be added by Zhan: run "Verify that you can build and run the project in the current git repo." in your own clone and paste a summary of the output here._

#### Specification review rounds

_To be added by Zhan: the review rounds you ran, if any, with the same columns as Askar's table; the team's shared rounds are in Askar's section._

#### Last specification review

_To be added by Zhan: your last review, the prompt and the LLM, if you ran one._

## Final submission

The instructor reviewed the draft on 2026-11-14 and requested changes. In
response:

- A hard break can also be two trailing spaces; a break tag is defined by the
  parser's `Node::Html`, so the list of non-cases went.
- Checking the implementation: it adds about 200 lines to `src/generation` (180
  net) and 28 lines of option plumbing. With the option off, every existing
  test passes unchanged; with it on, every example gives its expected output in
  each `textWrap` mode. Writing it showed two exceptions could be stated more
  simply, and the spec now does.
- `tests/br_render_check.rs` formats every spec case holding a `<br>` with the
  option off and on, renders both with pulldown-cmark and checks the HTML is
  the same. Removing the line-start or `<pre>` exception makes it fail.
- Reviews that run code: the draft's rounds were read-only, so the reviewer
  couldn't run anything. From round 7 on, each review ran in a throwaway
  worktree of the implementation, where the reviewer could write scratch files
  and run `cargo`.

### Askar Oralkhan

GitHub userid: askaroralkhan

#### Contributions

Applied the instructor's review, wrote the implementation and its tests (the
`Html_BrNewLine*` spec files and the pulldown-cmark rendering check), and ran
specification review rounds 7–8 with the reviewer able to run code (with Claude
Opus 5.5).

#### Specification review rounds

These continue the draft's numbering.

| Round | Findings | Main changes made                                                                                                                                                                                                                                                                                                                                                |
| ----- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 7     | 5        | Reviewer runs code from here on. A tag first on its line in the file is left alone, which covers a callout's `[!NOTE]` line; `<script>`/`<style>` join the `<pre>`-like tags; the rendering check compares option off and on over every spec case holding a `<br>` and keeps `<pre>` whitespace; CSS limitations stated; an unverified mdBook claim struck       |
| 8     | 5        | Findings below, all applied: wrapping could move a tag to a line start, which a second pass read differently, so the space before a tag that gets a break is a plain space; the table check reads a `*`/`+` bullet as `-`, as the formatter writes it; `<code>` joins the `<pre>`-like tags; a `<pre>` opened anywhere earlier in the file is a known limitation |

#### Last specification review

Prompting Claude Opus 5.5 (`claude-opus-5-5`), extra high effort (`xhigh`), in
Claude Code 2.1.280, running in a throwaway worktree of the implementation
where it could write scratch files and run `cargo`, with
`Review the spec in src/generation/br-new-line-spec.md.` produced the following
output on spec 57f5b93 and implementation 0e411db; strikethrough indicates LLM
output that we deemed incorrect or irrelevant. There is none: all five findings
were applied in 7c8b2ef, the last one (GitHub's `<code>` styling, which the
reviewer couldn't confirm) by adding `<code>` to the `<pre>`-like tags, which
can only hold breaks back.

> I found two real bugs. Both break the spec's promise that formatting the output again leaves it unchanged (`br-new-line-spec.md:178`). I ran the trial implementation to confirm each one.
>
> I didn't edit the spec. For testing I added temporary scratch files under `tests/`, then deleted them, so the working tree is clean. `cargo test` passes.
>
> ## 1. Wrapping can put the tag at the start of a line, and the second pass changes it (high)
>
> The "tag begins a line" exception (`br-new-line-spec.md:82`) is decided by where the tag sits in the input file. In `always` and `maintainAndWrap`, the space before a tag is a point where the line can wrap. When the tag lands right at the line width, the line wraps there:
>
> 1. Pass 1 puts `<br>` at the start of a line and still writes the break after it.
> 2. Pass 2 sees the tag beginning a line in the file, so the exception applies and the break is joined back up.
>
> With `textWrap: always, lineWidth: 40`:
>
> ```
> Lorem ipsum dolor sit amet, consectetur <br> adipiscing elit, sed do eiusmod <br> tempor incididunt ut labore et dolore <br> magna aliqua.
> ```
>
> Pass 1:
>
> ```
> Lorem ipsum dolor sit amet, consectetur
> <br>
> adipiscing elit, sed do eiusmod <br>
> tempor incididunt ut labore et dolore
> <br>
> magna aliqua.
> ```
>
> Pass 2:
>
> ```
> Lorem ipsum dolor sit amet, consectetur
> <br> adipiscing elit, sed do eiusmod
> <br>
> tempor incididunt ut labore et dolore
> <br> magna aliqua.
> ```
>
> - **Other inputs:** the same thing happens in bullet and ordered list items, in block quotes, and after a code span.
> - **`maintainAndWrap`:** the output is stable, but uneven. A file already wrapped as `…⏎<br> eeee` never gets the break.
> - **Why the tests miss it:** no existing spec case has a tag right at the line width. I ran every spec case containing `<br>` in all five modes at widths 80, 20 and 12 (3,735 runs), and none failed. Only inputs built for this case fail.
>
> **Suggested fix:** wherever the rule would write a break after a tag, write the separator before the tag as a plain space. Wrapping then can't separate the tag from what comes before it. Add an example such as `aaaa bbbb cccc dddd <br> eeee` with `textWrap: always, lineWidth: 20`, giving `aaaa bbbb cccc⏎dddd <br>⏎eeee`.
>
> ## 2. The delimiter-row check reads the list marker from the file, but the formatter rewrites it (medium)
>
> The table exception (`:98–102`) checks the next block's first line as it appears in the file (`generate.rs:756`, `generate.rs:2919`). But the formatter rewrites bullet markers, and only `-` makes the delimiter shape. This fails in three ways:
>
> - **A `*` sub-list becomes a table.** With the default list marker, `- a | b<br>c | d⏎  * | -` is written with the break, and the sub-list becomes `- | -`. Pass 2 then reads `c | d` over `- | -` as a table and writes `| c | d |⏎| - | - |`. So the second pass turns a nested list into a real table.
> - **With `unorderedListKind: asterisks`.** `- a | b<br>c | d⏎  - | -` holds the break back in pass 1 and writes `* | -`. Pass 2 then writes the break.
> - **At top level, in modes that join lines.** With `textWrap: never`, `a | b<br>c | d⏎* | -` gives `a | b<br>⏎c | d⏎⏎- | -` on pass 1 and `a | b<br> c | d⏎⏎- | -` on pass 2.
>
> So the spec's own claim, "Deciding this by the next block … gives the same output when the result is formatted again", doesn't hold yet.
>
> **Suggested fix:** for a bullet list, test its first line with the marker read as `-`, whatever the file or the config uses. Change that sentence in the Rule and the `gen_item` bullet in Implementation (`:203–207`) to match, and add the three cases as examples.
>
> ## Minor points
>
> - **Rule 4 (`:60`):** "filled" doesn't fit `maintainAndWrap`, which keeps line breaks and only wraps long lines.
> - **`<pre>` limitation (`:119`):** "a `<pre>` opened in an earlier paragraph" misses the likelier case. That's an HTML block such as `<div><pre>`, then a blank line, then Markdown. "Opened earlier in the file" covers both.
> - **Unverified:** I believe GitHub's stylesheet sets `white-space: break-spaces` on inline `code`. If so, the formatter's `x <code>a<br>⏎b</code>` would show an extra line on GitHub. I couldn't confirm it because the web fetch was declined. If it's true, it belongs in the known limitations or the `<pre>`-like list.
>
> ## What checks out
>
> - **Implementation section:** it matches the trial code function by function. The size estimate is accurate: 189 lines added and 169 net, against "about 190 (170 net)". The test files and their configurations match the spec's list.
> - **No render changes found:** pulldown-cmark rendered every case the same with the option off and on, including my extra edge cases.
> - **Inputs that behave correctly:** these gave stable, correctly indented output:
>   - setext h2 headings with a `-----` underline (the formatter reads setext before tables)
>   - definition-list definitions and task-list items
>   - CJK text
>   - `<span><br>b`
>   - text on the same line as a callout's `[!NOTE]`
>   - sentence mode with a sentence ending right before the tag
>   - `***` and `___` thematic breaks after the paragraph

#### Implementation review rounds

Not started yet.

#### Last implementation review

Not started yet.

### Daniil Chegodayev

GitHub userid: Daniil-Chegodayev

_To be added by Daniil: the same sections as above for the final submission._

### Zhan Dzholdybayev

GitHub userid: zhanapan

_To be added by Zhan: the same sections as above for the final submission._
