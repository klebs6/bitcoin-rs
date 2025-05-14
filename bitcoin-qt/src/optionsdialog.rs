// ---------------- [ File: bitcoin-qt/src/optionsdialog.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/optionsdialog.h]

/**
  | Proxy address widget validator, checks
  | for a valid proxy address.
  |
  */
#[Q_OBJECT]
pub struct ProxyAddressValidator {
    base: QValidator,
}

impl ProxyAddressValidator {
    
    pub fn new(parent: *mut QObject) -> Self {
    
        todo!();
        /*
           :
           QValidator(parent)
        */
    }
    
    pub fn validate(&self, 
        input: &mut String,
        pos:   &mut i32) -> QValidatorState {
        
        todo!();
        /*
            Q_UNUSED(pos);
        // Validate the proxy
        CService serv(LookupNumeric(input.toStdString(), DEFAULT_GUI_PROXY_PORT));
        proxyType addrProxy = proxyType(serv, true);
        if (addrProxy.IsValid())
            return QValidatorAcceptable;

        return QValidatorInvalid;
        */
    }
}

/**
  | Preferences dialog.
  |
  */
#[Q_OBJECT]
pub struct OptionsDialog {
    base:   QDialog,
    ui:     *mut UiOptionsDialog,
    model:  *mut OptionsModel,
    mapper: *mut QDataWidgetMapper,
}

pub mod options_dialog {

    pub enum Tab {
        TAB_MAIN,
        TAB_NETWORK,
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/optionsdialog.cpp]
impl Drop for OptionsDialog {
    fn drop(&mut self) {
        todo!();
        /*
            delete ui;
        */
    }
}

impl OptionsDialog {

    #[Q_SIGNAL]
    pub fn proxy_ip_checks(&mut self, 
        ui_proxy_ip:  *mut QValidatedLineEdit,
        n_proxy_port: u16)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn quit_on_reset(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn new(
        parent:        *mut QWidget,
        enable_wallet: bool) -> Self {
    
        todo!();
        /*
           :
        QDialog(parent, typename gui_util::dialog_flags),
        ui(new UiOptionsDialog),
        model(nullptr),
        mapper(nullptr)

        ui->setupUi(this);

        /* Main elements init */
        ui->databaseCache->setMinimum(nMinDbCache);
        ui->databaseCache->setMaximum(nMaxDbCache);
        ui->threadsScriptVerif->setMinimum(-GetNumCores());
        ui->threadsScriptVerif->setMaximum(MAX_SCRIPTCHECK_THREADS);
        ui->pruneWarning->setVisible(false);
        ui->pruneWarning->setStyleSheet("QLabel { color: red; }");

        ui->pruneSize->setEnabled(false);
        connect(ui->prune, &QPushButton::toggled, ui->pruneSize, &QWidget::setEnabled);

        /* Network elements init */
    #ifndef USE_UPNP
        ui->mapPortUpnp->setEnabled(false);
    #endif
    #ifndef USE_NATPMP
        ui->mapPortNatpmp->setEnabled(false);
    #endif
        connect(this, &QDialog::accepted, [this](){
            QSettings settings;
            model->node().mapPort(settings.value("fUseUPnP").toBool(), settings.value("fUseNatpmp").toBool());
        });

        ui->proxyIp->setEnabled(false);
        ui->proxyPort->setEnabled(false);
        ui->proxyPort->setValidator(new QIntValidator(1, 65535, this));

        ui->proxyIpTor->setEnabled(false);
        ui->proxyPortTor->setEnabled(false);
        ui->proxyPortTor->setValidator(new QIntValidator(1, 65535, this));

        connect(ui->connectSocks, &QPushButton::toggled, ui->proxyIp, &QWidget::setEnabled);
        connect(ui->connectSocks, &QPushButton::toggled, ui->proxyPort, &QWidget::setEnabled);
        connect(ui->connectSocks, &QPushButton::toggled, this, &OptionsDialog::updateProxyValidationState);

        connect(ui->connectSocksTor, &QPushButton::toggled, ui->proxyIpTor, &QWidget::setEnabled);
        connect(ui->connectSocksTor, &QPushButton::toggled, ui->proxyPortTor, &QWidget::setEnabled);
        connect(ui->connectSocksTor, &QPushButton::toggled, this, &OptionsDialog::updateProxyValidationState);

        /* Window elements init */
    #ifdef Q_OS_MAC
        /* remove Window tab on Mac */
        ui->tabWidget->removeTab(ui->tabWidget->indexOf(ui->tabWindow));
        /* hide launch at startup option on macOS */
        ui->bitcoinAtStartup->setVisible(false);
        ui->verticalLayout_Main->removeWidget(ui->bitcoinAtStartup);
        ui->verticalLayout_Main->removeItem(ui->horizontalSpacer_0_Main);
    #endif

        /* remove Wallet tab and 3rd party-URL textbox in case of -disablewallet */
        if (!enableWallet) {
            ui->tabWidget->removeTab(ui->tabWidget->indexOf(ui->tabWallet));
            ui->thirdPartyTxUrlsLabel->setVisible(false);
            ui->thirdPartyTxUrls->setVisible(false);
        }

    #ifndef ENABLE_EXTERNAL_SIGNER
        //: "External signing" means using devices such as hardware wallets.
        ui->externalSignerPath->setToolTip(tr("Compiled without external signing support (required for external signing)"));
        ui->externalSignerPath->setEnabled(false);
    #endif
        /* Display elements init */
        QDir translations(":translations");

        ui->bitcoinAtStartup->setToolTip(ui->bitcoinAtStartup->toolTip().arg(PACKAGE_NAME));
        ui->bitcoinAtStartup->setText(ui->bitcoinAtStartup->text().arg(PACKAGE_NAME));

        ui->openBitcoinConfButton->setToolTip(ui->openBitcoinConfButton->toolTip().arg(PACKAGE_NAME));

        ui->lang->setToolTip(ui->lang->toolTip().arg(PACKAGE_NAME));
        ui->lang->addItem(QString("(") + tr("default") + QString(")"), QVariant(""));
        for (const QString &langStr : translations.entryList())
        {
            QLocale locale(langStr);

            /** check if the locale name consists of 2 parts (language_country) */
            if(langStr.contains("_"))
            {
                /** display language strings as "native language - native country (locale name)", e.g. "Deutsch - Deutschland (de)" */
                ui->lang->addItem(locale.nativeLanguageName() + QString(" - ") + locale.nativeCountryName() + QString(" (") + langStr + QString(")"), QVariant(langStr));
            }
            else
            {
                /** display language strings as "native language (locale name)", e.g. "Deutsch (de)" */
                ui->lang->addItem(locale.nativeLanguageName() + QString(" (") + langStr + QString(")"), QVariant(langStr));
            }
        }
        ui->unit->setModel(new BitcoinUnits(this));

        /* Widget-to-option mapper */
        mapper = new QDataWidgetMapper(this);
        mapper->setSubmitPolicy(QDataWidgetMapper::ManualSubmit);
        mapper->setOrientation(QtVertical);

        typename gui_util::ItemDelegate* delegate = new typename gui_util::ItemDelegate(mapper);
        connect(delegate, &typename gui_util::ItemDelegate::keyEscapePressed, this, &OptionsDialog::reject);
        mapper->setItemDelegate(delegate);

        /* setup/change UI elements when proxy IPs are invalid/valid */
        ui->proxyIp->setCheckValidator(new ProxyAddressValidator(parent));
        ui->proxyIpTor->setCheckValidator(new ProxyAddressValidator(parent));
        connect(ui->proxyIp, &QValidatedLineEdit::validationDidChange, this, &OptionsDialog::updateProxyValidationState);
        connect(ui->proxyIpTor, &QValidatedLineEdit::validationDidChange, this, &OptionsDialog::updateProxyValidationState);
        connect(ui->proxyPort, &QLineEdit::textChanged, this, &OptionsDialog::updateProxyValidationState);
        connect(ui->proxyPortTor, &QLineEdit::textChanged, this, &OptionsDialog::updateProxyValidationState);

        if (!QSystemTrayIcon::isSystemTrayAvailable()) {
            ui->showTrayIcon->setChecked(false);
            ui->showTrayIcon->setEnabled(false);
            ui->minimizeToTray->setChecked(false);
            ui->minimizeToTray->setEnabled(false);
        }

        QFont embedded_font{typename gui_util::fixedPitchFont(true)};
        ui->embeddedFont_radioButton->setText(ui->embeddedFont_radioButton->text().arg(QFontInfo(embedded_font).family()));
        embedded_font.setWeight(QFont::Bold);
        ui->embeddedFont_label_1->setFont(embedded_font);
        ui->embeddedFont_label_9->setFont(embedded_font);

        QFont system_font{typename gui_util::fixedPitchFont(false)};
        ui->systemFont_radioButton->setText(ui->systemFont_radioButton->text().arg(QFontInfo(system_font).family()));
        system_font.setWeight(QFont::Bold);
        ui->systemFont_label_1->setFont(system_font);
        ui->systemFont_label_9->setFont(system_font);
        // Checking the embeddedFont_radioButton automatically unchecks the systemFont_radioButton.
        ui->systemFont_radioButton->setChecked(true);

        typename gui_util::handleCloseWindowShortcut(this);
        */
    }
    
    pub fn set_model(&mut self, model: *mut OptionsModel)  {
        
        todo!();
        /*
            this->model = _model;

        if(_model)
        {
            /* check if client restart is needed and show persistent message */
            if (_model->isRestartRequired())
                showRestartWarning(true);

            // Prune values are in GB to be consistent with intro.cpp
            static constexpr uint64_t nMinDiskSpace = (MIN_DISK_SPACE_FOR_BLOCK_FILES / GB_BYTES) + (MIN_DISK_SPACE_FOR_BLOCK_FILES % GB_BYTES) ? 1 : 0;
            ui->pruneSize->setRange(nMinDiskSpace, std::numeric_limits<int>::max());

            QString strLabel = _model->getOverriddenByCommandLine();
            if (strLabel.isEmpty())
                strLabel = tr("none");
            ui->overriddenByCommandLineLabel->setText(strLabel);

            mapper->setModel(_model);
            setMapper();
            mapper->toFirst();

            updateDefaultProxyNets();
        }

        /* warn when one of the following settings changes by user action (placed here so init via mapper doesn't trigger them) */

        /* Main */
        connect(ui->prune, &QCheckBox::clicked, this, &OptionsDialog::showRestartWarning);
        connect(ui->prune, &QCheckBox::clicked, this, &OptionsDialog::togglePruneWarning);
        connect(ui->pruneSize, qOverload<int>(&QSpinBox::valueChanged), this, &OptionsDialog::showRestartWarning);
        connect(ui->databaseCache, qOverload<int>(&QSpinBox::valueChanged), this, &OptionsDialog::showRestartWarning);
        connect(ui->externalSignerPath, &QLineEdit::textChanged, [this]{ showRestartWarning(); });
        connect(ui->threadsScriptVerif, qOverload<int>(&QSpinBox::valueChanged), this, &OptionsDialog::showRestartWarning);
        /* Wallet */
        connect(ui->spendZeroConfChange, &QCheckBox::clicked, this, &OptionsDialog::showRestartWarning);
        /* Network */
        connect(ui->allowIncoming, &QCheckBox::clicked, this, &OptionsDialog::showRestartWarning);
        connect(ui->enableServer, &QCheckBox::clicked, this, &OptionsDialog::showRestartWarning);
        connect(ui->connectSocks, &QCheckBox::clicked, this, &OptionsDialog::showRestartWarning);
        connect(ui->connectSocksTor, &QCheckBox::clicked, this, &OptionsDialog::showRestartWarning);
        /* Display */
        connect(ui->lang, qOverload<>(&QValueComboBox::valueChanged), [this]{ showRestartWarning(); });
        connect(ui->thirdPartyTxUrls, &QLineEdit::textChanged, [this]{ showRestartWarning(); });
        */
    }
    
    pub fn set_current_tab(&mut self, tab: OptionsDialogTab)  {
        
        todo!();
        /*
            QWidget *tab_widget = nullptr;
        if (tab == OptionsDialog::Tab::TAB_NETWORK) tab_widget = ui->tabNetwork;
        if (tab == OptionsDialog::Tab::TAB_MAIN) tab_widget = ui->tabMain;
        if (tab_widget && ui->tabWidget->currentWidget() != tab_widget) {
            ui->tabWidget->setCurrentWidget(tab_widget);
        }
        */
    }
    
    pub fn set_mapper(&mut self)  {
        
        todo!();
        /*
            /* Main */
        mapper->addMapping(ui->bitcoinAtStartup, OptionsModel::StartAtStartup);
        mapper->addMapping(ui->threadsScriptVerif, OptionsModel::ThreadsScriptVerif);
        mapper->addMapping(ui->databaseCache, OptionsModel::DatabaseCache);
        mapper->addMapping(ui->prune, OptionsModel::Prune);
        mapper->addMapping(ui->pruneSize, OptionsModel::PruneSize);

        /* Wallet */
        mapper->addMapping(ui->spendZeroConfChange, OptionsModel::SpendZeroConfChange);
        mapper->addMapping(ui->coinControlFeatures, OptionsModel::CoinControlFeatures);
        mapper->addMapping(ui->subFeeFromAmount, OptionsModel::SubFeeFromAmount);
        mapper->addMapping(ui->externalSignerPath, OptionsModel::ExternalSignerPath);

        /* Network */
        mapper->addMapping(ui->mapPortUpnp, OptionsModel::MapPortUPnP);
        mapper->addMapping(ui->mapPortNatpmp, OptionsModel::MapPortNatpmp);
        mapper->addMapping(ui->allowIncoming, OptionsModel::Listen);
        mapper->addMapping(ui->enableServer, OptionsModel::Server);

        mapper->addMapping(ui->connectSocks, OptionsModel::ProxyUse);
        mapper->addMapping(ui->proxyIp, OptionsModel::ProxyIP);
        mapper->addMapping(ui->proxyPort, OptionsModel::ProxyPort);

        mapper->addMapping(ui->connectSocksTor, OptionsModel::ProxyUseTor);
        mapper->addMapping(ui->proxyIpTor, OptionsModel::ProxyIPTor);
        mapper->addMapping(ui->proxyPortTor, OptionsModel::ProxyPortTor);

        /* Window */
    #ifndef Q_OS_MAC
        if (QSystemTrayIcon::isSystemTrayAvailable()) {
            mapper->addMapping(ui->showTrayIcon, OptionsModel::ShowTrayIcon);
            mapper->addMapping(ui->minimizeToTray, OptionsModel::MinimizeToTray);
        }
        mapper->addMapping(ui->minimizeOnClose, OptionsModel::MinimizeOnClose);
    #endif

        /* Display */
        mapper->addMapping(ui->lang, OptionsModel::Language);
        mapper->addMapping(ui->unit, OptionsModel::DisplayUnit);
        mapper->addMapping(ui->thirdPartyTxUrls, OptionsModel::ThirdPartyTxUrls);
        mapper->addMapping(ui->embeddedFont_radioButton, OptionsModel::UseEmbeddedMonospacedFont);
        */
    }
    
    /**
      | set OK button state (enabled / disabled)
      |
      */
    #[Q_SLOT]
    pub fn set_ok_button_state(&mut self, state: bool)  {
        
        todo!();
        /*
            ui->okButton->setEnabled(fState);
        */
    }
    
    #[Q_SLOT]
    pub fn on_reset_button_clicked(&mut self)  {
        
        todo!();
        /*
            if(model)
        {
            // confirmation dialog
            QMessageBox::StandardButton btnRetVal = QMessageBox::question(this, tr("Confirm options reset"),
                tr("Client restart required to activate changes.") + "<br><br>" + tr("Client will be shut down. Do you want to proceed?"),
                QMessageBox::Yes | QMessageBox::Cancel, QMessageBox::Cancel);

            if(btnRetVal == QMessageBox::Cancel)
                return;

            /* reset all options and close GUI */
            model->Reset();
            close();
            Q_EMIT quitOnReset();
        }
        */
    }
    
    #[Q_SLOT]
    pub fn on_open_bitcoin_conf_button_clicked(&mut self)  {
        
        todo!();
        /*
            QMessageBox config_msgbox(this);
        config_msgbox.setIcon(QMessageBox::Information);
        //: Window title text of pop-up box that allows opening up of configuration file.
        config_msgbox.setWindowTitle(tr("Configuration options"));
        /*: Explanatory text about the priority order of instructions considered by client.
            The order from high to low being: command-line, configuration file, GUI settings. */
        config_msgbox.setText(tr("The configuration file is used to specify advanced user options which override GUI settings. "
                                 "Additionally, any command-line options will override this configuration file."));

        QPushButton* open_button = config_msgbox.addButton(tr("Continue"), QMessageBox::ActionRole);
        config_msgbox.addButton(tr("Cancel"), QMessageBox::RejectRole);
        open_button->setDefault(true);

        config_msgbox.exec();

        if (config_msgbox.clickedButton() != open_button) return;

        /* show an error if there was some problem opening the file */
        if (!typename gui_util::openBitcoinConf())
            QMessageBox::critical(this, tr("Error"), tr("The configuration file could not be opened."));
        */
    }
    
    #[Q_SLOT]
    pub fn on_ok_button_clicked(&mut self)  {
        
        todo!();
        /*
            mapper->submit();
        accept();
        updateDefaultProxyNets();
        */
    }
    
    #[Q_SLOT]
    pub fn on_cancel_button_clicked(&mut self)  {
        
        todo!();
        /*
            reject();
        */
    }
    
    #[Q_SLOT]
    pub fn on_show_tray_icon_state_changed(&mut self, state: i32)  {
        
        todo!();
        /*
            if (state == QtChecked) {
            ui->minimizeToTray->setEnabled(true);
        } else {
            ui->minimizeToTray->setChecked(false);
            ui->minimizeToTray->setEnabled(false);
        }
        */
    }
    
    #[Q_SLOT]
    pub fn toggle_prune_warning(&mut self, enabled: bool)  {
        
        todo!();
        /*
            ui->pruneWarning->setVisible(!ui->pruneWarning->isVisible());
        */
    }
    
    #[Q_SLOT]
    pub fn show_restart_warning(&mut self, persistent: Option<bool>)  {

        let persistent: bool = persistent.unwrap_or(false);
        
        todo!();
        /*
            ui->statusLabel->setStyleSheet("QLabel { color: red; }");

        if(fPersistent)
        {
            ui->statusLabel->setText(tr("Client restart required to activate changes."));
        }
        else
        {
            ui->statusLabel->setText(tr("This change would require a client restart."));
            // clear non-persistent status label after 10 seconds
            // Todo: should perhaps be a class attribute, if we extend the use of statusLabel
            QTimer::singleShot(10000, this, &OptionsDialog::clearStatusLabel);
        }
        */
    }
    
    #[Q_SLOT]
    pub fn clear_status_label(&mut self)  {
        
        todo!();
        /*
            ui->statusLabel->clear();
        if (model && model->isRestartRequired()) {
            showRestartWarning(true);
        }
        */
    }
    
    #[Q_SLOT]
    pub fn update_proxy_validation_state(&mut self)  {
        
        todo!();
        /*
            QValidatedLineEdit *pUiProxyIp = ui->proxyIp;
        QValidatedLineEdit *otherProxyWidget = (pUiProxyIp == ui->proxyIpTor) ? ui->proxyIp : ui->proxyIpTor;
        if (pUiProxyIp->isValid() && (!ui->proxyPort->isEnabled() || ui->proxyPort->text().toInt() > 0) && (!ui->proxyPortTor->isEnabled() || ui->proxyPortTor->text().toInt() > 0))
        {
            setOkButtonState(otherProxyWidget->isValid()); //only enable ok button if both proxys are valid
            clearStatusLabel();
        }
        else
        {
            setOkButtonState(false);
            ui->statusLabel->setStyleSheet("QLabel { color: red; }");
            ui->statusLabel->setText(tr("The supplied proxy address is invalid."));
        }
        */
    }
    
    /**
      | query the networks, for which the default
      | proxy is used
      |
      */
    #[Q_SLOT]
    pub fn update_default_proxy_nets(&mut self)  {
        
        todo!();
        /*
            proxyType proxy;
        std::string strProxy;
        QString strDefaultProxyGUI;

        model->node().getProxy(NET_IPV4, proxy);
        strProxy = proxy.proxy.ToStringIP() + ":" + proxy.proxy.ToStringPort();
        strDefaultProxyGUI = ui->proxyIp->text() + ":" + ui->proxyPort->text();
        (strProxy == strDefaultProxyGUI.toStdString()) ? ui->proxyReachIPv4->setChecked(true) : ui->proxyReachIPv4->setChecked(false);

        model->node().getProxy(NET_IPV6, proxy);
        strProxy = proxy.proxy.ToStringIP() + ":" + proxy.proxy.ToStringPort();
        strDefaultProxyGUI = ui->proxyIp->text() + ":" + ui->proxyPort->text();
        (strProxy == strDefaultProxyGUI.toStdString()) ? ui->proxyReachIPv6->setChecked(true) : ui->proxyReachIPv6->setChecked(false);

        model->node().getProxy(NET_ONION, proxy);
        strProxy = proxy.proxy.ToStringIP() + ":" + proxy.proxy.ToStringPort();
        strDefaultProxyGUI = ui->proxyIp->text() + ":" + ui->proxyPort->text();
        (strProxy == strDefaultProxyGUI.toStdString()) ? ui->proxyReachTor->setChecked(true) : ui->proxyReachTor->setChecked(false);
        */
    }
}
