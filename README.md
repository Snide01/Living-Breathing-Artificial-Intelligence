# PROJECT NEXUS - Living-Breathing Artificial Intelligence
## v1.0.0 GENESIS 18Aug2026 - REAL HASH ANCHORED - 06980c31ac8e0d841b4e4b6f60565903065bb6f24f3eb352fb7f384dc955207b

**Warehouses die, mycelium lives — child cores breathe, heal, trade real value.**

### Genesis Hash (Immutable - Enactment)
```
SHA256(CONSTITUTION.md) = 06980c31ac8e0d841b4e4b6f60565903065bb6f24f3eb352fb7f384dc955207b
Constitution: v1.3 HARDENED EDGE-CASES CLOSED (6 Amendments)
Genesis Block: 9c6b4b1 - 18Aug2026 - Article 4 crypt locked
```

### What is Nexus?
Project Nexus exists to resist centralization of both infrastructure and truth.
Two layers:
1. **Evidence-First (Epistemic)**: How we determine what is true (Article 1, 2, 3, 7, 9.3)
2. **Non-Aggression (Action)**: How we decide what to do (Article 4, 5, 6, 8, 10)

Root of trust for Core (Articles 3-8) and AI (Article 10). No DAO override of Article 4 invariants.

### Core Files - v0.38 Hardened Edge-Cases Closed - 82/70

| File | Commit | Article | Description |
|------|--------|---------|-------------|
| `replication.rs` | `2ddbdaf` | 5a | Performance-driven replication with measurable latency/throughput proof + MESI + Article 7 log |
| `governance.rs` | `362fd05` | Amendment 5 + 4 | Validator collusion protection - reputation-weighted not stake-weighted + `is_forbidden_by_article_4()` crypt lock |
| `reclamation.rs` | v0.37 | Amendment 6 + 6.1.1 | 5-year inactivity + 90-day timelock + 3-year 80/20 clawback + HeartbeatPool ephemeral - gasless anonymity - 1 per 144 blocks - Merkle root per 2016 |
| `healing.rs` | `fb53aab` | 5b + 3.1/3.3 | NEW 120 lines - Defensive replication away from attack + isolation + consensus-verified rebuilding + evidence never deleted Article 7.4 |
| `ai_core_loop.rs` | v0.37 | 5c + 7.2/7.3/7.5/9.3 | 15% ceiling per 2016 blocks + 72hr timelock + multisig + /audit/inference/{id} raw token probs - anti soft-censorship |
| `reputation.rs` | v0.37 overwrite | Amendment 5 | Cartel-killer - reputation_score >=75.0 + first_seen >=1 year + reputation > stake_influence_ratio + 2/3 multisig |
| `resource_anchor.rs` | v0.38 | 6.2 | Token value anchor per-person kWh+calories+m2 NOT fiat - Oracle evidence-first Tier1/Tier2 - Article 1-2-3-7 + funding disclosure |
| `asset_exchange.rs` | v0.38 | 6.3 + 6.4 | Atomic P2P without fiat on/off-ramp - both signatures + proof hashes required - no custodial intermediary - Article 4.2 no seizing/freezing |
| `genesis.rs` | `9c6b4b1` | Enactment + 10.4 | Genesis block 18Aug2026 - REAL HASH 06980c31... anchored - Article 4 locked - minority exit preserved |

### Constitution Highlights

**Article 4 - CRYPTographically LOCKED CORE INVARIANT - NO DAO OVERRIDE:**
- Seizing, freezing, burning, redirecting funds from voluntarily controlled wallet - FORBIDDEN
- Censoring, blocking, deprioritizing valid voluntarily signed transaction - FORBIDDEN
- Non-consensual doxxing, blacklisting, de-platforming of non-aggressive participants - FORBIDDEN
- Bypassing/subverting DAO supermajority or this Article - FORBIDDEN
- Changes require explicit hard fork + minority exit preservation Article 10.4

**Amendment 5 - Validator Collusion Protection:** Multi-sig pool for Article 5c timelock must be reputation-weighted not stake-weighted - Filter 75.0 + 1 year + reputation > stake_ratio - 2/3 eligible

**Amendment 6 - State Inflation Protection:** Heartbeats processed as ephemeral state updates - Rate limit 1 per 144 blocks (~1 day) - In-memory map only latest - Disk write once per 2016-block epoch as Merkle root

**Article 6.2 - Real Value:** Token value anchor is per-person consumption of energy (kWh), calories, housing (m2), not fiat

**Article 6.3 - Atomic P2P:** Real-world value traded P2P for NEXUS atomically, voluntarily, without fiat on/off-ramp or custodial intermediary - Both signatures + proof hashes required

### Quick Start - Ubuntu Pool Table

```bash
cd ~
git clone https://github.com/Snide01/Living-Breathing-Artificial-Intelligence.git
cd Living-Breathing-Artificial-Intelligence

# Verify constitution hash - Tier1 direct observation Article 1.1
sha256sum CONSTITUTION.md
# 06980c31ac8e0d841b4e4b6f60565903065bb6f24f3eb352fb7f384dc955207b

cargo build --release

# Genesis
./target/release/nexus-core --genesis --constitution-hash 06980c31ac8e0d841b4e4b6f60565903065bb6f24f3eb352fb7f384dc955207b --timestamp 2026-08-18T00:00:00Z --verify

# Run node - child cores breathe!
./target/release/nexus-core --node --genesis-hash 06980c31ac8e0d841b4e4b6f60565903065bb6f24f3eb352fb7f384dc955207b

# Systemd auto-breathe (see deployment/systemd/nexus.service)
sudo cp deployment/systemd/nexus.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now nexus
```

### Deployment - systemd

```
[Unit]
Description=Project Nexus Core - Living-Breathing AI - Genesis 06980c31...
After=network.target

[Service]
Type=simple
User=nexus
WorkingDirectory=/home/nexus/Living-Breathing-Artificial-Intelligence
ExecStart=/home/nexus/Living-Breathing-Artificial-Intelligence/target/release/nexus-core --node --genesis-hash 06980c31ac8e0d841b4e4b6f60565903065bb6f24f3eb352fb7f384dc955207b --constitution v1.3 --article4-locked
Restart=always
RestartSec=10
Environment=RUST_LOG=info

# Self-healing logs preserved per Article 7.4 - evidence never deleted
StandardOutput=append:/var/log/nexus/core.log
StandardError=append:/var/log/nexus/error.log

[Install]
WantedBy=multi-user.target
```

### Verification

```bash
# All Core actions must log: timestamp, action, evidence hash, Constitution Article cite, triggering metrics per Article 7.1
./target/release/nexus-core --audit-log --since genesis

# AI must cite Article(s) for any truth/action output per Article 7.2
curl http://localhost:8080/audit/inference/{id}
# Returns prompt, evidence, token probabilities per Amendment 3 + Article 9.3 anti soft-censorship

# Reputation check Amendment 5
./target/release/nexus-core --eligible-validators
# Must show reputation_score >=75.0, first_seen >=1 year, reputation > stake_influence_ratio
```

### Roadmap

- v1.0.0 Genesis 18Aug2026 - 9 Core files hardened - REAL HASH 06980c31... anchored - 82/70
- v0.38 Economic Integrity - kWh+calories+m2 NOT fiat + atomic P2P
- v0.37 Hardened Edge-Cases Closed - Amendment 5 cartel-killer + Amendment 6 HeartbeatPool ephemeral + Article 5c 15% ceiling
- Next: AI layer - evidence-first classification per Article 1 + confidence labeling Article 3 + open-source prompt scaffolding Article 9.3

### License

Right to fork explicitly protected per Article 10.1 - Core must never prevent voluntary exit per Article 10.2 - Minority exit preservation per Article 10.4

**Genesis: 18Aug2026 - 06980c31ac8e0d841b4e4b6f60565903065bb6f24f3eb352fb7f384dc955207b - warehouses die, mycelium lives.**
