# Superscript

## Goal

Some Markdown renderers read `^text^` as superscript. The one that matters for
this project is pulldown-cmark with
[`ENABLE_SUPERSCRIPT`](https://docs.rs/pulldown-cmark/0.13.4/pulldown_cmark/struct.Options.html),
which the CodeChat Editor uses. Today this plugin's parser reads `^` as plain
text.

This change makes the parser read a superscript as its own node, so that:

1. the formatter never creates, removes or changes a superscript when it
   rewraps text or rewrites emphasis, and
2. the syntax tree (and the parser spec tests) show where a superscript is.

The formatter writes a superscript back with the delimiters it was written
with. There is only one delimiter character (`^`), so there is nothing to
normalize.

Out of scope:

- Subscript (`~text~`). Another team owns it
  ([PR #1](https://github.com/bjones1/dprint-plugin-markdown/pull/1)). Note
  that the CodeChat Editor turns subscript on, so a single `~` is subscript
  there, while this fork reads it as strikethrough and writes it back as
  `~~`, which turns a subscript into strikethrough.
- `<sup>` html, which is already kept as inline html.

## Syntax

The target is the released pulldown-cmark 0.13 with the options the CodeChat
Editor uses: `Options::all()` without smart punctuation
([`markdown_to_html`](https://github.com/bjones1/CodeChat_Editor/blob/main/server/src/processing.rs)).
Among other things, that turns on superscript, subscript, old-style footnotes
and wikilinks. 0.13.4 and 0.13.6 give the same result for every example below.

pulldown-cmark's rule is short: **`^` is parsed like `_`, and where `_` would
make emphasis or strong emphasis, `^` makes a superscript** (with one
difference, see Known differences below). In detail, using the
[CommonMark emphasis rules](https://spec.commonmark.org/0.31.2/#emphasis-and-strong-emphasis):

1. **Delimiter run.** A run of `^` that is not escaped (`\^`) and is not inside
   a code span, inline math, an autolink, raw html, or a link destination or
   title. pulldown-cmark also reads superscripts in image alt text, but this
   parser never parses alt text, so it never makes a node there. Formatting is
   still safe: the formatter only changes whitespace in alt text (for example
   `![x ^a␣␣⏎b^](/u)` becomes `![x ^a b^](/u)`), and swapping one kind of
   whitespace for another does not change what opens or closes (formatting
   rule 3).
2. **Opener and closer.** Same as for `_`. A run can open if the character
   after it exists and is not whitespace, and the character before it is
   missing, whitespace or punctuation. A run can close if the character before
   it exists and is not whitespace, and the character after it is missing,
   whitespace or punctuation. So `^` cannot start or end a superscript inside
   a word.
3. **Pairing.** Same procedure as for `_` (CommonMark's "process emphasis"). A
   `^` run never pairs with `*`, `_` or `~`.
4. **Length.** Where `_` would make strong emphasis (two characters from each
   side), `^` makes one superscript: `^^a^^` is one superscript, and `^^^a^^^`
   is a superscript inside a superscript, like `___a___`.
5. **Content.** Any inline: emphasis, code spans, links, math, spaces, soft
   line breaks and hard line breaks. A blank line ends the paragraph, so it
   ends a superscript too.
6. **Footnotes.** With old-style footnotes on, `[^name]` is a footnote
   reference, whether or not `name` is defined, when a `]` closes it, `name`
   is not blank and holds no unescaped `[`, it does not become a link or
   image, and the `]` is not followed right away by a `[label]` with a
   non-blank label, unless that `[label]` is itself a footnote reference
   (`[^b]` with a non-blank `b`). So `[^a^][]` and `[^a^][^b]` start with a
   footnote, while `[^a^][x]`, `[^a^][^]` and `[^a^][^ ]` do not. A `!`
   before it does not change this. `name` is scanned as raw text up to the
   first `]`, and that `]` must be the one that closes the bracket. So in
   `[^a `]` b]` the first `]` is inside a code span, the scan stops there, and
   there is no footnote (the same holds for a `]` in math or an autolink).
   `name` is text: no `^` in it opens or closes a superscript. Otherwise the
   bracket is read like any other, so `[^a^ b`, `[^a [b] c^]`, `[^a^](/u)`
   and `[^a^][x]` all hold a superscript. A footnote reference also stops
   every link or image bracket before it from becoming a link or image, the
   same as a link stops earlier links: in `[b [^c] d](/u)` and
   `![b [^c] d](/u)` there is no link and no image.

The results below come from running pulldown-cmark 0.13.6 with those options
(the middle column is the html it writes; `⏎` is a line break):

| Input                 | Result                                           | Why                         |
| --------------------- | ------------------------------------------------ | --------------------------- |
| `a ^b c^ d`           | `a <sup>b c</sup> d`                             | spaces are allowed          |
| `(^a^)`               | `(<sup>a</sup>)`                                 | punctuation around is fine  |
| `^+^ not`             | `<sup>+</sup> not`                               |                             |
| `^a⏎b^`               | `<sup>a⏎b</sup>`                                 | soft line break inside      |
| `x^2^`                | `x^2^`                                           | inside a word, like `x_2_`  |
| `1^st^ place`         | `1^st^ place`                                    | inside a word               |
| `^a^b^c^`             | `<sup>a^b^c</sup>`                               | inner `^` are inside a word |
| `x^ a^`               | `x^ a^`                                          | opener before a space       |
| `^^a^^`               | `<sup>a</sup>`                                   | rule 4                      |
| `^^^a^^^`             | `<sup><sup>a</sup></sup>`                        | rule 4                      |
| `^bar^^`              | `<sup>bar</sup>^`                                | one `^` left over           |
| `\^a^`                | `^a^`                                            | escaped                     |
| `` `x^2^` ``          | `<code>x^2^</code>`                              | code span first             |
| `$x^2^$`              | math `x^2^`                                      | math first                  |
| `*^a^*`               | `<em><sup>a</sup></em>`                          | nesting                     |
| `^*a*^`               | `<sup><em>a</em></sup>`                          | nesting                     |
| `[^1] and x^2^`       | footnote `1`, then `and x^2^`                    | rule 6                      |
| `[^a^]`               | footnote `a^`                                    | rule 6                      |
| `^x [^a b^] c`        | `^x `, footnote `a b^`, ` c`                     | rule 6                      |
| `^x [^a^ b] c^`       | `<sup>x `, footnote `a^ b`, ` c</sup>`           | rule 6                      |
| `[^a^ b`              | `[<sup>a</sup> b`                                | no `]`, so no footnote      |
| `[^a^](/u)`           | link to `/u` holding `<sup>a</sup>`              | a link, so no footnote      |
| `[^a [b] c^]`         | `[<sup>a [b] c</sup>]`                           | `[` in the name             |
| `[^a^][x]`            | `[<sup>a</sup>][x]`                              | a `[label]` follows         |
| `[^a^][]`             | footnote `a^`, then `[]`                         | the label is blank          |
| `^x ![^a b^] c`       | `^x !`, footnote `a b^`, ` c`                    | rule 6                      |
| ``^x [^a `]` b^] c^`` | `<sup>x [<sup>a <code>]</code> b</sup>] c</sup>` | `]` in a code span          |
| `^a [b^ [^c] d](/u)`  | `<sup>a [b</sup> `, footnote `c`, ` d](/u)`      | no link around a footnote   |
| `^中^⏎字`             | `<sup>中</sup>⏎字`                               | the break ends the word     |
| `^中^字`              | `^中^字`                                         | inside a word               |

Note that `x^2^`, the most common use, is **not** a superscript in
pulldown-cmark 0.13 (it is in Pandoc). The unreleased pulldown-cmark master
branch changes this: its
[`super_sub.txt`](https://github.com/pulldown-cmark/pulldown-cmark/blob/master/pulldown-cmark/specs/super_sub.txt)
allows `x^2^a` and `H^+^`, stops runs of different lengths from pairing
(`^bar^^`), and gives `^` and `~` separate lower bounds. It also stops a `^`
followed by punctuation from opening unless a letter, digit, `~` or `^` comes
right before it, anywhere in a line, so `a (^+^) b` and `^*a*^` have no
superscript there. That would change rules 2 and 4, several rows of the table
above (`^+^ not`, `x^2^`, `1^st^ place`, `^a^b^c^`, `^bar^^`, `^*a*^`,
`[^1] and x^2^`, `^a [b^ [^c] d](/u)`, `^中^字`), and the Known differences
below.

### Known differences

In pulldown-cmark 0.13 the "process emphasis" step keeps one lower bound for
`^` and `~` together, not one per run length as CommonMark does for `_`. So a
`^` or `~` run that can close but finds no opener stops every later `^` or `~`
run from pairing with an opener before it:

| Input        | pulldown-cmark 0.13 | Same input with `_` |
| ------------ | ------------------- | ------------------- |
| `^a .^^. b^` | no superscript      | `<em>a .__. b</em>` |
| `^x~~ y^`    | no superscript      | `<em>x~~ y</em>`    |
| `~~a b^ c~~` | no strikethrough    | (not affected)      |

This parser keeps a lower bound per character, run length mod 3, and whether
the closer can also open (`openers_bottom` in `src/parser/inline.rs`), so it
reads a superscript in the first two rows and a strikethrough in the third.

Second, pulldown-cmark reads a wikilink target and a heading attribute block
as text: `^a [[b^]] c^` is `<sup>a <a href="b^">b^</a> c</sup>`, and
`# ^a {#b^}` is `<h1 id="b^">^a</h1>`. This parser reads neither, so it pairs
`b^` with `^a` in both.

Third, this parser makes a footnote node for a defined `name` even when `(…)`
or `[…]` follows, where pulldown-cmark makes a link or reads the brackets as
text: with `[^a]: n` defined, `[^a](/u)` is `<a href="/u">^a</a>`, and
`^p [^a][x] q^` is `^p [<sup>a][x] q</sup>`. This predates superscript
support.

Fourth, rule 6 drops only the `^` delimiters inside a footnote name; `*` and
`_` there still pair, as they do today. pulldown-cmark pairs none of them, so
in `*x ^y [^a* b] z^` it reads a superscript and this parser reads emphasis
from `*x` to `a*` and no superscript. Dropping `*` and `_` too would change
current output, so it is left out.

Fifth, this parser's inline math differs from pulldown-cmark's: it has no
rule for braces and does not let a `$` before a digit close. So in
`x ^y $a^ {b$` pulldown-cmark reads no math and can pair `^y` with `a^`,
while this parser reads `$a^ {b$` as math.

We accept these differences: these inputs are rare, and a superscript is
written back as it was. The only rewrite that can change what a `^` run
next to such text does is a hard break rewrite, and formatting rule 3 keeps
a hard break after a `^` run as it was written. So at worst the formatter is
more careful than it needs to be there.

## Data structures

No new node type. A superscript is a `TextDecoration` with a new kind:

```rust
pub enum TextDecorationKind {
  Emphasis,
  Strong,
  Strikethrough,
  Superscript,
}
```

Its `span` covers its delimiters, and `children` are the nodes between them.
The debug json writes it as `"decoration": "Superscript"`, like the other kinds.

The number of `^` characters a superscript was written with (1 or 2) is not
stored. It is the distance from the start of its `span` to the start of its
first child, since the content starts right after the delimiter. The content is
never empty, because an opener cannot be followed by whitespace or the end of
the text.

## Parsing

All changes are in `src/parser/inline.rs`. Because `^` follows the `_` rules,
most of the code is reused as is:

- `parse()` sends `^` to `parse_emphasis_delimiter`, the same as `*`, `_` and
  `~`.
- `flanking()` already treats every character other than `*` and `~~` like
  `_`, so it needs no change.
- `parse_close_bracket()`: where a link or image bracket turns out not to be
  a link and is a footnote reference by rule 6, it drops the `^` delimiters
  after its `[`, so none of them pairs. It decides this itself rather than
  through `match_footnote_reference()`, which differs in three ways: it
  stops `name` at the first `]` even inside a code span (`match_label` in
  `src/parser/links.rs`), it never matches an image because it looks for
  `^` right after the opener's start, which is the `!` (`links.rs:90`), and
  it takes `name` with `source_slice`, which fails when `name` is wrapped
  inside a blockquote or list (`links.rs:94`). So the check uses the `[`
  (one past the `!` for an image), requires `name` to end at this closing
  bracket, reads `name` from the label text, and looks for a `[label]` after
  the `]`. Doing the same for `*` and `_` would also match pulldown-cmark,
  but it changes current output, so it is left out.
- When a bracket is read as a footnote reference by rule 6, defined or not
  (so in both `parse_open_bracket()` and `parse_close_bracket()`), every
  earlier link and image bracket becomes inactive, as link brackets do after
  a link (`parse_close_bracket()`, "links can't be nested"). pulldown-cmark
  clears its whole bracket stack there, images included. Without this,
  `^a [b^ [^c] d](/u)` is a link here and `![b [^c] d](/u)` an image, with no
  superscript, while pulldown-cmark reads plain text and a superscript.
- `find_opener()` needs no change.
- `wrap_emphasis()` uses one or two characters from each side, as it does for
  `_`, and makes a `Superscript` decoration either way.

## Formatting

All changes are in `src/generation/generate.rs`:

- `gen_text_decoration()` writes `^` or `^^`. Its last match arm is a
  catch-all that writes `~~`, so the new case must come before it.
- `decoration_delimiter()` (`resolve`) returns the written delimiter for a
  superscript, measured as described in Data structures. So does
  `written_delimiter()`, which `decoration_delimiter()` returns instead while
  decorations are preserved. Reading the delimiter off the file text is wrong
  for `^^^a^^^`.
- `unescaped_delimiter_runs()`, `run_can_open()` and `run_can_close()` read
  `^` runs too, with the `_` rules, so they count in `pairable` (rule 4).
- `decoration_keeps_its_delimiters()` returns false for a superscript (rule 3
  below).
- In a heading, a line break between a superscript and Chinese or Japanese
  text must come out as a space (rule 3 below). Whether newlines are off is
  only known at print time (`with_no_new_lines`, generate.rs:773), so the
  break cannot be decided while generating. Where either node is a
  superscript, the `keeps_break_between()` branch (generate.rs:483-487)
  pushes `new_line_or_space_if_newlines_disabled()` instead of a bare
  `NewLine`, and the path that joins such text in a heading with the default
  config (today `_中_⏎字⏎===` becomes `# _中_字`) does the same.
- A hard break that comes right after a `^` run is written as it was, with
  its backslash or its spaces, whatever `hardBreakKind` says (rule 3 below).
- `tests/commonmark_spec_check.rs` renders decorations to html; it needs a
  `sup` case.

Rules:

1. A superscript is written with the same number of `^` it was written with
   (see Data structures), then its content, then the same delimiter again.
   The content is formatted like the content of emphasis, so it may be
   wrapped at its spaces.
2. Wrapping cannot break a superscript. A line break only replaces a space or
   goes between two Chinese or Japanese characters (`wrapUnspacedScripts`).
   An opener is never followed by a space, a closer is never preceded by one,
   and `^` is not such a character, so the `^` characters stay attached to a
   word.
3. Whether a `^` run opens or closes depends only on whether its neighbors
   are whitespace, punctuation or something else. Wrapping swaps a space for a
   line break or back (both whitespace), and changing `*` to `_` or back swaps
   one punctuation character for another, so neither changes it. Three cases
   do. `wrapUnspacedScripts` drops a line break between a decoration and Chinese
   or Japanese text on either side, so `^中^⏎字` would become `^中^字`, which
   is not a superscript. So a line break next to a superscript is never
   dropped. The same break is also lost in a setext heading, which is
   written as an ATX heading on one line. That happens with the default
   config (`_中_⏎字⏎===` becomes `# _中_字` today) and, through
   `keeps_break_between()`, with `wrapUnspacedScripts` off and `textWrap` set
   to `never`, `always` or `sentence` (`*中。*⏎字⏎===` becomes `# _中。_字`).
   So `^中^⏎字⏎===` would become `# ^中^字`. In a heading the break next to a
   superscript is written as a space. And `hardBreakKind` (backslash by
   default) rewrites a hard break, which changes the character right after a
   `^` run before it: `x ^␣␣⏎b^` (`␣` is a space) would become `x ^\⏎b^`,
   which is a superscript, `x ^\⏎b^` with `doubleSpace` would become
   `x ^␣␣⏎b^`, which is not, and `^^a .^␣␣⏎b` would become `^^a .^\⏎b`, where
   the closer can now open too and the rule of three stops it pairing. So a
   hard break right after a `^` run is written as it was. This also covers
   the known differences above, where this parser cannot see what
   pulldown-cmark would pair, so rule 4's check could not catch a change.
4. Where a paragraph has more than one decoration, or a decoration and a hard
   break, the formatter already prints its output, parses it again, and checks
   that the decorations are the same (`decorations_need_checking` and
   `reads_back_decorations` in `src/generation/generate.rs`). A superscript is
   a decoration, so it counts toward that condition and is compared in the
   check with no extra code. A `^` run that can open or close starts the
   check too, like a `*` or `_` run (`pairable`), as a second guard for the
   hard break case of rule 3. A paragraph with a single superscript, no hard
   break and no other pairable run is not checked, which is why rule 3 must
   hold on its own.
5. Changing an emphasis delimiter next to a superscript (`emphasisKind`) needs
   no new rule: `^` is punctuation, so it does not make `_` intraword.

## Configuration

No new option. The parser always reads superscripts, the same as it always
reads math and footnotes.

This does change some output for renderers without superscript support. A
superscript can hide emphasis that overlaps it: in `^a *b^ c*`, the `*b^ c*`
is no longer emphasis, so `emphasisKind` no longer rewrites it. Formatting
rule 3 keeps a hard break right after a `^` run as it was written
(`x ^␣␣⏎b`), and in a heading writes a space between a superscript and
Chinese or Japanese text. And a footnote reference inside brackets now stops
them from becoming a link or image (rule 6), so `[b [^c] d](/u)` is no longer
a link node, which matches how pulldown-cmark renders it. All of these are
rare, and none changes how the text renders.

## Tests

- `tests/parser_specs/superscript.txt`: the syntax tree for every example in
  the tables above, plus:
  - leftover runs: `^^a^`, `^^^a^^`, `^^^^a^^^^`;
  - a defined footnote inside a superscript, and `[^a^]`, `[^1] x^2^`,
    `^x [^a b^] c` and `^x [^a^ b] c^` with and without a footnote
    definition (the same superscripts either way), and `^x ![^a b^] c`,
    `[^a [b] c^]` and `[^␣]`;
  - `[^a^][x]`, `^x [^a b^][y] c`, `[^a^][]`, `[^a^][^b]`, `[^a^][^]` and
    `[^a^][^ ]`;
  - ``^x [^a `]` b^] c^`` (a `]` in a code span);
  - `^a [b^ [^c] d](/u)` and `![b [^c] d](/u)`, with and without `[^c]`
    defined (no link or image node);
  - `> ^x [^a⏎> b^] y` (a footnote name wrapped in a blockquote, no
    superscript);
  - a superscript in a table cell, in a heading, and with a hard break inside;
  - `![^a^](/u)`, which must have no superscript node.
- Formatter specs in `tests/specs/TextDecoration/`, one file per config:
  - `TextDecoration_Superscript.txt` (default config): each example is
    unchanged apart from `*` emphasis, which the default `emphasisKind`
    writes as `_` (`*^a^*` becomes `_^a^_`); `x ^␣␣⏎b^`, `^^a .^␣␣⏎b`,
    `*x ^␣␣⏎y [^a* b] z^` and `x ^␣␣⏎y $a^ {b$` keep their two spaces;
    `^中^⏎字⏎===` becomes `# ^中^ 字`;
  - `TextDecoration_Superscript_TextWrap_Always.txt`: a long superscript is
    wrapped at its spaces and joined again;
  - `TextDecoration_Superscript_EmphasisKind_Asterisks.txt`: emphasis inside
    and next to a superscript follows `emphasisKind: asterisks`;
  - `TextDecoration_Superscript_HardBreakKind_DoubleSpace.txt`: `x ^\⏎b^`
    keeps its backslash;
  - `TextDecoration_Superscript_WrapUnspacedScripts.txt`
    (`textWrap: never, wrapUnspacedScripts: true`): `^中^⏎字` becomes
    `^中^ 字` and `字⏎^中^` becomes `字 ^中^`, not joined;
  - `TextDecoration_Superscript_Heading_TextWrap_Never.txt` and
    `TextDecoration_Superscript_Heading_TextWrap_Sentence.txt`:
    `^中^⏎字⏎===` becomes `# ^中^ 字` and `字⏎^中^⏎===` becomes `# 字 ^中^`.
- The existing tests, CommonMark spec check and corpus check must still pass.

## Open questions

- Should reading superscripts be an option that is off by default, matching
  pulldown-cmark, where `ENABLE_SUPERSCRIPT` is off by default? The
  Configuration section lists what turning it on changes.
- Should we follow pulldown-cmark master (superscript inside a word) now,
  before it is released?
- The subscript spec (PR #1) follows Pandoc: `H~2~O` is a subscript and
  `~a b~` is not. pulldown-cmark 0.13 does the opposite. Both specs should
  target the same renderer.
