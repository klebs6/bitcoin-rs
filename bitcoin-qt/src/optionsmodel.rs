crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/optionsmodel.h]

lazy_static!{
    /*
    extern const char *DEFAULT_GUI_PROXY_HOST;
    */
}

pub const DEFAULT_GUI_PROXY_PORT: u16 = 9050;

/**
  | Convert configured prune target MiB
  | to displayed GB. Round up to avoid underestimating
  | max disk usage.
  |
  */
#[inline] pub fn prune_mib_togb(mib: i64) -> i32 {
    
    todo!();
        /*
            return (mib * 1024 * 1024 + GB_BYTES - 1) / GB_BYTES;
        */
}

/**
  | Convert displayed prune target GB to
  | configured MiB. Round down so roundtrip
  | GB -> MiB -> GB conversion is stable.
  |
  */
#[inline] pub fn prune_gb_to_mib(gb: i32) -> i64 {
    
    todo!();
        /*
            return gb * GB_BYTES / 1024 / 1024;
        */
}

pub const DEFAULT_GUI_PROXY_HOST: &'static str = "127.0.0.1";

/**
  | Helper function to copy contents from
  | one QSettings to another.
  | 
  | By using allKeys this also covers nested
  | settings in a hierarchy.
  |
  */
pub fn copy_settings(
        dst: &mut QSettings,
        src: &QSettings)  {
    
    todo!();
        /*
            for (const QString& key : src.allKeys()) {
            dst.setValue(key, src.value(key));
        }
        */
}

/**
  | Back up a QSettings to an ini-formatted
  | file.
  |
  */
pub fn backup_settings(
        filename: &Box<Path>,
        src:      &QSettings)  {
    
    todo!();
        /*
            qInfo() << "Backing up GUI settings to" << typename gui_util::boostPathToQString(filename);
        QSettings dst(typename gui_util::boostPathToQString(filename), QSettings::IniFormat);
        dst.clear();
        CopySettings(dst, src);
        */
}

pub struct ProxySetting {
    is_set: bool,
    ip:     String,
    port:   String,
}

pub fn get_proxy_setting(
        settings: &mut QSettings,
        name:     &String) -> ProxySetting {
    
    todo!();
        /*
            static const ProxySetting default_val = {false, DEFAULT_GUI_PROXY_HOST, QString("%1").arg(DEFAULT_GUI_PROXY_PORT)};
        // Handle the case that the setting is not set at all
        if (!settings.contains(name)) {
            return default_val;
        }
        // contains IP at index 0 and port at index 1
        QStringList ip_port = typename gui_util::SplitSkipEmptyParts(settings.value(name).toString(), ":");
        if (ip_port.size() == 2) {
            return {true, ip_port.at(0), ip_port.at(1)};
        } else { // Invalid: return default
            return default_val;
        }
        */
}

pub fn set_proxy_setting(
        settings: &mut QSettings,
        name:     &String,
        ip_port:  &ProxySetting)  {
    
    todo!();
        /*
            settings.setValue(name, QString{ip_port.ip + QLatin1Char(':') + ip_port.port});
        */
}

pub fn get_default_proxy_address() -> String {
    
    todo!();
        /*
            return QString("%1:%2").arg(DEFAULT_GUI_PROXY_HOST).arg(DEFAULT_GUI_PROXY_PORT);
        */
}

/**
  | Interface from Qt to configuration
  | data structure for Bitcoin client.
  | 
  | To Qt, the options are presented as a
  | list with the different options laid
  | out vertically.
  | 
  | This can be changed to a tree once the
  | settings become sufficiently complex.
  |
  */
#[Q_OBJECT]
pub struct OptionsModel {
    base: QAbstractListModel,

    node:                           Rc<RefCell<dyn NodeInterface>>, // default = nullptr

    /* --------------- Qt-only settings  --------------- */
    show_tray_icon:                 bool,

    minimize_to_tray:               bool,
    minimize_on_close:              bool,
    language:                       String,
    n_display_unit:                 i32,
    str_third_party_tx_urls:        String,
    use_embedded_monospaced_font:   bool,
    coin_control_features:          bool,
    sub_fee_from_amount:            bool,

    /**
      | settings that were overridden by command-line
      |
      */
    str_overridden_by_command_line: String,
}

pub mod options_model {

    pub enum OptionID {
        StartAtStartup,         // bool
        ShowTrayIcon,           // bool
        MinimizeToTray,         // bool
        MapPortUPnP,            // bool
        MapPortNatpmp,          // bool
        MinimizeOnClose,        // bool
        ProxyUse,               // bool
        ProxyIP,                // QString
        ProxyPort,              // int
        ProxyUseTor,            // bool
        ProxyIPTor,             // QString
        ProxyPortTor,           // int
        DisplayUnit,            // BitcoinUnits::Unit
        ThirdPartyTxUrls,       // QString
        Language,               // QString
        UseEmbeddedMonospacedFont, // bool
        CoinControlFeatures,    // bool
        SubFeeFromAmount,       // bool
        ThreadsScriptVerif,     // int
        Prune,                  // bool
        PruneSize,              // int
        DatabaseCache,          // int
        ExternalSignerPath,     // QString
        SpendZeroConfChange,    // bool
        Listen,                 // bool
        Server,                 // bool
        OptionIDRowCount,
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/optionsmodel.cpp]
impl OptionsModel {
    
    /**
      | Explicit getters
      |
      */
    pub fn get_show_tray_icon(&self) -> bool {
        
        todo!();
        /*
            return m_show_tray_icon;
        */
    }
    
    pub fn get_minimize_to_tray(&self) -> bool {
        
        todo!();
        /*
            return fMinimizeToTray;
        */
    }
    
    pub fn get_minimize_on_close(&self) -> bool {
        
        todo!();
        /*
            return fMinimizeOnClose;
        */
    }
    
    pub fn get_display_unit(&self) -> i32 {
        
        todo!();
        /*
            return nDisplayUnit;
        */
    }
    
    pub fn get_third_party_tx_urls(&self) -> String {
        
        todo!();
        /*
            return strThirdPartyTxUrls;
        */
    }
    
    pub fn get_use_embedded_monospaced_font(&self) -> bool {
        
        todo!();
        /*
            return m_use_embedded_monospaced_font;
        */
    }
    
    pub fn get_coin_control_features(&self) -> bool {
        
        todo!();
        /*
            return fCoinControlFeatures;
        */
    }
    
    pub fn get_sub_fee_from_amount(&self) -> bool {
        
        todo!();
        /*
            return m_sub_fee_from_amount;
        */
    }
    
    pub fn get_overridden_by_command_line(&mut self) -> &String {
        
        todo!();
        /*
            return strOverriddenByCommandLine;
        */
    }

    /* ---------------- Explicit setters  ---------------- */

    /* --------------- Restart flag helper  --------------- */
    pub fn node(&self) -> Rc<RefCell<dyn NodeInterface>> {
        
        todo!();
        /*
            assert(m_node); return *m_node;
        */
    }
    
    pub fn set_node(&mut self, node: Rc<RefCell<dyn NodeInterface>>)  {
        
        todo!();
        /*
            assert(!m_node); m_node = &node;
        */
    }

    #[Q_SIGNAL]
    pub fn display_unit_changed(&mut self, unit: i32)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn coin_control_features_changed(&mut self, _0: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn show_tray_icon_changed(&mut self, _0: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn use_embedded_monospaced_font_changed(&mut self, _0: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn new(
        parent:         Option<*mut QObject>,
        reset_settings: Option<bool>) -> Self {

        let reset_settings: bool = reset_settings.unwrap_or(false);
    
        todo!();
        /*
        : q_abstract_list_model(parent),

            Init(resetSettings);
        */
    }
    
    /**
      | Add option to list of GUI options overridden
      | through command line/config file
      |
      */
    pub fn add_overridden_option(&mut self, option: &String)  {
        
        todo!();
        /*
            strOverriddenByCommandLine += QString::fromStdString(option) + "=" + QString::fromStdString(gArgs.GetArg(option, "")) + " ";
        */
    }

    /**
      | Writes all missing QSettings with their
      | default values
      |
      */
    pub fn init(&mut self, reset_settings: Option<bool>)  {

        let reset_settings: bool = reset_settings.unwrap_or(false);
        
        todo!();
        /*
            if (resetSettings)
            Reset();

        checkAndMigrate();

        QSettings settings;

        // Ensure restart flag is unset on client startup
        setRestartRequired(false);

        // These are Qt-only settings:

        // Window
        if (!settings.contains("fHideTrayIcon")) {
            settings.setValue("fHideTrayIcon", false);
        }
        m_show_tray_icon = !settings.value("fHideTrayIcon").toBool();
        Q_EMIT showTrayIconChanged(m_show_tray_icon);

        if (!settings.contains("fMinimizeToTray"))
            settings.setValue("fMinimizeToTray", false);
        fMinimizeToTray = settings.value("fMinimizeToTray").toBool() && m_show_tray_icon;

        if (!settings.contains("fMinimizeOnClose"))
            settings.setValue("fMinimizeOnClose", false);
        fMinimizeOnClose = settings.value("fMinimizeOnClose").toBool();

        // Display
        if (!settings.contains("nDisplayUnit"))
            settings.setValue("nDisplayUnit", BitcoinUnits::BTC);
        nDisplayUnit = settings.value("nDisplayUnit").toInt();

        if (!settings.contains("strThirdPartyTxUrls"))
            settings.setValue("strThirdPartyTxUrls", "");
        strThirdPartyTxUrls = settings.value("strThirdPartyTxUrls", "").toString();

        if (!settings.contains("fCoinControlFeatures"))
            settings.setValue("fCoinControlFeatures", false);
        fCoinControlFeatures = settings.value("fCoinControlFeatures", false).toBool();

        // These are shared with the core or have a command-line parameter
        // and we want command-line parameters to overwrite the GUI settings.
        //
        // If setting doesn't exist create it with defaults.
        //
        // If gArgs.SoftSetArg() or gArgs.SoftSetBoolArg() return false we were overridden
        // by command-line and show this in the UI.

        // Main
        if (!settings.contains("bPrune"))
            settings.setValue("bPrune", false);
        if (!settings.contains("nPruneSize"))
            settings.setValue("nPruneSize", DEFAULT_PRUNE_TARGET_GB);
        SetPruneEnabled(settings.value("bPrune").toBool());

        if (!settings.contains("nDatabaseCache"))
            settings.setValue("nDatabaseCache", (i64)nDefaultDbCache);
        if (!gArgs.SoftSetArg("-dbcache", settings.value("nDatabaseCache").toString().toStdString()))
            addOverriddenOption("-dbcache");

        if (!settings.contains("nThreadsScriptVerif"))
            settings.setValue("nThreadsScriptVerif", DEFAULT_SCRIPTCHECK_THREADS);
        if (!gArgs.SoftSetArg("-par", settings.value("nThreadsScriptVerif").toString().toStdString()))
            addOverriddenOption("-par");

        if (!settings.contains("strDataDir"))
            settings.setValue("strDataDir", typename gui_util::getDefaultDataDirectory());

        // Wallet
    #ifdef ENABLE_WALLET
        if (!settings.contains("bSpendZeroConfChange"))
            settings.setValue("bSpendZeroConfChange", true);
        if (!gArgs.SoftSetBoolArg("-spendzeroconfchange", settings.value("bSpendZeroConfChange").toBool()))
            addOverriddenOption("-spendzeroconfchange");

        if (!settings.contains("external_signer_path"))
            settings.setValue("external_signer_path", "");

        if (!gArgs.SoftSetArg("-signer", settings.value("external_signer_path").toString().toStdString())) {
            addOverriddenOption("-signer");
        }

        if (!settings.contains("SubFeeFromAmount")) {
            settings.setValue("SubFeeFromAmount", false);
        }
        m_sub_fee_from_amount = settings.value("SubFeeFromAmount", false).toBool();
    #endif

        // Network
        if (!settings.contains("fUseUPnP"))
            settings.setValue("fUseUPnP", DEFAULT_UPNP);
        if (!gArgs.SoftSetBoolArg("-upnp", settings.value("fUseUPnP").toBool()))
            addOverriddenOption("-upnp");

        if (!settings.contains("fUseNatpmp")) {
            settings.setValue("fUseNatpmp", DEFAULT_NATPMP);
        }
        if (!gArgs.SoftSetBoolArg("-natpmp", settings.value("fUseNatpmp").toBool())) {
            addOverriddenOption("-natpmp");
        }

        if (!settings.contains("fListen"))
            settings.setValue("fListen", DEFAULT_LISTEN);
        if (!gArgs.SoftSetBoolArg("-listen", settings.value("fListen").toBool()))
            addOverriddenOption("-listen");

        if (!settings.contains("server")) {
            settings.setValue("server", false);
        }
        if (!gArgs.SoftSetBoolArg("-server", settings.value("server").toBool())) {
            addOverriddenOption("-server");
        }

        if (!settings.contains("fUseProxy"))
            settings.setValue("fUseProxy", false);
        if (!settings.contains("addrProxy"))
            settings.setValue("addrProxy", GetDefaultProxyAddress());
        // Only try to set -proxy, if user has enabled fUseProxy
        if ((settings.value("fUseProxy").toBool() && !gArgs.SoftSetArg("-proxy", settings.value("addrProxy").toString().toStdString())))
            addOverriddenOption("-proxy");
        else if(!settings.value("fUseProxy").toBool() && !gArgs.GetArg("-proxy", "").empty())
            addOverriddenOption("-proxy");

        if (!settings.contains("fUseSeparateProxyTor"))
            settings.setValue("fUseSeparateProxyTor", false);
        if (!settings.contains("addrSeparateProxyTor"))
            settings.setValue("addrSeparateProxyTor", GetDefaultProxyAddress());
        // Only try to set -onion, if user has enabled fUseSeparateProxyTor
        if ((settings.value("fUseSeparateProxyTor").toBool() && !gArgs.SoftSetArg("-onion", settings.value("addrSeparateProxyTor").toString().toStdString())))
            addOverriddenOption("-onion");
        else if(!settings.value("fUseSeparateProxyTor").toBool() && !gArgs.GetArg("-onion", "").empty())
            addOverriddenOption("-onion");

        // Display
        if (!settings.contains("language"))
            settings.setValue("language", "");
        if (!gArgs.SoftSetArg("-lang", settings.value("language").toString().toStdString()))
            addOverriddenOption("-lang");

        language = settings.value("language").toString();

        if (!settings.contains("UseEmbeddedMonospacedFont")) {
            settings.setValue("UseEmbeddedMonospacedFont", "true");
        }
        m_use_embedded_monospaced_font = settings.value("UseEmbeddedMonospacedFont").toBool();
        Q_EMIT useEmbeddedMonospacedFontChanged(m_use_embedded_monospaced_font);
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            QSettings settings;

        // Backup old settings to chain-specific datadir for troubleshooting
        BackupSettings(gArgs.GetDataDirNet() / "guisettings.ini.bak", settings);

        // Save the strDataDir setting
        QString dataDir = typename gui_util::getDefaultDataDirectory();
        dataDir = settings.value("strDataDir", dataDir).toString();

        // Remove all entries from our QSettings object
        settings.clear();

        // Set strDataDir
        settings.setValue("strDataDir", dataDir);

        // Set that this was reset
        settings.setValue("fReset", true);

        // default setting for OptionsModel::StartAtStartup - disabled
        if (typename gui_util::GetStartOnSystemStartup())
            typename gui_util::SetStartOnSystemStartup(false);
        */
    }
    
    pub fn row_count(&self, parent: Option<&QModelIndex>) -> i32 {

        let parent: &QModelIndex = unsafe { parent.unwrap_or(&QModelIndex::new()) };
        
        todo!();
        /*
            return OptionIDRowCount;
        */
    }

    pub fn set_prune_enabled(&mut self, 
        prune: bool,
        force: Option<bool>)  {

        let force: bool = force.unwrap_or(false);
        
        todo!();
        /*
            QSettings settings;
        settings.setValue("bPrune", prune);
        const int64_t prune_target_mib = PruneGBtoMiB(settings.value("nPruneSize").toInt());
        std::string prune_val = prune ? ToString(prune_target_mib) : "0";
        if (force) {
            gArgs.ForceSetArg("-prune", prune_val);
            return;
        }
        if (!gArgs.SoftSetArg("-prune", prune_val)) {
            addOverriddenOption("-prune");
        }
        */
    }
    
    pub fn set_prune_targetgb(&mut self, 
        prune_target_gb: i32,
        force:           Option<bool>)  {

        let force: bool = force.unwrap_or(false);
        
        todo!();
        /*
            const bool prune = prune_target_gb > 0;
        if (prune) {
            QSettings settings;
            settings.setValue("nPruneSize", prune_target_gb);
        }
        SetPruneEnabled(prune, force);
        */
    }

    /**
       read QSettings values and return them
      */
    pub fn data(&self, 
        index: &QModelIndex,
        role:  Option<i32>) -> QVariant {

        let role: i32 = role.unwrap_or(QtDisplayRole.try_into().unwrap());
        
        todo!();
        /*
            if(role == QtEditRole)
        {
            QSettings settings;
            switch(index.row())
            {
            case StartAtStartup:
                return typename gui_util::GetStartOnSystemStartup();
            case ShowTrayIcon:
                return m_show_tray_icon;
            case MinimizeToTray:
                return fMinimizeToTray;
            case MapPortUPnP:
    #ifdef USE_UPNP
                return settings.value("fUseUPnP");
    #else
                return false;
    #endif // USE_UPNP
            case MapPortNatpmp:
    #ifdef USE_NATPMP
                return settings.value("fUseNatpmp");
    #else
                return false;
    #endif // USE_NATPMP
            case MinimizeOnClose:
                return fMinimizeOnClose;

            // default proxy
            case ProxyUse:
                return settings.value("fUseProxy", false);
            case ProxyIP:
                return GetProxySetting(settings, "addrProxy").ip;
            case ProxyPort:
                return GetProxySetting(settings, "addrProxy").port;

            // separate Tor proxy
            case ProxyUseTor:
                return settings.value("fUseSeparateProxyTor", false);
            case ProxyIPTor:
                return GetProxySetting(settings, "addrSeparateProxyTor").ip;
            case ProxyPortTor:
                return GetProxySetting(settings, "addrSeparateProxyTor").port;

    #ifdef ENABLE_WALLET
            case SpendZeroConfChange:
                return settings.value("bSpendZeroConfChange");
            case ExternalSignerPath:
                return settings.value("external_signer_path");
            case SubFeeFromAmount:
                return m_sub_fee_from_amount;
    #endif
            case DisplayUnit:
                return nDisplayUnit;
            case ThirdPartyTxUrls:
                return strThirdPartyTxUrls;
            case Language:
                return settings.value("language");
            case UseEmbeddedMonospacedFont:
                return m_use_embedded_monospaced_font;
            case CoinControlFeatures:
                return fCoinControlFeatures;
            case Prune:
                return settings.value("bPrune");
            case PruneSize:
                return settings.value("nPruneSize");
            case DatabaseCache:
                return settings.value("nDatabaseCache");
            case ThreadsScriptVerif:
                return settings.value("nThreadsScriptVerif");
            case Listen:
                return settings.value("fListen");
            case Server:
                return settings.value("server");
            default:
                return QVariant();
            }
        }
        return QVariant();
        */
    }

    /**
      | write QSettings values
      |
      */
    pub fn set_data(&mut self, 
        index: &QModelIndex,
        value: &QVariant,
        role:  Option<i32>) -> bool {

        let role: i32 = role.unwrap_or((*QtEditRole).try_into().unwrap());
        
        todo!();
        /*
            bool successful = true; /* set to false on parse error */
        if(role == QtEditRole)
        {
            QSettings settings;
            switch(index.row())
            {
            case StartAtStartup:
                successful = typename gui_util::SetStartOnSystemStartup(value.toBool());
                break;
            case ShowTrayIcon:
                m_show_tray_icon = value.toBool();
                settings.setValue("fHideTrayIcon", !m_show_tray_icon);
                Q_EMIT showTrayIconChanged(m_show_tray_icon);
                break;
            case MinimizeToTray:
                fMinimizeToTray = value.toBool();
                settings.setValue("fMinimizeToTray", fMinimizeToTray);
                break;
            case MapPortUPnP: // core option - can be changed on-the-fly
                settings.setValue("fUseUPnP", value.toBool());
                break;
            case MapPortNatpmp: // core option - can be changed on-the-fly
                settings.setValue("fUseNatpmp", value.toBool());
                break;
            case MinimizeOnClose:
                fMinimizeOnClose = value.toBool();
                settings.setValue("fMinimizeOnClose", fMinimizeOnClose);
                break;

            // default proxy
            case ProxyUse:
                if (settings.value("fUseProxy") != value) {
                    settings.setValue("fUseProxy", value.toBool());
                    setRestartRequired(true);
                }
                break;
            case ProxyIP: {
                auto ip_port = GetProxySetting(settings, "addrProxy");
                if (!ip_port.is_set || ip_port.ip != value.toString()) {
                    ip_port.ip = value.toString();
                    SetProxySetting(settings, "addrProxy", ip_port);
                    setRestartRequired(true);
                }
            }
            break;
            case ProxyPort: {
                auto ip_port = GetProxySetting(settings, "addrProxy");
                if (!ip_port.is_set || ip_port.port != value.toString()) {
                    ip_port.port = value.toString();
                    SetProxySetting(settings, "addrProxy", ip_port);
                    setRestartRequired(true);
                }
            }
            break;

            // separate Tor proxy
            case ProxyUseTor:
                if (settings.value("fUseSeparateProxyTor") != value) {
                    settings.setValue("fUseSeparateProxyTor", value.toBool());
                    setRestartRequired(true);
                }
                break;
            case ProxyIPTor: {
                auto ip_port = GetProxySetting(settings, "addrSeparateProxyTor");
                if (!ip_port.is_set || ip_port.ip != value.toString()) {
                    ip_port.ip = value.toString();
                    SetProxySetting(settings, "addrSeparateProxyTor", ip_port);
                    setRestartRequired(true);
                }
            }
            break;
            case ProxyPortTor: {
                auto ip_port = GetProxySetting(settings, "addrSeparateProxyTor");
                if (!ip_port.is_set || ip_port.port != value.toString()) {
                    ip_port.port = value.toString();
                    SetProxySetting(settings, "addrSeparateProxyTor", ip_port);
                    setRestartRequired(true);
                }
            }
            break;

    #ifdef ENABLE_WALLET
            case SpendZeroConfChange:
                if (settings.value("bSpendZeroConfChange") != value) {
                    settings.setValue("bSpendZeroConfChange", value);
                    setRestartRequired(true);
                }
                break;
            case ExternalSignerPath:
                if (settings.value("external_signer_path") != value.toString()) {
                    settings.setValue("external_signer_path", value.toString());
                    setRestartRequired(true);
                }
                break;
            case SubFeeFromAmount:
                m_sub_fee_from_amount = value.toBool();
                settings.setValue("SubFeeFromAmount", m_sub_fee_from_amount);
                break;
    #endif
            case DisplayUnit:
                setDisplayUnit(value);
                break;
            case ThirdPartyTxUrls:
                if (strThirdPartyTxUrls != value.toString()) {
                    strThirdPartyTxUrls = value.toString();
                    settings.setValue("strThirdPartyTxUrls", strThirdPartyTxUrls);
                    setRestartRequired(true);
                }
                break;
            case Language:
                if (settings.value("language") != value) {
                    settings.setValue("language", value);
                    setRestartRequired(true);
                }
                break;
            case UseEmbeddedMonospacedFont:
                m_use_embedded_monospaced_font = value.toBool();
                settings.setValue("UseEmbeddedMonospacedFont", m_use_embedded_monospaced_font);
                Q_EMIT useEmbeddedMonospacedFontChanged(m_use_embedded_monospaced_font);
                break;
            case CoinControlFeatures:
                fCoinControlFeatures = value.toBool();
                settings.setValue("fCoinControlFeatures", fCoinControlFeatures);
                Q_EMIT coinControlFeaturesChanged(fCoinControlFeatures);
                break;
            case Prune:
                if (settings.value("bPrune") != value) {
                    settings.setValue("bPrune", value);
                    setRestartRequired(true);
                }
                break;
            case PruneSize:
                if (settings.value("nPruneSize") != value) {
                    settings.setValue("nPruneSize", value);
                    setRestartRequired(true);
                }
                break;
            case DatabaseCache:
                if (settings.value("nDatabaseCache") != value) {
                    settings.setValue("nDatabaseCache", value);
                    setRestartRequired(true);
                }
                break;
            case ThreadsScriptVerif:
                if (settings.value("nThreadsScriptVerif") != value) {
                    settings.setValue("nThreadsScriptVerif", value);
                    setRestartRequired(true);
                }
                break;
            case Listen:
                if (settings.value("fListen") != value) {
                    settings.setValue("fListen", value);
                    setRestartRequired(true);
                }
                break;
            case Server:
                if (settings.value("server") != value) {
                    settings.setValue("server", value);
                    setRestartRequired(true);
                }
                break;
            default:
                break;
            }
        }

        Q_EMIT dataChanged(index, index);

        return successful;
        */
    }

    /**
      | Updates current unit in memory, settings
      | and emits displayUnitChanged(newUnit)
      | signal
      |
      */
    pub fn set_display_unit(&mut self, value: &QVariant)  {
        
        todo!();
        /*
            if (!value.isNull())
        {
            QSettings settings;
            nDisplayUnit = value.toInt();
            settings.setValue("nDisplayUnit", nDisplayUnit);
            Q_EMIT displayUnitChanged(nDisplayUnit);
        }
        */
    }
    
    pub fn set_restart_required(&mut self, required: bool)  {
        
        todo!();
        /*
            QSettings settings;
        return settings.setValue("fRestartRequired", fRequired);
        */
    }
    
    pub fn is_restart_required(&self) -> bool {
        
        todo!();
        /*
            QSettings settings;
        return settings.value("fRestartRequired", false).toBool();
        */
    }
    
    /**
      | Check settings version and upgrade
      | default values if required
      |
      */
    pub fn check_and_migrate(&mut self)  {
        
        todo!();
        /*
            // Migration of default values
        // Check if the QSettings container was already loaded with this client version
        QSettings settings;
        static const char strSettingsVersionKey[] = "nSettingsVersion";
        int settingsVersion = settings.contains(strSettingsVersionKey) ? settings.value(strSettingsVersionKey).toInt() : 0;
        if (settingsVersion < CLIENT_VERSION)
        {
            // -dbcache was bumped from 100 to 300 in 0.13
            // see https://github.com/bitcoin/bitcoin/pull/8273
            // force people to upgrade to the new value if they are using 100MB
            if (settingsVersion < 130000 && settings.contains("nDatabaseCache") && settings.value("nDatabaseCache").toLongLong() == 100)
                settings.setValue("nDatabaseCache", (i64)nDefaultDbCache);

            settings.setValue(strSettingsVersionKey, CLIENT_VERSION);
        }

        // Overwrite the 'addrProxy' setting in case it has been set to an illegal
        // default value (see issue #12623; PR #12650).
        if (settings.contains("addrProxy") && settings.value("addrProxy").toString().endsWith("%2")) {
            settings.setValue("addrProxy", GetDefaultProxyAddress());
        }

        // Overwrite the 'addrSeparateProxyTor' setting in case it has been set to an illegal
        // default value (see issue #12623; PR #12650).
        if (settings.contains("addrSeparateProxyTor") && settings.value("addrSeparateProxyTor").toString().endsWith("%2")) {
            settings.setValue("addrSeparateProxyTor", GetDefaultProxyAddress());
        }
        */
    }
}
