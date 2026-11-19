# Specification Review - Round 1

**Date**: 2026-11-09
**Reviewer**: Claude 4 Sonnet (code-review skill, high effort)
**Document**: indented-code-tabs-spec.md

## Findings: 5 CONFIRMED issues

### 1. Contradictory list behavior examples (HIGH severity)
**Location**: Lines 119-433 (multiple sections)
**Issue**: Three different examples showed contradictory outputs for code blocks in lists:
- Example 1: `\t   code` (tab + 3 spaces)
- Example 2: `   \tcode` (3 spaces + tab)
- Example 3: `  \tcode` (2 spaces + tab)

**Resolution**: Removed all contradictory examples and meta-commentary. Replaced with single clear example showing 2 spaces (list continuation) + 1 tab (code block indent).

### 2. Unresolved open questions (HIGH severity)
**Location**: Lines 448-454
**Issue**: Three open questions remained unresolved:
1. Should tab replace 4-column code indent in lists?
2. Should input tabs be preserved or normalized?
3. Should docs warn about compatibility?

**Resolution**: Removed "Open Questions" section. Added "Design Decisions" section with resolved answers:
1. Yes, tab replaces 4-column code indent (list indent remains spaces)
2. Always normalize to configured style
3. Added compatibility note in Design Decisions

### 3. Meta-commentary left in specification (MEDIUM severity)
**Location**: Lines 365, 396, 400, 407, 433
**Issue**: Unprofessional phrases remained in spec:
- "Actually, on review:"
- "Wait, this is getting complex"
- "Let me revise this section"
- "## Revised:" (indicating prior section was wrong)

**Resolution**: Removed all meta-commentary and revision markers. Presented single clean version of each section.

### 4. "Top-level" term undefined (MEDIUM severity)
**Location**: Line 460 (Success Criteria)
**Issue**: Success criteria used "top-level indented code blocks" without defining the term.

**Resolution**: Added "Terminology" section defining:
- Top-level: code blocks at document/blockquote/list level (not nested containers)
- Clarified there are no "nested" indented code blocks (content is literal)

### 5. Hypothetical implementation code (MEDIUM severity)
**Location**: Lines 206-213
**Issue**: Suggested using `TAB_INDENT_MARKER` constant without verifying it exists in codebase.

**Resolution**: Replaced hypothetical code with discussion of implementation approaches. Noted that `indent_level` is a space count and suggested examining `with_indent_times` to find appropriate modification point.

## Changes Summary

**Major changes:**
- Removed 90+ lines of contradictory examples and meta-commentary
- Added "Terminology" section (12 lines)
- Added "Design Decisions" section (11 lines)
- Removed "Open Questions" section
- Rewrote "Code Blocks in Lists" to single consistent example
- Improved "Implementation Notes" to avoid hypothetical code

**Lines changed**: ~110 lines modified/removed, ~25 lines added

## Remaining Concerns

None identified. Specification should be ready for implementation after this round.
