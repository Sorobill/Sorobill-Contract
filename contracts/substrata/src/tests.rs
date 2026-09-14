#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env, String,
};

use crate::{errors::SubstrataError, types::{BillingInterval, BillingOutcome}, SubstrataContract, SubstrataContractClient};

fn plan_name(e: &Env) -> String {
    String::from_str(e, "Pro Plan")
}

fn setup() -> (Env, SubstrataContractClient<'static>, Address, Address, Address) {
    let e = Env::default();
    e.mock_all_auths();

    let contract_id = e.register(SubstrataContract, ());
    let client = SubstrataContractClient::new(&e, &contract_id);

    let admin = Address::generate(&e);
    let merchant = Address::generate(&e);
    let subscriber = Address::generate(&e);

    client.initialize(&admin);

    (e, client, admin, merchant, subscriber)
}

fn setup_token(
    e: &Env,
    contract_id: &Address,
    to: &Address,
    amount: i128,
) -> Address {
    let token_admin = Address::generate(e);
    let token_id = e.register_stellar_asset_contract_v2(token_admin.clone());
    let token_addr = token_id.address();

    let asset_client = StellarAssetClient::new(e, &token_addr);
    asset_client.mint(to, &amount);

    let token_client = TokenClient::new(e, &token_addr);
    token_client.approve(to, contract_id, &amount, &(e.ledger().sequence() + 10_000));

    token_addr
}

#[test]
fn test_create_plan() {
    let (e, client, _admin, merchant, _sub) = setup();
    let token = Address::generate(&e);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &100,
        &BillingInterval::Monthly,
        &token,
    );

    assert_eq!(plan_id, 0);
    let plan = client.get_plan(&plan_id);
    assert_eq!(plan.price, 100);
    assert!(plan.active);
    assert_eq!(client.plan_count(), 1);
}

#[test]
fn test_create_plan_invalid_price() {
    let (e, client, _admin, merchant, _sub) = setup();
    let token = Address::generate(&e);

    let err = client
        .try_create_plan(&merchant, &plan_name(&e), &0, &BillingInterval::Monthly, &token)
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SubstrataError::InvalidPrice.into());
}

#[test]
fn test_create_plan_empty_name() {
    let (e, client, _admin, merchant, _sub) = setup();
    let token = Address::generate(&e);
    let empty = String::from_str(&e, "");

    let err = client
        .try_create_plan(&merchant, &empty, &100, &BillingInterval::Monthly, &token)
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SubstrataError::InvalidPlanName.into());
}

#[test]
fn test_update_plan_price() {
    let (e, client, _admin, merchant, _sub) = setup();
    let token = Address::generate(&e);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &100,
        &BillingInterval::Monthly,
        &token,
    );

    client.update_plan_price(&merchant, &plan_id, &200);
    assert_eq!(client.get_plan(&plan_id).price, 200);
}

#[test]
fn test_deactivate_plan_blocks_subscribe() {
    let (e, client, _admin, merchant, subscriber) = setup();
    let token = Address::generate(&e);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &100,
        &BillingInterval::Monthly,
        &token,
    );

    client.deactivate_plan(&merchant, &plan_id);

    let err = client
        .try_subscribe(&subscriber, &plan_id)
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SubstrataError::PlanInactive.into());
}

#[test]
fn test_reactivate_plan() {
    let (e, client, _admin, merchant, subscriber) = setup();
    let token = Address::generate(&e);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &100,
        &BillingInterval::Monthly,
        &token,
    );

    client.deactivate_plan(&merchant, &plan_id);
    client.reactivate_plan(&merchant, &plan_id);
    client.subscribe(&subscriber, &plan_id);
    assert!(client.get_subscription(&subscriber, &plan_id).active);
}

#[test]
fn test_subscribe_and_get() {
    let (e, client, _admin, merchant, subscriber) = setup();
    let token = Address::generate(&e);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &100,
        &BillingInterval::Monthly,
        &token,
    );

    client.subscribe(&subscriber, &plan_id);

    let sub = client.get_subscription(&subscriber, &plan_id);
    assert!(sub.active);
    assert!(!sub.paused);
    assert_eq!(sub.failed_attempts, 0);
}

#[test]
fn test_double_subscribe_fails() {
    let (e, client, _admin, merchant, subscriber) = setup();
    let token = Address::generate(&e);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &100,
        &BillingInterval::Monthly,
        &token,
    );

    client.subscribe(&subscriber, &plan_id);

    let err = client
        .try_subscribe(&subscriber, &plan_id)
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SubstrataError::AlreadySubscribed.into());
}

#[test]
fn test_cancel_subscription() {
    let (e, client, _admin, merchant, subscriber) = setup();
    let token = Address::generate(&e);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &100,
        &BillingInterval::Monthly,
        &token,
    );

    client.subscribe(&subscriber, &plan_id);
    client.cancel(&subscriber, &plan_id);

    let sub = client.get_subscription(&subscriber, &plan_id);
    assert!(!sub.active);
}

#[test]
fn test_pause_and_resume() {
    let (e, client, _admin, merchant, subscriber) = setup();
    let token = Address::generate(&e);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &100,
        &BillingInterval::Monthly,
        &token,
    );

    client.subscribe(&subscriber, &plan_id);
    client.pause(&subscriber, &plan_id);

    assert!(client.get_subscription(&subscriber, &plan_id).paused);

    client.resume(&subscriber, &plan_id);
    assert!(!client.get_subscription(&subscriber, &plan_id).paused);
}

#[test]
fn test_billing_not_due() {
    let (e, client, admin, merchant, subscriber) = setup();
    let contract_id = client.address.clone();
    let token = setup_token(&e, &contract_id, &subscriber, 1_000);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &100,
        &BillingInterval::Monthly,
        &token,
    );

    client.subscribe(&subscriber, &plan_id);

    let err = client
        .try_execute_billing(&admin, &subscriber, &plan_id)
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SubstrataError::BillingNotDue.into());
}

#[test]
fn test_successful_billing() {
    let (e, client, admin, merchant, subscriber) = setup();
    let contract_id = client.address.clone();
    let token = setup_token(&e, &contract_id, &subscriber, 1_000);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &100,
        &BillingInterval::Monthly,
        &token,
    );

    client.subscribe(&subscriber, &plan_id);

    e.ledger().with_mut(|l| {
        l.timestamp += BillingInterval::Monthly.as_secs() + 1;
    });

    client.execute_billing(&admin, &subscriber, &plan_id);

    let token_client = TokenClient::new(&e, &token);
    assert_eq!(token_client.balance(&merchant), 100);
    assert_eq!(token_client.balance(&subscriber), 900);
}

#[test]
fn test_insufficient_balance_increments_failed_attempts() {
    let (e, client, admin, merchant, subscriber) = setup();
    let contract_id = client.address.clone();
    let token = setup_token(&e, &contract_id, &subscriber, 50);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &100,
        &BillingInterval::Monthly,
        &token,
    );

    client.subscribe(&subscriber, &plan_id);

    e.ledger().with_mut(|l| {
        l.timestamp += BillingInterval::Monthly.as_secs() + 1;
    });

    let outcome = client.execute_billing(&admin, &subscriber, &plan_id);
    assert_eq!(outcome, BillingOutcome::Failed);
    assert_eq!(
        client.get_subscription(&subscriber, &plan_id).failed_attempts,
        1
    );
}

#[test]
fn test_three_failures_auto_cancel() {
    let (e, client, admin, merchant, subscriber) = setup();
    let contract_id = client.address.clone();
    let token = setup_token(&e, &contract_id, &subscriber, 50);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &100,
        &BillingInterval::Daily,
        &token,
    );

    client.subscribe(&subscriber, &plan_id);

    for _ in 0..3 {
        e.ledger().with_mut(|l| {
            l.timestamp += BillingInterval::Daily.as_secs() + 1;
        });
        let outcome = client.execute_billing(&admin, &subscriber, &plan_id);
        assert_eq!(outcome, BillingOutcome::Failed);
    }

    let sub = client.get_subscription(&subscriber, &plan_id);
    assert!(!sub.active, "subscription should be auto-cancelled after 3 failures");
}

#[test]
fn test_billing_blocked_while_paused() {
    let (e, client, admin, merchant, subscriber) = setup();
    let contract_id = client.address.clone();
    let token = setup_token(&e, &contract_id, &subscriber, 1_000);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &100,
        &BillingInterval::Monthly,
        &token,
    );

    client.subscribe(&subscriber, &plan_id);
    client.pause(&subscriber, &plan_id);

    e.ledger().with_mut(|l| {
        l.timestamp += BillingInterval::Monthly.as_secs() + 1;
    });

    let err = client
        .try_execute_billing(&admin, &subscriber, &plan_id)
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SubstrataError::SubscriptionPaused.into());
}

#[test]
fn test_unauthorized_billing_rejected() {
    let (e, client, _admin, merchant, subscriber) = setup();
    let contract_id = client.address.clone();
    let token = setup_token(&e, &contract_id, &subscriber, 1_000);
    let rando = Address::generate(&e);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &100,
        &BillingInterval::Monthly,
        &token,
    );

    client.subscribe(&subscriber, &plan_id);

    e.ledger().with_mut(|l| {
        l.timestamp += BillingInterval::Monthly.as_secs() + 1;
    });

    let err = client
        .try_execute_billing(&rando, &subscriber, &plan_id)
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SubstrataError::Unauthorized.into());
}

#[test]
fn test_get_admin() {
    let (_e, client, admin, _merchant, _sub) = setup();
    assert_eq!(client.get_admin(), admin);
}

#[test]
fn test_double_initialize_fails() {
    let (e, client, _admin, _merchant, _sub) = setup();
    let other = Address::generate(&e);
    let err = client.try_initialize(&other).unwrap_err().unwrap();
    assert_eq!(err, SubstrataError::AlreadyInitialized.into());
}

#[test]
fn test_custom_interval_zero_rejected() {
    let (e, client, _admin, merchant, _sub) = setup();
    let token = Address::generate(&e);

    let err = client
        .try_create_plan(
            &merchant,
            &plan_name(&e),
            &100,
            &BillingInterval::Custom(0),
            &token,
        )
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SubstrataError::InvalidInterval.into());
}

#[test]
fn test_unauthorized_price_update() {
    let (e, client, _admin, merchant, _sub) = setup();
    let token = Address::generate(&e);
    let other = Address::generate(&e);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &100,
        &BillingInterval::Weekly,
        &token,
    );

    let err = client
        .try_update_plan_price(&other, &plan_id, &50)
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SubstrataError::Unauthorized.into());
}


#[test]
fn test_custom_interval_billing() {
    let (e, client, admin, merchant, subscriber) = setup();
    let contract_id = client.address.clone();
    let token = setup_token(&e, &contract_id, &subscriber, 1_000);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &25,
        &BillingInterval::Custom(60),
        &token,
    );

    client.subscribe(&subscriber, &plan_id);
    e.ledger().with_mut(|l| {
        l.timestamp += 61;
    });
    let outcome = client.execute_billing(&admin, &subscriber, &plan_id);
    assert_eq!(outcome, BillingOutcome::Paid);
}
