<h1 id="cc--subscript-spec">Subscript support</h1>

Goal
----

dprint is a formatter: it reads Markdown and writes Markdown. The goal is that,
when the user turns subscript on, the formatter keeps every subscript in the
document a subscript (`H~2~O` stays `H~2~O`) and writes no decoration the file
didn't hold: the output holds the same decorations, each of the same kind over
the same text and in the same place among the text around it, give or take the
whitespace that wrapping moves. A subscript is a pair of single tildes, as in the
[pulldown-cmark subscript syntax](https://github.com/pulldown-cmark/pulldown-cmark/blob/1c1a3123207d9e07fdfcf3d3418a047907185afb/pulldown-cmark/specs/super_sub.txt),
read with this parser's own delimiter rules. Both parsers read a pair of
single `~` with one-sided flanking -- a `~` may sit within a word, as `*` may
-- so they agree on the ordinary cases (`H~2~O`, `~x~`, `10~20 and 30~40`,
nested `~a ~b~ c~`, and `a ~ b ~ c` as text). They part company over
punctuation flanking and over how a closer matches an opener run of another
length, in both directions: `~a ~~b~` is `<sub>a ~~b</sub>` there and text
here, `~a~~ b~` is text there and a subscript over `a~~ b` here, `~~a~b~~` is a
strikethrough here and text there, and `~.~a~a` is `<sub>.</sub>` there and
`<sub>a</sub>` here. Either way the formatter writes the text back unchanged,
which is what matters for a formatter.

Problem
-------

The parser already reads a pair of one or two `~` as
[`TextDecorationKind::Strikethrough`](ast.rs), and
[`gen_text_decoration`](../generation/generate.rs) writes a strikethrough with
`~~`, except in the text of a reference link, where the delimiter it was written
with is kept. So today `~x~` is rewritten as `~~x~~`, which turns a subscript into
a strikethrough, and `H~2~O` isn't read as a decoration at all, since a single
`~` can't sit within a word.

Configuration
-------------

Add the boolean option `subscript`, default `false`, to `Configuration` in
`src/configuration/types.rs`, to `resolve_config.rs` and `builder.rs` there, and
to `deployment/schema.json`. It is separate from any superscript option, since
pulldown-cmark has separate `ENABLE_SUBSCRIPT` and `ENABLE_SUPERSCRIPT` flags
and only subscript changes how existing text (`~x~`) is read. The schema description warns that
turning it on reads `~x~` as subscript rather than strikethrough, and reads a
pair of single `~` within words as subscript too (`10~20 and 30~40`).

With `subscript` off, the formatter's output is unchanged for every input; the
existing specs, which all run with the default, check this. That is a
statement about compatibility, not about correctness: the bugs formatter rule
3 describes have always been reachable with `~~` and the option off
(`~`␣␣⏎`a~a~` writes a strikethrough the file didn't hold), and the fixes are
gated on the option so that turning it off keeps today's output, byte for
byte. Turning it on can also change the output of text holding no `~` at all,
since rule 2's whole-block comparison is used for every block with the option
on: a hard break the formatter would otherwise write as `\` beside a lone `*`
is kept as two spaces (`x *`␣␣⏎`a*a*`), where writing the `\` moves the
emphasis onto the first `a`. (A lone `*` at the start of a block is an empty
bullet marker rather than a delimiter, so the text before it is part of the
case.) That is an older bug of the same kind as rule
3's, fixed only with the option on so that turning it off keeps today's
output.

Parser changes
--------------

`parse` and `parse_inline_content` in `mod.rs` take a `subscript: bool`, which
is stored in `InlineContext`; `format_text`, `format_range` and the re-parse in
`generate.rs` pass `configuration.subscript`, while the parser's spec-file tests
and `commonmark_spec_check` pass `false`.
With it on, three things change in
[`inline.rs`](inline.rs); everything else about `~` (run length, escapes, code
spans, links) stays as it is. Pairing changes only as rule 3 describes.

1. `TextDecorationKind` in [`ast.rs`](ast.rs) gains `Subscript`. In
   `wrap_emphasis`, a pair of single `~` is `Subscript`; a pair of `~~` stays
   `Strikethrough`. With the option off, both stay `Strikethrough`, as today.
2. In `flanking`, a single `~` may sit within a word, as `*` and `~~` already
   may, so `H~2~O` holds a subscript. The CommonMark flanking rules still
   apply, so `a ~ b ~ c` is text, and `10~20 and 30~40` is a subscript
   spanning `20 and 30`, as in pulldown-cmark.
3. In `find_opener`, the CommonMark "rule of three" doesn't apply to `~`
   while `subscript` is on.
   Skipping a `~~` under that rule would pair the single tildes around it, so `~a~~b~` would be a
   subscript where pulldown-cmark reads text. The rule of three was also the
   only thing that let a closer look past a nearest opener of another length,
   so a `~~` closer with `subscript` on would stop at a lone `~` in its own
   content and nothing would pair: `~~a~b~~` would be text where it is a
   strikethrough today. A `~~` closer therefore reaches back past an opener
   *shorter* than itself, which keeps `~~a~b~~` a strikethrough while leaving
   `~a~~b~` text, since a longer nearest opener still blocks a closer.

A closer still gives up at the nearest opener when that opener is *longer*
than it, as strikethrough does today, so `~a ~~b~` is text where pulldown-cmark
reads `<sub>a ~~b</sub>`. The formatter writes it back unchanged either way, so
this is left as it is.

Formatter changes
-----------------

1. A subscript is always written with a single `~`, whatever the emphasis and
   strong settings are, including within the text of a reference link, where
   decorations are written as they were. `resolve` and `written_delimiter` in
   `generate.rs` each get a `Subscript` arm returning `~`.
2. The formatter re-parses what it writes to check that the decorations
   survived (`parse_inline_content` in `generate.rs`) with the same `subscript`
   setting. With the option off the check compares each decoration's kind and
   its content, with whitespace and backslashes stripped, in the order they are
   written. With it on, that is not enough: a decoration can be handed content
   that reads the same while sitting somewhere else, as in `~`␣␣⏎`a~a~`, whose
   subscript moves from the second `a` to the first and whose content is `a`
   either way. So with the option on the block's whole content is compared with
   each decoration marked where it sits (`written_shape` in `generate.rs`); a
   tree comparison would be stronger still, and would reject the rendering of
   rule 3's second case, which is wanted. The nodes whose text the comparison
   leaves out -- inline HTML, math, an image, a footnote reference -- carry no
   mark of their own, so a decoration that swapped places with one of them
   would read the same; the writer never reorders nodes, so that doesn't
   happen. When the check disagrees, the block is
   written again with decorations and hard breaks as the file had them, which
   also stops a line break being dropped; if that still disagrees, the file's
   own text is written unchanged.
3. A run of `~` counts as a run that can pair when the formatter decides
   whether what it writes needs reading back (`unescaped_delimiter_runs` in
   `generate.rs`, gathered with `configuration.subscript`). Otherwise
   formatting can *make* a subscript out of text: writing a double-space hard
   break as `\` puts the backslash against a lone `~`, which then pairs with a
   `~` within a later word (`~`␣␣⏎`a~b`), and dropping a line break between
   two characters of an unspaced script joins a lone `~` to a subscript that
   follows it, moving the subscript onto other content. Each then falls back as
   rule 2 describes: the first is written unchanged, and in the second the break
   is written out as a space (`~日`⏎`~本~` becomes `~日 ~本~`).
4. Nothing else changes: line wrapping, escaping and the other rules that keep
   strikethrough intact apply to subscript as well, so the output holds the
   same decorations -- same kind, same content apart from whitespace, in the
   same place, which is the check of rule 2, not an identical tree -- and
   formatting twice gives the same text.

Test cases
----------

The "on" rows go in `tests/specs/TextDecoration/TextDecoration_Subscript_On.txt`
(`~~ subscript: true ~~`), the `H~2~O` "off" row in `TextDecoration_All.txt`,
which already has the `~a~` one. How pulldown-cmark reads each input was
measured at commit `1c1a312` for the draft of this spec. The spec
runner already formats each output a second time and checks it doesn't change.
A wrapping case, `aaaa H~2~O` becoming `aaaa`⏎`H~2~O`, goes in
`TextDecoration_Subscript_TextWrap_Always.txt` with `lineWidth: 6`, and the
unspaced-script break of formatter rule 3 in
`TextDecoration_Subscript_Unspaced.txt`, which needs `wrapUnspacedScripts` and
a `textWrap` that re-flows lines; the file uses
`lineWidth: 12, textWrap: always, wrapUnspacedScripts: true`, and with the
default `textWrap: maintain` the input comes back unchanged.
`tests/specs/Range/Range_Subscript_On.txt` formats a range with the option on.
Two kinds of case go in `TextDecoration_Subscript_Emphasis.txt`. Where a
subscript and an emphasis interleave, the pair that closes first takes the
shared delimiter, as in pulldown-cmark. Where punctuation follows a `~`, the
`~` opens only at a line start or after whitespace or punctuation, so
`a~(*b~ c*` has no subscript.

Of the four cases of formatter rules 2 and 3, `~`␣␣⏎`a~b` needs rule 3, and
`~`␣␣⏎`a~a~` needs rule 2 alone -- its hard break already asks for the check,
since the block holds a decoration and a break. The two unspaced-script cases
need more: `~日`⏎`~本~` needs rule 3, and `~日本`⏎`~日本~` needs both, rule 3
to ask for the check across a soft break at all and rule 2's marked comparison
to see the subscript move. The generated text of
`keeps_subscripts_through_formatting` reaches the hard-break shapes, so a
regression of either rule fails it there. It does generate a soft break
between two characters of an unspaced script, but never one where a lone `~`
and the subscript after it make the loss of that break change what the
decorations hold -- formatting those texts is unchanged with either fix
reverted -- which is why the two unspaced cases are spec cases in
`TextDecoration_Subscript_Unspaced.txt`.

Which `~` runs are read as what is pinned by
`reads_single_tildes_as_subscript_only_with_the_option` in `inline.rs`, which
asserts the decoration kinds with the option on, and for `H~2~O`, `~x~` and
`~~a~b~~` with it off as well, those being the readings the option changes or
must not change; the
parser's own spec tests and `commonmark_spec_check` pass `false`, while
`no_text_is_dropped` in `text_fuzz.rs` runs both values. The output rows below
are not all evidence of the feature: for `H~2~O`, `~~x~~`, `a ~ b ~ c`,
`x~y~~z`, `10~20 and 30~40`, `a \~b~ c`, `` `~x~` ``, `~a ~~b~`, `~~a~b~~` and
the reference-link row the output is the same with the option off, so they are
regression rows that keep the text they hold from being rewritten.
`keeps_subscripts_through_formatting` in `format_text.rs` formats generated
text with the option on and checks that formatting is stable and that the whole
text reads back with every decoration still of the same kind, over the same
content and in the same place, whitespace ignored -- the same marked comparison
as rule 2's `written_shape`, which is what catches a subscript moving onto
other content. A text with the same problem when the option is off doesn't
fail the test, since that problem isn't one of subscript's.

| Input                     | `subscript` | Output                       |
| ------------------------- | ----------- | ---------------------------- |
| `H~2~O`                   | on          | `H~2~O`                      |
| `~x~`                     | on          | `~x~`                        |
| `~~x~~`                   | on          | `~~x~~`                      |
| `~a~ and ~~b~~`           | on          | `~a~ and ~~b~~`              |
| `*~x~*`                   | on          | `_~x~_`                      |
| `~*x*~`                   | on          | `~_x_~`                      |
| `a ~ b ~ c`               | on          | `a ~ b ~ c`                  |
| `x~y~~z` (unequal runs)   | on          | `x~y~~z`                     |
| `10~20 and 30~40`         | on          | `10~20 and 30~40`            |
| `[~x~](u)`                | on          | `[~x~](u)`                   |
| `[~x~]`⏎⏎`[~x~]: u`       | on          | `[~x~]`⏎⏎`[~x~]: u`          |
| `~a~~b~`                  | on          | `~a~~b~`                     |
| `~~a ~b~ c~~`             | on          | `~~a ~b~ c~~`                |
| `a \~b~ c`                | on          | `a \~b~ c`                   |
| `` `~x~` ``               | on          | `` `~x~` ``                  |
| `~a ~b~ c~`               | on          | `~a ~b~ c~`                  |
| `a*b*~c~`                 | on          | `a*b*~c~`                    |
| `~a ~~b~` (see above)     | on          | `~a ~~b~`                    |
| `~~a~b~~`                 | on          | `~~a~b~~` (a strikethrough; text in pulldown-cmark) |
| `~`␣␣⏎`a~b`               | on          | unchanged, break kept        |
| `~日`⏎`~本~`               | on, `wrapUnspacedScripts` + `textWrap: always` | `~日 ~本~` (see rule 3) |
| `~x~`                     | off         | `~~x~~` (as today)           |
| `H~2~O`                   | off         | `H~2~O` (text, as today)     |

The `*~x~*` rows show that emphasis around or inside a subscript is still
written with the configured character (`_` by default).

Known limitations
-----------------

A hard break within a decoration doesn't mark what follows it as the start of a
line, since `paragraph_escapes` in `generate.rs` walks only a paragraph's own
children, so a `*` written there isn't escaped and the line reads as a list
item: `*~a`␣␣⏎`*`⏎`**a本~a~本` loses its paragraph and its subscript with
`textWrap: always`. The same input breaks with the option off when the
decoration is a strikethrough, so this is older than subscript and is left to
be fixed with the escaping path rather than here. For this text, though,
turning the option on is what makes it break: with the option off the
subscript is text, the `*` is escaped and the output is stable. It is the one
input found where the option on is worse than off. The output isn't stable
either: formatting it again writes the line out as a list item proper.
