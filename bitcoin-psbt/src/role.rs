crate::ix!();

pub enum PSBTRole {
    CREATOR,
    UPDATER,
    SIGNER,
    FINALIZER,
    EXTRACTOR
}

pub fn psbt_role_name(role: PSBTRole) -> String {

    match role {
        PSBTRole::CREATOR   => "creator".to_string(),
        PSBTRole::UPDATER   => "updater".to_string(),
        PSBTRole::SIGNER    => "signer".to_string(),
        PSBTRole::FINALIZER => "finalizer".to_string(),
        PSBTRole::EXTRACTOR => "extractor".to_string(),
    }
}
