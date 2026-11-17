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
follows a break tag in a paragraph, the formatter writes a line break between
the tag and what follows:

1. The whitespace between the tag and what follows is replaced by one line
   break. Whitespace before the tag is handled as today, except as rule 4
   says.
2. The tag is written exactly as it appeared (letter case, attributes, slash).
3. The new line starts at the paragraph's indentation (list item, block quote,
   footnote, definition), just as after a hard break.
4. If wrapping is active (`always`, `maintainAndWrap`), the text after the tag
   is wrapped as usual, but the line break after the tag is never removed. The
   space before such a tag is written as a plain space, not a place to wrap, so
   wrapping can't move the tag to the start of a line. A tag that already
   begins a line keeps the word after it beside it the same way, so wrapping
   can't leave the tag alone where a block begins.
5. The rule also applies in the text of an inline or full reference link
   (`[a<br>b](/u)`, `[a<br>b][r]`), which `always`, `never` and `sentence`
   otherwise keep on one line, just as a `\` hard break there already does.

The formatter writes the line break only when what follows is text that begins
with an alphabetic character (Unicode `Alphabetic`, in any script). Apart from a
table's header row and a definition-list term, which have exceptions of their
own below, nothing that starts a block at the start of a line begins with one:
list markers, `#`, `>`, code fences, thematic breaks, footnote definitions
(`[^1]:`), HTML tags and comments, and setext underlines all begin with
something else. Neither do another break tag (so `<br><br>` stays together) or a
hard break. Other text, such as `**bold**`, `` `code` `` or `[a link](/u)`,
stays on the tag's line, which keeps the rule simple.

Even when what follows begins with a letter, the formatter writes no line break
after the tag when:

- **The tag is in a heading (ATX or setext), a table cell or a definition-list
  term.** Most of these are written on one line, and a heading's text is what
  its anchor is made from: GitHub turns spaces into hyphens and drops line
  breaks, so `a<br> b` would change its anchor from `#a-b` to `#ab`.
- **The tag begins a line:** it is the first node of its paragraph, the first
  node after a hard break, or has a line ending between it and the node before
  it with the same parent (as after a callout's `[!NOTE]` line). A line holding
  only the tag would start an
  [HTML block](https://spec.commonmark.org/0.31.2/#html-blocks) (type 7) where a
  block begins, which would turn what follows into raw HTML. Where a block can't
  begin, a break would be harmless, but it is left alone too to keep the check
  simple. A task-list box isn't part of the paragraph, so `- [ ] <br>b` is left
  alone; `_<br>b_ c` isn't, since its line begins with `_`.
- **The tag is in the text of a shortcut or collapsed reference link**
  (`[a<br>b]`, `[a<br>b][]`). That text is also the label the link is matched
  by, and a line break would change the label.
- **The paragraph begins with `[` and holds `]:`**, as a
  [link reference definition](https://spec.commonmark.org/0.31.2/#link-reference-definitions)
  does (`[x]: <br>b`, `[x]: a<br>b c`). Such a paragraph isn't a definition
  while text follows its destination on the same line, and ending that line
  could complete one.
- **What follows starts with `www.`** (`a<br>www.example.com`). GitHub links
  `www.` only at the start of a line or after whitespace, `*`, `_`, `~` or `(`,
  so a break would turn the text into a link. GitHub links `http://` and
  `https://` after a `>` already, so they aren't affected, although the GFM
  spec's prose gives every extended autolink the same rule as `www.`.
- **A new line could become a table's header:** the paragraph holds a
  delimiter cell without a `|` (`:-:`, `-:`, `:--`), which GitHub reads as a
  delimiter row on a line of its own (`a | b<br>c⏎:-:`); or the block after it
  is a bullet list whose first item's text begins with `|`, as every delimiter
  row written as a list item does (`- | -`, `* | -`, also with the `| -` on the
  next line or inside a block quote), or whose first item begins with a table or
  a list, which the formatter may write that way. This includes
  a list item's nested list, in a task-list item too
  (`- [ ] a | b<br>c | d⏎  - | -`). These tests read the parsed list and the
  paragraph's words, with a `\` hard break left off a word, so they don't change
  when the formatter rewrites a marker or a hard break or joins or wraps lines,
  and the output is the same when the result is formatted again.
- **The paragraph holds an inline `<pre>`, `<listing>`, `<xmp>`, `<plaintext>`,
  `<textarea>`, `<script>`, `<style>`, `<code>` or `<tt>` tag** anywhere, in
  any letter case (`x <pre>a<br>b</pre>`), since a source line break inside one
  is shown, breaks the script or style, or is kept by some stylesheets for
  `<code>` and `<tt>`.

## Out of scope

- A `<br>` inside an HTML block; the HTML formatter and its `html.*` options
  handle those.
- Converting between `<br>` and `\` or two-space hard breaks. `hardBreakKind`
  still applies only to the latter.
- Wikilinks (`[[a<br>b]]`), which aren't CommonMark or GFM. The formatter reads
  them as text, so the break is written there (`[[a<br>⏎b]]`), as text wrapping
  already splits them today.
- Known limitations, rare enough to leave:
  - bracketed text whose label differs from a link definition only by the
    whitespace after a `<br>` (`[a<br>b]` with `[a<br> b]: /u`) becomes a link
    once the break is written;
  - CSS can show the break or hide the tag (`white-space: pre`, a `<br>` hidden
    on small screens), and a `<pre>` opened earlier in the file (in an HTML
    block or an earlier paragraph) is still open later; the formatter reads
    neither;
  - GitHub has no definition lists, so in `Term⏎: > a<br>b` the new `  > b`
    line starts a block quote there;
  - the formatter's parser reads a `>` after a task-list box as a block quote,
    and `[r]: /u` there as a link reference definition, which CommonMark
    doesn't (`- [ ] > a<br>b`, `- [ ] [r]: /u a<br>b`); the first already breaks
    `- [ ] > a⏎  b` with the option off;
  - wrapping can leave an inline tag other than `<br>` alone at the start of a
    block, where it starts an HTML block; today's formatter already does this
    (`- </span> aaaaaaaaaaaaaaaa` at width 16), and rule 4's plain space can
    move a wrap onto it (`- </span> aaaa <br>b` at width 16);
  - a renderer without footnotes reads a footnote definition as a link
    reference definition, which the break can complete (`[^1]: a<br>b`), and
    its later, indented paragraphs as code, where a break shows; the same goes
    for a definition's later paragraphs with `list.indentKind: pythonMarkdown`,
    which indents them by 4.

## Examples

`⏎` marks a line break. Every example has `lineBreakAfterBr: true` and otherwise
the default configuration (`textWrap: maintain`) unless noted. An example that
names no mode has the same output in every `textWrap` mode, except that
"Unchanged" means the output today's formatter gives in that mode. A link or
footnote example includes the definition it needs. Examples 10 and 32 were
removed during review; the other numbers are kept so earlier reviews still
match.

| #  | Input                                                                                                   | Output                                                                                                                                       |
| -- | ------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| 1  | `a<br>b`                                                                                                | `a<br>⏎b`                                                                                                                                    |
| 2  | `a<br>⏎b` with `textWrap: always`                                                                       | `a<br>⏎b` (today: `a<br> b`)                                                                                                                 |
| 3  | `a <br> b`                                                                                              | `a <br>⏎b`                                                                                                                                   |
| 4  | `a<BR>b`                                                                                                | `a<BR>⏎b`                                                                                                                                    |
| 5  | `a<br/>b`                                                                                               | `a<br/>⏎b`                                                                                                                                   |
| 6  | `a<br />b`                                                                                              | `a<br />⏎b`                                                                                                                                  |
| 7  | `a<br class="x">b`                                                                                      | `a<br class="x">⏎b`                                                                                                                          |
| 8  | `a<br><br>b`                                                                                            | `a<br><br>⏎b`                                                                                                                                |
| 9  | `a<br>`                                                                                                 | Unchanged.                                                                                                                                   |
| 11 | `a</br>b`                                                                                               | Unchanged.                                                                                                                                   |
| 12 | `a<br>- b`, `a<br># b`, `a<br>> b`, `a<br><div>b</div>`, `a<br>===`                                     | Unchanged.                                                                                                                                   |
| 13 | `<br>b`, `- <br>b`, `> <br>b`, `- [ ] <br>b`, `a\⏎<br>b`, `a⏎<br>b` (tag begins a line)                 | Unchanged.                                                                                                                                   |
| 14 | `<br><br>b`                                                                                             | `<br><br>⏎b`                                                                                                                                 |
| 15 | `# a<br>b`                                                                                              | Unchanged.                                                                                                                                   |
| 16 | `a<br>b⏎===` (setext heading)                                                                           | Unchanged with `heading.kind: setext`; `# a<br>b` with the default `atx`, as today.                                                          |
| 17 | A table cell holding `a<br>b`                                                                           | Unchanged.                                                                                                                                   |
| 18 | `Term<br>term⏎: definition` (definition-list term)                                                      | Unchanged.                                                                                                                                   |
| 19 | `- a<br>b`                                                                                              | `- a<br>⏎  b`                                                                                                                                |
| 20 | `> a<br>b`                                                                                              | `> a<br>⏎> b`                                                                                                                                |
| 21 | `[^1]: a<br>b`                                                                                          | `[^1]: a<br>⏎    b`                                                                                                                          |
| 22 | `[a<br>b](/u)`; `[a<br>b][r]⏎⏎[r]: /u` (inline and full reference link)                                 | `[a<br>⏎b](/u)`; `[a<br>⏎b][r]⏎⏎[r]: /u`                                                                                                     |
| 23 | `[a<br>b]` and `[a<br>b][]`, each with `⏎⏎[a<br>b]: /u` (shortcut and collapsed reference links)        | Unchanged.                                                                                                                                   |
| 24 | `a<br>\⏎b` (hard break after tag)                                                                       | Unchanged.                                                                                                                                   |
| 25 | `a<br>b` with `textWrap: never`                                                                         | `a<br>⏎b`                                                                                                                                    |
| 26 | `a. <br>b` with `textWrap: sentence`                                                                    | `a. <br>⏎b`                                                                                                                                  |
| 27 | `one two<br>three four five six seven eight` with `textWrap: always, lineWidth: 20`                     | `one two<br>⏎three four five six⏎seven eight`                                                                                                |
| 28 | `one two<br>three four five six seven eight` with `textWrap: maintainAndWrap, lineWidth: 20`            | `one two<br>⏎three four five six⏎seven eight`                                                                                                |
| 29 | `a<br>b` with `lineBreakAfterBr: false` (the default)                                                   | Unchanged.                                                                                                                                   |
| 30 | `[a<br>](/u)b` (tag last inside link text)                                                              | Unchanged.                                                                                                                                   |
| 31 | `_<br>b_ c` (tag first inside emphasis)                                                                 | `_<br>⏎b_ c`                                                                                                                                 |
| 33 | `a<br>www.example.com`                                                                                  | Unchanged.                                                                                                                                   |
| 34 | `a<br>https://example.com`                                                                              | `a<br>⏎https://example.com`                                                                                                                  |
| 35 | `a<br>[^1]: x⏎⏎[^1]: y` (footnote reference followed by `:`)                                            | Unchanged.                                                                                                                                   |
| 36 | `a<br>2. b`, and `a<br>1.⏎b` with `textWrap: never`                                                     | Unchanged.                                                                                                                                   |
| 37 | `a<br>* *b*⏎===` with `heading.kind: setext`                                                            | Unchanged.                                                                                                                                   |
| 38 | `a<br>*.b**`                                                                                            | Unchanged.                                                                                                                                   |
| 39 | `a<br>**b**`, `a<br>[b](/u)`                                                                            | Unchanged.                                                                                                                                   |
| 40 | `a \| b<br>c \| d⏎- \| -`                                                                               | Unchanged.                                                                                                                                   |
| 41 | `x <pre>a<br>b</pre>`, `x <PRE>_a<br>b_</PRE>`                                                          | Unchanged.                                                                                                                                   |
| 42 | `# a<br>b`, `a<br> b⏎===` and `<br>b⏎===` with `heading.kind: setext`                                   | Unchanged (`a<br>b⏎======`, `a<br> b⏎=======` and `<br>b⏎=====`, as today).                                                                  |
| 43 | `- a \| b<br>c \| d⏎  - \| -`                                                                           | Unchanged.                                                                                                                                   |
| 44 | `> [!NOTE]⏎> <br>b` (tag first on its line after a callout marker)                                      | Unchanged.                                                                                                                                   |
| 45 | `x <script>s = "a<br>b"</script>`                                                                       | Unchanged.                                                                                                                                   |
| 46 | `aaaa bbbb cccc dddd <br> eeee` with `textWrap: always` or `maintainAndWrap`, `lineWidth: 20`           | `aaaa bbbb cccc⏎dddd <br>⏎eeee`                                                                                                              |
| 47 | `- a \| b<br>c \| d⏎  * \| -`                                                                           | Unchanged.                                                                                                                                   |
| 48 | `a \| b<br>c \| d⏎* \| -`                                                                               | Unchanged.                                                                                                                                   |
| 49 | `x <code>a<br>b</code>`                                                                                 | Unchanged.                                                                                                                                   |
| 50 | `[x]: <br>b`, `[x]: a<br>b c`, `[x]:⏎a<br>b c` (paragraph that begins like a link reference definition) | Unchanged.                                                                                                                                   |
| 51 | `a \| b<br>c \| d⏎- \| -⏎  x` with `textWrap: never`                                                    | Unchanged.                                                                                                                                   |
| 52 | `aaa<br>bbb⏎- \| - cccc dddd <pre>x</pre>` with `textWrap: always, lineWidth: 20`                       | Unchanged.                                                                                                                                   |
| 53 | `x <tt>a<br>b</tt>`                                                                                     | Unchanged.                                                                                                                                   |
| 54 | `- [ ] a \| b<br>c \| d⏎  - \| -⏎⏎  more`                                                               | Unchanged.                                                                                                                                   |
| 55 | `<br> https://example.com/a/long/path <br> See above.` with `textWrap: always, lineWidth: 20`           | `<br> https://example.com/a/long/path <br>⏎See above.` (today: `<br>⏎https://…⏎<br> See above.`, where the lone `<br>` starts an HTML block) |
| 56 | `- a<br>b⏎  - c \| d⏎    \| - \| - \|`                                                                  | Unchanged.                                                                                                                                   |
| 57 | `a⏎    > <br>b` (a `>` that is text, on a continuation line)                                            | `a > <br>⏎b`                                                                                                                                 |
| 58 | `a \| b<br>c⏎:-:`                                                                                       | Unchanged.                                                                                                                                   |
| 59 | `> a<br>b⏎>⏎> -⏎>   \| -` (the list check inside a block quote)                                         | Unchanged.                                                                                                                                   |
| 60 | `a<br>b -:␣␣⏎c` (`␣` is a space; a two-space hard break after a delimiter cell)                         | Unchanged.                                                                                                                                   |

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
  - `breaks_line_after_tag(next, context)`: the option is on, the node after
    the tag is a `Text` node beginning with an alphabetic character and not
    with `www.`, and the context isn't in link-label text or a block whose
    breaks are off.
  - `gen_nodes_within_breaks`: when the node is a break tag, decide once
    whether it begins a line (a line start of its paragraph, or a line ending
    between it and the node before it) and, if it doesn't, whether it gets a
    line break after it, with `breaks_line_after_tag` and the next node. If it
    gets one, write the space before it as a plain space rather than one the
    printer may wrap at; build today's separator after it as before, and push
    `if_true_or(…, condition_resolvers::is_forcing_no_newlines(), separator, NewLine)`,
    as `gen_hard_break` does, so that a table cell or definition-list term is
    written as today. If it begins a line, write the space after it as a plain
    space instead.
  - `keeps_br_lines`: before generating a block, turn its breaks off when it is
    a heading; or a paragraph that begins with `[` and holds `]:`, holds a
    delimiter cell without a `|` (`is_bare_delimiter_cell`, on each word with a
    trailing `\` left off), holds a `<pre>`-like tag (`holds_preformatted_tag`,
    which reads the nested nodes too), or whose next node is a bullet list that
    could begin with a delimiter row (`may_begin_with_delimiter_row`: its first
    item begins with a paragraph whose text starts with `|`, or with a table or
    a list). `gen_item` does the same for an item's content when its first
    nested list could, since an item keeps those in `sub_lists`, and
    `gen_task_list_marker_children` for the children it writes before a list.
  - `gen_link_text` marks link-label text when `is_the_name` is true;
    `is_preserving_decorations()` can't be reused, since the emphasis fallback
    in `gen_checked_decorations` sets it for a whole paragraph.
- `src/generation/gen_types.rs`: the context gets the link-label flag and the
  breaks-off flag.
- A trial implementation along these lines adds about 240 lines to
  `src/generation` (220 net), comments and rustfmt's layout included, and gives
  every example above its expected output in each `textWrap` mode.
- Tests: add these spec files, all with `lineBreakAfterBr: true` except the
  last:
  - `tests/specs/html/Html_BrNewLine.txt` (`textWrap: maintain`)
  - `tests/specs/html/Html_BrNewLine_TextWrap_Always.txt` (`lineWidth: 20, textWrap: always`)
  - `tests/specs/html/Html_BrNewLine_TextWrap_Never.txt` (`textWrap: never`)
  - `tests/specs/html/Html_BrNewLine_TextWrap_Sentence.txt` (`textWrap: sentence`)
  - `tests/specs/html/Html_BrNewLine_TextWrap_MaintainAndWrap.txt` (`lineWidth: 20, textWrap: maintainAndWrap`)
  - `tests/specs/html/Html_BrNewLine_Setext.txt` (`heading.kind: setext`), for
    examples 16, 37 and 42, which keep their headings unchanged
  - `tests/specs/html/Html_BrNewLine_Off.txt` (`lineBreakAfterBr: false`), for
    example 29

  An example that names a mode goes in that mode's file; every other example
  goes in all five `textWrap` files. Existing specs keep their expected output,
  since the option is off by default.
- Rendering check: add `pulldown-cmark` (0.13) as a dev-dependency and a test
  that formats each example, and each existing spec case whose input holds a
  `<br>`, with the option on, renders the output with every pulldown-cmark
  option except smart punctuation, wikilinks and the old footnote syntax, and
  asserts the HTML is the same as the input's, or as the output's with the
  option off where today's formatter already changes the page. Before comparing,
  whitespace after a `<br>` is removed and other runs of whitespace are
  collapsed to one space, except inside `<pre>`-like elements, where whitespace
  is shown. pulldown-cmark has no GFM `www.` autolinks, doesn't let a table
  interrupt a paragraph and makes no heading ids, so the `www.`, table and
  heading exceptions are checked by the spec files only.
