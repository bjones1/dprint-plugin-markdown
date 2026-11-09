# Heading attributes

A heading may end with an attribute block, as in `# Title {#id .class key="value"}`. The formatter keeps the block together and attached to the heading while it formats the heading text as it already does. CommonMark does not define attribute blocks, so the syntax of the block's contents follows [Python-Markdown's `attr_list`](https://python-markdown.github.io/extensions/attr_list/), with the stricter grammar below. This adds no configuration options and does not check what the attributes mean (for example, duplicate ids are kept).

Everything not described here is unchanged: a heading without a recognized block, including every case below that fails recognition, is formatted exactly as it is today, with its braces treated as ordinary text.

In this section, whitespace means spaces, tabs, and line breaks.

## Recognition

A heading has an attribute block when all of these hold:

1. The heading content ends with a block, ignoring trailing whitespace. The heading content is what CommonMark reads as the heading's text, so an ATX closing sequence after the block (`# Title {#id} ##`) is already gone, while hashes before the block (`# Title ## {#id}`) are heading text. The block starts at the leftmost `{` from which the rest of the content matches the grammar below. When there is no such `{`, the heading has no block, and an earlier `{...}` followed by other text is never used instead.
2. That `{` is not escaped (preceded by an odd number of backslashes) and is not inside any inline element that starts before it, such as a code span, link, emphasis, or inline HTML.
3. Heading text with at least one non-whitespace character comes before the block. Whitespace may separate them, including line breaks of a Setext heading. The text may touch the `{` only when it ends with plain text. A block touching the end of a link, image, code span, emphasis, or other inline element is not recognized, because dialects with inline attributes apply it to that element.

The block grammar:

```
block = "{" [":"] *ws attr *(1*ws attr) *ws "}"
attr  = "#" name / "." name / name "=" value / name
value = DQUOTE *dq-char DQUOTE / "'" *sq-char "'" / 1*char
```

Here `ws` is one whitespace character; `char` is any character except whitespace, `{`, `}`, `=`, `"`, `'`, and `\`; `dq-char` is any character except `"`; `sq-char` is any character except `'`; and `name` is one or more `char`s not starting with `#`, `.`, or `:`.

A quoted value ends at the next quote of the same kind. A backslash inside it has no special meaning, as in Python-Markdown. A quoted value may contain braces, whitespace, and Markdown syntax.

Examples of invalid blocks: `{}`, `{:}`, `{::x}`, `{#}`, `{.}`, `{key=}`, `{a="b}`, `{a=b=c}`, `{x\y}`.

## Output

- The heading text is formatted as it is today. It is followed by exactly one space and the block. The space replaces all whitespace between the text and the block, including any line break, in every `textWrap` mode. Hashes at the end of the text are not escaped, because the block follows them on the line.
- Inside the block, each run of whitespace outside quoted values becomes one space. Inside quoted values, each line break, together with the spaces and tabs around it, becomes one space. Everything else is written exactly as in the source: the optional colon, attribute order, duplicates, other whitespace in quoted values, and any Markdown-like text, which is not escaped or reformatted.
- The block is never broken across lines and never starts a line. The space before it is not a place where a line can break, so where the heading text wraps, the existing wrapping treats the block as part of the text's last word. The final line may exceed `lineWidth`.
- ATX headings stay on one line, as today. Level 1 and 2 headings become Setext only where the formatter already converts them, and the underline is sized with the existing rule, counting the block as part of its line.
- Formatting the output again with the same configuration produces the same output.

Python-Markdown only reads a heading's block after a space, so writing `Title{#id}` as `Title {#id}` makes that block take effect there. This is intended.

## Examples

### ATX

With `heading.kind: atx`:

```markdown
# Title{#id}

# Title {  #id   .note   title="a  {b}"  }

# Title {:  checked  key='x'} ##

# Title ## {#id}

# Title {t="*a*"}

# A   title\{draft}

# A   title {#id} {unfinished

#    {#id}

# Title {}

# `code {#id}`

# [Logo](u){width=10}
```

Output:

```markdown
# Title {#id}

# Title { #id .note title="a  {b}" }

# Title {: checked key='x'}

# Title ## {#id}

# Title {t="*a*"}

# A title\{draft}

# A title {#id} {unfinished

# {#id}

# Title {}

# `code {#id}`

# [Logo](u){width=10}
```

The last six headings have no recognized block: an escaped brace, an unclosed final candidate, no heading text, an empty block, a block inside a code span, and a block touching a link.

### Setext wrapping

With `heading.kind: setext`, `textWrap: always` and `lineWidth: 10`:

```markdown
# Alpha beta gamma{#id}

# Alpha beta{#id}

### Alpha beta gamma{#id}
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

### Setext line breaks

With `heading.kind: setext` and `textWrap: maintain`:

```markdown
猫
{#id
.c}
---
```

Output:

```markdown
猫 {#id .c}
-----------
```

The line breaks become spaces even though `maintain` keeps line breaks. The underline has eleven hyphens because `猫` is two columns wide.

### Lines CommonMark does not read as one heading

With `heading.kind: atx`, a block on the line after an ATX heading is a separate paragraph, because an ATX heading ends at its line. The formatter writes the paragraph after a blank line:

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
