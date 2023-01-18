crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/guiutil.h]

/**
  | Utility functions used by the Bitcoin
  | Qt UI.
  |
  */
pub mod gui_util {

    use super::*;

    /**
      | Use this flags to prevent a "What's This"
      | button in the title bar of the dialog
      | on
      | 
      | Windows.
      |
      */
    pub const DIALOG_FLAGS: u32 = 
        QtWindowTitleHint 
        | QtWindowSystemMenuHint 
        | QtWindowCloseButtonHint;

    /**
      | Qt event filter that intercepts ToolTipChange
      | events, and replaces the tooltip with
      | a rich text representation if needed.
      | This assures that Qt can word-wrap long
      | tooltip messages.
      | 
      | Tooltips longer than the provided size
      | threshold (in characters) are wrapped.
      |
      */
    #[Q_OBJECT]
    pub struct ToolTipToRichTextFilter {
        base:           QObject,
        size_threshold: i32,
    }

    /**
      | Qt event filter that intercepts QEvent::FocusOut
      | events for QLabel objects, and resets
      | their `textInteractionFlags' property
      | to get rid of the visible cursor.
      | 
      | This is a temporary fix of QTBUG-59514.
      |
      */
    #[Q_OBJECT]
    pub struct LabelOutOfFocusEventFilter {
        base: QObject,
    }

    ///-----------------------------
    #[Q_OBJECT]
    pub struct ThemedLabel {
        base:           QLabel,
        platform_style: *const PlatformStyle,
        image_filename: String,
        pixmap_width:   i32,
        pixmap_height:  i32,
    }

    ///-----------------------
    #[Q_OBJECT]
    pub struct ClickableLabel {
        base: ThemedLabel,
    }

    impl ClickableLabel {
        
        /**
          | Emitted when the label is clicked. The
          | relative mouse coordinates of the click
          | are passed to the signal.
          |
          */
        #[Q_SIGNAL]
        pub fn clicked(&mut self, point: &QPoint)  {
            
        }
    }

    ///-----------------------
    #[Q_OBJECT]
    pub struct ClickableProgressBar {
        base: QProgressBar,
    }

    impl ClickableProgressBar {

        /**
          | Emitted when the progressbar is clicked.
          | The relative mouse coordinates of the
          | click are passed to the signal.
          |
          */
        #[Q_SIGNAL]
        pub fn clicked(&mut self, point: &QPoint)  {
            
        }
    }

    pub type ProgressBar = ClickableProgressBar;

    ///----------------------
    #[Q_OBJECT]
    pub struct ItemDelegate {
        base: QItemDelegate,
    }

    impl ItemDelegate {
        
        pub fn new(parent: *mut QObject) -> Self {
        
            todo!();
            /*
            : q_item_delegate(parent),

            
            */
        }

        #[Q_SIGNAL]
        pub fn key_escape_pressed(&mut self)  {
            
        }
    }

    /**
      | Splits the string into substrings wherever
      | separator occurs, and returns the list
      | of those strings. Empty strings do not
      | appear in the result.
      | 
      | QString::split() signature differs
      | in different Qt versions:
      | 
      | - QString::SplitBehavior is deprecated
      | since Qt 5.15
      | 
      | - QtSplitBehavior was introduced
      | in Qt 5.14
      | 
      | If {QString|Qt}::SkipEmptyParts
      | behavior is required, use this function
      | instead of QString::split().
      |
      */
    pub fn split_skip_empty_parts<SeparatorType>(
            string:    &String,
            separator: &SeparatorType) -> QStringList {

        todo!();
            /*
                #if (QT_VERSION >= QT_VERSION_CHECK(5, 14, 0))
                return string.split(separator, QtSkipEmptyParts);
            #else
                return string.split(separator, QString::SkipEmptyParts);
            #endif
            */
    }

    /**
      | Queue a function to run in an object's
      | event loop. This can be replaced by a
      | call to the QMetaObject::invokeMethod
      | functor overload after Qt 5.10, but
      | for now use a QObject::connect for compatibility
      | with older Qt versions, based on https://stackoverflow.com/questions/21646467/how-to-execute-a-functor-or-a-lambda-in-a-given-thread-in-qt-gcd-style
      |
      */
    pub fn object_invoke<Fn>(
            object:     *mut QObject,
            function:   Fn,
            connection: Option<QtConnectionType>)  {

        let connection: QtConnectionType =
                     connection.unwrap_or(QtQueuedConnection);

        todo!();
            /*
                QObject source;
                QObject::connect(&source, &QObject::destroyed, object, std::forward<Fn>(function), connection);
            */
    }

    /**
      | A drop-in replacement of QObject::connect
      | function (see: https://doc.qt.io/qt-5/qobject.html#connect-3),
      | that guaranties that all exceptions
      | are handled within the slot.
      | 
      | -----------
      | @note
      | 
      | This function is incompatible with
      | Qt private signals.
      |
      */
    pub fn exception_safe_connect<Sender, Signal, Receiver, Slot, R>(
            sender:   Sender,
            signal:   Signal,
            receiver: Receiver,
            method:   Slot,
            ty:       Option<QtConnectionType>) -> R {

        let ty: QtConnectionType =
            ty.unwrap_or(QtAutoConnection);

        todo!();
            /*
                return QObject::connect(
                    sender, signal, receiver,
                    [sender, receiver, method](auto&&... args) {
                        bool ok{true};
                        try {
                            (receiver->*method)(std::forward<decltype(args)>(args)...);
                        } catch (const NonFatalCheckError& e) {
                            PrintSlotException(&e, sender, receiver);
                            ok = QMetaObject::invokeMethod(
                                qApp, "handleNonFatalException",
                                blockingGUIThreadConnection(),
                                Q_ARG(QString, QString::fromStdString(e.what())));
                        } catch (const std::exception& e) {
                            PrintSlotException(&e, sender, receiver);
                            ok = QMetaObject::invokeMethod(
                                qApp, "handleRunawayException",
                                blockingGUIThreadConnection(),
                                Q_ARG(QString, QString::fromStdString(e.what())));
                        } catch (...) {
                            PrintSlotException(nullptr, sender, receiver);
                            ok = QMetaObject::invokeMethod(
                                qApp, "handleRunawayException",
                                blockingGUIThreadConnection(),
                                Q_ARG(QString, "Unknown failure occurred."));
                        }
                        assert(ok);
                    },
                    type);
            */
    }

    /**
      | Create human-readable string from
      | date
      |
      */
    pub fn date_time_str_from_qdatetime(date: &QDateTime) -> String {
        
        todo!();
            /*
                return QLocale::system().toString(date.date(), QLocale::ShortFormat) + QString(" ") + date.toString("hh:mm");
            */
    }

    pub fn date_time_str(n_time: i64) -> String {
        
        todo!();
            /*
                return dateTimeStr(QDateTime::fromSecsSinceEpoch(nTime));
            */
    }

    /**
      | Return a monospace font
      |
      */
    pub fn fixed_pitch_font(use_embedded_font: Option<bool>) -> QFont {
        let use_embedded_font: bool = use_embedded_font.unwrap_or(false);
        
        todo!();
            /*
                if (use_embedded_font) {
                return {"Roboto Mono"};
            }
            return QFontDatabase::systemFont(QFontDatabase::FixedFont);
            */
    }

    /**
      | Just some dummy data to generate a convincing
      | random-looking (but consistent) address
      |
      */
    pub const DUMMYDATA: &[u8] = 
    &[
        0xeb,0x15,0x23,0x1d,0xfc,0xeb,0x60,0x92,0x58,0x86,0xb6,0x7d,0x06,0x52,0x99,0x92,0x59,0x15,0xae,0xb1,0x72,0xc0,0x66,0x47
    ];

    /**
      | Generate a dummy address with invalid
      | CRC, starting with the network prefix.
      |
      */
    pub fn dummy_address(params: &ChainParams) -> String {
        
        todo!();
            /*
                std::vector<unsigned char> sourcedata = params.Base58Prefix(CChainParams::PUBKEY_ADDRESS);
            sourcedata.insert(sourcedata.end(), dummydata, dummydata + sizeof(dummydata));
            for(int i=0; i<256; ++i) { // Try every trailing byte
                std::string s = EncodeBase58(sourcedata);
                if (!IsValidDestinationString(s)) {
                    return s;
                }
                sourcedata[sourcedata.size()-1] += 1;
            }
            return "";
            */
    }

    /**
      | Set up widget for address
      |
      */
    pub fn setup_address_widget(
            widget: *mut QValidatedLineEdit,
            parent: *mut QWidget)  {
        
        todo!();
            /*
                parent->setFocusProxy(widget);

            widget->setFont(fixedPitchFont());
            // We don't want translators to use own addresses in translations
            // and this is the only place, where this address is supplied.
            widget->setPlaceholderText(QObject::tr("Enter a Bitcoin address (e.g. %1)").arg(
                QString::fromStdString(DummyAddress(Params()))));
            widget->setValidator(new BitcoinAddressEntryValidator(parent));
            widget->setCheckValidator(new BitcoinAddressCheckValidator(parent));
            */
    }

    /**
      | Connects an additional shortcut to
      | a QAbstractButton. Works around the
      | one shortcut limitation of the button's
      | shortcut property.
      | 
      | -----------
      | @param[in] button
      | 
      | QAbstractButton to assign shortcut
      | to
      | ----------
      | @param[in] shortcut
      | 
      | QKeySequence to use as shortcut
      |
      */
    pub fn add_button_shortcut(
            button:   *mut QAbstractButton,
            shortcut: &QKeySequence)  {
        
        todo!();
            /*
                QObject::connect(new QShortcut(shortcut, button), &QShortcut::activated, [button]() { button->animateClick(); });
            */
    }

    /**
      | Parse "bitcoin:" URI into recipient
      | object, return true on successful parsing
      |
      */
    pub fn parse_bitcoinuri_with_qurl(
            uri: &QUrl,
            out: *mut SendCoinsRecipient) -> bool {
        
        todo!();
            /*
                // return if URI is not valid or is no bitcoin: URI
            if(!uri.isValid() || uri.scheme() != QString("bitcoin"))
                return false;

            SendCoinsRecipient rv;
            rv.address = uri.path();
            // Trim any following forward slash which may have been added by the OS
            if (rv.address.endsWith("/")) {
                rv.address.truncate(rv.address.length() - 1);
            }
            rv.amount = 0;

            QUrlQuery uriQuery(uri);
            QList<QPair<QString, QString> > items = uriQuery.queryItems();
            for (QList<QPair<QString, QString> >::iterator i = items.begin(); i != items.end(); i++)
            {
                bool fShouldReturnFalse = false;
                if (i->first.startsWith("req-"))
                {
                    i->first.remove(0, 4);
                    fShouldReturnFalse = true;
                }

                if (i->first == "label")
                {
                    rv.label = i->second;
                    fShouldReturnFalse = false;
                }
                if (i->first == "message")
                {
                    rv.message = i->second;
                    fShouldReturnFalse = false;
                }
                else if (i->first == "amount")
                {
                    if(!i->second.isEmpty())
                    {
                        if(!BitcoinUnits::parse(BitcoinUnits::BTC, i->second, &rv.amount))
                        {
                            return false;
                        }
                    }
                    fShouldReturnFalse = false;
                }

                if (fShouldReturnFalse)
                    return false;
            }
            if(out)
            {
                *out = rv;
            }
            return true;
            */
    }

    ///----------------------
    pub fn parse_bitcoinuri(
            uri: String,
            out: *mut SendCoinsRecipient) -> bool {
        
        todo!();
            /*
                QUrl uriInstance(uri);
            return parseBitcoinURI(uriInstance, out);
            */
    }

    pub fn format_bitcoinuri(info: &SendCoinsRecipient) -> String {
        
        todo!();
            /*
                bool bech_32 = info.address.startsWith(QString::fromStdString(Params().Bech32HRP() + "1"));

            QString ret = QString("bitcoin:%1").arg(bech_32 ? info.address.toUpper() : info.address);
            int paramCount = 0;

            if (info.amount)
            {
                ret += QString("?amount=%1").arg(BitcoinUnits::format(BitcoinUnits::BTC, info.amount, false, BitcoinUnits::SeparatorStyle::NEVER));
                paramCount++;
            }

            if (!info.label.isEmpty())
            {
                QString lbl(QUrl::toPercentEncoding(info.label));
                ret += QString("%1label=%2").arg(paramCount == 0 ? "?" : "&").arg(lbl);
                paramCount++;
            }

            if (!info.message.isEmpty())
            {
                QString msg(QUrl::toPercentEncoding(info.message));
                ret += QString("%1message=%2").arg(paramCount == 0 ? "?" : "&").arg(msg);
                paramCount++;
            }

            return ret;
            */
    }

    /**
      | Returns true if given address+amount
      | meets "dust" definition
      |
      */
    pub fn is_dust(
            node:    Rc<RefCell<dyn NodeInterface>>,
            address: &String,
            amount:  &Amount) -> bool {
        
        todo!();
            /*
                TxDestination dest = DecodeDestination(address.toStdString());
            CScript script = GetScriptForDestination(dest);
            CTxOut txOut(amount, script);
            return IsDust(txOut, node.getDustRelayFee());
            */
    }

    /**
      | HTML escaping for rich text controls
      |
      */
    pub fn html_escape(
            str_:       &String,
            multi_line: Option<bool>) -> String {

        let multi_line: bool = multi_line.unwrap_or(false);
        
        todo!();
            /*
                QString escaped = str.toHtmlEscaped();
            if(fMultiLine)
            {
                escaped = escaped.replace("\n", "<br>\n");
            }
            return escaped;
            */
    }

    /**
      | Copy a field of the currently selected
      | entry of a view to the clipboard. Does
      | nothing if nothing is selected.
      | 
      | -----------
      | @param[in] column
      | 
      | Data column to extract from the model
      | ----------
      | @param[in] role
      | 
      | Data role to extract from the model @see
      | TransactionView::copyLabel, TransactionView::copyAmount,
      | TransactionView::copyAddress
      |
      */
    pub fn copy_entry_data(
            view:   *const QAbstractItemView,
            column: i32,
            role:   Option<i32>)  {

        let role: i32 = role.unwrap_or(*QtEditRole);
        
        todo!();
            /*
                if(!view || !view->selectionModel())
                return;
            QModelIndexList selection = view->selectionModel()->selectedRows(column);

            if(!selection.isEmpty())
            {
                // Copy first item
                setClipboard(selection.at(0).data(role).toString());
            }
            */
    }

    /**
      | Return a field of the currently selected
      | entry as a QString. Does nothing if nothing
      | is selected.
      | 
      | -----------
      | @param[in] column
      | 
      | Data column to extract from the model
      | @see TransactionView::copyLabel,
      | TransactionView::copyAmount, TransactionView::copyAddress
      |
      */
    pub fn get_entry_data(
            view:   *const QAbstractItemView,
            column: i32) -> QList<QModelIndex> {
        
        todo!();
            /*
                if(!view || !view->selectionModel())
                return QList<QModelIndex>();
            return view->selectionModel()->selectedRows(column);
            */
    }

    /**
      | Returns true if the specified field
      | of the currently selected view entry
      | is not empty.
      | 
      | -----------
      | @param[in] column
      | 
      | Data column to extract from the model
      | ----------
      | @param[in] role
      | 
      | Data role to extract from the model @see
      | TransactionView::contextualMenu
      |
      */
    pub fn has_entry_data(
            view:   *const QAbstractItemView,
            column: i32,
            role:   i32) -> bool {
        
        todo!();
            /*
                QModelIndexList selection = getEntryData(view, column);
            if (selection.isEmpty()) return false;
            return !selection.at(0).data(role).toString().isEmpty();
            */
    }

    /**
      | Loads the font from the file specified
      | by file_name, aborts if it fails.
      |
      */
    pub fn load_font(file_name: &String)  {
        
        todo!();
            /*
                const int id = QFontDatabase::addApplicationFont(file_name);
            assert(id != -1);
            */
    }

    /**
      | Determine default data directory for
      | operating system.
      |
      */
    pub fn get_default_data_directory() -> String {
        
        todo!();
            /*
                return boostPathToQString(GetDefaultDataDir());
            */
    }

    /**
      | Get save filename, mimics QFileDialog::getSaveFileName,
      | except that it appends a default suffix
      | when no suffix is provided by the user.
      | 
      | -----------
      | @param[in] parent
      | 
      | Parent window (or 0)
      | ----------
      | @param[in] caption
      | 
      | Window caption (or empty, for default)
      | ----------
      | @param[in] dir
      | 
      | Starting directory (or empty, to default
      | to documents directory)
      | ----------
      | @param[in] filter
      | 
      | Filter specification such as "Comma
      | Separated Files (*.csv)"
      | ----------
      | @param[out] selectedSuffixOut
      | 
      | Pointer to return the suffix (file type)
      | that was selected (or 0).
      | 
      | Can be useful when choosing the save
      | file format based on suffix.
      |
      */
    pub fn get_save_file_name(
            parent:              *mut QWidget,
            caption:             &String,
            dir:                 &String,
            filter:              &String,
            selected_suffix_out: *mut String) -> String {
        
        todo!();
            /*
                QString selectedFilter;
            QString myDir;
            if(dir.isEmpty()) // Default to user documents location
            {
                myDir = QStandardPaths::writableLocation(QStandardPaths::DocumentsLocation);
            }
            else
            {
                myDir = dir;
            }
            /* Directly convert path to native OS path separators */
            QString result = QDir::toNativeSeparators(QFileDialog::getSaveFileName(parent, caption, myDir, filter, &selectedFilter));

            /* Extract first suffix from filter pattern "Description (*.foo)" or "Description (*.foo *.bar ...) */
            QRegExp filter_re(".* \\(\\*\\.(.*)[ \\)]");
            QString selectedSuffix;
            if(filter_re.exactMatch(selectedFilter))
            {
                selectedSuffix = filter_re.cap(1);
            }

            /* Add suffix if needed */
            QFileInfo info(result);
            if(!result.isEmpty())
            {
                if(info.suffix().isEmpty() && !selectedSuffix.isEmpty())
                {
                    /* No suffix specified, add selected suffix */
                    if(!result.endsWith("."))
                        result.append(".");
                    result.append(selectedSuffix);
                }
            }

            /* Return selected suffix if asked to */
            if(selectedSuffixOut)
            {
                *selectedSuffixOut = selectedSuffix;
            }
            return result;
            */
    }

    /**
      | Get open filename, convenience wrapper
      | for QFileDialog::getOpenFileName.
      | 
      | -----------
      | @param[in] parent
      | 
      | Parent window (or 0)
      | ----------
      | @param[in] caption
      | 
      | Window caption (or empty, for default)
      | ----------
      | @param[in] dir
      | 
      | Starting directory (or empty, to default
      | to documents directory)
      | ----------
      | @param[in] filter
      | 
      | Filter specification such as "Comma
      | Separated Files (*.csv)"
      | ----------
      | @param[out] selectedSuffixOut
      | 
      | Pointer to return the suffix (file type)
      | that was selected (or 0).
      | 
      | Can be useful when choosing the save
      | file format based on suffix.
      |
      */
    pub fn get_open_file_name(
            parent:              *mut QWidget,
            caption:             &String,
            dir:                 &String,
            filter:              &String,
            selected_suffix_out: *mut String) -> String {
        
        todo!();
            /*
                QString selectedFilter;
            QString myDir;
            if(dir.isEmpty()) // Default to user documents location
            {
                myDir = QStandardPaths::writableLocation(QStandardPaths::DocumentsLocation);
            }
            else
            {
                myDir = dir;
            }
            /* Directly convert path to native OS path separators */
            QString result = QDir::toNativeSeparators(QFileDialog::getOpenFileName(parent, caption, myDir, filter, &selectedFilter));

            if(selectedSuffixOut)
            {
                /* Extract first suffix from filter pattern "Description (*.foo)" or "Description (*.foo *.bar ...) */
                QRegExp filter_re(".* \\(\\*\\.(.*)[ \\)]");
                QString selectedSuffix;
                if(filter_re.exactMatch(selectedFilter))
                {
                    selectedSuffix = filter_re.cap(1);
                }
                *selectedSuffixOut = selectedSuffix;
            }
            return result;
            */
    }

    /**
      | Get connection type to call object slot
      | in GUI thread with invokeMethod. The
      | call will be blocking.
      | 
      | -----------
      | @return
      | 
      | If called from the GUI thread, return
      | a QtDirectConnection.
      | 
      | If called from another thread, return
      | a QtBlockingQueuedConnection.
      |
      */
    pub fn blocking_gui_thread_connection() -> QtConnectionType {
        
        todo!();
            /*
                if(QThread::currentThread() != qApp->thread())
            {
                return QtBlockingQueuedConnection;
            }
            else
            {
                return QtDirectConnection;
            }
            */
    }

    pub fn check_point(
            p: &QPoint,
            w: *const QWidget) -> bool {
        
        todo!();
            /*
                QWidget *atW = QApplication::widgetAt(w->mapToGlobal(p));
            if (!atW) return false;
            return atW->window() == w;
            */
    }

    /**
      | Determine whether a widget is hidden
      | behind other windows
      |
      */
    pub fn is_obscured(w: *mut QWidget) -> bool {
        
        todo!();
            /*
                return !(checkPoint(QPoint(0, 0), w)
                && checkPoint(QPoint(w->width() - 1, 0), w)
                && checkPoint(QPoint(0, w->height() - 1), w)
                && checkPoint(QPoint(w->width() - 1, w->height() - 1), w)
                && checkPoint(QPoint(w->width() / 2, w->height() / 2), w));
            */
    }

    /**
      | Activate, show and raise the widget
      |
      */
    pub fn bring_to_front(w: *mut QWidget)  {
        
        todo!();
            /*
                #ifdef Q_OS_MAC
            ForceActivation();
        #endif

            if (w) {
                // activateWindow() (sometimes) helps with keyboard focus on Windows
                if (w->isMinimized()) {
                    w->showNormal();
                } else {
                    w->show();
                }
                w->activateWindow();
                w->raise();
            }
            */
    }

    /**
      | Set shortcut to close window
      |
      */
    pub fn handle_close_window_shortcut(w: *mut QWidget)  {
        
        todo!();
            /*
                QObject::connect(new QShortcut(QKeySequence(QtCTRL + QtKey_W), w), &QShortcut::activated, w, &QWidget::close);
            */
    }

    /**
      | Open debug.log
      |
      */
    pub fn open_debug_logfile()  {
        
        todo!();
            /*
                fs::path pathDebug = gArgs.GetDataDirNet() / "debug.log";

            /* Open debug.log with the associated application */
            if (fs::exists(pathDebug))
                QDesktopServices::openUrl(QUrl::fromLocalFile(boostPathToQString(pathDebug)));
            */
    }

    /**
      | Open the config file
      |
      */
    pub fn open_bitcoin_conf() -> bool {
        
        todo!();
            /*
                fs::path pathConfig = GetConfigFile(gArgs.GetArg("-conf", BITCOIN_CONF_FILENAME));

            /* Create the file */
            fsbridge::ofstream configFile(pathConfig, std::ios_base::app);

            if (!configFile.good())
                return false;

            configFile.close();

            /* Open bitcoin.conf with the associated application */
            bool res = QDesktopServices::openUrl(QUrl::fromLocalFile(boostPathToQString(pathConfig)));
        #ifdef Q_OS_MAC
            // Workaround for macOS-specific behavior; see #15409.
            if (!res) {
                res = QProcess::startDetached("/usr/bin/open", QStringList{"-t", boostPathToQString(pathConfig)});
            }
        #endif

            return res;
            */
    }

    impl ToolTipToRichTextFilter {

        pub fn new(
            size_threshold: i32,
            parent:         *mut QObject) -> Self {
        
            todo!();
            /*
            : q_object(parent),
            : size_threshold(_size_threshold),

            
            */
        }
        
        pub fn event_filter(&mut self, 
            obj: *mut QObject,
            evt: *mut QEvent) -> bool {
            
            todo!();
            /*
                if(evt->type() == QEvent::ToolTipChange)
            {
                QWidget *widget = static_cast<QWidget*>(obj);
                QString tooltip = widget->toolTip();
                if(tooltip.size() > size_threshold && !tooltip.startsWith("<qt") && !QtmightBeRichText(tooltip))
                {
                    // Envelop with <qt></qt> to make sure Qt detects this as rich text
                    // Escape the current message as HTML and replace \n by <br>
                    tooltip = "<qt>" + HtmlEscape(tooltip, true) + "</qt>";
                    widget->setToolTip(tooltip);
                    return true;
                }
            }
            return QObject::eventFilter(obj, evt);
            */
        }
    }

    impl LabelOutOfFocusEventFilter {

        pub fn new(parent: *mut QObject) -> Self {
        
            todo!();
            /*
            : q_object(parent),

            
            */
        }
        
        pub fn event_filter(&mut self, 
            watched: *mut QObject,
            event:   *mut QEvent) -> bool {
            
            todo!();
            /*
                if (event->type() == QEvent::FocusOut) {
                auto focus_out = static_cast<QFocusEvent*>(event);
                if (focus_out->reason() != QtPopupFocusReason) {
                    auto label = qobject_cast<QLabel*>(watched);
                    if (label) {
                        auto flags = label->textInteractionFlags();
                        label->setTextInteractionFlags(QtNoTextInteraction);
                        label->setTextInteractionFlags(flags);
                    }
                }
            }

            return QObject::eventFilter(watched, event);
            */
        }
    }

    #[cfg(WIN32)]
    pub fn startup_shortcut_path() -> Path {
        
        todo!();
            /*
                std::string chain = gArgs.GetChainName();
            if (chain == CBaseChainParams::MAIN)
                return GetSpecialFolderPath(CSIDL_STARTUP) / "Bitcoin.lnk";
            if (chain == CBaseChainParams::TESTNET) // Remove this special case when CBaseChainParams::TESTNET = "testnet4"
                return GetSpecialFolderPath(CSIDL_STARTUP) / "Bitcoin (testnet).lnk";
            return GetSpecialFolderPath(CSIDL_STARTUP) / strprintf("Bitcoin (%s).lnk", chain);
            */
    }

    #[cfg(WIN32)]
    pub fn get_start_on_system_startup() -> bool {
        
        todo!();
            /*
                // check for Bitcoin*.lnk
            return fs::exists(StartupShortcutPath());
            */
    }

    #[cfg(WIN32)]
    pub fn set_start_on_system_startup(auto_start: bool) -> bool {
        
        todo!();
            /*
                // If the shortcut exists already, remove it for updating
            fs::remove(StartupShortcutPath());

            if (fAutoStart)
            {
                CoInitialize(nullptr);

                // Get a pointer to the IShellLink interface.
                IShellLinkW* psl = nullptr;
                HRESULT hres = CoCreateInstance(CLSID_ShellLink, nullptr,
                    CLSCTX_INPROC_SERVER, IID_IShellLinkW,
                    reinterpret_cast<c_void**>(&psl));

                if (SUCCEEDED(hres))
                {
                    // Get the current executable path
                    WCHAR pszExePath[MAX_PATH];
                    GetModuleFileNameW(nullptr, pszExePath, ARRAYSIZE(pszExePath));

                    // Start client minimized
                    QString strArgs = "-min";
                    // Set -testnet /-regtest options
                    strArgs += QString::fromStdString(strprintf(" -chain=%s", gArgs.GetChainName()));

                    // Set the path to the shortcut target
                    psl->SetPath(pszExePath);
                    PathRemoveFileSpecW(pszExePath);
                    psl->SetWorkingDirectory(pszExePath);
                    psl->SetShowCmd(SW_SHOWMINNOACTIVE);
                    psl->SetArguments(strArgs.toStdWString().c_str());

                    // Query IShellLink for the IPersistFile interface for
                    // saving the shortcut in persistent storage.
                    IPersistFile* ppf = nullptr;
                    hres = psl->QueryInterface(IID_IPersistFile, reinterpret_cast<c_void**>(&ppf));
                    if (SUCCEEDED(hres))
                    {
                        // Save the link by calling IPersistFile::Save.
                        hres = ppf->Save(StartupShortcutPath().wstring().c_str(), TRUE);
                        ppf->Release();
                        psl->Release();
                        CoUninitialize();
                        return true;
                    }
                    psl->Release();
                }
                CoUninitialize();
                return false;
            }
            return true;
            */
    }

    /**
      | Follow the Desktop Application Autostart
      | Spec: https://specifications.freedesktop.org/autostart-spec/autostart-spec-latest.html
      |
      */
    #[cfg(Q_OS_LINUX)]
    pub fn get_autostart_dir() -> Path {
        
        todo!();
            /*
                char* pszConfigHome = getenv("XDG_CONFIG_HOME");
            if (pszConfigHome) return fs::path(pszConfigHome) / "autostart";
            char* pszHome = getenv("HOME");
            if (pszHome) return fs::path(pszHome) / ".config" / "autostart";
            return fs::path();
            */
    }

    #[cfg(Q_OS_LINUX)]
    pub fn get_autostart_file_path() -> Path {
        
        todo!();
            /*
                std::string chain = gArgs.GetChainName();
            if (chain == CBaseChainParams::MAIN)
                return GetAutostartDir() / "bitcoin.desktop";
            return GetAutostartDir() / strprintf("bitcoin-%s.desktop", chain);
            */
    }

    #[cfg(Q_OS_LINUX)]
    pub fn get_start_on_system_startup() -> bool {
        
        todo!();
            /*
                fsbridge::ifstream optionFile(GetAutostartFilePath());
            if (!optionFile.good())
                return false;
            // Scan through file for "Hidden=true":
            std::string line;
            while (!optionFile.eof())
            {
                getline(optionFile, line);
                if (line.find("Hidden") != std::string::npos &&
                    line.find("true") != std::string::npos)
                    return false;
            }
            optionFile.close();

            return true;
            */
    }

    #[cfg(Q_OS_LINUX)]
    pub fn set_start_on_system_startup(auto_start: bool) -> bool {
        
        todo!();
            /*
                if (!fAutoStart)
                fs::remove(GetAutostartFilePath());
            else
            {
                char pszExePath[MAX_PATH+1];
                ssize_t r = readlink("/proc/self/exe", pszExePath, sizeof(pszExePath) - 1);
                if (r == -1)
                    return false;
                pszExePath[r] = '\0';

                fs::create_directories(GetAutostartDir());

                fsbridge::ofstream optionFile(GetAutostartFilePath(), std::ios_base::out | std::ios_base::trunc);
                if (!optionFile.good())
                    return false;
                std::string chain = gArgs.GetChainName();
                // Write a bitcoin.desktop file to the autostart directory:
                optionFile << "[Desktop Entry]\n";
                optionFile << "Type=Application\n";
                if (chain == CBaseChainParams::MAIN)
                    optionFile << "Name=Bitcoin\n";
                else
                    optionFile << strprintf("Name=Bitcoin (%s)\n", chain);
                optionFile << "Exec=" << pszExePath << strprintf(" -min -chain=%s\n", chain);
                optionFile << "Terminal=false\n";
                optionFile << "Hidden=false\n";
                optionFile.close();
            }
            return true;
            */
    }


    #[cfg(not(any(WIN32,Q_OS_LINUX)))]
    pub fn get_start_on_system_startup() -> bool {
        
        todo!();
            /*
                return false;
            */
    }

    #[cfg(not(any(WIN32,Q_OS_LINUX)))]
    pub fn set_start_on_system_startup(auto_start: bool) -> bool {
        
        todo!();
            /*
                return false;
            */
    }

    pub fn set_clipboard(str_: &String)  {
        
        todo!();
            /*
                QClipboard* clipboard = QApplication::clipboard();
            clipboard->setText(str, QClipboard::Clipboard);
            if (clipboard->supportsSelection()) {
                clipboard->setText(str, QClipboard::Selection);
            }
            */
    }

    /**
      | Convert QString to OS specific boost
      | path through UTF-8
      |
      */
    pub fn qstring_to_boost_path(path: &String) -> Box<Path> {
        
        todo!();
            /*
                return fs::u8path(path.toStdString());
            */
    }

    /**
      | Convert OS specific boost path to QString
      | through UTF-8
      |
      */
    pub fn boost_path_to_qstring(path: &Box<Path>) -> String {
        
        todo!();
            /*
                return QString::fromStdString(path.u8string());
            */
    }

    /**
      | Convert enum Network to QString
      |
      */
    pub fn network_to_qstring(net: Network) -> String {
        
        todo!();
            /*
                switch (net) {
            case NET_UNROUTABLE: return QObject::tr("Unroutable");
            case NET_IPV4: return "IPv4";
            case NET_IPV6: return "IPv6";
            case NET_ONION: return "Onion";
            case NET_I2P: return "I2P";
            case NET_CJDNS: return "CJDNS";
            case NET_INTERNAL: return QObject::tr("Internal");
            case NET_MAX: assert(false);
            } // no default case, so the compiler can warn about missing cases
            assert(false);
            */
    }

    /**
      | Convert enum ConnectionType to QString
      |
      */
    pub fn connection_type_to_qstring(
            conn_type:         ConnectionType,
            prepend_direction: bool) -> String {
        
        todo!();
            /*
                QString prefix;
            if (prepend_direction) {
                prefix = (conn_type == ConnectionType::INBOUND) ?
                             /*: An inbound connection from a peer. An inbound connection
                                 is a connection initiated by a peer. */
                             QObject::tr("Inbound") :
                             /*: An outbound connection to a peer. An outbound connection
                                 is a connection initiated by us. */
                             QObject::tr("Outbound") + " ";
            }
            switch (conn_type) {
            case ConnectionType::INBOUND: return prefix;
            //: Peer connection type that relays all network information.
            case ConnectionType::OUTBOUND_FULL_RELAY: return prefix + QObject::tr("Full Relay");
            /*: Peer connection type that relays network information about
                blocks and not transactions or addresses. */
            case ConnectionType::BLOCK_RELAY: return prefix + QObject::tr("Block Relay");
            //: Peer connection type established manually through one of several methods.
            case ConnectionType::MANUAL: return prefix + QObject::tr("Manual");
            //: Short-lived peer connection type that tests the aliveness of known addresses.
            case ConnectionType::FEELER: return prefix + QObject::tr("Feeler");
            //: Short-lived peer connection type that solicits known addresses from a peer.
            case ConnectionType::ADDR_FETCH: return prefix + QObject::tr("Address Fetch");
            } // no default case, so the compiler can warn about missing cases
            assert(false);
            */
    }

    /**
      | Convert seconds into a QString with
      | days, hours, mins, secs
      |
      */
    pub fn format_duration_str(secs: i32) -> String {
        
        todo!();
            /*
                QStringList strList;
            int days = secs / 86400;
            int hours = (secs % 86400) / 3600;
            int mins = (secs % 3600) / 60;
            int seconds = secs % 60;

            if (days)
                strList.append(QObject::tr("%1 d").arg(days));
            if (hours)
                strList.append(QObject::tr("%1 h").arg(hours));
            if (mins)
                strList.append(QObject::tr("%1 m").arg(mins));
            if (seconds || (!days && !hours && !mins))
                strList.append(QObject::tr("%1 s").arg(seconds));

            return strList.join(" ");
            */
    }

    /**
      | Format NodeStats.nServices bitmask
      | into a user-readable string
      |
      */
    pub fn format_services_str(mask: u64) -> String {
        
        todo!();
            /*
                QStringList strList;

            for (const auto& flag : serviceFlagsToStr(mask)) {
                strList.append(QString::fromStdString(flag));
            }

            if (strList.size())
                return strList.join(", ");
            else
                return QObject::tr("None");
            */
    }

    /**
      | Format a NodeStats.m_last_ping_time
      | into a user-readable string or display
      | N/A, if 0
      |
      */
    pub fn format_ping_time(ping_time: Duration /*micros*/) -> String {
        
        todo!();
            /*
                return (ping_time == std::chrono::microseconds::max() || ping_time == 0us) ?
                QObject::tr("N/A") :
                QObject::tr("%1 ms").arg(QString::number((int)(count_microseconds(ping_time) / 1000), 10));
            */
    }

    /**
      | Format a NodeCombinedStats.nTimeOffset
      | into a user-readable string
      |
      */
    pub fn format_time_offset(n_time_offset: i64) -> String {
        
        todo!();
            /*
                return QObject::tr("%1 s").arg(QString::number((int)nTimeOffset, 10));
            */
    }

    pub fn format_nice_time_offset(secs: i64) -> String {
        
        todo!();
            /*
                // Represent time from last generated block in human readable text
            QString timeBehindText;
            const int HOUR_IN_SECONDS = 60*60;
            const int DAY_IN_SECONDS = 24*60*60;
            const int WEEK_IN_SECONDS = 7*24*60*60;
            const int YEAR_IN_SECONDS = 31556952; // Average length of year in Gregorian calendar
            if(secs < 60)
            {
                timeBehindText = QObject::tr("%n second(s)","",secs);
            }
            else if(secs < 2*HOUR_IN_SECONDS)
            {
                timeBehindText = QObject::tr("%n minute(s)","",secs/60);
            }
            else if(secs < 2*DAY_IN_SECONDS)
            {
                timeBehindText = QObject::tr("%n hour(s)","",secs/HOUR_IN_SECONDS);
            }
            else if(secs < 2*WEEK_IN_SECONDS)
            {
                timeBehindText = QObject::tr("%n day(s)","",secs/DAY_IN_SECONDS);
            }
            else if(secs < YEAR_IN_SECONDS)
            {
                timeBehindText = QObject::tr("%n week(s)","",secs/WEEK_IN_SECONDS);
            }
            else
            {
                i64 years = secs / YEAR_IN_SECONDS;
                i64 remainder = secs % YEAR_IN_SECONDS;
                timeBehindText = QObject::tr("%1 and %2").arg(QObject::tr("%n year(s)", "", years)).arg(QObject::tr("%n week(s)","", remainder/WEEK_IN_SECONDS));
            }
            return timeBehindText;
            */
    }

    pub fn format_bytes(bytes: u64) -> String {
        
        todo!();
            /*
                if (bytes < 1'000)
                return QObject::tr("%1 B").arg(bytes);
            if (bytes < 1'000'000)
                return QObject::tr("%1 kB").arg(bytes / 1'000);
            if (bytes < 1'000'000'000)
                return QObject::tr("%1 MB").arg(bytes / 1'000'000);

            return QObject::tr("%1 GB").arg(bytes / 1'000'000'000);
            */
    }

    pub fn calculate_ideal_font_size(
            width:          i32,
            text:           &String,
            font:           QFont,
            min_point_size: Option<f64>,
            font_size:      f64) -> f64 {

        let min_point_size:   f64 = min_point_size.unwrap_or(4.0);
        //let start_point_size: f64 = start_point_size.unwrap_or(14.0);
        
        todo!();
            /*
                while(font_size >= minPointSize) {
                font.setPointSizeF(font_size);
                QFontMetrics fm(font);
                if (TextWidth(fm, text) < width) {
                    break;
                }
                font_size -= 0.5;
            }
            return font_size;
            */
    }

    impl ThemedLabel {
        
        pub fn new(
            platform_style: *const PlatformStyle,
            parent:         *mut QWidget) -> Self {
        
            todo!();
            /*


                : QLabel{parent}, m_platform_style{platform_style}
            assert(m_platform_style);
            */
        }
        
        pub fn set_themed_pixmap(&mut self, 
            image_filename: &String,
            width:          i32,
            height:         i32)  {
            
            todo!();
            /*
                m_image_filename = image_filename;
            m_pixmap_width = width;
            m_pixmap_height = height;
            updateThemedPixmap();
            */
        }
        
        pub fn change_event(&mut self, e: *mut QEvent)  {
            
            todo!();
            /*
                if (e->type() == QEvent::PaletteChange) {
                updateThemedPixmap();
            }

            QLabel::changeEvent(e);
            */
        }
        
        pub fn update_themed_pixmap(&mut self)  {
            
            todo!();
            /*
                setPixmap(m_platform_style->SingleColorIcon(m_image_filename).pixmap(m_pixmap_width, m_pixmap_height));
            */
        }
    }

    impl ClickableLabel {
        
        pub fn new(
            platform_style: *const PlatformStyle,
            parent:         *mut QWidget) -> Self {
        
            todo!();
            /*
                : ThemedLabel{platform_style, parent}
            */
        }
        
        pub fn mouse_release_event(&mut self, event: *mut QMouseEvent)  {
            
            todo!();
            /*
                Q_EMIT clicked(event->pos());
            */
        }
    }

    impl ClickableProgressBar {
        
        pub fn mouse_release_event(&mut self, event: *mut QMouseEvent)  {
            
            todo!();
            /*
                Q_EMIT clicked(event->pos());
            */
        }
    }

    impl ItemDelegate {
        
        pub fn event_filter(&mut self, 
            object: *mut QObject,
            event:  *mut QEvent) -> bool {
            
            todo!();
            /*
                if (event->type() == QEvent::KeyPress) {
                if (static_cast<QKeyEvent*>(event)->key() == QtKey_Escape) {
                    Q_EMIT keyEscapePressed();
                }
            }
            return QItemDelegate::eventFilter(object, event);
            */
        }
    }

    /**
      | Fix known bugs in QProgressDialog class.
      |
      */
    pub fn polish_progress_dialog(dialog: *mut QProgressDialog)  {
        
        todo!();
            /*
                #ifdef Q_OS_MAC
            // Workaround for macOS-only Qt bug; see: QTBUG-65750, QTBUG-70357.
            const int margin = TextWidth(dialog->fontMetrics(), ("X"));
            dialog->resize(dialog->width() + 2 * margin, dialog->height());
        #endif
            // QProgressDialog estimates the time the operation will take (based on time
            // for steps), and only shows itself if that estimate is beyond minimumDuration.
            // The default minimumDuration value is 4 seconds, and it could make users
            // think that the GUI is frozen.
            dialog->setMinimumDuration(0);
            */
    }

    /**
      | Returns the distance in pixels appropriate
      | for drawing a subsequent character
      | after text.
      | 
      | In Qt 5.12 and before the QFontMetrics::width()
      | is used and it is deprecated since Qt
      | 5.13.
      | 
      | In Qt 5.11 the QFontMetrics::horizontalAdvance()
      | was introduced.
      |
      */
    pub fn text_width(
            fm:   &QFontMetrics,
            text: &String) -> i32 {
        
        todo!();
            /*
                #if (QT_VERSION >= QT_VERSION_CHECK(5, 11, 0))
            return fm.horizontalAdvance(text);
        #else
            return fm.width(text);
        #endif
            */
    }

    /**
      | Writes to debug.log short info about
      | the used Qt and the host system.
      |
      */
    pub fn log_qt_info()  {
        
        todo!();
            /*
                #ifdef QT_STATIC
            const std::string qt_link{"static"};
        #else
            const std::string qt_link{"dynamic"};
        #endif
        #ifdef QT_STATICPLUGIN
            const std::string plugin_link{"static"};
        #else
            const std::string plugin_link{"dynamic"};
        #endif
            LogPrintf("Qt %s (%s), plugin=%s (%s)\n", qVersion(), qt_link, QGuiApplication::platformName().toStdString(), plugin_link);
            const auto static_plugins = QPluginLoader::staticPlugins();
            if (static_plugins.empty()) {
                LogPrintf("No static plugins.\n");
            } else {
                LogPrintf("Static plugins:\n");
                for (const QStaticPlugin& p : static_plugins) {
                    QJsonObject meta_data = p.metaData();
                    const std::string plugin_class = meta_data.take(QString("className")).toString().toStdString();
                    const int plugin_version = meta_data.take(QString("version")).toInt();
                    LogPrintf(" %s, version %d\n", plugin_class, plugin_version);
                }
            }

            LogPrintf("Style: %s / %s\n", QApplication::style()->objectName().toStdString(), QApplication::style()->metaObject()->className());
            LogPrintf("System: %s, %s\n", QSysInfo::prettyProductName().toStdString(), QSysInfo::buildAbi().toStdString());
            for (const QScreen* s : QGuiApplication::screens()) {
                LogPrintf("Screen: %s %dx%d, pixel ratio=%.1f\n", s->name().toStdString(), s->size().width(), s->size().height(), s->devicePixelRatio());
            }
            */
    }

    /**
      | Call QMenu::popup() only on supported
      | QT_QPA_PLATFORM.
      |
      */
    pub fn popup_menu(
            menu:      *mut QMenu,
            point:     &QPoint,
            at_action: Option<*mut QAction>)  {

        todo!();
            /*
                // The qminimal plugin does not provide window system integration.
            if (QApplication::platformName() == "minimal") return;
            menu->popup(point, at_action);
            */
    }

    /**
      | Returns the start-moment of the day
      | in local time.
      | 
      | QDateTime::QDateTime(const QDate&
      | date) is deprecated since Qt 5.15.
      | 
      | QDate::startOfDay() was introduced
      | in Qt 5.14.
      |
      */
    pub fn start_of_day(date: &QDate) -> QDateTime {
        
        todo!();
            /*
                #if (QT_VERSION >= QT_VERSION_CHECK(5, 14, 0))
            return date.startOfDay();
        #else
            return QDateTime(date);
        #endif
            */
    }

    /**
      | Returns true if pixmap has been set.
      | 
      | QPixmap* QLabel::pixmap() is deprecated
      | since Qt 5.15.
      |
      */
    pub fn has_pixmap(label: *const QLabel) -> bool {
        
        todo!();
            /*
                #if (QT_VERSION >= QT_VERSION_CHECK(5, 15, 0))
            return !label->pixmap(QtReturnByValue).isNull();
        #else
            return label->pixmap() != nullptr;
        #endif
            */
    }

    pub fn get_image(label: *const QLabel) -> QImage {
        
        todo!();
            /*
                if (!HasPixmap(label)) {
                return QImage();
            }

        #if (QT_VERSION >= QT_VERSION_CHECK(5, 15, 0))
            return label->pixmap(QtReturnByValue).toImage();
        #else
            return label->pixmap()->toImage();
        #endif
            */
    }

    /**
      | Replaces a plain text link with an HTML
      | tagged one.
      |
      */
    pub fn make_html_link(
            source: &String,
            link:   &String) -> String {
        
        todo!();
            /*
                return QString(source).replace(
                link,
                QLatin1String("<a href=\"") + link + QLatin1String("\">") + link + QLatin1String("</a>"));
            */
    }

    pub fn print_slot_exception(
            exception: *const Exception,
            sender:    *const QObject,
            receiver:  *const QObject)  {
        
        todo!();
            /*
                std::string description = sender->metaObject()->className();
            description += "->";
            description += receiver->metaObject()->className();
            PrintExceptionContinue(exception, description.c_str());
            */
    }

    /**
      | Shows a QDialog instance asynchronously,
      | and deletes it on close.
      |
      */
    pub fn show_modal_dialog_and_delete_on_close(dialog: *mut QDialog)  {
        
        todo!();
            /*
                dialog->setAttribute(QtWA_DeleteOnClose);
            dialog->setWindowModality(QtApplicationModal);
            dialog->show();
            */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/guiutil.cpp]

#[cfg(Q_OS_MAC)]
pub fn force_activation()  {
    
    todo!();
        /*
        
        */
}
