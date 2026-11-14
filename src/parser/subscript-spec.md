<h1 id="cc--subscript-spec">Subscript support</h1>

Goal
----

dprint is a formatter: it reads Markdown and writes Markdown. The goal is that,
when the user turns subscript on, the formatter keeps every subscript in the
document a subscript (`H~2~O` stays `H~2~O`) and writes nothing that parses
differently from what it read. Which `~` runs are subscript follows the
[pulldown-cmark subscript syntax](https://github.com/pulldown-cmark/pulldown-cmark/blob/1c1a3123207d9e07fdfcf3d3418a047907185afb/pulldown-cmark/specs/super_sub.txt):
a pair of single tildes.

Problem
-------

The parser already reads a pair of one or two `~` as
[`TextDecorationKind::Strikethrough`](ast.rs), and
[`gen_text_decoration`](../generation/generate.rs) writes every strikethrough
with `~~`. So today `~x~` is rewritten as `~~x~~`, which turns a subscript into
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
existing specs, which all run with the default, check this.

Parser changes
--------------

`parse` and `parse_inline_content` in `mod.rs` take a `subscript: bool`, which
is stored in `InlineContext`; `format_text`, `format_range` and the re-parse in
`generate.rs` pass `configuration.subscript`, and the parser's own tests pass
`false`.
With it on, three things change in
[`inline.rs`](inline.rs); everything else about `~` (run length, pairing,
escapes, code spans, links) stays as it is.

1. `TextDecorationKind` gains `Subscript`. In `wrap_emphasis`, a pair of
   single `~` is `Subscript`; a pair of `~~` stays `Strikethrough`. With the
   option off, both stay `Strikethrough`, as today.
2. In `flanking`, a single `~` may sit within a word, as `*` and `~~` already
   may, so `H~2~O` holds a subscript. The CommonMark flanking rules still
   apply, so `a ~ b ~ c` is text, and `10~20 and 30~40` is a subscript
   spanning `20 and 30`, as in pulldown-cmark.
3. In `find_opener`, the CommonMark "rule of three" doesn't apply to `~`
   while `subscript` is on.
   Skipping a `~~` under that rule would pair the single tildes around it, so `~a~~b~` would be a
   subscript where pulldown-cmark reads text.

A closer still gives up at the nearest opener when its length differs, as
strikethrough does today, so `~a ~~b~` is text where pulldown-cmark reads `<sub>a ~~b</sub>`. The
formatter writes it back unchanged either way, so this is left as it is.

Formatter changes
-----------------

1. A subscript is always written with a single `~`, whatever the emphasis and
   strong settings are, including within the text of a reference link, where
   decorations are written as they were. `resolve` and `written_delimiter` in
   `generate.rs` each get a `Subscript` arm returning `~`.
2. The formatter re-parses what it writes to check that the decorations
   survived (`parse_inline_content` in `generate.rs`) with the same `subscript`
   setting.
3. Nothing else changes: line wrapping, escaping and the other rules that keep
   strikethrough intact apply to subscript as well, so the output parses to the
   same tree and formatting twice gives the same text.

Test cases
----------

The "on" rows go in `tests/specs/TextDecoration/TextDecoration_Subscript_On.txt`
(`~~ subscript: true ~~`), the `H~2~O` "off" row in `TextDecoration_All.txt`,
which already has the `~x~` one. How pulldown-cmark reads each input was
measured at commit `1c1a312` for the draft of this spec. The spec
runner already formats each output a second time and checks it doesn't change.
`keeps_subscripts_through_formatting` in `format_text.rs` formats generated
text with the option on and checks the same, and that the decorations survive.
A wrapping case, `aaaa H~2~O` becoming `aaaa`⏎`H~2~O`, goes in
`TextDecoration_Subscript_TextWrap_Always.txt` with `lineWidth: 6`.

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
| `~x~`                     | off         | `~~x~~` (as today)           |
| `H~2~O`                   | off         | `H~2~O` (text, as today)     |

The `*~x~*` rows show that emphasis around or inside a subscript is still
written with the configured character (`_` by default).
