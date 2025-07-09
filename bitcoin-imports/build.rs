use scan_crate_for_typedefs::*;

fn main() -> std::io::Result<()> {

    // This environment variable is set during `cargo publish`, or if we are building the
    // crate in a package/verify scenario.
    if std::env::var_os("CARGO_PUBLISH").is_some() {
        // Skip scanning entirely for publish or packaging.
        return Ok(());
    }

    // Otherwise, we are presumably in local dev. Try to find the workspace Cargo.toml:
    let workspace_cargo = std::path::PathBuf::from("..").join("Cargo.toml");
    if !workspace_cargo.exists() {
        // If it doesn't exist, skip or fail gracefully
        eprintln!("Skipping scan; no workspace Cargo.toml at {:?}", workspace_cargo);
        return Ok(());
    }

    let _typemap = PersistentWorkspaceTypeMap::new_with_path("..")?;

    Ok(())
}

