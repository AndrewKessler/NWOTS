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

    println!(
        "Address file: {}",
        auth.address_file
    );

    println!(
        "Address index: {}",
        auth.address_index
    );

    /*
     * -------------------------------------------------------------------------
     * 1. Verify the local asset namespace.
     * -------------------------------------------------------------------------
     */

    let manifest_root =
        crypto_namespace::verify_manifest(
            &auth.manifest
        )?;

    println!(
        "Manifest ROOT_HASH: {}",
        manifest_root
    );

    /*
     * -------------------------------------------------------------------------
     * 2. Query the Lineage genesis transaction.
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
        "Item creation address: {}",
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
     * 3. Verify blockchain metadata against the local manifest.
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
     * 4. Resolve the expected current-owner address.
     * -------------------------------------------------------------------------
     */

    let expected_address =
        chain_react::read_address(
            &auth.address_file,
            auth.address_index,
        )?;

    println!(
        "Expected owner address: {}",
        expected_address
    );

    /*
     * -------------------------------------------------------------------------
     * 5. Query CURRENT UTXO state.
     * -------------------------------------------------------------------------
     *
     * This is the Phase 6 change.
     *
     * We no longer require the Item to remain at its creation
     * address. We ask the current UTXO set whether the configured
     * address currently possesses this Item.
     */

    let current_owner =
        runtime.block_on(
            chain_react::verify_current_ownership(
                &item,
                &expected_address,
            )
        )?;

    println!(
        "Blockchain ownership authenticated."
    );

    println!(
        "Current Item owner: {}",
        current_owner.address
    );

    println!(
        "Current Item outpoint: {}:{}",
        current_owner.outpoint_tx_hash,
        current_owner.outpoint_index
    );

    println!(
        "NWOTs authentication successful."
    );

    Ok(())
}