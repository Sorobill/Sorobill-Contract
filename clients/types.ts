/**
 * Thin TypeScript helpers for invoking the Sorobill Soroban contract.
 * Used by Sorobill-Backend and Sorobill-App.
 */

export const SOROBILL_CONTRACT_METHODS = [
  "initialize",
  "get_admin",
  "plan_count",
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

export interface PlanView {
  merchant: string;
  name: string;
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
}

export const TESTNET = {
  networkPassphrase: "Test SDF Network ; September 2015",
  rpcUrl: "https://soroban-testnet.stellar.org",
  horizonUrl: "https://horizon-testnet.stellar.org",
} as const;
