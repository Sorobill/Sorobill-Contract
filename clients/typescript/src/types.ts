/** Hand-written stubs mirroring Sorobill 0.3.0 entrypoints. */

export type BillingInterval =
  | { tag: "Daily" }
  | { tag: "Weekly" }
  | { tag: "Monthly" }
  | { tag: "Yearly" }
  | { tag: "Custom"; values: [bigint] };

export type BillingOutcome = { tag: "Paid" } | { tag: "Failed" };

export interface Plan {
  merchant: string;
  name: string;
  description: string;
  price: bigint;
  interval: BillingInterval;
  token: string;
  active: boolean;
}

export interface Subscription {
  subscriber: string;
  plan_id: bigint;
  next_billing: bigint;
  active: boolean;
  failed_attempts: number;
  last_charged: bigint;
  paused: boolean;
  grace_deadline: bigint;
}

export const CONTRACT_VERSION = "0.3.0";
export const DEFAULT_GRACE_SECS = 172_800;
export const MAX_FAILED_ATTEMPTS = 3;

/** Method names for RPC / binding generation cross-checks. */
export const METHODS = [
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
