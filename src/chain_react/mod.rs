use reqwest::Client;
use serde_json::Value;
use std::fs;
use std::path::Path;

const STORAGE_API: &str = "https://storage.lineage.to";
const MEMPOOL_API: &str = "https://mempool.lineage.to";

#[derive(Debug, Clone)]
pub struct LineageItem {
    pub output_index: usize,
    pub owner_address: String,
    pub genesis_hash: String,
    pub metadata: String,
}

#[derive(Debug, Clone)]
pub struct CurrentItemOwner {
    pub address: String,
    pub genesis_hash: String,
    pub metadata: String,
    pub outpoint_tx_hash: String,
    pub outpoint_index: usize,
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
            status,
            body
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
         * The genesis transaction returned by Storage may contain
         * genesis_hash = null. For the genesis Item, the transaction
         * hash itself is the Item's identity.
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


/*
 * Phase 6:
 *
 * Query the CURRENT UTXO state for one known address and determine
 * whether that address currently holds the authenticated Item.
 *
 * We deliberately do NOT inspect historical blocks here.
 *
 * The Lineage mempool's balance endpoint is the authoritative
 * current UTXO view for the requested address.
 */
pub async fn verify_current_ownership(
    item: &LineageItem,
    expected_address: &str,
) -> Result<CurrentItemOwner, String> {
    let client = Client::new();

    let url = format!(
        "{}/v1/balances/query",
        MEMPOOL_API
    );

    let body = serde_json::json!({
        "addresses": [expected_address]
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

    /*
     * We expect:
     *
     * balance
     *   address_list
     *     expected_address
     *       [
     *         {
     *           out_point: {
     *             t_hash,
     *             n
     *           },
     *           value: {
     *             Item: {
     *               amount,
     *               genesis_hash,
     *               metadata
     *             }
     *           }
     *         }
     *       ]
     */

    let entries = balance
        .get("balance")
        .and_then(|v| v.get("address_list"))
        .and_then(|v| v.get(expected_address))
        .and_then(|v| v.as_array())
        .ok_or_else(|| {
            format!(
                "no current UTXO entries found for address {}",
                expected_address
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

        let metadata = live_item
            .get("metadata")
            .and_then(|v| v.as_str())
            .unwrap_or_default();

        let amount = live_item
            .get("amount")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        if amount == 0 {
            continue;
        }

        /*
         * The genesis hash identifies the Item across transfers.
         *
         * The OWNER address does NOT need to know or contain the
         * genesis hash. The current UTXO entry does.
         */
        if !genesis_hash.eq_ignore_ascii_case(
            &item.genesis_hash
        ) {
            continue;
        }

        /*
         * Metadata is also checked. This prevents us from accepting
         * a different Item merely because it happens to have the
         * same genesis identity representation.
         */
        if !metadata.eq_ignore_ascii_case(
            &item.metadata
        ) {
            continue;
        }

        let out_point = entry
            .get("out_point")
            .ok_or_else(|| {
                format!(
                    "current Item for {} has no out_point",
                    expected_address
                )
            })?;

        let outpoint_tx_hash = out_point
            .get("t_hash")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                format!(
                    "current Item for {} has no out_point.t_hash",
                    expected_address
                )
            })?;

        let outpoint_index = out_point
            .get("n")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| {
                format!(
                    "current Item for {} has no out_point.n",
                    expected_address
                )
            })?;

        println!(
            "Current Item ownership verified."
        );

        println!(
            "Current owner: {}",
            expected_address
        );

        println!(
            "Current Item outpoint: {}:{}",
            outpoint_tx_hash,
            outpoint_index
        );

        return Ok(CurrentItemOwner {
            address: expected_address.to_string(),
            genesis_hash: genesis_hash.to_string(),
            metadata: metadata.to_string(),
            outpoint_tx_hash: outpoint_tx_hash.to_string(),
            outpoint_index: outpoint_index as usize,
        });
    }

    Err(format!(
        "Item {} is not currently owned by {}",
        item.genesis_hash,
        expected_address
    ))
}

pub fn read_address(
    address_file: &str,
    address_index: usize,
) -> Result<String, String> {
    let path = Path::new(address_file);

    let contents =
        fs::read_to_string(path)
            .map_err(|e| {
                format!(
                    "failed to read address file {}: {}",
                    path.display(),
                    e
                )
            })?;

    for line in contents.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let fields: Vec<&str> =
            line.split_whitespace().collect();

        if fields.len() != 2 {
            return Err(format!(
                "invalid address entry: {}",
                line
            ));
        }

        let index: usize =
            fields[0].parse().map_err(|_| {
                format!(
                    "invalid address index in {}",
                    line
                )
            })?;

        if index == address_index {
            let address = fields[1];

            if address.is_empty() {
                return Err(format!(
                    "address {} is empty",
                    address_index
                ));
            }

            return Ok(address.to_string());
        }
    }

    Err(format!(
        "address index {} not found in {}",
        address_index,
        path.display()
    ))
}