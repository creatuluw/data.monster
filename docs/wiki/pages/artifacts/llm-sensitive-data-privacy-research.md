---
type: Artifact
title: LLM Sensitive Data Privacy Research
description: Research report (15 cited sources) on how to use LLMs with sensitive data in a data-analyst app. Compiled 2026-09-12 from web research; motivated by Data Monste
tags: [privacy, llm, analyst, security, research]
timestamp: "2026-09-11T21:44:25.196Z"
---

# LLM Sensitive Data Privacy Research

Research report (15 cited sources) on how to use LLMs with sensitive data in a data-analyst app. Compiled 2026-09-12 from web research; motivated by Data Monster's Analyst feature, which chats with an LLM (cloud APIs or local llama.cpp models) about data held in local DuckDB.

## What it documents

- The **Analyst** entry in the wiki glossary (`docs/wiki/glossary.md`) — LLM chat over data — is the feature this research informs.
- The recommended architecture for LLM data analysts: **schema-only text-to-SQL** — send schema + question to the LLM, execute generated SQL locally (DuckDB), return only aggregates or PII-masked results. "The model can't leak what it never saw."
- Three deployment tiers by sensitivity: local open-weight models (strongest, no egress/GDPR processor), enterprise cloud tiers with zero-data-retention contracts (OpenAI/Anthropic/Azure/Bedrock/Vertex), and never-consumer-endpoint data classes (patient records, HR/salary, customer PII, credentials).
- PII masking layer for text that must leave: Microsoft Presidio (local detection + replace/hash/encrypt, reversible Faker-based anonymization), with EDPB 2025 caveat that "anonymized" data rarely survives LLM-scale re-identification.
- Advanced privacy-enhancing tech (DP, federated learning, TEEs, FHE) — relevant for training, overkill for inference-time analysts; TEEs are the practical option for untrusted cloud inference.

## Key takeaway for Data Monster

The local-first DuckDB + llama.cpp path is already the gold-standard privacy story. Identified gaps if the Analyst feature is hardened:
1. Schema-only prompting when a cloud API model is selected (never send raw rows).
2. Read-only, SELECT-validated execution of LLM-generated SQL.
3. Aggregate/sanitized results fed back to the model instead of raw rows.

## Details

- **Format**: Markdown report with numbered citations, compiled 2026-09-12.
- **Location**: `docs/research/llm-sensitive-data-privacy.md`
- **Generated from**: web research session (vendor docs, OWASP LLM Top 10, EDPB/CNIL/GDPR guidance, text-to-SQL architecture pieces).
