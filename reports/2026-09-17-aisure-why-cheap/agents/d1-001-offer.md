# d1-001 — The offer and its fine print

Method note: aisure.uk is a client-rendered React SPA (Vite; Lovable-built, Supabase EU backend, Stripe). Server HTML is an empty shell, so findings below were extracted from the actual JS bundles served for each route (same content users see). Strings are verbatim from bundle code.

## Findings

- **Offer**: "Every AI Tool. One Platform. Free to Start." — chat, image, video, voice, code, research, documents, translation, automation in one workspace. [source: https://aisure.uk/ meta + https://aisure.uk/assets/Landing-Ck-1GQL-.js] — verified
- **Pricing**: Free plan "£0 — no credit card needed to start" with "a daily allowance across every AISure studio"; AISure Pro **£9.99/month** or **£99/year** ("two months free"), auto-renewing via Stripe. [source: https://aisure.uk/assets/Pricing-D6rZQq0l.js + index bundle] — verified
- **"Free forever" is NOT on the site** — zero occurrences in homepage, landing, or pricing bundles. Site wording is "Free to Start" / "Start free". The "Free forever. £9.99/mo" phrasing appears only in their Facebook ads. [source: bundle greps (0 hits); Facebook via DuckDuckGo result "AISure.uk - Every AI. One app. Free forever. £9.99/mo..."] — verified (absence); reported (FB tagline)
- **Models are named explicitly**, not "GPT-class": landing says "GPT-5, Gemini 3 Pro, Claude and more — same chat, same files, same context. Pick the best model for each task in one click"; Pro upsell says "Every premium model — Claude, GPT-5.6 and Gemini Pro"; gamification reward says "Top-tier models like Gemini 2.5 Pro and GPT-5 in chat" (unlocked at user level 3). Also comparison tables vs ChatGPT/Claude/Gemini. [source: Landing + index bundles] — verified
- **Free-tier limits are real and enforced per tool**: code reads `free_daily_limit`, `used`, `limit_reached`, `reset_in_seconds` per tool from the backend; UI says "You've used today's free [tool] allowance" and "Free allowance resets in …". Exact numeric caps are server-side, not published in any fetched bundle. [source: https://aisure.uk/assets/index-B5TWEVbn.js] — verified
- **What "unlimited" means**: "No daily caps on any tool" / "Pro lifts every daily cap, removes all adverts and unlocks every premium model". **No fair-use clause exists** — zero matches for "fair use/usage" in Terms, pricing, or index bundles. Terms add: "Features, models and limits may change as the Service evolves." [source: Pricing + Terms + index bundles] — verified
- **Softening details**: "Failed generations, safety refusals and dropped connections never count against your allowance"; referral: "Invite a friend and you both get extra daily runs — free"; "Not ready to pay? Earn more runs today instead." Heavy gamification (levels, streaks, Daily Spark) gates perks including premium models. [source: index bundle] — verified
- **Free tier is ad-monetized**: AdBanner component; GA4 + Google Ads + Meta + TikTok pixels (`VITE_GOOGLE_ADS_ID` etc.); Pro sells "No adverts anywhere"; cookie banner: analytics/marketing cookies off until consent. [source: index bundle CSP + code] — verified
- **Terms of Service** (found at /legal/terms): contracting entity **Noahsure Group LTD**, 1 Canada Square, Canary Wharf, London E14 5AA; England & Wales law + exclusive jurisdiction; age of majority required; bans scraping, "circumventing rate limits", reselling; user keeps content ownership but grants "worldwide, royalty-free licence to host, process and transmit User Content solely to provide and improve the Service"; service "AS IS"; liability capped at greater of amounts paid or **£100** per 12-month period. [source: https://aisure.uk/assets/Terms-DPpRzmpT.js] — verified
- **Payments**: Stripe is processor and **seller of record** (merchant-of-record setup; "Stripe Payments Europe Limited … acts as our payment processor and an independent data controller"); live Stripe publishable key in pricing bundle. Refunds: 14-day no-questions full refund incl. first billing period; renewals not auto-refundable but reviewed in good faith; statutory UK/EU withdrawal rights preserved. [source: Terms + https://aisure.uk/assets/Refunds-Ca25t9XK.js + Privacy chunks] — verified
- **Privacy/data (UK GDPR + DPA 2018 claimed)**: collects email, full name, optional company/country/industry; usage events, tools used, timestamps, session IDs, "model outcomes"; **prompt metadata only (length, intent category, hash) — raw prompt text stored only if you opt in** via Settings → Data & AI Improvement (consent basis, 12-month retention); billing records 7 years; account data deleted +30 days. Card data never touches them (Stripe tokenises; they get last4/brand/country only). US transfers under UK IDTA + SCCs. Rights + ICO complaint route listed. [source: https://aisure.uk/assets/Privacy-fQ2z1bFP.js] — verified
- **Security page**: AES-256 at rest (Lovable Cloud / Supabase EU), Row-Level Security on every table, 7-day PITR backups, Sentry, 72-hour breach notification (UK GDPR Art. 34). Sub-processor list is loaded dynamically from the database (content not in bundle). [source: https://aisure.uk/assets/Security-CQU4-Wcd.js + SubProcessors chunk] — verified
- **Growth surfaces**: hundreds of programmatic-SEO pages (/uk/* personas + cities, /free/*, /vs/*, /for/*), /advertise (sell ad slots), /affiliate, /enterprise (quote flow), /investor/:token, named personas (/claudia, /elizabeth, /manuella, /marcella), /playbook + /safeguarding-playbook (paid one-time products per Refund policy). [source: sitemap-core/uk/features/etc + route table] — verified

## Tensions

- "Free forever" ad tagline vs site's own "Free to Start" — the free tier has hard enforced daily caps and ads; the FB ad phrasing oversells what the site itself claims.
- "Unlimited" Pro with no published fair-use clause, while actual model spend is uncapped per £9.99 user — economics depend entirely on unpublished server-side throttling ("priority queue" language suggests queuing, not guaranteed capacity).
- Terms grant a licence to "improve the Service" with User Content, while Privacy says raw prompts are opt-in only — the licence vs the consent gate are not obviously reconciled.
- Names frontier models (GPT-5, GPT-5.6, Gemini 3 Pro, Claude) at £9.99 all-in — cheaper than any single frontier subscription; no model-provider or cost disclosure anywhere.
- Liability capped at £100 while marketing targets businesses (lawyers, accountants, NHS staff pages).

## Open questions

- Actual numeric daily caps per tool (server-side only — needs an account to read `free_daily_limit` values).
- Sub-processor list contents (loaded from DB at runtime; includes the actual AI model providers?).
- Whether "GPT-5.6"/"Gemini 3 Pro" labels map to genuine flagship API models or cheaper proxies — unverifiable from public pages.
- Companies House record for Noahsure Group LTD (DDG search returned no usable results; not verified independently).

## Search trail

- Queries: `aisure.uk pricing £9.99` (DuckDuckGo HTML) → found FB tagline "Free forever. £9.99/mo" + an unrelated Apple Community £9.99-weekly-sub thread (discarded, wrong company). `"Noahsure Group" LTD` → no usable results.
- Fetched: aisure.uk/ (SPA shell), robots.txt, sitemap.xml + 6 sub-sitemaps (route inventory), /assets/index-B5TWEVbn.js (301KB main bundle — pricing, limits, models, gamification), Landing, Pricing, Terms, Refunds, Privacy, Security, SubProcessors, LegalLayout, api chunks. DPA chunk (1.4MB) fetched but is mostly an embedded PDF generator — discarded except DPA-download confirmation.
- Useful: all bundles above. Discarded: rendered-page HTML for /pricing /terms /privacy /uk /blog (identical empty SPA shell), a_Dpa.js bulk.
