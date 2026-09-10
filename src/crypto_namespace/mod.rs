use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct ManifestLeaf {
    path: String,
    file_hash: String,
    leaf_hash: String,
}

fn sha256_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let data = fs::read(path).map_err(|e| {
        format!(
            "failed to read asset {}: {}",
            path.display(),
            e
        )
    })?;

    Ok(sha256_bytes(&data))
}

fn calculate_leaf_hash(
    path: &str,
    file_hash: &str,
) -> String {
    let mut hasher = Sha256::new();

    hasher.update(b"CN-LEAF\0");
    hasher.update(path.as_bytes());
    hasher.update(b"\0");
    hasher.update(file_hash.as_bytes());

    hex::encode(hasher.finalize())
}

fn calculate_parent_hash(
    left: &str,
    right: &str,
) -> String {
    let mut hasher = Sha256::new();

    hasher.update(b"CN-NODE\0");
    hasher.update(left.as_bytes());
    hasher.update(b"\0");
    hasher.update(right.as_bytes());

    hex::encode(hasher.finalize())
}

fn calculate_merkle_root(
    leaves: &[String],
) -> String {
    if leaves.is_empty() {
        return sha256_bytes(b"CN-EMPTY");
    }

    let mut level = leaves.to_vec();

    while level.len() > 1 {
        let mut next = Vec::new();

        let mut index = 0;

        while index < level.len() {
            if index + 1 == level.len() {
                // Preserve the original crypto_namespace
                // behavior: promote an odd node unchanged.
                next.push(level[index].clone());
                index += 1;
                continue;
            }

            let parent = calculate_parent_hash(
                &level[index],
                &level[index + 1],
            );

            next.push(parent);

            index += 2;
        }

        level = next;
    }

    level[0].clone()
}

fn parse_manifest(
    manifest_path: &Path,
) -> Result<(PathBuf, String, Vec<ManifestLeaf>), String> {
    let contents =
        fs::read_to_string(manifest_path)
            .map_err(|e| {
                format!(
                    "failed to read manifest {}: {}",
                    manifest_path.display(),
                    e
                )
            })?;

    let mut root_path: Option<PathBuf> = None;
    let mut declared_root: Option<String> = None;
    let mut leaves = Vec::new();

    for line in contents.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let fields: Vec<&str> =
            line.split('\t').collect();

        if fields.len() == 1
            && fields[0] == "CRYPTO_NAMESPACE_MANIFEST_V1"
        {
            continue;
        }

        if fields.len() >= 2 {
            match fields[0] {
                "ROOT" => {
                    root_path =
                        Some(PathBuf::from(fields[1]));
                    continue;
                }

                "ROOT_HASH" => {
                    declared_root =
                        Some(fields[1].to_string());
                    continue;
                }

                "FILES" => {
                    continue;
                }

                _ => {}
            }
        }

        if fields.len() != 3 {
            return Err(format!(
                "invalid manifest entry: {}",
                line
            ));
        }

        leaves.push(ManifestLeaf {
            path: fields[0].replace('\\', "/"),
            file_hash: fields[1].to_string(),
            leaf_hash: fields[2].to_string(),
        });
    }

    let root_path = root_path.ok_or_else(|| {
        "manifest does not contain ROOT".to_string()
    })?;

    let declared_root =
        declared_root.ok_or_else(|| {
            "manifest does not contain ROOT_HASH"
                .to_string()
        })?;

    if leaves.is_empty() {
        return Err(
            "manifest contains no asset entries"
                .to_string()
        );
    }

    Ok((
        root_path,
        declared_root,
        leaves,
    ))
}

pub fn verify_manifest(
    manifest_path: &str,
) -> Result<String, String> {
    let manifest_path = Path::new(manifest_path);

    let (
        root_path,
        declared_root,
        mut leaves,
    ) = parse_manifest(manifest_path)?;

    /*
     * The original manifest generator sorts leaves
     * lexicographically by canonical relative path.
     */
    leaves.sort_by(|a, b| {
        a.path.cmp(&b.path)
    });

    let mut calculated_leaves =
        Vec::with_capacity(leaves.len());

    for leaf in &leaves {
        let asset_path =
            root_path.join(
                Path::new(&leaf.path)
            );

        /*
         * -------------------------------------------------------------
         * 1. Hash the actual file.
         * -------------------------------------------------------------
         */

        let actual_file_hash =
            sha256_file(&asset_path)?;

        if !actual_file_hash
            .eq_ignore_ascii_case(
                &leaf.file_hash
            )
        {
            return Err(format!(
                "asset hash mismatch for {}: \
                 manifest={}, actual={}",
                leaf.path,
                leaf.file_hash,
                actual_file_hash
            ));
        }

        /*
         * -------------------------------------------------------------
         * 2. Recalculate the leaf hash.
         * -------------------------------------------------------------
         */

        let calculated_leaf =
            calculate_leaf_hash(
                &leaf.path,
                &actual_file_hash,
            );

        if !calculated_leaf
            .eq_ignore_ascii_case(
                &leaf.leaf_hash
            )
        {
            return Err(format!(
                "leaf hash mismatch for {}: \
                 manifest={}, calculated={}",
                leaf.path,
                leaf.leaf_hash,
                calculated_leaf
            ));
        }

        calculated_leaves.push(
            calculated_leaf
        );
    }

    /*
     * -------------------------------------------------------------
     * 3. Reconstruct the Merkle tree.
     * -------------------------------------------------------------
     */

    let calculated_root =
        calculate_merkle_root(
            &calculated_leaves
        );

    /*
     * -------------------------------------------------------------
     * 4. Verify manifest ROOT_HASH.
     * -------------------------------------------------------------
     */

    if !calculated_root
        .eq_ignore_ascii_case(
            &declared_root
        )
    {
        return Err(format!(
            "manifest Merkle root mismatch: \
             declared={}, calculated={}",
            declared_root,
            calculated_root
        ));
    }

    println!(
        "Manifest Merkle root verified: {}",
        calculated_root
    );

    Ok(calculated_root)
}

/*
 * Backwards-compatible helper.
 *
 * This remains available so existing code does not
 * immediately break, but Phase 5 authentication should
 * use verify_manifest().
 */
pub fn read_manifest_root(
    manifest_path: &str,
) -> Result<String, String> {
    let manifest_path = Path::new(manifest_path);

    let (_, declared_root, _) =
        parse_manifest(manifest_path)?;

    Ok(declared_root)
}