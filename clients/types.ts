/**
 * Thin TypeScript helpers for invoking the Sorobill Soroban contract.
 * Used by Sorobill-Backend and Sorobill-App.
 */

export const SOROBILL_CONTRACT_METHODS = [
  "initialize",
  "get_admin",
  "version",
  "set_grace_period",
  "get_grace_period",
  "plan_count",
  "is_subscribed",
  "create_plan",
  "update_plan_price",
  "deactivate_plan",
  "reactivate_plan",
  "get_plan",
  "subscribe",
  "cancel",
  "pause",
  "resume",
  "get_subscription",
  "execute_billing",
] as const;

export type SorobillMethod = (typeof SOROBILL_CONTRACT_METHODS)[number];

export type BillingInterval =
  | { tag: "Daily" }
  | { tag: "Weekly" }
  | { tag: "Monthly" }
  | { tag: "Yearly" }
  | { tag: "Custom"; values: [bigint] };

export type BillingOutcome = { tag: "Paid" } | { tag: "Failed" };

export interface PlanView {
  merchant: string;
  name: string;
  description: string;
  price: bigint;
  interval: BillingInterval;
  token: string;
  active: boolean;
}

export interface SubscriptionView {
  subscriber: string;
  planId: bigint;
  nextBilling: bigint;
  active: boolean;
  failedAttempts: number;
  lastCharged: bigint;
  paused: boolean;
  graceDeadline: bigint;
}

export const CONTRACT_VERSION = "0.3.0";
export const DEFAULT_GRACE_SECS = 172_800;
export const MAX_FAILED_ATTEMPTS = 3;

export const TESTNET = {
  networkPassphrase: "Test SDF Network ; September 2015",
  rpcUrl: "https://soroban-testnet.stellar.org",
  horizonUrl: "https://horizon-testnet.stellar.org",
  demoUi: "https://sorobill-app.vercel.app",
} as const;
