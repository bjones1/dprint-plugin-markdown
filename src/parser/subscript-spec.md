<h1 id="cc--subscript-spec">Subscript support</h1>

Add subscript (`H~2~O` renders as H<sub>2</sub>O) to this plugin's own parser and
formatter, reimplementing the semantics of the
[pulldown-cmark subscript spec](https://github.com/pulldown-cmark/pulldown-cmark/blob/1c1a3123207d9e07fdfcf3d3418a047907185afb/pulldown-cmark/specs/super_sub.txt)
(`ENABLE_SUBSCRIPT`) at commit `1c1a312`. Released pulldown-cmark 0.13.4
differs: there `H~2~O` is not subscript, since the delimiters need flanking.
The parser expectations below were run against that commit and rules 4 and 5
read from its source; the formatter expectations are targets.

Problem
-------

[`parse_emphasis_delimiter`](inline.rs) already reads a run of one or two `~` as
[`TextDecorationKind::Strikethrough`](ast.rs), as GFM does. With
`ENABLE_SUBSCRIPT`, pulldown-cmark reads a single-`~` pair as `<sub>` and keeps
`~~x~~` as strikethrough. The two readings of `~x~` conflict, so subscript is
opt-in; with the option off, behavior is exactly today's, including writing a
single-`~` strikethrough with two (`~a~` becomes `~~a~~`, see
`tests/specs/TextDecoration/TextDecoration_All.txt`).

Configuration
-------------

1. Add the boolean field `subscript` (default `false`) to `Configuration` in
   [`types.rs`](../configuration/types.rs); parsing and formatting read it. The
   field is internal until step 2.
2. Once open question 1 is settled (a shared flag would also change the table
   above), add the key to the resolver, the JSON
   schema and the README. The README entry warns that turning the option on
   changes how an existing `~x~` is read.

| `subscript` | `~x~`         | `~~x~~`       |
| ----------- | ------------- | ------------- |
| `false`     | strikethrough | strikethrough |
| `true`      | subscript     | strikethrough |

Parsing
-------

These rules apply only with `subscript` on; with it off, `parse_emphasis_delimiter`
is unchanged.

1. Add `TextDecorationKind::Subscript`, produced only when `subscript` is
   `true`.
2. A run of one or two `~` is a delimiter; a longer run is text. At the start of
   a line, after up to three spaces or a container marker such as `>`, a run of
   three or more is a code fence instead.
3. An opener and a closer pair only when their runs have the same length.
4. A closer pairs with the nearest earlier opener of the same length, skipping
   openers of another length. Once a pair wraps its content, the openers
   strictly between its delimiters are dropped and become text; openers
   outside the pair stay available. The pair of a length-1 run is subscript, of
   a length-2 run strikethrough. A closer that finds no opener of its
   length makes every opener before it unavailable, though it can still open
   itself: `~a~~b~` is text, but in `~a ~~b~` the `~~` can only open, so it is
   `<sub>a ~~b</sub>`. (This is `find_match` in pulldown-cmark's `parse.rs`.)
5. A subscript delimiter may sit within a word (`H~2~O`), but an opener must be
   followed, and a closer preceded, by a character that isn't Unicode
   whitespace; a line start or end counts as whitespace. So `a ~ b ~ c` is
   text. A length-1 opener followed by punctuation opens only after whitespace,
   punctuation or a line start (`a~(b)~` is text, `(~x~)` is subscript); a
   length-1 closer needs nothing more. A run that can both close and open
   closes first, which is why `~a~b~` is `<sub>a</sub>b~`. (This is
   `delim_run_can_open` and `delim_run_can_close` in pulldown-cmark's
   `firstpass.rs`.) A `~~` run uses CommonMark flanking and may sit within a word (`flanking` in
   [`inline.rs`](inline.rs)).
6. `\~` is a literal tilde, never a delimiter; `\\~` is a literal backslash then
   a delimiter.
7. Pairs nest, so `~a ~b~ c~` is a subscript within a subscript. A closer left
   without an opener is text.
8. Subscript content may hold emphasis, strong, code spans and links, and
   subscript may sit within those. A pair may enclose a whole link but can't
   start inside one and end outside it, and can't cross a block boundary. A code span's content, a link destination, an autolink and
   an inline HTML tag are never scanned for delimiters.
9. Content may span a soft line break.
10. Subscript and emphasis never partially overlap. Where their delimiters
    interleave, the pair whose closer comes first takes its content, and the
    other delimiter is text: `~a *b~ c*` is `<sub>a *b</sub> c*`, and
    `*a ~b* c~` is `<em>a ~b</em> c~`. A pair that falls wholly within
    another nests, in either order (rule 8). Measured by running commit
    `1c1a312`.

Formatting
----------

1. Subscript is written with a single `~`.
2. A literal `~` is written `\~` if, bare, it would change the tree when the
   output is parsed again (decided by re-parsing the candidate output and
   comparing trees); otherwise it is left as is (`a ~ b ~ c`).
3. Text wrapping breaks a line only at whitespace, so a delimiter touching
   non-whitespace is never split from it and `H~2~O` never becomes `H~2` and
   `~O`. A space inside the content (`~a b~`) is a break point. A break is
   refused, not escaped, where the next line would begin with a run of three or
   more `~`.
4. Formatting is idempotent: the output parses to the same tree.
5. With `subscript` off, rule 1 doesn't apply. Rules 2 to 4 still apply, and
   rule 2 escapes only to keep the tree, so documents that are already
   formatted don't change.

Test cases
----------

Parser, with `subscript` on; "text" means no tag.

| Input                       | Expected HTML                        | Rules |
| --------------------------- | ------------------------------------ | ----- |
| `~sub~`                     | `<sub>sub</sub>`                     | 4     |
| `H~2~O`                     | `H<sub>2</sub>O`                     | 5     |
| `~~gone~~`                  | `<del>gone</del>`                    | 4     |
| `~a b~`                     | `<sub>a b</sub>`                     | 5     |
| `a ~ b ~ c`                 | text                                 | 5     |
| `~ ~`                       | text                                 | 5     |
| `~`U+00A0`x~`               | text                                 | 5     |
| `~~`                        | text                                 | 4     |
| `~a~~b~`                    | text                                 | 4     |
| `~a ~~b~`                   | `<sub>a ~~b</sub>`                   | 4     |
| `a~(b)~`                    | text                                 | 5     |
| `~"x"~`                     | `<sub>"x"</sub>`                     | 5     |
| `~a~~b~~c`                  | `~a<del>b</del>c`                    | 3, 4  |
| `~a [l](http://x/~u/~b) ~c~` | `~a <a href="http://x/~u/~b">l</a> <sub>c</sub>` | 8 |
| `<http://x/~u/~b>`          | an autolink, `~` kept                | 8     |
| `a~“b”~`                    | text                                 | 5     |
| `~foo~~`                    | text                                 | 3     |
| `~~foo~`                    | text                                 | 3     |
| `a ~~~ b`                   | text                                 | 2     |
| `a~~~b~~~c`                 | text                                 | 2     |
| `~~~`⏎`x`⏎`~~~`             | `<pre><code>x</code></pre>`          | 2     |
| `~This is \~sub~`           | `<sub>This is ~sub</sub>`            | 6     |
| `a\~~b~`                    | `a~<sub>b</sub>`                     | 6     |
| `\\~x~`                     | `\<sub>x</sub>`                      | 6     |
| `~This ~~is not stricken.~` | `<sub>This ~~is not stricken.</sub>` | 3, 4  |
| `~~This ~is~~ stricken.~`   | `<del>This ~is</del> stricken.~`     | 3, 4  |
| `~a ~b~ c~`                 | `<sub>a <sub>b</sub> c</sub>`        | 7     |
| `~a~b~`                     | `<sub>a</sub>b~`                     | 5     |
| `` ~`a~b`~ ``               | `<sub><code>a~b</code></sub>`        | 8     |
| `~*em*~`                    | `<sub><em>em</em></sub>`             | 8     |
| `*~x~*`                     | `<em><sub>x</sub></em>`              | 8     |
| `**a ~b~ c**`               | `<strong>a <sub>b</sub> c</strong>`  | 8     |
| `~[l](u)~`                  | `<sub><a href="u">l</a></sub>`       | 8     |
| `[~x~](u)`                  | `<a href="u"><sub>x</sub></a>`       | 8     |
| `~[a~](u)`                  | `~<a href="u">a~</a>`                | 8     |
| `~a`⏎⏎`b~`                  | `<p>~a</p>` and `<p>b~</p>`          | 8     |
| `# H~2~O`                   | `<h1>H<sub>2</sub>O</h1>`            | 5     |
| `H~2~O`⏎`-----`             | `<h2>H<sub>2</sub>O</h2>`            | 5     |
| `- H~2~O`                   | `<li>H<sub>2</sub>O</li>`            | 5     |
| `> H~2~O`                   | `<blockquote>` holding `H<sub>2</sub>O` | 5  |
| `\| a \|`⏎`\|---\|`⏎`\| H~2~O \|` | `<td>H<sub>2</sub>O</td>`      | 5     |
| `~a`⏎`b~`                   | `<sub>a`⏎`b</sub>`                   | 9     |
| `~a\`⏎`b~`                  | `<sub>a<br />`⏎`b</sub>`             | 9     |
| `> ~~~`⏎`> x`⏎`> ~~~`       | `<blockquote>` holding a code block  | 2     |
| `~a\~`                      | `~a~`                                | 6     |
| `` `\~` ``                  | `<code>\~</code>`                    | 8     |
| `a \~b~ c`                  | `a ~b~ c`                            | 6     |
| `~a~ ~b~`                   | `<sub>a</sub> <sub>b</sub>`          | 4     |
| `# H~2~O #`                 | `<h1>H<sub>2</sub>O</h1>`            | 5     |
| `![H~2~O](u)`               | `<img src="u" alt="H2O" />`          | 8     |
| `NH~4~^+^`                  | `NH<sub>4</sub>^+^`                  | 5     |
| `(~x~)`                     | `(<sub>x</sub>)`                     | 5     |
| `a~~x~~b`                   | `a<del>x</del>b`                     | 4     |
| `~a *b~ c*`                 | `<sub>a *b</sub> c*`                 | 10    |
| `*a ~b* c~`                 | `<em>a ~b</em> c~`                   | 10    |
| `**a ~b** c~`               | `<strong>a ~b</strong> c~`           | 10    |
| `~a **b~ c**`               | `<sub>a **b</sub> c**`               | 10    |
| `~*a~*`                     | `<sub>*a</sub>*`                     | 10    |
| `*~a*~`                     | `<em>~a</em>~`                       | 10    |
| `~a *b* c~`                 | `<sub>a <em>b</em> c</sub>`          | 8, 10 |
| `*a ~b~ c*`                 | `<em>a <sub>b</sub> c</em>`          | 8, 10 |

The `NH~4~^+^` row changes to `<sup>` once superscript lands.

With `subscript` off: `H~2~O` and `a~x~b` are text, `~x~` is `<del>x</del>`,
and `a~~x~~b` is `a<del>x</del>b`.

Formatter; the output column is what the formatter must write.

| Input                       | `subscript` | Output                           |
| --------------------------- | ----------- | -------------------------------- |
| `H~2~O`                     | on          | `H~2~O`                          |
| `~x~`                       | on          | `~x~`                            |
| `~~x~~`                     | on          | `~~x~~`                          |
| `~~x~~`                     | off         | `~~x~~`                          |
| `~x~`                       | off         | `~~x~~` (as today)               |
| `a\~~b~`                    | on          | `a\~~b~`                         |
| `\\~x~`                     | on          | `\\~x~`                          |
| `a ~ b ~ c`                 | on          | `a ~ b ~ c`                      |
| `a \~b~ c`                  | on          | `a \~b~ c`                       |
| `~a ~b~ c~`                 | on          | `~a ~b~ c~`                      |
| `H~2~O`                     | off         | `H~2~O` (text, as before)        |
| `~a\~`                      | on          | `~a\~`                           |
| `~a\`⏎`b~`                  | on          | `~a\`⏎`b~`                       |
| `*~x~*`                     | on          | `*~x~*`                          |
| `~a`⏎`b~`                   | on          | `~a`⏎`b~`                        |
| `aaaa H~2~O`, `lineWidth` 6 | on          | `aaaa`⏎`H~2~O`                   |

Every case is formatted twice and must give identical output. Add them to
[`text_fuzz.rs`](text_fuzz.rs), which runs with `subscript` both on and off.

Open questions
--------------

1. Should `subscript` instead be one flag shared with superscript? Settle this
   with the superscript team before configuration step 2, since renaming the key
   later breaks configurations.
2. Settled: subscript and emphasis resolve by whichever pair closes first, as
   parsing rule 10 now states. The eight rows citing rule 10 were measured by
   building commit `1c1a312` and rendering each case with `ENABLE_SUBSCRIPT`
   and `ENABLE_STRIKETHROUGH`. Formatter rows for these cases are still to be
   written, including whether a literal `*` beside a subscript delimiter needs
   escaping, since rule 10 changes how it pairs.
