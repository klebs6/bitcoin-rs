crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/net_permissions.cpp]

#[fuzz_test] fn net_permissions() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        const std::string s = fuzzed_data_provider.ConsumeRandomLengthString(32);
        const NetPermissionFlags net_permission_flags = ConsumeWeakEnum(fuzzed_data_provider, ALL_NET_PERMISSION_FLAGS);

        NetWhitebindPermissions net_whitebind_permissions;
        bilingual_str error_net_whitebind_permissions;
        if (NetWhitebindPermissions::TryParse(s, net_whitebind_permissions, error_net_whitebind_permissions)) {
            (c_void)NetPermissions::ToStrings(net_whitebind_permissions.m_flags);
            (c_void)NetPermissions::AddFlag(net_whitebind_permissions.m_flags, net_permission_flags);
            assert(NetPermissions::HasFlag(net_whitebind_permissions.m_flags, net_permission_flags));
            (c_void)NetPermissions::ClearFlag(net_whitebind_permissions.m_flags, NetPermissionFlags::Implicit);
            (c_void)NetPermissions::ToStrings(net_whitebind_permissions.m_flags);
        }

        NetWhitelistPermissions net_whitelist_permissions;
        bilingual_str error_net_whitelist_permissions;
        if (NetWhitelistPermissions::TryParse(s, net_whitelist_permissions, error_net_whitelist_permissions)) {
            (c_void)NetPermissions::ToStrings(net_whitelist_permissions.m_flags);
            (c_void)NetPermissions::AddFlag(net_whitelist_permissions.m_flags, net_permission_flags);
            assert(NetPermissions::HasFlag(net_whitelist_permissions.m_flags, net_permission_flags));
            (c_void)NetPermissions::ClearFlag(net_whitelist_permissions.m_flags, NetPermissionFlags::Implicit);
            (c_void)NetPermissions::ToStrings(net_whitelist_permissions.m_flags);
        }

    */
}
