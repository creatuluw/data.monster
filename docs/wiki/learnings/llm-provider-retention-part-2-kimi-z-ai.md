---
type: Learning
title: "LLM provider retention, part 2: Kimi, Z.ai, Together, Qwen — Kimi policy contradiction, Z.ai DPA strength, tier framework"
description: "Follow-up research (2026-09-11) on Kimi (Moonshot), Z.ai (Zhipu/GLM), Together AI, and Qwen (Alibaba Model Studio), verified against primary docs. Extends [[llm"
tags: [llm, privacy, data-retention, research, providers]
timestamp: "2026-09-11T21:51:49.230Z"
---

# LLM provider retention, part 2: Kimi, Z.ai, Together, Qwen — Kimi policy contradiction, Z.ai DPA strength, tier framework

Follow-up research (2026-09-11) on Kimi (Moonshot), Z.ai (Zhipu/GLM), Together AI, and Qwen (Alibaba Model Studio), verified against primary docs. Extends [[llm-api-data-retention-no-training-no-storage-local-models-a]].

## Key findings

- **Kimi (Moonshot)** — trust gap of the group: Help Center FAQ says "never trains / no persistent storage," but the binding Privacy Policy says user content helps "optimize our models" (training use, legitimate interest) with indefinite retention ("as long as necessary"). When they conflict, the policy wins — treat Kimi API as training-permitted, retention-unspecified. International API runs via Singapore entity (Moonshot AI Pte. Ltd., Chinese parent) with mandatory content-safety review.
- **Z.ai (Zhipu/GLM)** — API DPA §4(b) claims content is "processed in real-time… not saved on our servers" (effectively zero content storage by default), plus delete-on-termination and law-enforcement-request notification — a proper GDPR-style processor agreement, ZDR by default. Consumer z.ai chat *does* train. Caveat: third-party reviews note the no-storage claim lacks documented corroboration. GLM open weights are MIT.
- **Together AI** — three independent toggles: storage (default **ON** — turn off for ZDR, self-serve), training (default off), passthrough (default on; ZDR forces it off). Passthrough models forward prompts to the upstream vendor's policy. Third-party models (DeepSeek, Qwen) hosted by Together never call home — DeepSeek-via-Together runs in North American data centers, the pragmatic way to use DeepSeek without China-jurisdiction exposure.
- **Qwen (Alibaba)** — three very different doors: qwen.ai chat (consumer terms, trains), Model Studio/DashScope ("will never use your data for model training", China-mainland or Singapore region), and open weights (Apache-2.0, self-host). Qwen 3 Max is also served via Cloudflare Workers AI with ZDR.

## Tier framework (big-12 view)

- **Tier 1** (documented, contractual no-training + workable ZDR): OpenAI, Anthropic, Azure, Google, xAI, Cohere, Mistral
- **Tier 2** (good defaults, self-serve controls, but storage-on-by-default or thin corroboration): Together (toggle ZDR on), Z.ai (strong DPA, uncorroborated)
- **Tier 3** (assume public-grade handling): DeepSeek first-party, Kimi (policy contradiction), Qwen-via-China-region

Rule of thumb: **marketing FAQ ≠ terms of service** — always verify against the binding privacy policy/DPA. For Data Monster workspaces with sensitive data, local llama.cpp still beats every contract on this list.
