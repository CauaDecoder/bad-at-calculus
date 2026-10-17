//! Run from any directory; destinations are anchored to the Cargo manifest.

use std::{error::Error, fs, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<_> = std::env::args().skip(1).collect();
    if arguments.iter().any(|argument| argument != "--check") || arguments.len() > 1 {
        return Err(
            "usage: cargo run -p vx-api --example generate_content_contracts -- [--check]".into(),
        );
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let check = arguments
        .first()
        .is_some_and(|argument| argument == "--check");
    for (name, schema) in vx_api::contracts::schemas() {
        let content = format!("{}\n", serde_json::to_string_pretty(&schema)?);
        write_contract(
            &root.join(format!("contracts/{name}.schema.json")),
            &content,
            check,
        )?;
    }
    write_contract(
        &root.join("apps/web/src/lib/domain/generated/content.ts"),
        &vx_api::contracts::typescript(),
        check,
    )?;
    Ok(())
}

fn write_contract(path: &Path, content: &str, check: bool) -> Result<(), Box<dyn Error>> {
    if check {
        if fs::read_to_string(path)? != content {
            return Err(format!("outdated contract: {}", path.display()).into());
        }
    } else {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
    }
    Ok(())
}
