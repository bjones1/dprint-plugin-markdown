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
and wikilinks. The normative reference is pulldown-cmark **0.13.6**, configured
with `Options::all() & !Options::ENABLE_SMART_PUNCTUATION`. The supplied
examples were reported as matching 0.13.4 and 0.13.6; this revision does not
claim an independent execution of them. Reference results and repository
function descriptions must be verified when building acceptance fixtures.

pulldown-cmark's rule is short: **`^` is parsed like `_`, and where `_` would
make emphasis or strong emphasis, `^` makes a superscript** (with one
difference, see Known differences below). The syntax rules describe the
reference renderer. The fork follows them except for the explicitly enumerated
compatibility differences below. In detail, using the
[CommonMark emphasis rules](https://spec.commonmark.org/0.31.2/#emphasis-and-strong-emphasis):

1. **Delimiter run.** A run of `^` that is not escaped (`\^`) and is not inside
   a code span, math token, an autolink, an inline HTML token, or a link
   destination or title. Carets inside `<span title="^a^">` are literal;
   ordinary text in `<span>^a^</span>` remains eligible for superscript.
   HTML blocks follow the existing block parser. pulldown-cmark also reads superscripts in image alt text, but this
   parser never parses alt text, so it never makes a node there. Formatting is
   required to preserve its target-renderer interpretation. The described
   formatter changes whitespace in alt text (for example
   `![x ^a␣␣⏎b^](/u)` becomes `![x ^a b^](/u)`), and swapping one kind of
   whitespace for another preserves neighbor whitespace classification.
   Reference fixtures must still verify the resulting alt text; that
   observation alone is not a complete formatting-safety proof.
2. **Opener and closer.** Same as for `_`. Before and after refer to characters
   outside the complete unescaped delimiter run. A run can open if the character
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
   `` [^a `]` b] `` the first `]` is inside a code span, the scan stops there, and
   there is no footnote (the same holds for a `]` in math or an autolink).
   `name` is text: no `^` in it opens or closes a superscript. Otherwise the
   bracket is read like any other, so `[^a^ b`, `[^a [b] c^]`, `[^a^](/u)`
   and `[^a^][x]` all hold a superscript. A footnote reference also stops
   every link or image bracket before it from becoming a link or image, the
   same as a link stops earlier links: in `[b [^c] d](/u)` and
   `![b [^c] d](/u)` there is no link and no image.

The supplied results below describe pulldown-cmark 0.13.6 with those options
(the middle column describes its rendering; `⏎` is a line break). Unless a
fixture explicitly supplies a definition, footnote and link-reference labels
are undefined. These are reference expectations, not automatically fork AST
expectations; the documented compatibility differences take precedence for
the fork. A fixture that supplies a definition must include that definition
in its exact input and state its expected result separately.

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
| `<span title="^a^">x</span>` | `<span title="^a^">x</span>`              | carets in an inline HTML attribute are literal |
| `<span>^a^</span>`    | `<span><sup>a</sup></span>`                      | inline HTML tokens do not hide ordinary text between tags |
| `<div>^a^</div>`      | `<div>^a^</div>`                                 | an HTML block is handled by the existing block parser |
| `*^a^*`               | `<em><sup>a</sup></em>`                          | nesting                     |
| `^*a*^`               | `<sup><em>a</em></sup>`                          | nesting                     |
| `~~^a^~~`             | `<del><sup>a</sup></del>`                        | strikethrough outside       |
| `^~~a~~^`             | `<sup><del>a</del></sup>`                        | strikethrough inside        |
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
text. With `[^a]: n` included in the input and no definition for `[x]`, the
reference renderer reads `[^a](/u)` as `<a href="/u">^a</a>` and
`^p [^a][x] q^` as `^p [<sup>a][x] q</sup>`. The fork instead recognizes the
defined footnote at `[^a]`; its separate expected AST must record that node
and the resulting treatment of the surrounding carets and suffix. This
predates superscript support. Test both defined and undefined variants with
complete source inputs.

Fourth, rule 6 drops only the `^` delimiters inside a footnote name; `*` and
`_` there still pair, as they do today. pulldown-cmark pairs none of them, so
in `*x ^y [^a* b] z^` it reads a superscript and this parser reads emphasis
from `*x` to `a*` and no superscript. Dropping `*` and `_` too would change
current output, so it is left out.

Fifth, this parser's inline math differs from pulldown-cmark's: it has no
rule for braces and does not let a `$` before a digit close. So in
`x ^y $a^ {b$` pulldown-cmark reads no math and can pair `^y` with `a^`,
while this parser reads `$a^ {b$` as math.

These are intentional parser compatibility differences, not proof that all
formatting is safe. For each difference, formatting must preserve the reference renderer's
interpretation or match a precise established baseline exception under Scope
and baseline. New feature-induced regressions are not accepted. Compare the reference
renderer's interpretation before and after formatting independently of the
fork's own reparse check. Reference comparisons are acceptance tests; the
formatter need not invoke pulldown-cmark at runtime. The existing single-tilde
subscript mismatch must have an exact per-fixture exception, not a blanket
exemption for every input containing `~`.


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

Each node consumes one or two caret characters from each side, including
nodes nested within longer runs. Preserve that consumed width on generation.
Width may be derived from `first_child.span.start - decoration.span.start`
only after verifying these repository invariants:

* Spans use the existing repository unit and endpoint convention; record both
  explicitly in fixtures rather than assuming bytes or characters.
* The first child's source span starts immediately after the opening delimiter,
  including escapes, entities, code spans, and nested decorations.
* A matched superscript always has a child, and its computed width is 1 or 2.
  Opener eligibility alone does not establish the child invariant.

If these invariants do not hold, store the consumed delimiter width explicitly
using the repository's conventions. Both normal and preservation generation
paths must use the selected representation. Do not read the entire contiguous
caret run as one node's delimiter: `^^^a^^^` has nested delimiter ownership.


## Scope and baseline

| Category | Required treatment |
| --- | --- |
| Superscript parsing and formatting | Implement the specified nodes, delimiter ownership, preservation rules, and acceptance fixtures |
| Explicit footnote bracket changes | Implement rule 6's enclosing-link/image behavior, including inputs without superscript candidates, with separate regression coverage |
| Other existing parser/formatter differences | Preserve the established baseline and prevent additional regressions caused by this feature; general repair is out of scope |

The reference renderer remains the compatibility target. Intentional fork AST
differences do not require the formatter to produce the same AST as the
reference. Existing reference-rendering changes made by the unmodified
formatter, including single-tilde normalization, must be captured as precise
baseline exceptions. An existing parser difference does not automatically
permit a new formatter-induced rendering difference.

For each relevant baseline exception, record complete input, configuration,
unmodified formatter output, and reference interpretations before and after.
Use a known pre-change revision. Do not regenerate that baseline from the
implementation under test. Require no additional semantic differences beyond
the recorded baseline except the explicit in-scope behavior changes above.
If baseline evidence is unavailable, do not assume an exemption; use source
preservation for the uncertain region and record the missing evidence.

## Required rewrite-safety strategy

The formatter must select a safe behavior for each candidate rewrite. A passing
fork-based reparse check does not establish safety where the fork cannot see
the reference renderer's interpretation.

1. Identify compatibility-sensitive regions conservatively from source context,
   not only recognized superscript nodes. Cover the documented mixed caret/tilde
   lower-bound cases, wikilinks, heading attributes, footnote/link interactions,
   footnote-label emphasis interactions, and differing math boundaries.
2. A transformation in such a region is permitted only when a documented
   preservation rule establishes its safety for the applicable source context.
   State the rule's preconditions, protected region, and allowed changes. The
   hard-break and separator rules are examples, not proof that all other
   transformations are safe.
3. Where the fork cannot establish safety and no applicable preservation rule
   exists, preserve the containing source region using the fallback scope below.
   A conservative detector may protect more source than strictly necessary.
4. Document detector coverage and fallback paths before enabling rewrites in
   these contexts. If the detector cannot reliably isolate a region, expand
   protection to its enclosing block. Detection must include contexts that the
   fork represents as literal text or opaque tokens.

Reference acceptance tests validate these rules and their preconditions;
example coverage alone is not the runtime safety mechanism. No runtime
pulldown-cmark dependency is required. This strategy is scoped to avoiding new
regressions from this feature, not implementing every missing extension.

### Fallback regions

Semantic preservation takes precedence over requested wrapping width and
formatting preferences. Fallback restores a source region large enough to
preserve the relevant block and inline interpretation. Do not retain only a
superscript node while rewriting its unsafe neighbors.

| Context | Minimum fallback region |
| --- | --- |
| Paragraph | Complete paragraph source with original internal breaks; include affected container prefixes where necessary |
| Heading | Complete heading block, including ATX markers or setext underline and any attribute syntax |
| Table cell | Complete cell source when cell boundaries and table structure are unchanged; otherwise the complete table block |
| List or blockquote content | Complete affected child block with its original container-prefix context; expand to the item, quote, or enclosing list when boundaries or nesting could change |

If a rewrite changes heading syntax, container prefixes, or block boundaries,
restore the enclosing block rather than its inline content alone. Preserve
original protected source except for the explicitly configured output line-ending
style where safe. The fallback mechanism may retain a broader region. Avoid
mixing restored source with already emitted prefixes from a rejected candidate;
buffer or otherwise discard that candidate before emitting fallback. If safety
requires a larger scope, continue expanding to a stable enclosing region.

## Suggested implementation: parsing

The following edit locations are implementation guidance, subject to a
repository audit; behavioral requirements and fixtures take precedence.
Proposed parser changes are in `src/parser/inline.rs`. Because `^` follows the `_` rules,
most of the code is reused as is:

- `parse()` sends `^` to `parse_emphasis_delimiter`, the same as `*`, `_` and
  `~`.
- `flanking()` already treats every character other than `*` and `~~` like
  `_`, so it may be reusable; verify character classification and run handling.
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
- `find_opener()` is a candidate for reuse; verify its behavior against the
  required syntax and documented exceptions before deciding whether it changes.
- `wrap_emphasis()` uses one or two characters from each side, as it does for
  `_`, and makes a `Superscript` decoration either way.

## Suggested implementation: generation

Proposed generation changes are in `src/generation/generate.rs`:

- `gen_text_decoration()` writes `^` or `^^`. Its last match arm is a
  catch-all that writes `~~`, so the new case must come before it.
- `decoration_delimiter()` (`resolve`) returns the written delimiter for a
  superscript, using the width representation selected in Data structures. So does
  `written_delimiter()`, which `decoration_delimiter()` returns instead while
  decorations are preserved. Reading the delimiter off the file text is wrong
  for `^^^a^^^`.
- `unescaped_delimiter_runs()`, `run_can_open()` and `run_can_close()` read
  `^` runs too, with the `_` rules, so they count in `pairable` (rule 4).
- Inspect `decoration_keeps_its_delimiters()` and the associated print paths.
  A superscript-specific result may be needed to implement the separator
  requirements below; select it from verified helper semantics, not its name.
- In a heading, a line break between a superscript and Chinese or Japanese
  text must come out as a space (rule 3 below). Whether newlines are off is
  only known at print time (`with_no_new_lines`, generate.rs:773), so the
  break cannot be decided while generating. Where either node is a
  superscript, the `keeps_break_between()` branch (generate.rs:483-487)
  pushes `new_line_or_space_if_newlines_disabled()` instead of a bare
  `NewLine`, and the path that joins such text in a heading with the default
  config (today `_中_⏎字⏎===` becomes `# _中_字`) does the same.
- A hard break whose source marker begins immediately after a caret run
  retains its original marker spelling, regardless of `hardBreakKind`.
  Apply the exact source-boundary and scope rules in rule 3 below.
- `tests/commonmark_spec_check.rs` renders decorations to html; it needs a
  `sup` case.

## Required formatting behavior

1. A superscript is written with the same number of `^` it was written with
   (see Data structures), then its content, then the same delimiter again.
   The content is formatted like the content of emphasis, so it may be
   wrapped at its spaces.
2. Wrapping must preserve block structure as well as inline superscript
   pairing. For example, wrapping `^alpha # beta^` before `#` must not
   introduce a heading. Existing safeguards against introducing headings,
   lists, block quotes, and fences apply inside superscript content. The
   following neighbor-classification observations are not a complete proof
   of wrapping safety. A line break only replaces a space or
   goes between two Chinese or Japanese characters (`wrapUnspacedScripts`).
   An opener is never followed by a space, a closer is never preceded by one,
   and `^` is not such a character, so the `^` characters stay attached to a
   word.
3. Whether a `^` run is eligible to open or close depends on whether its neighbors
   are whitespace, punctuation or something else. Eligibility does not alone
   determine pairing, which also depends on precedence and delimiter processing.
   Wrapping swaps a space for a
   line break or back (both whitespace), and changing `*` to `_` or back swaps
   one punctuation character for another, so neither changes it. Three cases
   do. `wrapUnspacedScripts` drops a line break between a decoration and Chinese
   or Japanese text on either side, so `^中^⏎字` would become `^中^字`, which
   is not a superscript. A soft line break beside a superscript must retain
   a whitespace separator. It may remain a line break or become a space where
   the formatting context requires one, but must not become an empty string.
   Hard breaks follow the separate marker-preservation rule below. The same
   separator is also lost in a setext heading, which is
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
   hard-break marker is preserved when its marker begins immediately after
   a source caret run: either a backslash following the run or the complete
   trailing-space sequence following it. Preserve the backslash spelling or
   the exact trailing-space count, subject only to the configured line-ending
   style. For trailing-space markers, adjacency is measured from the first
   trailing space, not from the character immediately before the newline.
   This protection deliberately applies conservatively to literal and escaped
   caret runs when followed by an actual hard-break node. It does not create
   hard breaks inside code or math; their source-sensitive content follows
   existing preservation rules. Reference tests must additionally cover every
   known difference: the fork's own reparse check cannot establish preservation
   of reference-renderer pairing.
4. Where a paragraph has more than one decoration, or a decoration and a hard
   break, the formatter already prints its output, parses it again, and checks
   that the decorations are the same (`decorations_need_checking` and
   `reads_back_decorations` in `src/generation/generate.rs`). A superscript is
   a decoration, so it counts toward that condition and is compared in the
   check; verify that the existing comparison handles the new enum kind,
   nesting, and literal text rather than assuming no additional code is needed. A `^` run that can open or close starts the
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
observable behavior changes. Reference-renderer preservation must be tested;
no universal rendering guarantee is made for renderers without superscript.

## Tests

- `tests/parser_specs/superscript.txt`: the expected fork syntax tree for
  every example in the tables above, plus the cases below. Reference fixtures
  separately assert the pinned renderer's interpretation. For documented
  differences, record both expectations explicitly; do not copy a reference
  result into a fork fixture. Every defined/undefined-label variant includes
  the complete corresponding definition context in its exact source input.
  Additional cases:
  - `~~^a^~~` and `^~~a~~^` (superscript nested with strikethrough in both
    directions);
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
  - `<span title="^a^">x</span>` (no superscript from the attribute),
    `<span>^a^</span>` (superscript in inline text), and `<div>^a^</div>`
    (no superscript inside an HTML block); assert both the pinned renderer
    result and the fork AST, and ensure formatter fixtures preserve the
    attribute value while retaining the superscript in inline text;
  - `![^a^](/u)`, which must have no superscript node.
- Formatter specs in `tests/specs/TextDecoration/`, one file per config:
  - `TextDecoration_Superscript.txt` (default config): each example is
    assigned an exact expected output. The default `emphasisKind`
    writes `*` emphasis as `_` (`*^a^*` becomes `_^a^_`); `x ^␣␣⏎b^`, `^^a .^␣␣⏎b`,
    `*x ^␣␣⏎y [^a* b] z^` and `x ^␣␣⏎y $a^ {b$` keep their two spaces;
    `~~^a^~~` and `^~~a~~^` keep both decoration pairs;
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

## Compatibility decisions

* Parsing is always enabled; there is no new option.
* Released pulldown-cmark 0.13.6 with the options above is normative. Master
  branch behavior is informative future work, not an alternative for this change.
* Superscript and subscript must use the CodeChat renderer as their shared
  compatibility target. The reported Pandoc-based behavior in subscript
  [PR #1](https://github.com/bjones1/dprint-plugin-markdown/pull/1) is a
  coordination issue to resolve with that team before integrated acceptance.
  This spec does not itself change or approve changes to their implementation.

## Required footnote behavior and label information

This feature also changes enclosing link/image recognition around footnote
references, including documents containing no superscript delimiters. This is
an intentional behavior change within this spec, not merely a superscript
implementation detail. Give it separate regression fixtures with defined and
undefined footnotes and with no carets except the footnote marker.

An enclosing bracket preceding the recognized footnote must not subsequently
create a link or image where rule 6 prohibits one. Its source remains available
as literal text. Carets within the recognized footnote label cannot participate
in superscript pairing; the intentional existing emphasis difference remains.
Delimiters outside the label must retain the scope and processing order required
by the expected AST. Later independent links/images remain eligible normally.

Stack clearing, bracket inactivation, and removal of delimiter candidates are
candidate mechanisms, not normative behavior. Choose operations that produce
the required trees. Do not assume clearing a bracket stack and marking entries
inactive are interchangeable; assert their observable effects in nested cases.

The implementation must distinguish the following information when applying
rule 6. These are conceptual views, not a requirement to add three structures,
a new source-map subsystem, or a second label parser. Reuse existing source
slices, offsets, and label helpers where they preserve the required distinctions:

* **Source label:** the original source spelling with source-position mapping.
  Use it to determine escaping and the raw closing-bracket candidate; do not
  treat decoded entities as source bracket delimiters.
* **Logical label source:** source spelling across continuation lines after
  block-container prefixes are removed, retaining escapes, entities, and a
  mapping back to source offsets. This is what “label text” means in the
  parser guidance; it is not concatenated rendered inline text.
* **Lookup label:** the existing parser's representation for resolving defined
  footnotes. Do not silently replace its normalization as part of this feature.

Before implementation, record the pinned renderer's exact treatment of
blankness, tabs and Unicode whitespace, escaped brackets, and entities in both
`name` and a following `[label]`. Implement those verified outcomes; do not
choose decoded versus source blankness by assumption. Whether the following
label has a definition must be included in the fixtures. Verify that a raw
closing candidate corresponds to the structural closing bracket as required
by rule 6, including candidates inside code, math, or autolinks.

Required additional inputs (backslashes below are literal source characters):

```markdown
[^a\]b^]
[^a\[b^]
[^&#32;]
[^a^][&#32;]
[^a^][x]
> ^x [^a
> b^] y
- ^x [^a
  b^] y
```

Add actual tab-only labels and Unicode-whitespace labels; record their code
points in the fixture description. These cases require pinned reference
results, not guessed expected trees.

## Acceptance requirements

Parser fixtures state exact inputs and expected ASTs. Assert nesting,
leftover literal carets, source spans, and delimiter ownership for `^^a^`,
`^^^a^^`, `^^^a^^^`, and `^^^^a^^^^`. Verify span recovery using:

```markdown
^\*a^
^&amp;^
^`a`^
^*a*^
^^^a^^^
```

Formatter fixtures state exact configuration and expected output for each
case, including heading conversion. Do not claim all examples are unchanged:
normal emphasis rewriting and explicitly specified heading transformations
remain permitted. Cover literal caret text in paragraphs with no superscript
nodes as well as recognized superscripts.

Run equivalent LF and CRLF fixtures; AST structure must agree after accounting
for source offsets and the configured output newline style. Define `⏎` as a
logical line ending and `␣` as one ASCII space in documentary examples. Actual
fixture inputs contain real line endings and spaces.

Within Scope and baseline, compare each parser's own input/output
interpretations independently. Preserve
block kinds, inline kinds and nesting, literal text, heading levels and
attributes, link/image destinations and titles, footnote reference identities,
code/math contents, and hard breaks. These properties must retain their
interpretation within each parser. Source-marker changes such as setext-to-ATX
heading conversion remain permitted when those properties are preserved. Ordinary spaces and soft breaks may be
normalized where wrapping permits it; do not normalize source-sensitive
code/math content or hard breaks. Source offsets may change after formatting
and are checked against each input separately. Reference comparisons may use
normalized events rather than exact HTML strings. Enumerate the precise
expected compatibility exceptions per fixture; never skip an entire fixture
because it contains unsupported syntax.

Add hard-break boundary fixtures with two and three actual trailing spaces,
backslash markers, literal/escaped caret runs, and carets inside code or math.
Assert exact marker spelling and space counts where protection applies, and
assert that code/math content is not reclassified as hard-break syntax. Include
LF and CRLF variants and both `hardBreakKind` settings.

For every known difference, test the documented safety detector, applicable
preservation rules, and fallback scope, including cases represented only as
literal text or opaque tokens in the fork. Assert that later unaffected blocks
can still format normally. Test supported transformations, including
`emphasisKind`, `hardBreakKind`, wrapping/joining, heading conversion, and
`wrapUnspacedScripts`, including relevant combinations. Include block-syntax
wrap hazards and separate footnote bracket-inactivation regressions.

Apply Required rewrite-safety strategy and its context-specific fallback
regions. Include paragraph, heading, table-cell, and list/blockquote fallback
fixtures, including rejected heading conversion and changed block boundaries.
A fallback may exceed the requested width. Test this outcome explicitly and
verify idempotence under each formatter configuration.

## Preimplementation checks and completion

Before implementation:

* Verify the proposed repository helper semantics, span invariants, and actual
  fallback support. Suggested function names, return values, stack operations,
  and source line references do not override behavioral requirements.
* Record pinned-reference outcomes for the additional label cases and the
  separate expected fork ASTs.
* Record relevant pre-change baseline exceptions and the explicit footnote
  behavior changes; do not expand this task into general formatter repair.
* Document the conservative detector coverage, preservation-rule preconditions,
  and fallback region for each compatibility-sensitive context. Conceptual label
  views may share existing structures; require only the information necessary
  for correct behavior.

Completion requires the explicit fixtures, existing tests, CommonMark spec
check, and corpus check to pass with baseline exceptions and authorized behavior
changes identified. Unrelated failures must be reported against their baseline
rather than silently accepted or repaired outside the feature scope.
