use serde::{Deserialize, Serialize};

/// The state machine for the DTaP transaction lifecycle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BridgeState {
    /// Initial state, waiting for user input.
    Idle,
    /// User has inputs amount and clicked pay, validating the non-custodial crypto tx on-chain.
    AssetLockPending,
    /// On-chain finality reached, now routing liquidity via JIT API.
    LiquidityRouting,
    /// Voucher code obtained, translating to structured payload.
    TokenMints,
    /// Payload pushed to mobile screen, rendering barcode.
    ActiveBarcode(String),
    /// Transaction completed, POS cleared.
    Completed,
    /// Terminal state for any failure.
    Failed(String),
}

/// Request structure for initiating a JIT bridge swap.
#[derive(Debug, Deserialize)]
pub struct BridgeRequest {
    /// The transaction hash of the user's asset lock on-chain.
    pub tx_hash: String,
    /// Merchant identifier for the voucher provider.
    pub merchant_id: String,
    /// Amount in fiat currency (e.g., USD).
    pub amount_fiat: f64,
    /// Currency code (e.g., USD, EUR).
    pub currency: String,
}

/// Response structure from the JIT router, indicating the next state and optional voucher payload.
#[derive(Debug, Serialize)]
pub struct BridgeResponse {
    /// The target state after processing this request.
    pub target_state: BridgeState,
    /// The voucher payload (e.g., barcode serial/PIN) if applicable.
    pub voucher_payload: Option<String>,
}