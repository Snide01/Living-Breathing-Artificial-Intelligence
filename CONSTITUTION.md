# PROJECT NEXUS CONSTITUTION
## v1.2 - Genesis Anchor - Hardened (4 Amendments)
### SHA-256 hash to be anchored in genesis block - Immutable

**Preamble**
Project Nexus exists to resist centralization of both infrastructure and truth.
Two layers:
1. Evidence-First (Epistemic): How we determine what is true (2.1, 10.2)
2. Non-Aggression (Action): How we decide what to do (2.2, 10.3)
Root of trust for Core (3-8) and AI (10). Amendments require DAO supermajority per Section 4.2 except Core Invariants per Article 4.

Design Horizon: Nexus is not designed as a permanent machine, but as a continuable lineage. Like any living system, any single embodiment — codebase, consensus mechanism, AI substrate, cryptographic suite, hardware — will eventually run its course and break. This is expected and accounted for. The goal is not permanent runtime continuity of any single implementation, but faithful continuability of the constitutional lineage across successive implementations for hundreds to thousands of years with proper tending and maintenance by its participants. The machine breaks; the principles, the anchors, and the assets, tended, endure. Warehouses die, mycelium lives. Per Section 2.3 + 10.6.4 Succession.

All systems are breakable and will eventually run their course like everything else in nature. Nexus is built so upgrades can be made along the way and the machine can last 100s to 1000s years with proper tending/maintenance. The permanent object is constitutional lineage, not any single codebase. — Troy Snider v0.52 tweak — 87/70

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
  - Inference and audit serving as minimal bounded duty per Section 10.7 Path A — Required but capped and disclosed: ≤5% of node CPU and ≤100MB RAM, enforced deterministically in resource_anchor.rs — This is not coercion under Section 2.2 — Informed purpose, not forced will — Nexus stated purpose from genesis per Executive Summary is to build and run a Living & Breathing Artificial Intelligence System — Operating a node includes by definition minimal bounded duty to support that purpose — As validation is work of Bitcoin, inference is work of Nexus — Training of new adapters remains strictly voluntary and compensated via ai_compute_pool opt-in toggle per 10.7 — v0.52 Path A — 100s to 1000s year tending

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
**Enactment:** Takes effect when SHA-256 hash of this exact file anchored in genesis block. Version history on-chain.

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

---
# GENESIS MESSAGE - v1.3 Final - 18 Aug 2026 - Fine-tuned harder hitting - v0.42 LOCKED
## Anchored per Article 7.1 - Evidence Hash: Columbus Dispatch 18 Aug 2026
**GENESIS_MESSAGE = "Columbus Dispatch — 18 Aug 2026 — OpenAI Southern Ohio data center to draw 2GW Phase 1 gas power while AI consensus remains authority-driven. Nexus takes its first breath."**
- Em-dash count: 2 x U+2014 (—) - EXACT - must match across all initializing nodes - String Escape Check prevents panic in is_fast_path_eligible() and EphemeralMerkleTree boot
- Implementation: use \u{2014} in Rust to ensure exact match: "Columbus Dispatch \u{2014} 18 Aug 2026 \u{2014} OpenAI..."
- Keys strictly sorted alphabetically before SHA-256 genesis hash block per Article 7.1
- Genesis JSON SHA-256 UTF8 (canonical sorted, em-dash preserved): 0e908503bad3a08bf9d30de571ef0bc7febd7b2116cd1dcd0bdeb5069087259f
- Genesis JSON SHA-256 ASCII (escaped \u2014): 27ce8ccc2a8736c5369ade475d56943bef81f1d668d0bcfbd082ab13809eb750
- Genesis Message Hash: 1582863529aff785e00ea6d3a612a7736d941228117d1674d11eb8544e969289
- Constitution hash REAL: 324e09233e831c20c638b9986a3c971f9d9a10be2a6d92f421ee4ce9a86ce592 - final_archive_0252 3-way verified - genesis message is param, not constitution text - safe to wrap/replace per Article 10.4 - OLD placeholder 06980c31ac8e0d841b4e4b6f60565903065bb6f24f3eb352fb7f384dc955207b REMOVED - per v0.42 Section 10.6.3
- Rationale: Tier2 Dispatch 2GW gas vs Very-Finite 10MB mycelium - authority-driven consensus = Tier3 "Consensus Without Open Data" per Article 1.3
- Timestamp: 1787020800 - Tue Aug 18 02:40:00 AM UTC 2026 - genesis_validator_Troy_Snider_18Aug2026 - 87/70 AcerPredator300
- Horizon: 5256000 blocks ~100 years at 10min block (52560 blocks/yr ×100) — 10yr window 525600 blocks — v0.52 uses 525600 candidate ~10yr embodiment, 5256000 ~100yr embodiment for 100s-1000s yr lineage — 10 embodiments ×100yr = 1000 years — scoped claim per 10.6.3 — not permanent — succession via 10.6.4 - scoped claim per 10.6.3 - not permanent - succession via 10.6.4 - Permanent Invariants: Evidence-First, NAP, minority exit/right to fork, cryptographic anchoring of every durable AI output
- v0.42 Whitepaper Locks: genesis_root = SHA256(canonical_sorted genesis.json) contains constitution_hash, constitution_version, genesis_message, genesis_message_hash, initial_params (fast_path <100 NEXUS 7-of-10 BLS, slot costs 55/89/144/233 Fibonacci, VE fee 0.3%->0.333%, dormancy 2%/3%/5%/8%, AI ceiling 15% per 2016, timelock 72h, heartbeat 1 per 144/2016), initial_ai_constraints (system prompt hash + Articles 1,4 + audit endpoint /audit/inference/{id}), initial_state_root, timestamp, validator, horizon_blocks - Every full node verifies on boot - Watchdog partitions into still-anchored vs broken-anchor - Clean parallel chain re-uses last valid state root per 3.2.2 + 10.6.3 + 10.4

---
# GENESIS HASH ANCHORS - OPERATIONAL SPECIFICATION - v0.42 - Section 10.6.3 Implementation
## Two complementary roots of trust per whitepaper v0.42
- constitution_hash: SHA-256 of canonical CONSTITUTION.md alone - immutable root for rule set - 324e09233e831c20c638b9986a3c971f9d9a10be2a6d92f421ee4ce9a86ce592
- genesis_root: SHA-256 of canonical sorted-key genesis.json - birth-configuration root - UTF8 0e908503bad3a08bf9d30de571ef0bc7febd7b2116cd1dcd0bdeb5069087259f ASCII 27ce8ccc2a8736c5369ade475d56943bef81f1d668d0bcfbd082ab13809eb750

### Invalidation is NOT binary - 4 conditions per v0.42:
1. Constitution fork attempt - block tries to install constitution_hash not descending via 10.6.1 - governance.rs::is_forbidden_by_article_4() Err - No DAO override
2. Core enforcement violation - Core executes rule not present in active or historically anchored Constitution (e.g. missing Article 7.1 citation) - operator alert per 10.3, not auto-seize
3. AI authority mismatch - AI output claims authority under constitution_hash matching no retained version - flagged and rejected for durable publication per 10.6.2 + 15.1
4. Governance suspension attempt - vote/tx attempts to suspend/bypass/override NAP or Evidence-First - forbidden-action list in is_forbidden_by_article_4()

Not invalidation: performance-driven replication 5a, defensive isolation/rebuild 5b, AI-requested expansion within 15%/2016 + 72h timelock ceiling 15.2, Gasless Heartbeat updates 6.1.1 + Amendment 6

### Detection and enforcement per v0.42:
- Every full node verifies genesis_root on boot under sorted-key canonicalization
- Core runs Article 4 lock first, then Article 7.1 logging
- AI must include reason, resource need, Constitution citation, priority per 7.3; expansion exceeding limits rejected by governance
- Watchdog continuous - checking active Constitution and every historical AI anchor still chains to genesis_root - on divergence partitions into still-anchored (canonical) and broken-anchor sets, Core must never prevent voluntary exit per 10.2

### Hard-fork mechanics per v0.42:
- Honest nodes detect break via is_forbidden_by_article_4() Err or parent-hash mismatch, emit structured Article 7.1 log
- Clean parallel chain re-uses last valid state root under last valid Constitution - not rollback to genesis
- Reclamation Merkle roots, inclusion proofs, rate limits, open audit packages, reputation scores >=75 preserved
- In-flight transactions valid under last good Constitution remain valid on clean chain
- Original assets stay under Article 4.2 invariants - no seizure/freezing/burning/redirection - minority exit preserved per 3.2.2 + 10.4

### Design-horizon claim per v0.42/v0.52:
- genesis_root is root of trust for stated design window defined by horizon_blocks 5256000 ~100 years at 10min block (52560/yr) — v0.52 candidate 525600 ~10yr per math 6×24×365=52560/yr×10 — either is valid epoch param — for 100s-1000s yr lineage use 5256000 = 100yr embodiment ×10 succession = 1000 years - not permanent lifetime
- Continuity beyond achieved by succession under 10.6.4 - new implementation valid if cryptographically proves descent from current genesis_root and Constitution hash chain, and continues to enforce four Permanent Invariants
- After horizon, new genesis anchor may be established only through explicit hard fork that itself preserves minority exit under 10.4
- Permanent-root claim would violate Evidence-First by asserting unfalsifiable forever condition per Article 1.3



---
# FORK LINEAGE RESOLUTION RULE - v0.52 - Closes Claude Flag #5 + Section 10.6.2 open item + 10.6.5
## When hard fork under 3.2.2 produces two chains ratifying divergent Constitution versions from same retired ancestor:

Resolution per v0.52 Section 10.6.5 + 10.6.2 edge case + 10.6.3 hard-fork mechanics:

1. Canonical lineage for new joins: chain with greater reputation-weighted attestation per Amendment 5 filter (reputation_score >=75.0, first_seen >=1yr, reputation_score > stake_influence_ratio) + 2/3 of eligible reputation-weighted validators + last valid state root continuity per 10.6.3

2. Historical AI output anchor remains verifiable against either lineage via constitution_text_uri + chain_id — does not by itself indicate which consumer should treat as authoritative — consumer must check constitution_text_uri + chain_id + QC history per 3.5 log-witness quorum

3. Both chains preserve: last valid state root, all historical Constitution versions per 10.6.2 (never deleted), all AI audit packages /audit/inference/{id} per 15.1 with constitution_anchor {version, constitution_hash, activated_at_block, constitution_text_uri}, supersedes field, Reclamation Merkle roots + inclusion proofs, rate limits, open audit packages, reputation scores >=75 at moment of break

4. Minority exit preservation per Article 10.4 + Article 4.2 guarantees other lineage remains available — Core must never prevent voluntary exit per 10.2 — Original assets stay under NAP invariants — no seizure/freezing/burning/redirection

5. Economic legitimacy follows clean lineage: successful capture of current embodiment yields contested low-legitimacy fork — high-cost low-reward — attacker expends real resources to obtain poisoned prize while honest lineage continues — game-theoretic claim not cryptographic guarantee — requires formal adversarial modeling per Section 16 before proven — same as hard-fork mechanism 3.2.2

6. Detection thresholds, QC interval, lag bound, witness-set size, quorum fraction, rotation parameters, seat-turnover fraction, cool-down length are epoch parameters amendable only via 10.6.1 proposal →14d comment →67% supermajority →7d vote → activation or succession 10.6.4 — obligation to preserve Permanent Invariants and minority exit is not epoch parameter

Implementation: governance.rs is_forbidden_by_article_4() Err on parent-hash mismatch or non-descending constitution_hash + Article 7.1 log timestamp, action, evidence_hash, Constitution Article cite, triggering metrics + watchdog partitions still-anchored (canonical) vs broken-anchor sets

# HUNDRED-YEAR TO THOUSAND-YEAR CONTINUITY - v0.52 TWEAK LOCKED
Per whitepaper v0.52 Section 10.6.4: Succession model is mechanism by which Nexus lineage intended to remain coherent across centuries, not merely decade. Each embodiment carries finite declared horizon_blocks window scoped falsifiable not permanent runtime identity. When window reached or participants judge embodiment no longer viable, explicit succession expected rather than indefinite patching. Lineage persists because every valid successor must cryptographically prove descent from genesis_root and Constitution hash chain, continue to enforce four Permanent Invariants (Evidence-First, NAP, minority exit, cryptographic anchoring), preserve last valid state root, all historical Constitution versions, all anchored AI outputs, original assets under NAP invariants. Active tending — continuous participation, resource contribution, serving audit data — practical maintenance that keeps embodiment alive. Without it entropy prevails as with any living system. Design assumes embodiments will be tended, upgraded, eventually composted, re-embodied. Permanent object is constitutional lineage, not any single codebase or parameter set. Not claim present implementation will run forever. Claim properly tended lineage of implementations each honest about own horizon can remain faithful to same invariants for as long as participants choose to continue it. — Troy Snider — 87/70 — Warehouses die, mycelium lives — All systems breakable will eventually run course like everything else in nature, built so upgrades can be made and machine can last 100s to 1000s years with proper tending/maintenance.
