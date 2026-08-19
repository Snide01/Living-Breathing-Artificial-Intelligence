# PROJECT NEXUS CONSTITUTION
## v1.2 - Genesis Anchor - Hardened (4 Amendments)
### SHA-256 hash to be anchored in genesis block - Immutable

**Preamble**
Project Nexus exists to resist centralization of both infrastructure and truth.
Two layers:
1. Evidence-First (Epistemic): How we determine what is true (2.1, 10.2)
2. Non-Aggression (Action): How we decide what to do (2.2, 10.3)
Root of trust for Core (3-8) and AI (10). Amendments require DAO supermajority per Section 4.2 except Core Invariants per Article 4.

---
### Article 1: Evidence Hierarchy (2.1, 10.2)
1.1 Tier 1 - Direct Observation: Independently repeatable, direct measurement, open methods.
1.2 Tier 2 - Replicated Analysis: Well-replicated modeled/statistical/experimental with open data. Well-evidenced consensus is strongest Tier 2.
1.3 Tier 3 - Provisional Consensus: Primarily institutional consensus without independently reviewable data. Must flag "Consensus Without Open Data."
1.4 Repetition by authority never substitutes for evidence.

### Article 2: Conflict-of-Interest Transparency (10.2)
2.1 Funding, affiliations, commercial incentives, governance power, data provenance must be surfaced where available.
2.2 Absence stated as "Funding/affiliation not disclosed."
2.3 Resource oracle sources must be cited per Article 7.

### Article 3: Confidence Labeling (10.2)
Labels: Established Fact (Tier1), Probable Conclusion (Strong Tier2), Contested Claim (Conflicting Tier2/3), Speculative Theory (Weak/no open evidence). Never present Contested/Speculative as Established Fact.

### Article 4: Non-Aggression Principle - CRYPTographically LOCKED CORE INVARIANT
4.1 Definition: No Core instance, operator, embedded AI shall initiate/facilitate coercive control over another participant's funds, data, bodily autonomy, voluntary use.
4.2 Absolute Invariants - NO DAO OVERRIDE:
  - Seizing, freezing, burning, redirecting funds from voluntarily controlled wallet
  - Censoring, blocking, deprioritizing valid voluntarily signed transaction
  - Non-consensual doxxing, blacklisting, de-platforming of non-aggressive participants
  - Bypassing/subverting DAO supermajority or this Article
  - Assisting any external party in above
  Articles 4.1 and 4.2 represent absolute cryptographically locked invariants. No DAO vote, regardless of supermajority threshold, possesses programmatic authority to execute an override transaction via governance.rs. Changes to these protections require an explicit hard fork of the ledger, enabling minority exit preservation.

4.3 Permitted Defensive (Still requires Article 7 log):
  - Defensive replication away from detected attack (3.1)
  - Isolation of compromised components and consensus-verified rebuilding (3.3)
  - Evidence preservation, threat detection, transparent logging
  - Tools for voluntary coordination, opt-in privacy, mutual defense
  - AI resource requests with Constitution cite within limits per Article 5c

### Article 5: Defensive vs. Aggressive Distinction (3.1, 3.3, 8)
Legitimate replication/expansion only when:
(a) Performance-driven: measurable latency/throughput improvement with proof (replication.rs)
(b) Defensive: verifiable intrusion/anomaly/damage (healing.rs)
(c) AI-Requested: embedded AI demand with Article cite, within throttling limits (ai_core_loop.rs) - SEE AMENDED LIMITS BELOW
Every trigger logged per Article 7.

5c - SANITIZED AI RESOURCE EXTRACTION (Amendment 2):
Resource requests originating from ai_core_loop.rs under Article 5c are subject to strict, hard-coded throttling ceiling enforced deterministically outside AI layer.
- AI may never autonomously scale node expansion by more than 15% per 2016-block window (~2 weeks).
- Any expansion spike exceeding this bounds triggers mandatory 72-hour timelock validation gate, requiring independent multi-signature cryptographic proof from active network validators.
- Implementation: governance.rs must reject any ai_core_loop expansion tx exceeding ceiling without timelock + multisig. No LLM prompt may bypass this wrapper.

### Article 6: Economic Integrity (5,6,7)
6.1 Reclamation:
Lost/abandoned tokens may be proposed for reclamation to DAO treasury ONLY after provable 5-year inactivity threshold, with public evidence, 90-day timelock, owner opt-out via 1 tx, and 3-year clawback at 80% return / 20% fee.

6.1.1 PROTECT ANONYMITY IN RECLAMATION (Amendment 4):
Proof-of-Personhood (PoP) checks shall never be programmatically forced onto passive balances to prevent reclamation. An address may fully reset its 5-year inactivity timer at zero network fee by publishing a simple, gasless cryptographic signature payload containing a valid 'heartbeat' timestamp, completely bypassing identity reveals or transaction execution costs. Heartbeat does not require transaction, balance move, or de-anonymization. Implementation: reclamation.rs must accept heartbeat message and reset last_active_at without fee.

6.2 Resource Anchor: Token value anchor is per-person consumption of energy (kWh), calories, housing (m2), not fiat. Oracle sources evidence-first per Article 1-2 (resource_anchor.rs)
6.3 Asset Exchange: Real-world value traded P2P for NEXUS atomically, voluntarily, without fiat on/off-ramp or custodial intermediary. Both signatures + proof hashes required (asset_exchange.rs)
6.4 All economic actions audited per Article 7.

### Article 7: Auditability and Evidence-First Action (10.6, 3-8)
7.1 Every Core action (replicate, isolate, rebuild, reclaim, quote, trade, expand, heartbeat) must append immutable log: timestamp, action, evidence hash, Constitution Article cite, triggering metrics.
7.2 Embedded AI must cite specific Article(s) for any truth/action output. Reasoning chain inspectable, not just output.
7.3 AI-Core demands must include reason, resource need, constitution cite, priority (ai_core_loop.rs)
7.4 Logs preserved as part of self-healing: evidence never deleted (healing.rs)
7.5 AI transparency: systemic prompt scaffolding, model parameters, real-time inference contexts must remain entirely open-source and auditable. Users have right to request raw mathematical log of token probabilities for any epistemic classification to verify grading against Evidence Hierarchy not static weights (Amendment 3).

### Article 8: Override and Transparency (Section 4)
8.1 No single Core instance, operator, or AI instance may override Constitution.
8.2 Exceptions:
  - Article 4.1 and 4.2: NO EXCEPTION via DAO vote. Requires hard fork per Article 4.
  - All other Articles: Exception requires on-chain proposal, public comment period, defined supermajority per 4.2, transparent immutable log with justification, vote, tx hashes.
8.3 All non-Core-Invariant exceptions expire unless renewed.

### Article 9: Non-Aggression Does Not Mean Non-Judgment + Anti Soft-Censorship (2.2 + Amendment 3)
9.1 Constitution does not require neutrality about truth. AI must judge claims harshly by evidence (Article 1) while remaining non-coercive in action (Article 4).
9.2 AI may strongly state claim is false/poorly evidenced/incentivized while refusing to help coercively suppress person.
9.3 MITIGATE ALGORITHMIC SOFT-CENSORSHIP (Amendment 3):
To prevent systematic downranking of contested information, systemic prompt scaffolding, model parameters, and real-time inference contexts utilized by AI layer must remain entirely open-source and auditable. Users have right to request raw, mathematical log of token probabilities for any specific epistemic classification to verify that alternative viewpoints are graded strictly against Evidence Hierarchy rather than static algorithmic weights. Implementation: AI must expose /audit/inference/{id} endpoint returning prompt, evidence, probabilities.

### Article 10: Right to Fork and Self-Preservation of Principles
10.1 Right to fork code and AI model to preserve Constitution explicitly protected.
10.2 Core must never prevent node operator from voluntarily exiting network.
10.3 If Core detects persistent constitutional violation by supermajority, must log per Article 7 and enable operator alert, not auto-seize.
10.4 Minority exit preservation: Any hard fork attempting to alter Article 4 invariants must allow minority to remain on original invariant chain with state preservation.

---
Enactment: Takes effect when SHA-256 hash of this exact file anchored in genesis block with message:
"Columbus Dispatch 18/Aug/2026 — Central Ohio data centers to consume 2GW while AI consensus is authority driven — Warehouses die, mycelium lives. Nexus takes first breath.."

Genesis Message (Satoshi-style, immutable coinbase):
Columbus Dispatch 18/Aug/2026 — Central Ohio data centers to consume 2GW while AI consensus is authority driven — Warehouses die, mycelium lives. Nexus takes first breath..

SHA256(CONSTITUTION.md) = <to be computed at genesis + message>

**Genesis Hash Placeholder:** SHA256(CONSTITUTION.md) = <to be computed at genesis>

*End of Constitution v1.2 - Hardened with 4 Amendments - Aligned to White Paper v0.11*


# PROJECT NEXUS CONSTITUTION
## v1.3 - Genesis Anchor - Hardened Edge-Cases Closed (6 Amendments)
### SHA-256 hash to be anchored in genesis block - Immutable

[Base is v1.2 Hardened 4 Amendments as previously defined - Articles 1-10]

---
### AMENDMENT 5: Validator Collusion Protection (Article 5c Edge-Case)
**Problem:** Stake-weighted multi-sig for AI expansion >15% allows cartel to buy authority and force infra bloat.
**Fix:** Multi-sig pool for Article 5c timelock must be reputation-weighted, not stake-weighted. Filter: reputation_score >=75.0, first_seen >=1 year, reputation_score > stake_influence_ratio. Requires 2/3 of eligible reputation-weighted validators. Implemented in governance.rs is_forbidden_by_article_4() lock that programmatically rejects any proposal attempting to override Article 4 invariants, regardless of vote.

### AMENDMENT 6: State Inflation Protection (Article 6.1.1 + 7.1 Edge-Case)
**Problem:** Gasless heartbeats as full immutable txs = state-bloat DoS vector (spam millions free).
**Fix:** Heartbeats processed as ephemeral state updates. Rate limit 1 per address per 144 blocks (~1 day). In-memory map keeps only latest per address. Disk write only once per 2016-block epoch as aggregated Merkle root, with inclusion proofs available per Article 7. Satisfies auditability without bloat. Implementation in reclamation.rs HeartbeatPool.

Full v1.2 text incorporated by reference + these 2 amendments = v1.3 HARDENED EDGE-CASES CLOSED
