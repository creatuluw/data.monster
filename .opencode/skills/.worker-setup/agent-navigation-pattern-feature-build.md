# Agent Navigation Pattern — Building a Feature

## Task

**Build a feature to log in with Google.**

This document traces how an LLM agent navigates the worker taxonomy to solve this task. At each layer, the agent activates specific concepts, gathers what it needs, and produces artifacts that feed the next layer.

---

## Navigation Overview

```
WORKER TAXONOMY — Concepts Activated for This Task

Worker
├── Intent
│   ├── Vision          ✦ Users authenticate securely via Google
│   ├── Goal            ✦ Implement Google OAuth login
│   ├── Objective       ✦ Ship working Google Sign-In on login page
│   └── Desired Outcomes ✦ Users click → authenticate → logged in
├── Direction
│   ├── Approach         ✦ Use established auth protocol (OAuth 2.0)
│   ├── Strategy         ✦ Leverage libraries, build incrementally
│   └── Principles       ✦ Security first, minimal data, graceful errors
├── Guidance
│   ├── Framework        ✦ OAuth 2.0 / OpenID Connect spec
│   ├── Patterns         ✦ Redirect flow, token exchange, session mgmt
│   ├── Architecture     ✦ Auth layer, middleware, user store
│   └── Models           ✦ User-Identity map, session model, auth state
├── Methodology
│   ├── Rules            ✦ Never store credentials, validate server-side, HTTPS
│   ├── Methods          ✦ Token verify, user lookup, session establish
│   └── Processes        ✦ Register → Configure → Implement → Handle → Test
└── Skills
    ├── Tactic           ✦ Read docs, inspect existing code, test with real account
    ├── Technique        ✦ JWT validation, OAuth state param, cookie sessions
    ├── Practice         ✦ Write tests per step, cover edge cases
    ├── Heuristic        ✦ Match email → link user; no match → create user
    └── Tool Use         ✦ Google Console, OAuth lib, HTTP client, test runner
```

---

## Layer-by-Layer Walkthrough

### 1. Intent — What and Why

The agent starts by establishing intent. Without intent, it has no scope, no success criteria, and no way to know when it's done.

**Vision**
> Users can securely access the platform using their existing Google identity — no new passwords to remember, no friction at the door.

The agent holds this as its north star. Every decision downstream is tested against this vision.

**Goal**
> Implement Google OAuth login functionality that allows users to sign in with their Google account.

The agent decomposes the vision into a concrete goal. This scopes the work: it's about authentication via Google, not building a whole identity system.

**Objective**
> Ship a working "Sign in with Google" button on the login page that completes the full OAuth flow and creates or matches a user account.

The agent makes the goal time-bound and verifiable. When there's a button that works end-to-end, the objective is met.

**Desired Outcomes**
> - A user clicks "Sign in with Google" and is redirected to Google's consent screen.
> - After consent, the user is redirected back and logged in.
> - If the user already exists (matched by email), they are linked — no duplicate accounts.
> - If the user is new, an account is created automatically.
> - Errors (denied consent, network failure, expired token) are handled gracefully.

These are the observable conditions the agent will validate against.

---

### 2. Direction — Which Way and Why

With intent clear, the agent chooses a direction. This layer prevents it from wandering into dead ends.

**Approach**
> Use an established authentication protocol rather than building a custom solution.

The agent identifies the stance: *don't reinvent auth*. This immediately narrows the search space and rules out bad paths (like handling Google passwords directly).

**Strategy**
> 1. Leverage an existing OAuth library compatible with the tech stack.
> 2. Build incrementally: configure → button → redirect → callback → user creation → session.
> 3. Test each step before moving to the next.

The agent sequences its effort. Each step produces a verifiable checkpoint. If something breaks, it knows exactly where.

**Principles**
> - **Security first** — Never handle raw Google credentials. Always validate tokens server-side.
> - **Minimal data** — Only request the scopes we need (email, profile). Don't over-ask.
> - **Graceful degradation** — If Google auth fails, show a clear error. Never leave the user stuck.
> - **Idempotency** — Linking an existing user should not create duplicates.

These principles constrain all downstream decisions. They act as guardrails.

---

### 3. Guidance — What to Use and Follow

With direction set, the agent gathers structured knowledge to inform implementation.

**Framework**
> **OAuth 2.0 / OpenID Connect** — The agent adopts this as its primary framework. It defines the roles (client, auth server, resource server), the flows (authorization code), and the token types (access, ID, refresh).

> If the tech stack is known, the agent also adopts a specific library framework (e.g., NextAuth.js for Next.js, passport.js for Express, Spring Security OAuth for Java).

**Patterns**

The agent identifies the key patterns it will implement:

| Pattern | Description |
|---------|-------------|
| **Redirect-based auth flow** | User is sent to Google, then returned with an auth code |
| **Token exchange** | Auth code is traded for access/ID tokens server-side |
| **User matching / creation** | Existing users are linked; new users are provisioned |
| **Session establishment** | After auth, a session is created for the user |
| **Error handling** | Each failure mode has a specific recovery path |

**Architecture**

The agent sketches the architectural components:

```
┌──────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────┐
│  Login   │────▶│  Auth        │────▶│  Google      │────▶│  User    │
│  Page    │     │  Controller  │     │  OAuth API   │     │  Store   │
│  (Button)│     │  (Callback)  │     │  (Tokens)    │     │  (DB)    │
└──────────┘     └──────┬───────┘     └──────────────┘     └──────────┘
                        │
                        ▼
                 ┌──────────────┐
                 │  Session     │
                 │  Manager     │
                 └──────────────┘
```

Key architectural decisions:
- Auth logic lives in a dedicated controller/middleware, not scattered in UI code.
- Token validation happens server-side, never client-side.
- User store handles matching and creation atomically.

**Models**

The agent defines the data models it needs:

```
User
├── id (internal)
├── email
├── name
├── google_id
├── created_at
└── updated_at

Auth Session
├── session_id
├── user_id
├── created_at
└── expires_at

OAuth State (ephemeral)
├── state_token (CSRF protection)
├── code_verifier (PKCE)
└── redirect_url
```

---

### 4. Methodology — How to Do It

With guidance in place, the agent defines the rules, methods, and processes it will follow.

**Rules**

| Rule | Rationale |
|------|-----------|
| Never store Google credentials | Security — we only store our own session tokens |
| Always validate tokens server-side | Security — client-side validation can be bypassed |
| Use HTTPS for all auth endpoints | Security — prevent token interception |
| Implement CSRF protection (state parameter) | Security — prevent authorization code injection |
| Request minimal OAuth scopes | Privacy — only email and profile |
| Handle all error states explicitly | UX — never leave user on a blank screen |
| Link existing users by email, don't duplicate | Data integrity — single source of truth per user |

**Methods**

The agent defines the specific procedures it will implement:

| Method | Input | Output | Steps |
|--------|-------|--------|-------|
| `initiateGoogleAuth()` | User click | Redirect to Google | Generate state, build auth URL, redirect |
| `handleOAuthCallback()` | Auth code + state | Authenticated user | Validate state, exchange code for tokens, extract user info, find or create user, establish session |
| `verifyIdToken()` | ID token | Verified claims | Verify signature, check issuer, check audience, check expiry |
| `findOrCreateUser()` | Google profile | User record | Query by email or google_id, create if not found, return user |
| `establishSession()` | User record | Session cookie | Create session, set cookie, redirect to app |

**Processes**

The agent maps the end-to-end process:

```
PROCESS 1: Setup (one-time)
──────────────────────────────
Step 1  Register application in Google Cloud Console
Step 2  Configure OAuth consent screen
Step 3  Obtain client_id and client_secret
Step 4  Add credentials to application config
Step 5  Install/configure OAuth library


PROCESS 2: Runtime Auth Flow (per user)
────────────────────────────────────────
Step 1  User clicks "Sign in with Google"
Step 2  Client requests auth URL from server
Step 3  Server generates state + code_verifier, returns auth URL
Step 4  Client redirects user to Google
Step 5  User authenticates and consents on Google
Step 6  Google redirects back with auth code + state
Step 7  Server validates state matches original
Step 8  Server exchanges auth code for tokens
Step 9  Server verifies ID token signature and claims
Step 10 Server extracts user info from ID token
Step 11 Server calls findOrCreateUser()
Step 12 Server establishes session
Step 13 Server redirects user to app (now logged in)


PROCESS 3: Error Handling
──────────────────────────
If user denies consent     → Redirect to login with message
If state mismatch          → Reject, log warning, redirect to login
If token exchange fails    → Show error, suggest retry
If token expired           → Prompt re-authentication
If user creation fails     → Show error, log for investigation
If network error           → Show generic error, suggest retry
```

---

### 5. Skills — Doing It

This is where the agent executes. It draws on specific tactics, techniques, practiced patterns, heuristics, and tools.

**Tactics**

The agent takes these immediate, context-dependent actions:

| Tactic | When |
|--------|------|
| Read Google's OAuth documentation to confirm current endpoint URLs and required parameters | Before writing any code |
| Inspect existing auth code in the codebase to understand current session management | Before designing the integration |
| Check if an OAuth library is already a dependency | Before adding a new one |
| Test with a real Google account in a staging environment | After initial implementation |
| Monitor server logs during first test runs | To catch silent failures |

**Techniques**

The agent applies these refined, repeatable techniques:

**JWT/ID Token Validation:**
```
1. Parse token header to find key ID
2. Fetch Google's public keys (JWKS endpoint)
3. Find matching public key
4. Verify signature
5. Verify issuer (accounts.google.com)
6. Verify audience (our client_id)
7. Verify token is not expired
8. Extract claims (sub, email, name)
```

**OAuth State Parameter (CSRF Protection):**
```
1. Generate cryptographically random state string
2. Store state in server-side session or cookie
3. Include state in auth URL
4. On callback, compare returned state with stored state
5. Reject if mismatch
```

**Cookie-based Session Management:**
```
1. After successful auth, generate session ID
2. Store session in database/cache with user_id and expiry
3. Set HTTP-only, Secure, SameSite cookie
4. On subsequent requests, validate session from cookie
```

**Practice**

The agent applies deliberate practice patterns to ensure quality:

- **Test each auth step independently** — Test token exchange separately from user creation separately from session setup.
- **Test edge cases** — Existing user, new user, user denies consent, expired token, invalid state.
- **Test the happy path end-to-end** — Full flow from click to logged-in.
- **Review security** — Check that tokens aren't logged, cookies are secure, state is validated.

**Heuristics**

The agent applies these rules-of-thumb when full analysis is impractical:

| Heuristic | Application |
|-----------|-------------|
| If the user's Google email matches an existing account, link them — don't create a duplicate | User matching |
| If in doubt about which scopes to request, start with the minimum (email, profile) and add only if needed | Scope selection |
| If the OAuth library supports PKCE, use it — it adds security with almost no cost | Security |
| If something works in staging but not production, check redirect URIs first | Debugging |
| If token validation fails, check the clock skew between servers | Debugging |

**Tool Use**

| Tool | Purpose |
|------|---------|
| **Google Cloud Console** | Register app, configure consent screen, obtain credentials |
| **OAuth library** (e.g., NextAuth, passport.js, Spring Security) | Handle protocol details, reduce custom code |
| **HTTP client** (e.g., fetch, axios) | Exchange auth code for tokens |
| **JWT library** (e.g., jose, jsonwebtoken) | Verify ID token signatures |
| **Testing framework** (e.g., Jest, pytest) | Write and run auth flow tests |
| **Database client** | Query and create user records |
| **Logging/monitoring** | Track auth events and failures |

---

## Navigation Pattern Summary

The agent's path through the taxonomy for this task follows this pattern:

```
1. START → Intent
   "What am I building and why?"
   Activates: Vision → Goal → Objective → Desired Outcomes
   Produces: Clear scope and success criteria

2. Intent → Direction
   "How should I approach this?"
   Activates: Approach → Strategy → Principles
   Produces: A plan of attack with guardrails

3. Direction → Guidance
   "What knowledge do I need?"
   Activates: Framework → Patterns → Architecture → Models
   Produces: Structural blueprint for implementation

4. Guidance → Methodology
   "What are the exact steps and rules?"
   Activates: Rules → Methods → Processes
   Produces: Concrete procedures with constraints

5. Methodology → Skills
   "How do I execute this right now?"
   Activates: Tactics → Techniques → Practice → Heuristics → Tool Use
   Produces: Working code, passing tests, deployed feature

6. Skills → Intent (FEEDBACK)
   "Did I achieve what I set out to do?"
   Validates desired outcomes against actual results
   Produces: Confirmation that the feature works as intended
```

### What Was Activated vs. Skipped

The agent did not activate every concept equally. For this task:

| Concept | Activation Level | Why |
|---------|-----------------|-----|
| Vision | Low | The vision is straightforward — it's a well-understood feature type |
| Goal | High | Clear goal definition is critical for scoping |
| Objective | High | Concrete milestones keep the agent on track |
| Desired Outcomes | High | These become the acceptance criteria |
| Approach | High | Choosing OAuth is the single most important decision |
| Strategy | High | Incremental build strategy prevents getting lost |
| Principles | High | Security principles prevent critical mistakes |
| Framework | High | OAuth 2.0 / OpenID Connect is the primary knowledge source |
| Patterns | High | Auth flow patterns are the templates for implementation |
| Architecture | Medium | Simple component layout, not a complex system |
| Models | Medium | Only a few data models needed |
| Rules | High | Security and data integrity rules are essential |
| Methods | High | Each method maps to a function the agent must implement |
| Processes | High | The step-by-step process is the main execution guide |
| Tactic | Medium | A few key actions, not extensive tactical maneuvering |
| Technique | High | Token validation, session management require refined skill |
| Practice | Medium | Standard testing practices apply |
| Heuristic | Medium | A few rules-of-thumb guide edge case handling |
| Tool Use | High | Multiple tools required (Google Console, OAuth lib, test runner) |

### Key Takeaway

For a **feature build** task, the agent's navigation is **methodology-heavy**. The critical path runs through:
- **Approach** (choosing OAuth)
- **Framework** (OAuth 2.0 spec)
- **Patterns** (auth flow, token exchange)
- **Methods** (specific procedures to implement)
- **Processes** (step-by-step execution order)
- **Technique** (refined implementation skills like JWT validation)

The agent leans most heavily on the middle layers — Guidance and Methodology — because building a well-understood feature type is primarily about applying established knowledge correctly, not about inventing new approaches.