use std::fs;
use std::path::Path;

pub fn read_manifest_root<P: AsRef<Path>>(
    manifest_path: P,
) -> Result<String, String> {
    let path = manifest_path.as_ref();

    let contents = fs::read_to_string(path)
        .map_err(|e| {
            format!(
                "failed to read manifest {}: {}",
                path.display(),
                e
            )
        })?;

    for line in contents.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut fields = line.split_whitespace();

        let Some(key) = fields.next() else {
            continue;
        };

        if key == "ROOT_HASH" {
            let root_hash = fields
                .next()
                .ok_or_else(|| {
                    "manifest contains an empty ROOT_HASH"
                        .to_string()
                })?;

            return Ok(root_hash.to_string());
        }
    }

    Err(
        "manifest does not contain a ROOT_HASH entry"
            .to_string()
    )
}