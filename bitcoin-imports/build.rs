use scan_crate_for_typedefs::*;

fn main() -> std::io::Result<()> {

    let typemap = PersistentWorkspaceTypeMap::new_with_path("..")?;

    Ok(())
}

