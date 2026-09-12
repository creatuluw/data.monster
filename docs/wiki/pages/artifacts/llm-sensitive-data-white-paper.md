---
type: Artifact
title: LLM & Sensitive Data White Paper
description: Dutch-language white paper ("LLM's & Gevoelige Data") condensing the LLM privacy research into a single self-contained HTML file designed for mobile reading.
tags: [research, llm, privacy, whitepaper, html-doc]
timestamp: "2026-09-11T21:48:13.325Z"
---

# LLM & Sensitive Data White Paper

Dutch-language white paper ("LLM's & Gevoelige Data") condensing the LLM privacy research into a single self-contained HTML file designed for mobile reading.

## What it documents

- Risks and patterns for using LLMs with sensitive data: memorisatie, leakage, prompt-injectie, API-retentie, schaduw-AI
- Deployment trade-offs (lokaal / enterprise cloud / consumer cloud)
- The schema-only text-to-SQL pattern (vraag+schema → LLM → lokale DuckDB → aggregaten → antwoord) as the core privacy pattern
- PII-maskering (Presidio proxy-patroon), PET's, compliance (AVG / HIPAA / SOC 2), and a 6-item checklist
- Condensed from the source research `docs/research/llm-sensitive-data-privacy.md`

## Details

- **Format**: single self-contained HTML file (inline SVG diagram, zero external dependencies, dark-mode aware, print button, offline-capable)
- **Location**: `docs/research/llm-gevoelige-data-whitepaper.html`
- **Constraints**: mobile-first single-column layout, body content ≈1.100 tokens (cap 1.500)
- Built with the html-docs skill; standard template customized to single-column scannable cards for phone reading

## Source

- `docs/research/llm-sensitive-data-privacy.md` — full research this white paper condenses
