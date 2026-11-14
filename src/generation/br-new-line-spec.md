# A `<br>` starts a new line

A rendered line break after an inline `<br>` tag should also be a source line
break, wherever adding one can't change how a CommonMark or GFM renderer shows
the document. A
[hard line break](https://spec.commonmark.org/0.31.2/#hard-line-breaks) (`\`
or two spaces at the end of a line) already behaves this way; `<br>` is the
[raw HTML](https://spec.commonmark.org/0.31.2/#raw-html) spelling of the same
[break](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-br-element).

This is a rule about the formatter's output, not about a browser's rendering.
If the text after the tag is in the same paragraph, then the source should keep
that break visible as well.

## Problem

With `textWrap` set to `always`, `never` or `sentence`, the formatter joins the
line after a `<br>` to the one before it, as it does any line break within a
paragraph:

```markdown
First line<br>
second line.
```

is written as `First line<br> second line.`. With `textWrap: always`, the
joined text is then wrapped at the line width rather than at the `<br>`. No mode
breaks the line after a `<br>` written mid-line (`a<br>b`). The rule below adds
that behaviour as an option.

## Definitions

- **Break tag:** an inline HTML node (`Node::Html` that isn't an
  [HTML block](https://spec.commonmark.org/0.31.2/#html-blocks)) holding a start
  tag named `br` in any letter case, with or without attributes and a `/`
  before the `>`: `<br>`, `<BR>`, `<br/>`, `<br />`, `<br class="x">`.
- **What follows:** the next node after the break tag with the same parent
  (the paragraph, or the emphasis, link or other inline element holding the
  tag), skipping whitespace (spaces, tabs, and line endings). A tag that is the
  last thing inside an inline element, as in `[a<br>](/u)b`, has nothing
  following it.

## Rule

A new boolean option, `lineBreakAfterBr`, turns the rule on. It is `false` by
default, which writes a `<br>` as today. The rule is opt-in because existing
files would otherwise be rewritten, and because a renderer that shows every
source line break (GitHub comments, markdown-it with `breaks: true`) would show
an extra line after the `<br>`.

When `lineBreakAfterBr` is `true`, in every `textWrap` mode, when something
follows a break tag in a paragraph or setext heading, the formatter writes a
line break between the tag and what follows:

1. The whitespace between the tag and what follows is replaced by one line
   break. Whitespace before the tag is handled as today.
2. The tag is written exactly as it appeared (letter case, attributes, slash).
3. The new line starts at the paragraph's indentation (list item, block quote,
   footnote, definition), just as after a hard break.
4. If wrapping is active (`always`, `maintainAndWrap`), the text after the tag
   is wrapped as usual, but the line break after the tag is never removed. The
   space before such a tag is written as a plain space, not a place to wrap, so
   wrapping can't move the tag to the start of a line.
5. The rule also applies in the text of an inline or full reference link
   (`[a<br>b](/u)`, `[a<br>b][r]`), which `always`, `never` and `sentence`
   otherwise keep on one line, just as a `\` hard break there already does.

The formatter writes the line break only when what follows is text that
begins with an alphabetic character (Unicode `Alphabetic`, in any script).
Nothing that starts a block at the start of a line begins with one: list markers, `#`, `>`, code fences, thematic
breaks, footnote definitions (`[^1]:`), HTML tags and comments, and setext
underlines all begin with something else. Neither do another break tag (so
`<br><br>` stays together) or a hard break. Other text, such as `**bold**`,
`` `code` `` or `[a link](/u)`, stays on the tag's line, which keeps the rule
simple.

Even when what follows begins with a letter, the formatter writes today's
separator when:

- **The tag is in a heading written as ATX, a table cell or a definition-list
  term.** These are written on one line. A heading written as setext can span
  lines, so the rule applies there, including to `# a<br>b` with
  `heading.kind: setext`.
- **The tag begins a line:** it is the first node of its paragraph or setext
  heading, the first node after a hard break, or the first thing on its line
  in the file after any `>` markers and indentation (as after a callout's
  `[!NOTE]` line). A line holding only the tag would start an
  [HTML block](https://spec.commonmark.org/0.31.2/#html-blocks) (type 7) where
  a block begins, which would turn what follows into raw HTML. Where a block
  can't begin, a break would be harmless, but it is left alone too to keep the
  check simple. A task-list box isn't a node, so `- [ ] <br>b` is left alone;
  `_<br>b_ c` isn't, since its line begins with `_`.
- **The tag is in the text of a shortcut or collapsed reference link**
  (`[a<br>b]`, `[a<br>b][]`). That text is also the label the link is matched
  by, and a line break would change the label.
- **What follows starts with `www.`** (`a<br>www.example.com`). GitHub links
  `www.` only at the start of a line or after whitespace, `*`, `_`, `~` or `(`,
  so a break would turn the text into a link. `http://` and `https://` are
  linked after a `>` already, so they aren't affected.
- **The block after the paragraph begins with a line shaped like a table
  delimiter row** (`- | -`) once a bullet is written as `-`, which the
  formatter may rewrite it to, including a list item's first nested list
  (`- a | b<br>c | d⏎  - | -`, `- a | b<br>c | d⏎  * | -`). A new line just above that row could become a
  table's header. Deciding this by the next block, rather than by the next
  source line, gives the same output when the result is formatted again.
- **The paragraph or setext heading holds an inline `<pre>`, `<listing>`,
  `<xmp>`, `<plaintext>`, `<textarea>`, `<script>`, `<style>` or `<code>` tag**
  anywhere, in any letter case (`x <pre>a<br>b</pre>`), since a source line
  break inside one is shown, breaks the script or style, or is kept by some
  stylesheets for `<code>`.

## Out of scope

- A `<br>` inside an HTML block; the HTML formatter and its `html.*` options
  handle those.
- Converting between `<br>` and `\` or two-space hard breaks. `hardBreakKind`
  still applies only to the latter.
- Known limitations, rare enough to leave:
  - bracketed text whose label differs from a link definition only by the
    whitespace after a `<br>` (`[a<br>b]` with `[a<br> b]: /u`) becomes a link
    once the break is written;
  - CSS can show the break or hide the tag (`white-space: pre`, a `<br>` hidden
    on small screens), and a `<pre>` opened earlier in the file (in an HTML
    block or an earlier paragraph) is still open later; the formatter reads
    neither.

## Examples

`⏎` marks a line break. Every example has `lineBreakAfterBr: true` and otherwise
the default configuration (`textWrap: maintain`) unless noted. An example that
names no mode has the same output in every `textWrap` mode, except that
"Unchanged" means the output today's formatter gives in that mode. A link or
footnote example includes the definition it needs. Examples 10 and 32 were
removed during review; the other numbers are kept so earlier reviews still
match.

| #  | Input                                                                                                                                | Output                                                                                    |
| -- | ------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------- |
| 1  | `a<br>b`                                                                                                                             | `a<br>⏎b`                                                                                 |
| 2  | `a<br>⏎b` with `textWrap: always`                                                                                                    | `a<br>⏎b` (today: `a<br> b`)                                                              |
| 3  | `a <br> b`                                                                                                                           | `a <br>⏎b`                                                                                |
| 4  | `a<BR>b`                                                                                                                             | `a<BR>⏎b`                                                                                 |
| 5  | `a<br/>b`                                                                                                                            | `a<br/>⏎b`                                                                                |
| 6  | `a<br />b`                                                                                                                           | `a<br />⏎b`                                                                               |
| 7  | `a<br class="x">b`                                                                                                                   | `a<br class="x">⏎b`                                                                       |
| 8  | `a<br><br>b`                                                                                                                         | `a<br><br>⏎b`                                                                             |
| 9  | `a<br>`                                                                                                                              | Unchanged.                                                                                |
| 11 | `a</br>b`                                                                                                                            | Unchanged.                                                                                |
| 12 | `a<br>- b`, `a<br># b`, `a<br>> b`, `a<br><div>b</div>`, `a<br>===`                                                                  | Unchanged.                                                                                |
| 13 | `<br>b`, `- <br>b`, `> <br>b`, `- [ ] <br>b`, `a\⏎<br>b`, `a⏎<br>b`, and `<br>b⏎===` with `heading.kind: setext` (tag begins a line) | Unchanged.                                                                                |
| 14 | `<br><br>b`                                                                                                                          | `<br><br>⏎b`                                                                              |
| 15 | `# a<br>b`                                                                                                                           | Unchanged.                                                                                |
| 16 | `a<br>b⏎===` (setext heading)                                                                                                        | `a<br>⏎b⏎=====` with `heading.kind: setext`; `# a<br>b` with the default `atx`, as today. |
| 17 | A table cell holding `a<br>b`                                                                                                        | Unchanged.                                                                                |
| 18 | `Term<br>term⏎: definition` (definition-list term)                                                                                   | Unchanged.                                                                                |
| 19 | `- a<br>b`                                                                                                                           | `- a<br>⏎  b`                                                                             |
| 20 | `> a<br>b`                                                                                                                           | `> a<br>⏎> b`                                                                             |
| 21 | `[^1]: a<br>b`                                                                                                                       | `[^1]: a<br>⏎    b`                                                                       |
| 22 | `[a<br>b](/u)`; `[a<br>b][r]⏎⏎[r]: /u` (inline and full reference link)                                                              | `[a<br>⏎b](/u)`; `[a<br>⏎b][r]⏎⏎[r]: /u`                                                  |
| 23 | `[a<br>b]` and `[a<br>b][]`, each with `⏎⏎[a<br>b]: /u` (shortcut and collapsed reference links)                                     | Unchanged.                                                                                |
| 24 | `a<br>\⏎b` (hard break after tag)                                                                                                    | Unchanged.                                                                                |
| 25 | `a<br>b` with `textWrap: never`                                                                                                      | `a<br>⏎b`                                                                                 |
| 26 | `a. <br>b` with `textWrap: sentence`                                                                                                 | `a. <br>⏎b`                                                                               |
| 27 | `one two<br>three four five six seven eight` with `textWrap: always, lineWidth: 20`                                                  | `one two<br>⏎three four five six⏎seven eight`                                             |
| 28 | `one two<br>three four five six seven eight` with `textWrap: maintainAndWrap, lineWidth: 20`                                         | `one two<br>⏎three four five six⏎seven eight`                                             |
| 29 | `a<br>b` with `lineBreakAfterBr: false` (the default)                                                                                | Unchanged.                                                                                |
| 30 | `[a<br>](/u)b` (tag last inside link text)                                                                                           | Unchanged.                                                                                |
| 31 | `_<br>b_ c` (tag first inside emphasis)                                                                                              | `_<br>⏎b_ c`                                                                              |
| 33 | `a<br>www.example.com`                                                                                                               | Unchanged.                                                                                |
| 34 | `a<br>https://example.com`                                                                                                           | `a<br>⏎https://example.com`                                                               |
| 35 | `a<br>[^1]: x⏎⏎[^1]: y` (footnote reference followed by `:`)                                                                         | Unchanged.                                                                                |
| 36 | `a<br>2. b`, and `a<br>1.⏎b` with `textWrap: never`                                                                                  | Unchanged.                                                                                |
| 37 | `a<br>* *b*⏎===` with `heading.kind: setext`                                                                                         | Unchanged.                                                                                |
| 38 | `a<br>*.b**`                                                                                                                         | Unchanged.                                                                                |
| 39 | `a<br>**b**`, `a<br>[b](/u)`                                                                                                         | Unchanged.                                                                                |
| 40 | `a \| b<br>c \| d⏎- \| -`                                                                                                            | Unchanged.                                                                                |
| 41 | `x <pre>a<br>b</pre>`, `x <PRE>_a<br>b_</PRE>`                                                                                       | Unchanged.                                                                                |
| 42 | `# a<br>b` with `heading.kind: setext`                                                                                               | `a<br>⏎b⏎=====`                                                                           |
| 43 | `- a \| b<br>c \| d⏎  - \| -`                                                                                                        | Unchanged.                                                                                |
| 44 | `> [!NOTE]⏎> <br>b` (tag first on its line after a callout marker)                                                                   | Unchanged.                                                                                |
| 45 | `x <script>s = "a<br>b"</script>`                                                                                                    | Unchanged.                                                                                |
| 46 | `aaaa bbbb cccc dddd <br> eeee` with `textWrap: always` or `maintainAndWrap`, `lineWidth: 20`                                        | `aaaa bbbb cccc⏎dddd <br>⏎eeee`                                                           |
| 47 | `- a \| b<br>c \| d⏎  * \| -`                                                                                                        | Unchanged.                                                                                |
| 48 | `a \| b<br>c \| d⏎* \| -`                                                                                                            | Unchanged.                                                                                |
| 49 | `x <code>a<br>b</code>`                                                                                                              | Unchanged.                                                                                |

Formatting any output again leaves it unchanged.

## Implementation

- Configuration: add `line_break_after_br: bool` to `Configuration` in
  `src/configuration/types.rs`, read `lineBreakAfterBr` (default `false`) in
  `resolve_config.rs`, add a method to `builder.rs` and call it in its
  `check_all_values_set` test (raising the option count by one), and document
  the option in `deployment/schema.json`.
- `src/generation/generate.rs`:
  - `is_break_tag(node)`: `node` is a `Node::Html` whose `is_block` field is
    `false` and whose text is `<`, then `br` in any ASCII case, then
    whitespace, `/` or `>`.
  - `breaks_line_after_tag(last_node, node, context)`: the option is on,
    `last_node` is a break tag, `node` is a `Text` node beginning with an
    alphabetic character and not with `www.`, the tag doesn't begin a line (a
    line start of its paragraph or setext heading, or only whitespace and `>`
    between it and the start of its line in the file), and the context isn't in
    link-label text or a block whose breaks are off.
  - `gen_nodes_within_breaks`: where the tag `node` will get a break after it
    (`breaks_line_after_tag` with the next node), write the space before it as
    a plain space rather than one the printer may wrap at. Build today's
    separator before `node` as before,
    and where `breaks_line_after_tag` holds, push
    `if_true_or(…, condition_resolvers::is_forcing_no_newlines(), separator, NewLine)`,
    as `gen_hard_break` does, so that an ATX heading, table cell or
    definition-list term is written as today.
  - Before generating a paragraph or heading, turn its breaks off when it holds
    a `<pre>`-like tag (`holds_preformatted_tag`, which reads the nested nodes
    too) or when the next node's first line passes
    `is_table_delimiter_shape` (`src/parser/block.rs`, made `pub`), with a
    `*` or `+` bullet read as `-`. `gen_item`
    does the same for an item's content when its first nested list begins
    that way, since an item keeps those in `sub_lists`.
  - `gen_heading`: only when the option is on, record a setext heading's line
    starts (its first child and any child after a hard break) for the
    line-start check. `gen_link_text` marks link-label text when `is_the_name`
    is true; `is_preserving_decorations()` can't be reused, since the emphasis
    fallback in `gen_checked_decorations` sets it for a whole paragraph.
- `src/generation/gen_types.rs`: the context gets the link-label flag, the
  setext line starts (in their own field, since `gen_hard_break`, `gen_text`
  and `text_from` read `is_line_start`) and the breaks-off flag.
- A trial implementation along these lines adds about 200 lines to
  `src/generation` (180 net), comments included, and gives every example above its
  expected output in each
  `textWrap` mode.
- Tests: add these spec files, all with `lineBreakAfterBr: true` except the
  last:
  - `tests/specs/html/Html_BrNewLine.txt` (`textWrap: maintain`)
  - `tests/specs/html/Html_BrNewLine_TextWrap_Always.txt` (`lineWidth: 20, textWrap: always`)
  - `tests/specs/html/Html_BrNewLine_TextWrap_Never.txt` (`textWrap: never`)
  - `tests/specs/html/Html_BrNewLine_TextWrap_Sentence.txt` (`textWrap: sentence`)
  - `tests/specs/html/Html_BrNewLine_TextWrap_MaintainAndWrap.txt` (`lineWidth: 20, textWrap: maintainAndWrap`)
  - `tests/specs/html/Html_BrNewLine_Setext.txt` (`heading.kind: setext`), for
    examples 13, 16, 37 and 42
  - `tests/specs/html/Html_BrNewLine_Off.txt` (`lineBreakAfterBr: false`), for
    example 29

  An example that names a mode goes in that mode's file; every other example
  goes in all five `textWrap` files. Existing specs keep their expected output,
  since the option is off by default.
- Rendering check: add `pulldown-cmark` (0.13) as a dev-dependency and a test
  that formats each example, and each existing spec case whose input holds a
  `<br>`, with the option off and on, renders both outputs with every
  pulldown-cmark option except smart punctuation, and asserts the HTML is the
  same. Before comparing, whitespace after a `<br>` is removed and other runs
  of whitespace are collapsed to one space, except inside `<pre>`-like
  elements, where whitespace is shown. pulldown-cmark has no GFM `www.`
  autolinks and doesn't let a table interrupt a paragraph, so the `www.` and
  table exceptions are checked by the spec files only.
