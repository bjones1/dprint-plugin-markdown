### dprint heading attributes

**General Syntax & Grammer:

* dprint heading attributes look like this ###Heading{#custom-ID}, everything inside the curly brackets includes the attributes for dprint

- whenever you see opening tag, be sure to put the tag directly at the end of each statement
- An opening tag is always the opening bracket `{` and the closing tag is the closing bracket `}`
- The actual heading is after the there `###` characters.
- exclude any whitespace at the end of the closing curly brackets. When the whitespace includes a newline, format the closing tag on the same line.

* Supported:

  - {#ID}
  - {.class}
  - {#ID.class}
  - {#ID1.class1, #ID2.class2, ..., #IDN.classN} (this means heading can have multiple attributes)
  - {key=value}
  - {}

**Formatting & Whitespace Rules:

* **Tag Placement:** The attribute block must be placed directly at the end of the heading statement without any spaces before the opening `{`.

**No Trailing Whitespace:** Exclude any whitespace immediately preceding or at the end of the closing curly bracket `}`.

**No leading Whitespace:** When heading call is made, remove any trailing whitespace from the heading markers. Each heading marker requires three `#` characters, so if there is a whitespace after the third one followed by the heading name, remove the whitespace while preserving the heading title.

**Newline Handling:** When a newline occurs within or immediately before the attribute block, the attribute block is merged onto the same line as heading title.

**attribute format:** include at least 1 space after each attribute by default to avoid the heading from looking cluttered.

**Heading Format:* Both `###Heading` and `### Heading` are valid inputs, just be sure in the latter case, you remove any leading whitespace that is before the Heading title

### Test Cases & Examples

** 1. Standard Heading with Attribute**

* **Input:** `### My Heading{#my-ID}`
* **Expected Output** `###My Heading{#my-ID}`

**2. Malformed Attribute Spacing**

* **Input** `### My Heading {my-ID}`
* **Expected Output:** `###My Heading{#my-ID}`

**3. Multiline / Newline Handling**

* **Input**

```markdown

### My Heading

{#my-ID}

* **Expected Output** `###My Heading{#my-ID}`
 
** 4. Standard Heading with no Attributes**

* **Input:** `### My Heading{}`

* **Expected Output** `###My Heading{}`
 
** 5. Heading with multiple attributes**

* **Input** ### My Heading{#ID1.class1,#ID2.class2}

* **Expected Output** ###My Heading{#ID1.class1, #ID2.class2}
 
** 6. Whitespace within attribute block

* **Input:** ###My Heading{#ID1.class1,#ID2.class2}

* **Expected Output:** ###My Heading{#ID1.class1, #ID2.class2}
 
** 7. Leading Whitespace at the header

* **Input:** ### My Heading{#ID1.class1, #ID2.class2}

* **Expected Output:** ###My Heading{#ID1.class1, #ID2.class2}
```
