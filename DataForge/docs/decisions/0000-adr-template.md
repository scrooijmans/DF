# ADR Template (MADR 4.0)

Use this template to create new Architecture Decision Records.

Copy this file to `NNNN-title-with-dashes.md` where NNNN is the next sequential number.

---

# [short title of solved problem and solution]

- **Status**: [proposed | accepted | deprecated | superseded by [ADR-NNNN](NNNN-filename.md)]
- **Date**: [YYYY-MM-DD when decision was last updated]
- **Decision-makers**: [list everyone involved in the decision]
- **Consulted**: [list those whose opinions were sought (typically domain experts)]
- **Informed**: [list those kept up-to-date on progress (typically stakeholders)]

## Context and Problem Statement

[Describe the context and problem statement in 2-3 sentences or as a question.]

## Decision Drivers

- [Driver 1: e.g., force, facing concern, ...]
- [Driver 2: e.g., force, facing concern, ...]
- ...

## Considered Options

1. [Option 1]
2. [Option 2]
3. [Option 3]
- ...

## Decision Outcome

**Chosen option**: "[Option N]", because [justification. e.g., only option that meets decision driver 1 | best addresses driver 2 | comes out best in comparison, see below].

### Consequences

**Positive:**
- [e.g., improvement of attribute, follows pattern, ...]
- ...

**Negative:**
- [e.g., compromise on attribute, introduces risk, ...]
- ...

## Confirmation

[Describe how the implementation will be verified to confirm the decision was correctly realized. E.g., code review, automated tests, design review checklist.]

## Pros and Cons of Options

### [Option 1]

[Description of option 1]

- **Good**: [argument]
- **Good**: [argument]
- **Bad**: [argument]
- **Bad**: [argument]

### [Option 2]

[Description of option 2]

- **Good**: [argument]
- **Bad**: [argument]
- **Neutral**: [argument]

### [Option 3]

[Description of option 3]

- **Good**: [argument]
- **Bad**: [argument]

## More Information

[Any additional context, links to related ADRs, implementation notes, or timeline.]

---

## Template Notes

Based on [MADR 4.0.0](https://adr.github.io/madr/) (September 2024).

**Required sections**: Context, Considered Options, Decision Outcome
**Optional sections**: Decision Drivers, Consequences, Confirmation, Pros/Cons, More Information

For simple decisions, use only the required sections. For significant architectural choices, include all sections.
