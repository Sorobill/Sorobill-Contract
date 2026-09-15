#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env, String,
};

use crate::{
    constants::{CONTRACT_VERSION, DEFAULT_GRACE_SECS, MAX_FAILED_ATTEMPTS},
    errors::SorobillError,
    types::{BillingInterval, BillingOutcome},
    SorobillContract, SorobillContractClient,
};

fn plan_name(e: &Env) -> String {
    String::from_str(e, "Pro Plan")
}

fn plan_desc(e: &Env) -> String {
    String::from_str(e, "A recurring subscription plan")
}

fn setup() -> (Env, SorobillContractClient<'static>, Address, Address, Address) {
    let e = Env::default();
    e.mock_all_auths();

    let contract_id = e.register(SorobillContract, ());
    let client = SorobillContractClient::new(&e, &contract_id);

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

fn create_monthly_plan(
    e: &Env,
    client: &SorobillContractClient<'_>,
    merchant: &Address,
    token: &Address,
    price: i128,
) -> u64 {
    client.create_plan(
        merchant,
        &plan_name(e),
        &plan_desc(e),
        &price,
        &BillingInterval::Monthly,
        token,
    )
}

#[test]
fn test_create_plan() {
    let (e, client, _admin, merchant, _sub) = setup();
    let token = Address::generate(&e);

    let plan_id = create_monthly_plan(&e, &client, &merchant, &token, 100);

    assert_eq!(plan_id, 0);
    let plan = client.get_plan(&plan_id);
    assert_eq!(plan.price, 100);
    assert!(plan.active);
    assert_eq!(plan.description, plan_desc(&e));
    assert_eq!(client.plan_count(), 1);
}

#[test]
fn test_create_plan_invalid_price() {
    let (e, client, _admin, merchant, _sub) = setup();
    let token = Address::generate(&e);

    let err = client
        .try_create_plan(
            &merchant,
            &plan_name(&e),
            &plan_desc(&e),
            &0,
            &BillingInterval::Monthly,
            &token,
        )
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SorobillError::InvalidPrice.into());
}

#[test]
fn test_create_plan_empty_name() {
    let (e, client, _admin, merchant, _sub) = setup();
    let token = Address::generate(&e);
    let empty = String::from_str(&e, "");

    let err = client
        .try_create_plan(
            &merchant,
            &empty,
            &plan_desc(&e),
            &100,
            &BillingInterval::Monthly,
            &token,
        )
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SorobillError::InvalidPlanName.into());
}

#[test]
fn test_update_plan_price() {
    let (e, client, _admin, merchant, _sub) = setup();
    let token = Address::generate(&e);

    let plan_id = create_monthly_plan(&e, &client, &merchant, &token, 100);

    client.update_plan_price(&merchant, &plan_id, &200);
    assert_eq!(client.get_plan(&plan_id).price, 200);
}

#[test]
fn test_deactivate_plan_blocks_subscribe() {
    let (e, client, _admin, merchant, subscriber) = setup();
    let token = Address::generate(&e);

    let plan_id = create_monthly_plan(&e, &client, &merchant, &token, 100);

    client.deactivate_plan(&merchant, &plan_id);

    let err = client
        .try_subscribe(&subscriber, &plan_id)
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SorobillError::PlanInactive.into());
}

#[test]
fn test_reactivate_plan() {
    let (e, client, _admin, merchant, subscriber) = setup();
    let token = Address::generate(&e);

    let plan_id = create_monthly_plan(&e, &client, &merchant, &token, 100);

    client.deactivate_plan(&merchant, &plan_id);
    client.reactivate_plan(&merchant, &plan_id);
    client.subscribe(&subscriber, &plan_id);
    assert!(client.get_subscription(&subscriber, &plan_id).active);
}

#[test]
fn test_subscribe_and_get() {
    let (e, client, _admin, merchant, subscriber) = setup();
    let token = Address::generate(&e);

    let plan_id = create_monthly_plan(&e, &client, &merchant, &token, 100);

    client.subscribe(&subscriber, &plan_id);

    let sub = client.get_subscription(&subscriber, &plan_id);
    assert!(sub.active);
    assert!(!sub.paused);
    assert_eq!(sub.failed_attempts, 0);
    assert_eq!(sub.grace_deadline, 0);
}

#[test]
fn test_double_subscribe_fails() {
    let (e, client, _admin, merchant, subscriber) = setup();
    let token = Address::generate(&e);

    let plan_id = create_monthly_plan(&e, &client, &merchant, &token, 100);

    client.subscribe(&subscriber, &plan_id);

    let err = client
        .try_subscribe(&subscriber, &plan_id)
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SorobillError::AlreadySubscribed.into());
}

#[test]
fn test_cancel_subscription() {
    let (e, client, _admin, merchant, subscriber) = setup();
    let token = Address::generate(&e);

    let plan_id = create_monthly_plan(&e, &client, &merchant, &token, 100);

    client.subscribe(&subscriber, &plan_id);
    client.cancel(&subscriber, &plan_id);

    let sub = client.get_subscription(&subscriber, &plan_id);
    assert!(!sub.active);
}

#[test]
fn test_pause_and_resume() {
    let (e, client, _admin, merchant, subscriber) = setup();
    let token = Address::generate(&e);

    let plan_id = create_monthly_plan(&e, &client, &merchant, &token, 100);

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

    let plan_id = create_monthly_plan(&e, &client, &merchant, &token, 100);

    client.subscribe(&subscriber, &plan_id);

    let err = client
        .try_execute_billing(&admin, &subscriber, &plan_id)
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SorobillError::BillingNotDue.into());
}

#[test]
fn test_successful_billing() {
    let (e, client, admin, merchant, subscriber) = setup();
    let contract_id = client.address.clone();
    let token = setup_token(&e, &contract_id, &subscriber, 1_000);

    let plan_id = create_monthly_plan(&e, &client, &merchant, &token, 100);

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

    let plan_id = create_monthly_plan(&e, &client, &merchant, &token, 100);

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
fn test_three_failures_enter_grace_by_default() {
    let (e, client, admin, merchant, subscriber) = setup();
    let contract_id = client.address.clone();
    let token = setup_token(&e, &contract_id, &subscriber, 50);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &plan_desc(&e),
        &100,
        &BillingInterval::Daily,
        &token,
    );

    client.subscribe(&subscriber, &plan_id);

    for _ in 0..MAX_FAILED_ATTEMPTS {
        e.ledger().with_mut(|l| {
            l.timestamp += BillingInterval::Daily.as_secs() + 1;
        });
        let outcome = client.execute_billing(&admin, &subscriber, &plan_id);
        assert_eq!(outcome, BillingOutcome::Failed);
    }

    let sub = client.get_subscription(&subscriber, &plan_id);
    assert!(sub.active, "default grace keeps subscription active");
    assert!(sub.grace_deadline > 0);
}

#[test]
fn test_three_failures_auto_cancel_when_grace_zero() {
    let (e, client, admin, merchant, subscriber) = setup();
    let contract_id = client.address.clone();
    let token = setup_token(&e, &contract_id, &subscriber, 50);

    client.set_grace_period(&admin, &0);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &plan_desc(&e),
        &100,
        &BillingInterval::Daily,
        &token,
    );

    client.subscribe(&subscriber, &plan_id);

    for _ in 0..MAX_FAILED_ATTEMPTS {
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

    let plan_id = create_monthly_plan(&e, &client, &merchant, &token, 100);

    client.subscribe(&subscriber, &plan_id);
    client.pause(&subscriber, &plan_id);

    e.ledger().with_mut(|l| {
        l.timestamp += BillingInterval::Monthly.as_secs() + 1;
    });

    let err = client
        .try_execute_billing(&admin, &subscriber, &plan_id)
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SorobillError::SubscriptionPaused.into());
}

#[test]
fn test_unauthorized_billing_rejected() {
    let (e, client, _admin, merchant, subscriber) = setup();
    let contract_id = client.address.clone();
    let token = setup_token(&e, &contract_id, &subscriber, 1_000);
    let rando = Address::generate(&e);

    let plan_id = create_monthly_plan(&e, &client, &merchant, &token, 100);

    client.subscribe(&subscriber, &plan_id);

    e.ledger().with_mut(|l| {
        l.timestamp += BillingInterval::Monthly.as_secs() + 1;
    });

    let err = client
        .try_execute_billing(&rando, &subscriber, &plan_id)
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SorobillError::Unauthorized.into());
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
    assert_eq!(err, SorobillError::AlreadyInitialized.into());
}

#[test]
fn test_custom_interval_zero_rejected() {
    let (e, client, _admin, merchant, _sub) = setup();
    let token = Address::generate(&e);

    let err = client
        .try_create_plan(
            &merchant,
            &plan_name(&e),
            &plan_desc(&e),
            &100,
            &BillingInterval::Custom(0),
            &token,
        )
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SorobillError::InvalidInterval.into());
}

#[test]
fn test_unauthorized_price_update() {
    let (e, client, _admin, merchant, _sub) = setup();
    let token = Address::generate(&e);
    let other = Address::generate(&e);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &plan_desc(&e),
        &100,
        &BillingInterval::Weekly,
        &token,
    );

    let err = client
        .try_update_plan_price(&other, &plan_id, &50)
        .unwrap_err()
        .unwrap();

    assert_eq!(err, SorobillError::Unauthorized.into());
}

#[test]
fn test_custom_interval_billing() {
    let (e, client, admin, merchant, subscriber) = setup();
    let contract_id = client.address.clone();
    let token = setup_token(&e, &contract_id, &subscriber, 1_000);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &plan_desc(&e),
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

#[test]
fn test_plan_count_increments() {
    let (e, client, _admin, merchant, _sub) = setup();
    let token = Address::generate(&e);
    assert_eq!(client.plan_count(), 0);
    client.create_plan(
        &merchant,
        &plan_name(&e),
        &plan_desc(&e),
        &10,
        &BillingInterval::Daily,
        &token,
    );
    client.create_plan(
        &merchant,
        &plan_name(&e),
        &plan_desc(&e),
        &20,
        &BillingInterval::Weekly,
        &token,
    );
    assert_eq!(client.plan_count(), 2);
}

#[test]
fn test_version_matches_constant() {
    let (e, client, _admin, _merchant, _sub) = setup();
    assert_eq!(
        client.version(),
        String::from_str(&e, CONTRACT_VERSION)
    );
}

#[test]
fn test_default_grace_period() {
    let (_e, client, _admin, _merchant, _sub) = setup();
    assert_eq!(client.get_grace_period(), DEFAULT_GRACE_SECS);
}

#[test]
fn test_set_grace_period_admin_only() {
    let (_e, client, admin, merchant, _sub) = setup();

    client.set_grace_period(&admin, &86_400);
    assert_eq!(client.get_grace_period(), 86_400);

    let err = client
        .try_set_grace_period(&merchant, &1)
        .unwrap_err()
        .unwrap();
    assert_eq!(err, SorobillError::Unauthorized.into());
}

#[test]
fn test_is_subscribed() {
    let (e, client, _admin, merchant, subscriber) = setup();
    let token = Address::generate(&e);
    let plan_id = create_monthly_plan(&e, &client, &merchant, &token, 100);

    assert!(!client.is_subscribed(&subscriber, &plan_id));
    client.subscribe(&subscriber, &plan_id);
    assert!(client.is_subscribed(&subscriber, &plan_id));
    client.cancel(&subscriber, &plan_id);
    assert!(!client.is_subscribed(&subscriber, &plan_id));
}

#[test]
fn test_grace_deadline_cancel_after_expiry() {
    let (e, client, admin, merchant, subscriber) = setup();
    let contract_id = client.address.clone();
    let token = setup_token(&e, &contract_id, &subscriber, 50);

    client.set_grace_period(&admin, &3_600);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &plan_desc(&e),
        &100,
        &BillingInterval::Daily,
        &token,
    );
    client.subscribe(&subscriber, &plan_id);

    for _ in 0..MAX_FAILED_ATTEMPTS {
        e.ledger().with_mut(|l| {
            l.timestamp += BillingInterval::Daily.as_secs() + 1;
        });
        assert_eq!(
            client.execute_billing(&admin, &subscriber, &plan_id),
            BillingOutcome::Failed
        );
    }

    let deadline = client.get_subscription(&subscriber, &plan_id).grace_deadline;
    assert!(deadline > 0);
    assert!(client.get_subscription(&subscriber, &plan_id).active);

    e.ledger().with_mut(|l| {
        l.timestamp = deadline;
    });
    let outcome = client.execute_billing(&admin, &subscriber, &plan_id);
    assert_eq!(outcome, BillingOutcome::Failed);
    assert!(!client.get_subscription(&subscriber, &plan_id).active);
}

#[test]
fn test_successful_billing_clears_grace() {
    let (e, client, admin, merchant, subscriber) = setup();
    let contract_id = client.address.clone();
    // Enough for failures then one success after top-up mint path: start with 50, fail 3x,
    // then mint more via new approval after topping balance.
    let token = setup_token(&e, &contract_id, &subscriber, 50);

    client.set_grace_period(&admin, &86_400);

    let plan_id = client.create_plan(
        &merchant,
        &plan_name(&e),
        &plan_desc(&e),
        &100,
        &BillingInterval::Custom(60),
        &token,
    );
    client.subscribe(&subscriber, &plan_id);

    for _ in 0..MAX_FAILED_ATTEMPTS {
        e.ledger().with_mut(|l| {
            l.timestamp += 61;
        });
        assert_eq!(
            client.execute_billing(&admin, &subscriber, &plan_id),
            BillingOutcome::Failed
        );
    }
    assert!(client.get_subscription(&subscriber, &plan_id).grace_deadline > 0);

    // Top up balance and allowance, then bill successfully within grace.
    let asset = StellarAssetClient::new(&e, &token);
    asset.mint(&subscriber, &200);
    TokenClient::new(&e, &token).approve(
        &subscriber,
        &contract_id,
        &200,
        &(e.ledger().sequence() + 10_000),
    );

    e.ledger().with_mut(|l| {
        l.timestamp += 61;
    });
    assert_eq!(
        client.execute_billing(&admin, &subscriber, &plan_id),
        BillingOutcome::Paid
    );
    let sub = client.get_subscription(&subscriber, &plan_id);
    assert_eq!(sub.failed_attempts, 0);
    assert_eq!(sub.grace_deadline, 0);
}
