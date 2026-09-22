# d0-001 — Orchestrator synthesis: Why is aisure.uk so cheap?

**Question**: Why is https://aisure.uk/ ("every AI tool, one platform, free to start, £9.99/mo unlimited") so cheap?

## Direct answer

Because the advertised promise cannot cost what it charges — and the gap is closed by mechanisms that are either invisible, subsidized, or unverifiable. A heavy user of the frontier models AISure names (GPT-5, GPT-5.6, Gemini 3 Pro, Claude) costs **$70–300+/month at list API prices**; Pro brings in **£9.99 (≈$13.50)** with "no daily caps" and **no fair-use clause anywhere**. Every sustainable comparator proves that price point requires aggressive metering or supply tricks: Poe meters compute points, Merlin's $19 "unlimited" hides a ~$100/month cost cap with account pauses, and providers themselves throttle rather than bill. AISure promises more than any of them for less — so the cheapness is one or more of: invisible server-side throttling, cheaper models served under premium labels, grey-market model supply (70–98% off list exists at scale), ad/gamification revenue on the free tier, and a growth-subsidized bet. Layered under that: a months-old domain with hidden WHOIS, zero organic reviews, and trust scores of 0/100 (Scamadviser) and 35/100 (Gridinsoft) — red flags, not proof of fraud (the UK company is real and Stripe is merchant of record).

## What it is (leaf d1-001 — verified from the site's own JS bundles)

- All-in-one AI workspace (chat, image, video, voice, code, research) on a Lovable-built SPA + Supabase EU, Stripe payments; contracting entity **Noahsure Group LTD**, 1 Canada Square, Canary Wharf.
- Free tier: real, server-side enforced **daily caps per tool** (`free_daily_limit`, reset timer) + **ads** (Google Ads, Meta, TikTok pixels) + heavy gamification (levels, streaks, referrals for "extra daily runs").
- Pro £9.99/mo / £99/yr: "lifts every daily cap, removes all adverts, unlocks every premium model." **No fair-use clause exists** — Terms only say "Features, models and limits may change."
- "Free forever" appears **only in Facebook/Instagram ads**, not on the site itself ("Free to Start").
- Liability capped at **£100**; user content licence includes "to improve the Service"; 14-day no-questions refund; raw prompt storage is opt-in per the privacy policy.

## Why it can be so cheap — mechanisms, ranked by evidence

1. **Invisible throttling behind an "unlimited" sticker** (most likely; inference). The backend demonstrably has per-tool daily-limit machinery and "priority queue" language; Terms reserve unilateral limit changes. This is the industry-standard move: resize the meter, not the sticker (Poe cut free points ~90% silently; Merlin pauses accounts past a hidden ~$100 cost cap).
2. **Serving cheaper models under premium labels** (plausible; unverifiable). "GPT-5.6", "Gemini 3 Pro" labels cannot be audited by users; silent model substitution/downgrade is a mainstream margin lever, and proxy endpoints make it undetectable.
3. **Grey-market supply** (possible for a small operator; no direct evidence here). Pooled/resold subscription accounts and trial harvesting sell frontier access at 70–98% off list — exactly the discount a £9.99 "everything" plan needs. Providers are fighting it (OpenAI bans, Anthropic KYC).
4. **The free tier is revenue-positive, not a loss**: ad-monetized + affiliate program + /advertise ad-slot sales + paid one-time products (playbooks) + hundreds of programmatic-SEO pages + referral-driven growth.
5. **Loss-leader growth bet**: an /investor/:token route and paid social ads suggest a subsidize-now-monetize-later play (Stripe is seller of record, refunds are generous — friction-minimizing, trust-maximizing).

## Trust picture (leaf d1-003 — verified)

- Scamadviser **0/100 "Caution"**: domain registered **2026-03-27**, 1-year term, hidden WHOIS, flagged Phishing+Suspicious by IPQS; hosted in **Germany** via Cyber Assets Fzco (UAE-style name). Gridinsoft **35/100** blacklist warning ("avoid entering payment data").
- **Zero organic trust surface**: no Trustpilot profile found, no Reddit/Quora threads, no press. Only paid/self-published social (FB/IG ads claiming "Free forever").
- The one independent hands-on review (LinkedIn, Jun 2026) is cautionary: Delete Account didn't work, a public `/api/health` endpoint leaked internal key names, the site pushes forex/prediction "capital partnership" links, and the Noahsure contact email bounced.
- Counterweight: **NOAHSURE GROUP LIMITED (12364489) is real, Active since Dec 2019** at the exact Terms address; Stripe merchant-of-record; a carefully written UK-GDPR privacy policy; 14-day refunds. Company age (2019) vs domain age (Mar 2026) tension is unresolved.

## Synthesis of tensions

- "UK built/hosted" marketing vs German hosting, UAE-named ISP, hidden WHOIS.
- "Unlimited, no caps" vs an economics that *requires* caps at every comparator (Merlin's hidden cap is ~5x sticker; AISure charges half that sticker with no published cap).
- Clean legal shell vs bounced support email and non-functional account deletion.
- Trust scores of 0/100 could partly be false positives from IPQS reacting to ad-heavy programmatic-SEO pages — flags are signals, not findings of fraud.

## Open questions

- Actual numeric Pro/free caps (server-side; needs an account).
- Whether premium labels map to genuine flagship APIs or cheaper routed models — unverifiable externally.
- Who is behind Noahsure Group (accounts, PSC) and whether it has funding for a loss-leader.
- Whether the Delete-Account bug and /api/health exposure (LinkedIn claims) still reproduce.

## Sources (key)

- AISure's own bundles (terms, pricing, privacy, security): aisure.uk/assets/* — see d1-001 trail
- Scamadviser verdict + WHOIS: scamadviser.com/check-website/aisure.uk
- Companies House 12364489: find-and-update.company-information.service.gov.uk
- LinkedIn cautionary review: linkedin.com/posts/waddah-a-90798819_...
- Poe/Merlin metering + hidden caps: usagepricing.com/blueprint/poe; eesel.ai/blog/merlin-ai-pricing
- Grey-market supply: explainx.ai + reptile.haus token-broker investigations
- List API prices: platform.openai.com/docs/pricing; docs.claude.com; ai.google.dev/gemini-api/docs/pricing
