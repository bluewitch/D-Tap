use crate::errors::{DTaPError, Result};
use crate::types::{BridgeRequest, BridgeResponse, BridgeState};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use reqwest::Client;

/// Mock response structure from the voucher provider API.
/// In practice, this would match the actual provider's response schema.
#[derive(Debug, Deserialize)]
struct VoucherProviderResponse {
    /// The voucher code (e.g., serial number or PIN).
    code: String,
    /// Optional: additional fields like expiration, etc.
    expires_at: Option<String>,
}

/// Handles the JIT proxy bridge logic: verifies ledger settlement, calls voucher provider, returns structured payload.
pub struct JitRouter {
    /// HTTP client for making external API calls.
    pub client: Client,
    /// Base URL of the voucher provider API (e.g., https://api.voucherprovider.com).
    pub provider_url: String,
    /// Authentication token for the provider API.
    pub auth_token: String,
}

impl JitRouter {
    /// Create a new JitRouter instance.
    ///
    /// # Arguments
    ///
    /// * `provider_url` - Base URL of the voucher provider API.
    /// * `auth_token` - Bearer token for authenticating with the provider.
    pub fn new(provider_url: String, auth_token: String) -> Self {
        Self {
            client: Client::new(),
            provider_url,
            auth_token,
        }
    }

    /// Execute the JIT proxy bridge swap.
    ///
    /// This function orchestrates the state transitions:
    /// 1. Validates the user's asset lock on-chain (simulated).
    /// 2. Calls the voucher provider API to mint a voucher.
    /// 3. Returns the next state and the voucher payload.
    ///
    /// # Arguments
    ///
    /// * `request` - The bridge request containing transaction details.
    ///
    /// # Returns
    ///
    /// A BridgeResponse indicating the next state and optional voucher payload.
    pub async fn execute_jit_swap(&self, request: BridgeRequest) -> Result<BridgeResponse> {
        // Step 1: Simulate progression to AssetLockPending and validate the ledger settlement.
        // In a real implementation, this would query a blockchain indexer or node.
        self.verify_ledger_settlement(&request.tx_hash).await?;

        // Step 2: Advance to LiquidityRouting and call the voucher provider API.
        let provider_response = self
            .client
            .post(format!("{}/v1/orders", self.provider_url))
            .bearer_auth(&self.auth_token)
            .json(&serde_json::json!({
                "operator_id": request.merchant_id,
                "amount": request.amount_fiat,
                "currency": request.currency
            }))
            .send()
            .await
            .map_err(DTaPError::NetworkError)?;

        // Step 3: Check if the provider call was successful.
        if provider_response.status().is_success() {
            // Step 4: Deserialize the response and extract the voucher code.
            // We map any JSON error to a DeserializationError with the error message.
            let body: VoucherProviderResponse = provider_response
                .json()
                .await
                .map_err(|e| DTaPError::DeserializationError(e.to_string()))?;

            // Step 5: Progress state to ActiveBarcode with the voucher code.
            Ok(BridgeResponse {
                target_state: BridgeState::ActiveBarcode(body.code.clone()),
                voucher_payload: Some(body.code),
            })
        } else {
            // Step 6: Handle provider error (e.g., insufficient funds, invalid request).
            // Get the status first, then read the body as text.
            let status = provider_response.status();
            let error_msg = provider_response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown provider error".to_string());
            Err(DTaPError::ProviderError(format!(
                "Provider returned status {}: {}",
                status,
                error_msg
            )))
        }
    }

    /// Simulates verifying that the user's asset lock transaction has reached finality on-chain.
    /// In a real implementation, this would query a blockchain node or indexer.
    ///
    /// # Arguments
    ///
    /// * `tx_hash` - The transaction hash to verify.
    ///
    /// # Returns
    ///
    /// Ok(()) if the transaction is confirmed, otherwise an error.
    async fn verify_ledger_settlement(&self, _tx_hash: &str) -> Result<()> {
        // Placeholder: In reality, we would poll a blockchain node until the transaction is confirmed.
        // For now, we assume it's always valid for the sake of scaffolding.
        Ok(())
    }
}