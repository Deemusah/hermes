use disputes::{
    DisputeOutcome, DisputeStatus, DisputesContract, DisputesContractClient,
    EscalationPolicy, COUNCIL_VOTE_UPHOLD,
};
use soroban_sdk::{Env, Symbol};

#[test]
fn dispute_lifecycle_is_persisted_and_uses_unique_ids() {
    let env = Env::default();
    let contract_id = env.register(DisputesContract, ());
    let client = DisputesContractClient::new(&env, &contract_id);
    let market_id = Symbol::new(&env, "MARKET1");
    let creator = Symbol::new(&env, "CREATOR");

    let opened = client.open_dispute(&market_id, &creator);
    assert_eq!(opened.id, 1);
    assert_eq!(opened.status, DisputeStatus::Open);
    assert_eq!(client.get_dispute(&opened.id), Some(opened.clone()));

    let mut stale_input = opened.clone();
    stale_input.creator = Symbol::new(&env, "FORGED");
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
    let withdrawn = client.withdraw_dispute(&second, &creator);
    assert_eq!(withdrawn.status, DisputeStatus::Withdrawn);
    assert_eq!(client.get_dispute(&second.id), Some(withdrawn));
}

#[test]
fn council_votes_accumulate_in_persisted_escalation_record() {
    let env = Env::default();
    let contract_id = env.register(DisputesContract, ());
    let client = DisputesContractClient::new(&env, &contract_id);
    let market_id = Symbol::new(&env, "MARKET1");
    let policy = EscalationPolicy {
        min_stake: 100,
        quorum: 10,
        council_threshold: 2,
    };

    let escalation = client.escalate_dispute(
        &policy,
        &market_id,
        &100,
        &9,
        &DisputeOutcome::ChallengerWins,
    );
    assert_eq!(client.get_escalation(&market_id), Some(escalation.clone()));

    let first_vote = client.council_vote(&policy, &escalation, &COUNCIL_VOTE_UPHOLD);
    assert_eq!(first_vote.council_votes, 1);
    assert_eq!(first_vote.uphold_votes, 1);
    assert!(!first_vote.resolved);

    // A stale caller-supplied record must not discard the vote already stored.
    let second_vote = client.council_vote(&policy, &escalation, &COUNCIL_VOTE_UPHOLD);
    assert_eq!(second_vote.council_votes, 2);
    assert_eq!(second_vote.uphold_votes, 2);
    assert!(second_vote.resolved);
    assert_eq!(client.get_escalation(&market_id), Some(second_vote));
}