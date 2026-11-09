# Heading attributes

A heading may end with an attribute block, as in `# Title {#id .class key="value"}`. The formatter keeps the block together and attached to the heading while it formats the heading text as it already does. CommonMark does not define attribute blocks, so the syntax of the block's contents is based on [Python-Markdown's `attr_list`](https://python-markdown.github.io/extensions/attr_list/), using the grammar below. This adds no configuration options and does not check what the attributes mean (for example, duplicate ids are kept).

Everything not described here is unchanged: a heading without a recognized block, including every case below that fails recognition, is formatted exactly as it is today, with its braces treated as ordinary text.

In this specification, whitespace means spaces (U+0020) and tabs (U+0009).

## Recognition

A heading has an attribute block when all of these hold:

1. The heading content ends with a block, ignoring trailing whitespace. The heading content is what the formatter's parser reads as the heading's text, so an ATX closing sequence after the block (`# Title {#id} ##`) is already gone, while hashes before the block (`# Title ## {#id}`) are heading text. The block starts at the leftmost `{` from which the rest of the content matches the grammar below. When there is no such `{`, the heading has no block, and an earlier `{...}` followed by other text is never used instead.
2. That `{` is not inside any inline element that starts before it, such as a code span, link, emphasis, or inline HTML.
3. Heading text with at least one non-whitespace character comes before the block, separated from it by one or more spaces, and the whole block, from `{` to `}`, is on the line where that text ends. A block that starts or continues on another line of a Setext heading is not recognized, because joining the lines could change what the text before it reads as (a `+` line would become a list item). A block touching the text is not recognized either: Python-Markdown requires the space, and dialects with inline attributes apply a touching block to the element before it, such as a link or a `[span]`.

The block grammar:

```
block = "{" [":"] *SP attr *(1*SP attr) *SP "}"
attr  = "#" name / "." name / name "=" value / name
value = DQUOTE *dq-char DQUOTE / "'" *sq-char "'" / 1*char
```

Here `SP` is one space; `char` is any character except whitespace, `{`, `}`, `=`, `"`, `'`, and `\`; `dq-char` is any character except `"`; `sq-char` is any character except `'`; and `name` is one or more `char`s not starting with `#`, `.`, or `:`.

A quoted value ends at the next quote of the same kind. A backslash inside it does not escape the quote. A quoted value may contain braces, whitespace, and Markdown syntax. Outside quoted values, a tab makes the block invalid, and so does a tab between the text and the block, since the formatter keeps tabs in heading text today.

Examples of invalid blocks: `{}`, `{:}`, `{::x}`, `{#}`, `{.}`, `{key=}`, `{a="b}`, `{a=b=c}`, `{x\y}`.

## Output

- The heading text is formatted as it is today. It is followed by exactly one space and the block. The space replaces the spaces between the text and the block. Hashes at the end of the text are not escaped, because the block follows them on the line. Decisions today's formatter makes from the whole heading content, such as how to write emphasis so that it reads back the same, are made on the text as written with the block. Where those decisions fall back to writing the heading content exactly as in the file, the block is written unchanged with it.
- Inside the block, each run of spaces outside quoted values becomes one space, unless it is inside an inline element that starts within the block, such as a code span, emphasis, or inline HTML. Everything else in the block is written exactly as in the source: the optional colon, attribute order, duplicates, whitespace in quoted values, and any Markdown-like text, which is not escaped or reformatted.
- The block is never broken across lines and never starts a line. The space before it is not a place where a line can break, so where the heading text wraps, the existing wrapping treats the block as part of the text's last word. The final line may exceed `lineWidth`.
- ATX headings stay on one line, as today. Level 1 and 2 headings become Setext only where the formatter already converts them, with its existing check made on the heading content including the block, and the underline is sized with the existing rule, counting the block as part of its line.
- Formatting the output again with the same configuration produces the same output, except where today's formatting of the heading text is already unstable.

## Examples

### ATX

With `heading.kind: atx`:

```markdown
# Title {  #id   .note   title="a  {b}"  }

# Title {:  checked  key='x'} ##

# Title ## {#id}

# Title {t="*a*"}

# A   title \{draft}

# A   title {#id} {unfinished

#    {#id}

# Title {}

# Title `{#id` }

# Title{#id}
```

Output:

```markdown
# Title { #id .note title="a  {b}" }

# Title {: checked key='x'}

# Title ## {#id}

# Title {t="*a*"}

# A title \{draft}

# A title {#id} {unfinished

# {#id}

# Title {}

# Title `{#id` }

# Title{#id}
```

The last six headings have no recognized block: a brace touching the backslash that escapes it, content that does not end with `}`, no heading text, an empty block, a block whose `{` is inside a code span, and a block touching the text.

### Setext wrapping

With `heading.kind: setext`, `textWrap: always` and `lineWidth: 10`:

```markdown
# Alpha beta gamma {#id}

# Alpha beta {#id}

### Alpha beta gamma {#id}
```

Output:

```markdown
Alpha beta
gamma {#id}
===========

Alpha
beta {#id}
==========

### Alpha beta gamma {#id}
```

`beta {#id}` does not fit after `Alpha`, so it moves to the next line as a unit. Level 3 stays ATX on one line.

### Setext underline and a block on its own line

With `heading.kind: setext` and `textWrap: maintain`:

```markdown
猫   {#id   .c}
---

Title
{#id}
---
```

Output:

```markdown
猫 {#id .c}
-----------

Title
{#id}
-----
```

The first underline has eleven hyphens because `猫` is two columns wide. The second heading's block is on a line of its own, so it is not recognized and `maintain` keeps the line break.

### Lines CommonMark does not read as one heading

With `heading.kind: atx`, a block on the line after an ATX heading is not part of the heading, because an ATX heading ends at its line. Here it is a paragraph, which the formatter writes after a blank line:

```markdown
### My Heading
{#my-ID}
```

Output:

```markdown
### My Heading

{#my-ID}
```

`###My Heading{#my-ID}` is a paragraph, not a heading, because no space follows the hashes, so it is left as it is.
