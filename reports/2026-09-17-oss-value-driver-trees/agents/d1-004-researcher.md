# Q: Which open-source financial-modeling / FP&A tools implement driver-based modeling or value-driver decomposition with data?

*Method note: no built-in websearch/websearch_deep tool in this session — degraded to TinyFish Search + Fetch (browser-rendered extraction) with multiple query variants; noted per finding.*

## Findings

1. **FinanceToolkit** (JerBouma) — github.com/JerBouma/FinanceToolkit — MIT (repo; PyPI page metadata not captured) — Python/pandas. `companies.models.get_extended_dupont_analysis()` computes the 5-factor Extended DuPont (interest burden, tax burden, margin, turnover, leverage) **from inputted balance sheet / income / cash-flow statements**. 5.4k★, 1,356 commits, pip-installable. **Verdict: the only mature OSS DuPont-decomposition engine tied to data.** Confidence: verified [source: https://github.com/JerBouma/FinanceToolkit]
2. **revenue-model-builder** (ljftwq-dev) — github.com/ljftwq-dev/revenue-model-builder — license not shown in fetched text — pure-stdlib Python. The most literal **driver tree**: `segment_revenue = market_base × penetration × share × price`, A/B/C credibility grades per driver, first-class residual, Monte Carlo + tornado sensitivity, pre-registered out-of-sample validation on 244 S&P 500 constituents. 7★, 92 commits. **Verdict: genuine value-driver-tree engine, single-author, near-zero adoption.** Confidence: verified [source: https://github.com/ljftwq-dev/revenue-model-builder]
3. **Modeleon** (modeleonai) — github.com/modeleonai/modeleon — Apache 2.0 — Python 3.12+ (`pip install modeleon`). `Variable` arithmetic tracks dependencies automatically; `MultiVariable` models concepts as driver hierarchies; compiles to **live Excel formulas** (`=B1*B2`) with per-cell audit trail and cross-cell dependency graph. 47★, 26 commits. **Verdict: formula-tree financial-model engineering — structural fit for driver trees, but no built-in FP&A semantics; early.** Confidence: verified [source: https://github.com/modeleonai/modeleon]
4. **openfpa** (JeffBrines / Guiderail) — github.com/JeffBrines/openfpa — MIT — Python. Agent-native FP&A workbench: tested finance kernel `pyfpa` (revenue, COGS, OpEx, working capital, debt, tax, cash flow, reconciliation), durable company memory in `.fpa/`, champion/challenger forecast loop scored against actuals. Driver-based in that the agent builds company-specific driver models — **not a fixed driver-tree/VDT application** ("It is not a universal financial model"). 5★, 2026, new. Confidence: verified [source: https://github.com/JeffBrines/openfpa]
5. **finance-bp-dashboard** (courage729) — github.com/courage729/finance-bp-dashboard — license not shown in fetched text — Python. Chinese management-analysis system: Excel statements in → single-file HTML dashboard with **DuPont analysis (杜邦分析)**, solvency decomposition, profit-center matrix, NPV/IRR sensitivity; "analysis logic fixed in code, not in models". 3★, 2 commits. Confidence: verified [source: https://github.com/courage729/finance-bp-dashboard]
6. **fpa-forecasting-scenario-engine** (Shikhar253) — github.com/Shikhar253/fpa-forecasting-scenario-engine — Python, driver-based revenue modeling + P&L projections + variance backtesting. 1★. Confidence: reported [source: https://github.com/topics/fpa]
7. **Negative results**: Grist/Baserow are generic spreadsheet-DB **formula engines** — no shipped driver decomposition (no FP&A driver-tree artifacts surfaced); Odoo = accounting/spreadsheet, no driver trees; **'pydunder' does not exist** (only Dunder Data pandas tutorials); no OSS FP&A driver-tree tool named 'Foresight' found; interactive VDT/what-if engines exist only in commercial EPM (SAP, Jedox, Anaplan, Abacum). Confidence: verified-as-absent (search variants below)

## Tensions

- Empty at the mature end: the largest real engine (FinanceToolkit) does statement-level DuPont, **not** interactive driver trees; the only literal driver-tree engine (revenue-model-builder) has 7★. Confirms an OSS gap where commercial EPM owns interactive VDT.
- Modeleon has the right *structure* (dependency-tracked formula trees) but zero FP&A domain content; openfpa has FP&A content but delegates tree structure to an AI agent rather than encoding it.

## Open questions

- Licenses of revenue-model-builder and finance-bp-dashboard unverified (not in fetched README text).
- Might Chinese/German-language communities have more dashboard-style DuPont tools (this one was found via an English query on a Chinese repo)?
- Does anything exist for **interactive** what-if VDT visualization (SAP-style) in OSS? Nothing found.

## Search trail

TinyFish search: "open source driver based planning FP&A" · "open source FP&A driver tree" · "DuPont analysis open source github python" · "value driver tree python github" · "github driver-based financial planning model open source" · "open source FP&A software github self-hosted planning" (personal-finance noise) · "\"OpenFP&A\" OR \"open source FP&A\" github Jedox Anaplan alternative" · "pypi dupont analysis package" · "pydunder python dupont library" (nonexistent) · "Grist driver based financial model FP&A template" (generic hits).
TinyFish fetch (deep): github.com/topics/fpa · github.com/topics/driver-based · repos: JerBouma/FinanceToolkit · JeffBrines/openfpa (page + raw README) · modeleonai/modeleon (page + raw README) · courage729/finance-bp-dashboard · ljftwq-dev/revenue-model-builder · pypi.org/project/financetoolkit.
