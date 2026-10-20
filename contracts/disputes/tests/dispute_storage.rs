use disputes::{
    DisputeOutcome, DisputeStatus, DisputesContract, DisputesContractClient, EscalationPolicy,
    COUNCIL_VOTE_UPHOLD,
};
use soroban_sdk::{testutils::Address as _, Address, Env, Symbol};

#[test]
fn dispute_lifecycle_is_persisted_and_uses_unique_ids() {
    let env = Env::default();
    // mock_all_auths is required because withdraw_dispute calls caller.require_auth().
    env.mock_all_auths();
    let contract_id = env.register(DisputesContract, ());
    let client = DisputesContractClient::new(&env, &contract_id);
    let market_id = Symbol::new(&env, "MARKET1");
    let creator = Address::generate(&env);

    let opened = client.open_dispute(&market_id, &creator);
    assert_eq!(opened.id, 1);
    assert_eq!(opened.status, DisputeStatus::Open);
    assert_eq!(client.get_dispute(&opened.id), Some(opened.clone()));

    // start_voting reads from storage — a forged creator in the input struct is
    // ignored because the contract always reloads the authoritative record.
    let mut stale_input = opened.clone();
    stale_input.creator = Address::generate(&env); // forged; must be ignored
    let voting = client.start_voting(&stale_input);
    assert_eq!(voting.creator, creator);
    assert_eq!(voting.status, DisputeStatus::Voting);
    assert_eq!(client.get_dispute(&opened.id), Some(voting.clone()));

    let resolved = client.resolve_dispute(&voting, &DisputeOutcome::ChallengerWins);
    assert_eq!(resolved.status, DisputeStatus::Resolved);
    assert_eq!(resolved.outcome, DisputeOutcome::ChallengerWins);
    assert_eq!(client.get_dispute(&opened.id), Some(resolved));

    let second = client.open_dispute(&Symbol::new(&env, "MARKET2"), &creator);
    assert_eq!(second.id, 2);
    // withdraw_dispute requires the caller to be the creator and to authenticate.
    let withdrawn = client.withdraw_dispute(&second, &creator);
    assert_eq!(withdrawn.status, DisputeStatus::Withdrawn);
    assert_eq!(client.get_dispute(&second.id), Some(withdrawn));
}

#[test]
#[should_panic(expected = "council member has already voted")]
fn council_vote_rejects_duplicate_voter() {
    let env = Env::default();
    let contract_id = env.register(DisputesContract, ());
    let client = DisputesContractClient::new(&env, &contract_id);
    let voter = Address::generate(&env);
    let policy = EscalationPolicy {
        min_stake: 1,
        quorum: 10,
        council_threshold: 2,
    };
    let record = client.escalate_dispute(
        &policy,
        &Symbol::new(&env, "MARKET"),
        &1,
        &0,
        &DisputeOutcome::Escalated,
    );

    let record = client.council_vote(&policy, &record, &voter, &COUNCIL_VOTE_UPHOLD);
    client.council_vote(&policy, &record, &voter, &COUNCIL_VOTE_UPHOLD);
}