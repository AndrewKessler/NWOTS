use reqwest::Client;
use serde_json::Value;

const STORAGE_API: &str = "https://storage.lineage.to";
const MEMPOOL_API: &str = "https://mempool.lineage.to";

#[derive(Debug, Clone)]
pub struct LineageItem {
    pub output_index: usize,
    pub owner_address: String,
    pub genesis_hash: String,
    pub metadata: String,
}

pub async fn get_item_from_transaction(
    transaction_id: &str,
) -> Result<LineageItem, String> {
    let client = Client::new();

    let url = format!(
        "{}/v1/blockchain-entries/{}",
        STORAGE_API,
        transaction_id
    );

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| {
            format!(
                "failed to query Lineage transaction: {}",
                e
            )
        })?;

    let status = response.status();

    let body = response
        .text()
        .await
        .map_err(|e| {
            format!(
                "failed to read Lineage transaction response: {}",
                e
            )
        })?;

    if !status.is_success() {
        return Err(format!(
            "Lineage transaction query failed with HTTP {}: {}",
            status, body
        ));
    }

    let transaction: Value =
        serde_json::from_str(&body)
            .map_err(|e| {
                format!(
                    "invalid Lineage transaction JSON: {}",
                    e
                )
            })?;

    let outputs = transaction
        .get("data")
        .and_then(|v| v.get("outputs"))
        .and_then(|v| v.as_array())
        .ok_or_else(|| {
            "transaction contains no outputs".to_string()
        })?;

    for (output_index, output) in outputs.iter().enumerate() {
        let Some(item) = output
            .get("value")
            .and_then(|v| v.get("Item"))
        else {
            continue;
        };

        let metadata = item
            .get("metadata")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                format!(
                    "Item output {} has no metadata",
                    output_index
                )
            })?;

        let owner_address = output
            .get("script_public_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                format!(
                    "Item output {} has no owner address",
                    output_index
                )
            })?;

        /*
         * The creation transaction has genesis_hash = null.
         *
         * For a Lineage Item created by this transaction,
         * the live chain-state representation uses the
         * transaction ID as its genesis hash.
         */
        let genesis_hash = item
            .get("genesis_hash")
            .and_then(|v| v.as_str())
            .unwrap_or(transaction_id);

        let amount = item
            .get("amount")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        if amount == 0 {
            continue;
        }

        return Ok(LineageItem {
            output_index,
            owner_address: owner_address.to_string(),
            genesis_hash: genesis_hash.to_string(),
            metadata: metadata.to_string(),
        });
    }

    Err(
        "transaction does not contain a positive Item output"
            .to_string()
    )
}

pub async fn verify_current_ownership(
    item: &LineageItem,
) -> Result<(), String> {
    let client = Client::new();

    let url = format!(
        "{}/v1/balances/query",
        MEMPOOL_API
    );

    /*
     * Phase 4 deliberately checks the address recorded
     * in the creation transaction.
     *
     * Phase 5 will make this transfer-proof by discovering
     * the current owner globally.
     */
    let body = serde_json::json!({
        "addresses": [item.owner_address]
    });

    let response = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| {
            format!(
                "failed to query Lineage balances: {}",
                e
            )
        })?;

    let status = response.status();

    let response_body = response
        .text()
        .await
        .map_err(|e| {
            format!(
                "failed to read Lineage balance response: {}",
                e
            )
        })?;

    if !status.is_success() {
        return Err(format!(
            "Lineage balance query failed with HTTP {}: {}",
            status,
            response_body
        ));
    }

    let balance: Value =
        serde_json::from_str(&response_body)
            .map_err(|e| {
                format!(
                    "invalid Lineage balance JSON: {}",
                    e
                )
            })?;

    let entries = balance
        .get("balance")
        .and_then(|v| v.get("address_list"))
        .and_then(|v| v.get(&item.owner_address))
        .and_then(|v| v.as_array())
        .ok_or_else(|| {
            format!(
                "no current balance entries found for {}",
                item.owner_address
            )
        })?;

    for entry in entries {
        let Some(live_item) = entry
            .get("value")
            .and_then(|v| v.get("Item"))
        else {
            continue;
        };

        let genesis_hash = live_item
            .get("genesis_hash")
            .and_then(|v| v.as_str())
            .unwrap_or_default();

        let amount = live_item
            .get("amount")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let metadata = live_item
            .get("metadata")
            .and_then(|v| v.as_str())
            .unwrap_or_default();

        if amount > 0
            && genesis_hash.eq_ignore_ascii_case(
                &item.genesis_hash
            )
            && metadata.eq_ignore_ascii_case(
                &item.metadata
            )
        {
            println!(
                "Current Item ownership verified."
            );

            return Ok(());
        }
    }

    Err(format!(
        "Item {} is no longer owned by {}",
        item.genesis_hash,
        item.owner_address
    ))
}