# Using LLMs with Sensitive Data — Research for an LLM Data Analyst

*Compiled 2026-09-12. Sources cited as [n]; list at bottom. Context: a desktop data-analyst app (local DuckDB + LLM chat / text-to-SQL) deciding how to handle sensitive customer data.*

## TL;DR

For an **LLM data analyst**, the strongest pattern is architectural, not cryptographic: **the LLM mostly needs the schema and the question, not the rows**. Send schema + question → LLM generates SQL → SQL runs locally (e.g. DuckDB) → only sanitized/aggregated results (if any) go back to the model. Layer PII masking on whatever text/rows do leave, and offer a fully local model path for regulated data. Cloud APIs are acceptable for most enterprise data only under enterprise tiers with no-training + zero-data-retention terms.

## 1. What's actually at risk

- **Memorization / training-data leakage.** LLMs can reproduce training data near-verbatim, especially repeated or distinctive sequences; bigger models memorize more. Fine-tuning on proprietary data is the riskiest variant — your confidential records end up in weights with no clean "delete" [4].
- **Inference-time leakage.** Badly scoped RAG/context surfaces records the user shouldn't see; output filtering after the fact is reactive, not preventive [4].
- **Prompt injection / extraction attacks.** Crafted inputs trick the model into revealing what it was told not to; #1 on OWASP's LLM Top 10 (LLM01:2025), with Sensitive Information Disclosure at #2 [1].
- **Third-party API retention & logging.** Where prompts are stored, for how long, who can read them, whether they train on them — all become compliance questions, not just technical ones [4]. Real incidents: OmniGPT (34M chat messages leaked, Feb 2025), DeepSeek ClickHouse exposure (Jan 2025), ~12k live API keys found in one public training crawl [5].
- **Shadow AI.** ~1 in 12 employee prompts to public models contains confidential info [5].

## 2. Decision 1: where does the model run?

| Option | Privacy posture | Notes |
|---|---|---|
| **Local / on-prem open-weight models** (llama.cpp, Ollama, vLLM + Llama/Qwen/Mistral/Gemma) | Strongest: data never leaves the machine. Under GDPR no processor, no transfer, no DPA needed. Right-to-erasure trivial (nothing persisted externally) [2][6] | Needs local hardware; quality of a local 70B-class model is "competitive with GPT-4-class for most business tasks" [6]. Fine-tuned local models still carry memorization risk [4] |
| **Enterprise cloud tiers / ZDR** | Acceptable for most sensitive enterprise data *with contracts*: OpenAI API zero data retention (no prompts stored after the request, no staff reads, no training without opt-in — reaffirmed Aug 2026) [7][8]; Anthropic ZDR agreements + HIPAA-ready APIs [9][10]; Azure OpenAI / AWS Bedrock / Vertex AI offer no-training + regional processing, ZDR via support request [11][12] | Still a cross-border transfer under GDPR → DPA required; regional endpoints; you remain the data controller [6]. ZDR ≠ nothing ever logged — abuse-detection retention windows still exist [13] |
| **Hybrid routing** | Best of both: a classifier/middleware routes sensitive queries to the local model, general queries to a strong cloud model [1] | Requires reliable sensitivity classification and policy enforcement in the middle layer |

Regulatory reality check [6]: GDPR Art. 44 restricts EU→US transfers; HIPAA requires a BAA (many consumer AI endpoints don't sign one); CNIL/BSI/EU AI Act all push toward on-prem processing of sensitive data.

**Data that should essentially never go to a consumer cloud LLM** [6]: patient records, HR/salary data, customer PII, privileged legal docs, unpublished financials, credentials/API keys.

## 3. The key pattern for a data analyst: schema-only + sandboxed text-to-SQL

This is the pattern that fits an LLM data analyst best, because the model's job (writing SQL) needs structure, not values [1][14]:

1. **Stage 1 — generate:** LLM sees only the question + table/column names + types (optionally a schema replica with synthetic rows), and produces SQL. It never sees real data.
2. **Stage 2 — execute:** the SQL runs against the real database locally, with row limits, read-only connections, and a query allowlist (SELECT only).
3. **Result handling (pick per sensitivity):**
   - Send back only **aggregates** (counts, sums, small summaries) — usually enough for the model to narrate findings;
   - Or anonymize result rows before they enter any prompt (PII masking, see §4), then de-anonymize the final answer locally [1].

Defense-in-depth benefits: even a fully compromised or jailbroken model can't leak what it never saw [1]. Additional hardening: run LLM-generated SQL on a read-only replica/connection; validate outputs; block outbound network from the model's execution context [1].

Cost: you must maintain schema sync and a masking step on results — "for certain high-risk data this pattern greatly reduces exposure" [1].

## 4. PII detection & anonymization layer (for whatever text must leave)

- **Microsoft Presidio** (open source, MIT, runs 100% locally): `analyzer` detects 50+ entity types (names, SSN, cards, emails, phones…) via spaCy NER + regex + context; `anonymizer` offers replace / mask / hash / redact / **encrypt** (reversible) [3]. The **proxy pattern**: detect & scrub locally → send sanitized text → restore real values in the response. Faker-based *reversible anonymization* (LangChain `PresidioReversibleAnonymizer`) produces realistic fakes ("Michael Brown" instead of `<PERSON>`), which yields more natural LLM output and handles the "model rephrased my placeholders" problem better [3].
- **Limitations to know**: context-dependent identifiers ("$8,200/month income" + "blue house on Oak & Main") slip through; custom entity formats need custom recognizers; anonymization ≠ anonymization — EDPB (2025) notes LLMs rarely achieve true anonymization because re-identification from combined attributes is easy [3][4].
- **Commercial options** [3]: Private AI (managed scrub/rehydrate), Protecto (handles restructured responses), John Snow Labs (clinical de-id, 98.6% F1 vs ~0.41 for general tools), Strac (browser DLP).
- **Data minimization is half the battle**: many analyst questions don't need the raw rows at all — send aggregates [1][3].

## 5. Advanced PETs — when masking + local isn't enough

- **Differential privacy (DP):** provable guarantee that any individual's data barely changes the output; used in training/fine-tuning to prevent memorization. Trade-off: noise degrades quality — practical epsilon ≈ 8–10 [4].
- **Federated learning:** train where the data lives, ship only gradients (healthcare, banks). Gradients themselves can leak → combine with DP + secure aggregation [4].
- **Confidential computing / TEEs (Intel SGX, AMD SEV, NVIDIA CC):** encrypted-memory enclaves so even the host/cloud admin can't inspect data during inference — practical today for cloud deployments you don't fully trust [1][4][15].
- **FHE (compute on encrypted data) / SMPC:** strongest theory, heavy overhead; emerging for LLM inference in finance/healthcare [4][15].

Most robust deployments combine two or three of these depending on threat model [4].

## 6. Compliance quick map

- **GDPR** [1][4][6]: you're the controller; cloud AI = processor → DPA mandatory. Data minimization, purpose limitation, DPIA before deploying (Art. 35). Right to erasure (Art. 17) is near-impossible for fine-tuned weights → prefer RAG/schema-only patterns where deletion = delete at source. Art. 22: human review for significant automated decisions.
- **HIPAA** [1][6]: PHI only to providers under a BAA; on-prem is the safe default; de-identified data falls out of scope if properly de-identified (hard).
- **SOC 2 / ISO 27001** [1][5]: treat the LLM + vector stores + logs as new assets; encrypt prompt/response logs; audit trails (log hashes/metadata, not raw prompts).
- **OWASP LLM Top 10** [1]: prompt injection, sensitive info disclosure, insecure output handling (don't blindly execute LLM-generated SQL — constrain it), supply chain (vet model sources).

## 7. Recommended architecture for an LLM data analyst app

1. **Local-first:** embedded DB (DuckDB) + local model (llama.cpp) = zero data egress by default; strong GDPR story, no DPA needed [2][6].
2. **Schema-only prompting** for text-to-SQL: send table/column names, types, sample *synthetic* values; never real rows [1].
3. **Least-privilege execution:** read-only connection, SELECT-only validation, row/time limits, sandboxed schema replica for generation [1].
4. **Aggregate-back results** where possible; PII-mask any raw values that must enter a prompt (Presidio-style, reversible mapping kept locally) [1][3].
5. **Cloud opt-in with guardrails:** if the user connects an API model, route through an anonymization proxy by default; document ZDR/DPA status per provider [3][7][9].
6. **Audit & logging:** log prompts as hashes/metadata, not raw content; keep mapping tables encrypted and local [3][6].

## Sources

1. StartupSoft — *How to Use LLMs with Enterprise and Sensitive Data* (whitepaper: architectures, sandboxed SQL, OWASP, GDPR/HIPAA/SOC2) — https://www.startupsoft.com/llm-sensitive-data-best-practices-guide/
2. LLM Configurator — *On-Premise LLMs vs Cloud APIs for EU / GDPR* — https://llmconfigurator.com/en/compare/local-ai-gdpr-compliance
3. John Oct — *Protecting Sensitive Data When Using AI Tools: Presidio and Alternatives* (proxy pattern, code, comparison) — https://johnoct.com/blog/2026/01/25/presidio-redaction-protecting-sensitive-data-ai-tools/
4. Duality Technologies — *LLMs and Data Privacy* (memorization, DP, FHE/SMPC/TEE/federated, GDPR/EDPB) — https://dualitytech.com/blog/llm-data-privacy/
5. Lasso Security — *LLM Data Privacy: Protecting Enterprise Data* (incidents, RAG/CBAC, frameworks) — https://www.lasso.security/blog/llm-data-privacy
6. LLM Configurator — *Private AI for Business 2026: GDPR & Enterprise Setup* (risk tiers, DPA table, Ollama deployment, checklist) — https://llmconfigurator.com/en/guides/private-ai-business-guide
7. OpenAI — *Offering Zero Data Retention for frontier models* (Aug 2026) — https://openai.com/index/offering-zero-data-retention-for-frontier-models/
8. Enterprise DNA — *OpenAI's ZDR Changes Enterprise AI Privacy* — https://enterprisedna.co/resources/news/openai-zero-data-retention-frontier-models-enterprise-privacy-2026/
9. Anthropic docs — *API and data retention* (ZDR, HIPAA-ready) — https://platform.claude.com/docs/en/api/data-usage
10. Anthropic privacy center — ZDR FAQ — https://privacy.claude.com
11. Microsoft Learn Q&A — enabling ZDR on Azure OpenAI (requires support request) — https://learn.microsoft.com/en-us/answers/questions/4372674/
12. Google — Gemini Enterprise Agent Platform zero data retention — https://docs.cloud.google.com/gemini-enterprise-agent-platform/resources/zero-data-retention
13. The Register — *OpenAI chases Anthropic's biz customers with ZDR pledge* (retention nuances) — https://www.theregister.com/ai-and-ml/2026/08/20/openai-chases-anthropics-biz-customers-with-zero-data-retention-pledge/5290609
14. Spice.ai / Kalvium Labs — text-to-SQL schema-aware generation practices — https://spice.ai/docs/text-to-sql ; https://www.kalviumlabs.ai
15. IACR ePrint 2026/105 — *Privacy-Preserving LLM Inference in Practice* (TEE → crypto-augmented trajectory) — https://eprint.iacr.org/2026/105
