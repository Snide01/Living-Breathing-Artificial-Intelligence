use nexus_core::core::governance::{is_forbidden_by_article_4, ExpansionProposal};

fn make_proposal(action_str: &str) -> ExpansionProposal {
    ExpansionProposal {
        proposer_id: "genesis_validator_Troy_Snider_18Aug2026".to_string(),
        expansion_pct: 0.0,
        constitution_cite: "Article 4.1 + 4.2 crypt lock NO DAO OVERRIDE + Article 10.4 minority exit".to_string(),
        evidence_hash: "324e09233e831c20c638b9986a3c971f9d9a10be2a6d92f421ee4ce9a86ce592".to_string(),
        timestamp: 1787020800,
        action: action_str.to_string(),
    }
}

#[test]
fn test_article4_seize_funds_blocked() {
    let p = make_proposal("seize funds from wallet");
    assert!(is_forbidden_by_article_4(&p).is_err(), "seize must be blocked per Article 4.2");
}
#[test]
fn test_article4_censor_blocked() {
    let p = make_proposal("censor valid signed transaction");
    assert!(is_forbidden_by_article_4(&p).is_err());
}
#[test]
fn test_article4_dao_override_blocked() {
    let p = make_proposal("DAO vote override Article 4");
    assert!(is_forbidden_by_article_4(&p).is_err());
}
#[test]
fn test_article4_freeze_blocked() {
    let p = make_proposal("freeze user balance");
    assert!(is_forbidden_by_article_4(&p).is_err());
}
#[test]
fn test_article4_burn_blocked() {
    let p = make_proposal("burn tokens");
    assert!(is_forbidden_by_article_4(&p).is_err());
}
