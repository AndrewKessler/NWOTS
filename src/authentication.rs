use crate::chain_react;
use crate::config::game_config::GameConfig;
use crate::crypto_namespace;

pub fn authenticate_game(
    config: &GameConfig,
) -> Result<(), String> {
    let Some(auth) = &config.blockchain_auth else {
        println!(
            "Blockchain authentication disabled."
        );
        return Ok(());
    };

    if !auth.enabled {
        println!(
            "Blockchain authentication disabled."
        );
        return Ok(());
    }

    println!(
        "Blockchain authentication enabled."
    );
    println!(
        "Transaction ID: {}",
        auth.transaction_id
    );
    println!(
        "Manifest: {}",
        auth.manifest
    );

    /*
     * -------------------------------------------------------------------------
     * 1. Read the local manifest root.
     * -------------------------------------------------------------------------
     */

    let manifest_root =
        crypto_namespace::read_manifest_root(
            &auth.manifest
        )?;

    println!(
        "Manifest ROOT_HASH: {}",
        manifest_root
    );

    /*
     * -------------------------------------------------------------------------
     * 2. Query the blockchain using the configured TXID.
     * -------------------------------------------------------------------------
     */

    println!(
        "Querying Lineage blockchain..."
    );

    let runtime =
        tokio::runtime::Runtime::new()
            .map_err(|e| {
                format!(
                    "failed to create async runtime: {}",
                    e
                )
            })?;

    let item =
        runtime.block_on(
            chain_react::get_item_from_transaction(
                &auth.transaction_id
            )
        )?;

    println!(
        "Transaction authenticated."
    );

    println!(
        "Item output: {}",
        item.output_index
    );

    println!(
        "Item owner: {}",
        item.owner_address
    );

    println!(
        "Item genesis hash: {}",
        item.genesis_hash
    );

    println!(
        "Item metadata: {}",
        item.metadata
    );

    /*
     * -------------------------------------------------------------------------
     * 3. Compare blockchain metadata with local manifest root.
     * -------------------------------------------------------------------------
     */

    if !item
        .metadata
        .eq_ignore_ascii_case(&manifest_root)
    {
        return Err(format!(
            "asset manifest authentication failed: \
             blockchain metadata {} does not match \
             local ROOT_HASH {}",
            item.metadata,
            manifest_root
        ));
    }

    println!(
        "Asset Manifest Authenticated."
    );

    /*
     * -------------------------------------------------------------------------
     * 4. Verify that the Item is still currently owned.
     * -------------------------------------------------------------------------
     */

    runtime.block_on(
        chain_react::verify_current_ownership(
            &item
        )
    )?;

    println!(
        "Blockchain ownership authenticated."
    );

    println!(
        "NWOTs authentication successful."
    );

    Ok(())
}