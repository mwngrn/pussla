# Design Choice: Creative Aliases for User Identification (Pussla-Alias)

## Status
**Accepted**

## Context
In a "Planning-as-Code" system like **Pussla**, users are typically represented by unique identifiers. While numeric IDs (e.g., `u891`) are functional, they are sterile and reduce engagement. We want to introduce creative aliases (e.g., `FishCatcher`, `mrBrown`) to humanize the process while maintaining strict privacy standards.

## Decision
We will implement a **Creative Alias System**. 
* Every user will select a unique "Pussla-alias" that serves as their public-facing identifier within the repository.
* The mapping between the alias and the real identity remains strictly confined to the protected `/identity/` directory.
* The `/planning/people/`, `/planning/roles/`, and `/planning/projects/` directories will exclusively use these aliases.

---

## Rationale

### 1. Human-Centric Design & Engagement
By allowing 100+ colleagues to choose a persona, we transform a bureaucratic resource planning task into a more engaging, gamified experience. It fosters company culture and makes the planning files more relatable.

### 2. Maintaining Privacy by Design (Pseudonymization)
Under GDPR, creative aliases function as **pseudonyms**. As long as the alias does not contain PII (Personally Identifiable Information), the data remains safe for AI analysis. An AI model can analyze "FishCatcher's" workload without ever knowing it refers to "Erik Andersson."

### 3. Separation of Concerns
This approach reinforces the separation between "Who someone is" (Identity) and "What someone does" (Allocation). It allows for a "need-to-know" access model where managers see the full picture, while general AI analysis only sees the aliases.

---

## Guidelines & Constraints

### 1. The "Anti-Doxxing" Rule
Users are encouraged to be creative but must avoid aliases that make them easily identifiable to external parties or automated systems (e.g., using their full name or highly specific job titles like `TheOnlyAccountantInStockholm`).

### 2. Consistency
Once an alias is chosen, it should remain consistent over time to ensure that historical data and AI-driven trend analysis (e.g., "How has FishCatcher's workload evolved over four quarters?") remain accurate.

### 3. Re-identification Flow
The Pussla GUI will perform a local "join" operation between the allocation data and the identity data. Authorized users will see the real names, while others (or AI services) see only the aliases.

---

## Consequences
* **Identity Management:** The `/identity/` folder becomes the critical "Source of Truth" for re-identification.
* **Cultural Fun:** The system becomes a conversation starter, increasing the likelihood of users keeping their data up to date.
* **Risk of Over-identification:** Internal colleagues might still guess who is who based on project context, but the technical privacy layer remains robust against external data leaks or AI training risks.
