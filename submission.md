## 1. Team contributions

| Team member          | One-sentence contribution summary |
|----------------------|---|
| [Alisher Toxanbayev] | Led the preparation of the heading-attributes specification by coordinating repository analysis, defining the project scope and regression-test strategy, and incorporating feedback from the human reviewer, Codex, and Claude. |


## 2. Individual compile-and-run output

**Required assignment prompt:**

> Verify that you can compile and run the project in the current git repo.

| Team member          | Individual result                           |
|----------------------|---------------------------------------------|
| [Alisher Toxanbayev] | TODO: update after cargo will bew installed |

**Environment note:** In the Codex environment used during the investigation, `command -v cargo` returned no path, so no Rust build or test suite could be run there. This is an environment limitation, not the member's local compile-and-run output.

## 3. Specification review history

| Round | Reviewer | Findings | Summary of changes or result |
|---|---|---:|---|
| 1 | Human reviewer | 5 comments | Requested Setext coverage, a precise heading-attribute definition, correction of an inaccurate Markdown example, less repetition, and a concrete implementation strategy. |
| 2 | Codex self-review, earlier draft | 0 actionable findings reported | Same-context review of an earlier version. Later human/independent reviews identified material issues, so this is retained only as review history, not as proof that the draft was defect-free. |
| 3 | Claude independent review 1 | 6 numbered findings (1 Critical, 5 Major), plus further observations | Identified undefined attribute grammar, the Setext test-row semantic contradiction, missing ATX-to-Setext investigation, ambiguous inline attachment, unclear width semantics, and incomplete submission records. |
| 4 | Codex self-review, revised draft | 3 actionable findings | Flagged a weak regression row, incomplete review reconciliation, and unexecuted formatter/parser tests. The test row was subsequently revised; execution remained outstanding. |
| 5 | Claude independent review 2 | 6 numbered findings (1 Critical, 5 Major), plus further test-matrix observations | Identified over-broad inline-node attachment, reverse promotion of ordinary text into attributes, separator ambiguity, missing recognition-stage decision, semantic canonicalization gaps, and an incomplete review record. |
| 6 | Codex self-review, latest revision | 3 actionable findings | Flagged a duplicate/incorrect regression case, incomplete Claude-review reconciliation, and the absence of executed formatter/parser tests. The regression case was corrected; execution remains outstanding. |

### 3.1 Human reviewer feedback

1. “What about setext headings?”
2. “A heading attribute is additional metadata associated with a Markdown heading.”
3. “This is wrong. Read the spec.”
4. “Repetitive”
5. “This is your job, not the LLMs. You need specific instructions on how to accomplish these goals, which is missing.”

### 3.2 Latest independent review — Claude Round 2

**Reviewer:** Claude. The exact model/version was not included in the supplied review report.

**Finding count:** 6 numbered findings: 5 Major and 1 Critical, plus additional regression-test observations.

**Reasoning effort:** Not reported in the supplied review artifact.

**Prompt record:** The exact text entered into Claude was not preserved in the review artifact. The review was requested using the independent technical-review brief prepared in this conversation. Because the submitted text cannot be independently verified, no reconstructed wording is presented as a verbatim prompt.

**Verbatim review output:** The complete report is retained in the accompanying file [Independent Review Heading Attributes Preservation (Round 2).md](Independent%20Review%20Heading%20Attributes%20Preservation%20%28Round%202%29.md). It should accompany this submission if the instructor requires the full review transcript.

**Main findings:** The final inline-node attachment rule was too restrictive; ordinary text could become an attribute block after reflow; the separator rule conflicted with the Setext separate-line case; the stage for attribute recognition was unspecified; semantic event canonicalization was incomplete; and the review history in `submission.md` needed correction.

### 3.3 Earlier Codex self-review

**Review type:** Same-context self-review; not independent validation.

**Finding count:** 0 actionable findings were reported in that earlier pass. This was an earlier-draft result and did not establish that the specification had no remaining defects.

### 3.4 Latest Codex self-review

**Review type:** Same-context self-review; not independent validation.

**Finding count:** 3 actionable findings were reported. The final self-review identified a regression-row issue, incomplete reconciliation of Claude's report, and the absence of executed tests. The regression-row issue was corrected; the other limitations are disclosed above.

**Prompt used:**

> Review `heading-attributes-2-specification.md` and `submission.md` against the current repository. Check the parser grammar and edge cases against the pinned pulldown-cmark 0.13.4 specification; check source claims about heading parsing, output conversion, wrapping and width; inspect test matrix coverage and arithmetic; and assess whether the semantic test compares the complete document without hiding meaningful differences. Identify remaining actionable inaccuracies, contradictions, omissions, and unsupported execution claims. Do not infer missing Claude materials or user/team identity. Return only findings with concrete corrections.

**Actual output recorded by Codex:**

> 1. Row 15 currently repeats the invalid internally split text from row 8 while describing a valid existing attribute block. Replace it with a complete single-line attribute block so this row tests the converse case.
> 2. Submission review history must not state the Claude round is fully reconciled until the complete report is supplied and checked. The current explicit TODO is appropriate; team member names and individual build outputs also remain required.
> 3. No formatter/parser behavior or proposed fixture output has been executed because `cargo` is unavailable. Keep the row-8 join behavior labeled as an inference and the entire matrix labeled not run.

**Correction recorded:** Row 15 was changed to use the valid single-line block `Heading {#my-id .foo}` and to assert that formatting does not split it.

**Model/version:** Codex reported GPT-6; the exact build identifier was not exposed in the available output.

**Reasoning effort:** Not exposed in the available output.

## 4. Specification and implementation status

The current specification defines a provisional `pulldown-cmark` heading-attributes dialect; distinguishes ATX, Setext, and ATX-to-Setext behavior; documents attribute-block recognition rules; specifies preservation of the complete trailing block and the final inline formatting unit; defines width in Unicode display columns; and proposes whole-document semantic comparison and formatter regression fixtures.

The production feature has not been implemented. The proposed formatter/parser regression cases have not been executed because `cargo` was unavailable in the Codex environment. The `pulldown-cmark` dialect remains provisional pending instructor confirmation. Repository-specific behavior described in the specification should be treated as source-inspected but not execution-verified until the relevant tests are run.

**Specification:** [heading-attributes-2-specification.md](heading-attributes-2-specification.md)