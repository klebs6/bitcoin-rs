// ---------------- [ File: bitcoin-qt/src/test_wallettests.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/test/wallettests.h]

#[Q_OBJECT]
pub struct WalletTests {
    base: QObject,
    node: Rc<RefCell<dyn NodeInterface>>,
}

impl WalletTests {
    
    pub fn new(node: Rc<RefCell<dyn NodeInterface>>) -> Self {
    
        todo!();
        /*
        : node(node),

        
        */
    }

    #[Q_SLOT]
    pub fn wallet_tests(&mut self)  {
        
        todo!();
        /*
            #ifdef Q_OS_MAC
        if (QApplication::platformName() == "minimal") {
            // Disable for mac on "minimal" platform to avoid crashes inside the Qt
            // framework when it tries to look up unimplemented cocoa functions,
            // and fails to handle returned nulls
            // (https://bugreports.qt.io/browse/QTBUG-49686).
            QWARN("Skipping WalletTests on mac build with 'minimal' platform set due to Qt bugs. To run AppTests, invoke "
                  "with 'QT_QPA_PLATFORM=cocoa test_bitcoin-qt' on mac, or else use a linux or windows build.");
            return;
        }
    #endif
        TestGUI(m_node);
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/test/wallettests.cpp]

/**
  | Press "Yes" or "Cancel" buttons in modal
  | send confirmation dialog.
  |
  */
pub fn confirm_send(
        text:   Option<&mut str>,
        cancel: Option<bool>) {

    let cancel: bool = cancel.unwrap_or(false);

    todo!();
        /*
            QTimer::singleShot(0, [text, cancel]() {
            for (QWidget* widget : QApplication::topLevelWidgets()) {
                if (widget->inherits("SendConfirmationDialog")) {
                    SendConfirmationDialog* dialog = qobject_cast<SendConfirmationDialog*>(widget);
                    if (text) *text = dialog->text();
                    QAbstractButton* button = dialog->button(cancel ? QMessageBox::Cancel : QMessageBox::Yes);
                    button->setEnabled(true);
                    button->click();
                }
            }
        });
        */
}

/**
  | Send coins to address and return txid.
  |
  */
pub fn send_coins(
        wallet:            &mut Wallet,
        send_coins_dialog: &mut SendCoinsDialog,
        address:           &TxDestination,
        amount:            Amount,
        rbf:               bool) -> u256 {
    
    todo!();
        /*
            QVBoxLayout* entries = sendCoinsDialog.findChild<QVBoxLayout*>("entries");
        SendCoinsEntry* entry = qobject_cast<SendCoinsEntry*>(entries->itemAt(0)->widget());
        entry->findChild<QValidatedLineEdit*>("payTo")->setText(QString::fromStdString(EncodeDestination(address)));
        entry->findChild<BitcoinAmountField*>("payAmount")->setValue(amount);
        sendCoinsDialog.findChild<QFrame*>("frameFee")
            ->findChild<QFrame*>("frameFeeSelection")
            ->findChild<QCheckBox*>("optInRBF")
            ->setCheckState(rbf ? QtChecked : QtUnchecked);
        uint256 txid;
        boost::signals2::scoped_connection c(wallet.NotifyTransactionChanged.connect([&txid](const uint256& hash, ChangeType status) {
            if (status == CT_NEW) txid = hash;
        }));
        ConfirmSend();
        bool invoked = QMetaObject::invokeMethod(&sendCoinsDialog, "sendButtonClicked", Q_ARG(bool, false));
        assert(invoked);
        return txid;
        */
}

/**
  | Find index of txid in transaction list.
  |
  */
pub fn find_tx(
        model: &QAbstractItemModel,
        txid:  &u256) -> QModelIndex {
    
    todo!();
        /*
            QString hash = QString::fromStdString(txid.ToString());
        int rows = model.rowCount({});
        for (int row = 0; row < rows; ++row) {
            QModelIndex index = model.index(row, 0, {});
            if (model.data(index, TransactionTableModel::TxHashRole) == hash) {
                return index;
            }
        }
        return {};
        */
}

/**
  | Invoke bumpfee on txid and check results.
  |
  */
pub fn bump_fee(
        view:            &mut TransactionView,
        txid:            &u256,
        expect_disabled: bool,
        expect_error:    String,
        cancel:          bool)  {
    
    todo!();
        /*
            QTableView* table = view.findChild<QTableView*>("transactionView");
        QModelIndex index = FindTx(*table->selectionModel()->model(), txid);
        QVERIFY2(index.isValid(), "Could not find BumpFee txid");

        // Select row in table, invoke context menu, and make sure bumpfee action is
        // enabled or disabled as expected.
        QAction* action = view.findChild<QAction*>("bumpFeeAction");
        table->selectionModel()->select(index, QItemSelectionModel::ClearAndSelect | QItemSelectionModel::Rows);
        action->setEnabled(expectDisabled);
        table->customContextMenuRequested({});
        QCOMPARE(action->isEnabled(), !expectDisabled);

        action->setEnabled(true);
        QString text;
        if (expectError.empty()) {
            ConfirmSend(&text, cancel);
        } else {
            ConfirmMessage(&text);
        }
        action->trigger();
        QVERIFY(text.indexOf(QString::fromStdString(expectError)) != -1);
        */
}

/**
  | Simple qt wallet tests.
  |
  | Test widgets can be debugged interactively
  | calling show() on them and manually running the
  | event loop, e.g.:
  |
  |     sendCoinsDialog.show();
  |     QEventLoop().exec();
  |
  | This also requires overriding the default
  | minimal Qt platform:
  |
  |@code
  |     QT_QPA_PLATFORM=xcb     src/qt/test/test_bitcoin-qt  # Linux
  |
  |     QT_QPA_PLATFORM=windows src/qt/test/test_bitcoin-qt  # Windows
  |
  |     QT_QPA_PLATFORM=cocoa   src/qt/test/test_bitcoin-qt  # macOS
  |@endcode
  */
pub fn testgui(node: Rc<RefCell<dyn NodeInterface>>)  {
    
    todo!();
        /*
            // Set up wallet and chain with 105 blocks (5 mature blocks for spending).
        TestChain100Setup test;
        for (int i = 0; i < 5; ++i) {
            test.CreateAndProcessBlock({}, GetScriptForRawPubKey(test.coinbaseKey.GetPubKey()));
        }
        auto wallet_client = typename interfaces::MakeWalletClient(*test.m_node.chain, *Assert(test.m_node.args));
        test.m_node.wallet_client = wallet_client.get();
        node.setContext(&test.m_node);
        std::shared_ptr<CWallet> wallet = std::make_shared<CWallet>(node.context()->chain.get(), "", CreateMockWalletDatabase());
        wallet->LoadWallet();
        wallet->SetWalletFlag(WALLET_FLAG_DESCRIPTORS);
        {
            LOCK(wallet->cs_wallet);
            wallet->SetupDescriptorScriptPubKeyMans();

            // Add the coinbase key
            FlatSigningProvider provider;
            std::string error;
            std::unique_ptr<Descriptor> desc = Parse("combo(" + EncodeSecret(test.coinbaseKey) + ")", provider, error, /* require_checksum=*/ false);
            assert(desc);
            WalletDescriptor w_desc(std::move(desc), 0, 0, 1, 1);
            if (!wallet->AddWalletDescriptor(w_desc, provider, "", false)) assert(false);
            TxDestination dest = GetDestinationForKey(test.coinbaseKey.GetPubKey(), wallet->m_default_address_type);
            wallet->SetAddressBook(dest, "", "receive");
            wallet->SetLastBlockProcessed(105, node.context()->chainman->ActiveChain().Tip()->GetBlockHash());
        }
        {
            WalletRescanReserver reserver(*wallet);
            reserver.reserve();
            CWallet::ScanResult result = wallet->ScanForWalletTransactions(Params().GetConsensus().hashGenesisBlock, 0 /* block height */, {} /* max height */, reserver, true /* fUpdate */);
            QCOMPARE(result.status, CWallet::ScanResult::SUCCESS);
            QCOMPARE(result.last_scanned_block, node.context()->chainman->ActiveChain().Tip()->GetBlockHash());
            QVERIFY(result.last_failed_block.IsNull());
        }
        wallet->SetBroadcastTransactions(true);

        // Create widgets for sending coins and listing transactions.
        std::unique_ptr<const PlatformStyle> platformStyle(PlatformStyle::instantiate("other"));
        SendCoinsDialog sendCoinsDialog(platformStyle.get());
        TransactionView transactionView(platformStyle.get());
        OptionsModel optionsModel;
        ClientModel clientModel(node, &optionsModel);
        WalletContext& context = *node.walletClient().context();
        AddWallet(context, wallet);
        WalletModel walletModel(typename interfaces::MakeWallet(context, wallet), clientModel, platformStyle.get());
        RemoveWallet(context, wallet, /* load_on_start= */ std::nullopt);
        sendCoinsDialog.setModel(&walletModel);
        transactionView.setModel(&walletModel);

        {
            // Check balance in send dialog
            QLabel* balanceLabel = sendCoinsDialog.findChild<QLabel*>("labelBalance");
            QString balanceText = balanceLabel->text();
            int unit = walletModel.getOptionsModel()->getDisplayUnit();
            CAmount balance = walletModel.wallet().getBalance();
            QString balanceComparison = BitcoinUnits::formatWithUnit(unit, balance, false, BitcoinUnits::SeparatorStyle::ALWAYS);
            QCOMPARE(balanceText, balanceComparison);
        }

        // Send two transactions, and verify they are added to transaction list.
        TransactionTableModel* transactionTableModel = walletModel.getTransactionTableModel();
        QCOMPARE(transactionTableModel->rowCount({}), 105);
        uint256 txid1 = SendCoins(*wallet.get(), sendCoinsDialog, PKHash(), 5 * COIN, false /* rbf */);
        uint256 txid2 = SendCoins(*wallet.get(), sendCoinsDialog, PKHash(), 10 * COIN, true /* rbf */);
        QCOMPARE(transactionTableModel->rowCount({}), 107);
        QVERIFY(FindTx(*transactionTableModel, txid1).isValid());
        QVERIFY(FindTx(*transactionTableModel, txid2).isValid());

        // Call bumpfee. Test disabled, canceled, enabled, then failing cases.
        BumpFee(transactionView, txid1, true /* expect disabled */, "not BIP 125 replaceable" /* expected error */, false /* cancel */);
        BumpFee(transactionView, txid2, false /* expect disabled */, {} /* expected error */, true /* cancel */);
        BumpFee(transactionView, txid2, false /* expect disabled */, {} /* expected error */, false /* cancel */);
        BumpFee(transactionView, txid2, true /* expect disabled */, "already bumped" /* expected error */, false /* cancel */);

        // Check current balance on OverviewPage
        OverviewPage overviewPage(platformStyle.get());
        overviewPage.setWalletModel(&walletModel);
        QLabel* balanceLabel = overviewPage.findChild<QLabel*>("labelBalance");
        QString balanceText = balanceLabel->text().trimmed();
        int unit = walletModel.getOptionsModel()->getDisplayUnit();
        CAmount balance = walletModel.wallet().getBalance();
        QString balanceComparison = BitcoinUnits::formatWithUnit(unit, balance, false, BitcoinUnits::SeparatorStyle::ALWAYS);
        QCOMPARE(balanceText, balanceComparison);

        // Check Request Payment button
        ReceiveCoinsDialog receiveCoinsDialog(platformStyle.get());
        receiveCoinsDialog.setModel(&walletModel);
        RecentRequestsTableModel* requestTableModel = walletModel.getRecentRequestsTableModel();

        // Label input
        QLineEdit* labelInput = receiveCoinsDialog.findChild<QLineEdit*>("reqLabel");
        labelInput->setText("TEST_LABEL_1");

        // Amount input
        BitcoinAmountField* amountInput = receiveCoinsDialog.findChild<BitcoinAmountField*>("reqAmount");
        amountInput->setValue(1);

        // Message input
        QLineEdit* messageInput = receiveCoinsDialog.findChild<QLineEdit*>("reqMessage");
        messageInput->setText("TEST_MESSAGE_1");
        int initialRowCount = requestTableModel->rowCount({});
        QPushButton* requestPaymentButton = receiveCoinsDialog.findChild<QPushButton*>("receiveButton");
        requestPaymentButton->click();
        QString address;
        for (QWidget* widget : QApplication::topLevelWidgets()) {
            if (widget->inherits("ReceiveRequestDialog")) {
                ReceiveRequestDialog* receiveRequestDialog = qobject_cast<ReceiveRequestDialog*>(widget);
                QCOMPARE(receiveRequestDialog->QObject::findChild<QLabel*>("payment_header")->text(), QString("Payment information"));
                QCOMPARE(receiveRequestDialog->QObject::findChild<QLabel*>("uri_tag")->text(), QString("URI:"));
                QString uri = receiveRequestDialog->QObject::findChild<QLabel*>("uri_content")->text();
                QCOMPARE(uri.count("bitcoin:"), 2);
                QCOMPARE(receiveRequestDialog->QObject::findChild<QLabel*>("address_tag")->text(), QString("Address:"));
                QVERIFY(address.isEmpty());
                address = receiveRequestDialog->QObject::findChild<QLabel*>("address_content")->text();
                QVERIFY(!address.isEmpty());

                QCOMPARE(uri.count("amount=0.00000001"), 2);
                QCOMPARE(receiveRequestDialog->QObject::findChild<QLabel*>("amount_tag")->text(), QString("Amount:"));
                QCOMPARE(receiveRequestDialog->QObject::findChild<QLabel*>("amount_content")->text(), QString::fromStdString("0.00000001 " + CURRENCY_UNIT));

                QCOMPARE(uri.count("label=TEST_LABEL_1"), 2);
                QCOMPARE(receiveRequestDialog->QObject::findChild<QLabel*>("label_tag")->text(), QString("Label:"));
                QCOMPARE(receiveRequestDialog->QObject::findChild<QLabel*>("label_content")->text(), QString("TEST_LABEL_1"));

                QCOMPARE(uri.count("message=TEST_MESSAGE_1"), 2);
                QCOMPARE(receiveRequestDialog->QObject::findChild<QLabel*>("message_tag")->text(), QString("Message:"));
                QCOMPARE(receiveRequestDialog->QObject::findChild<QLabel*>("message_content")->text(), QString("TEST_MESSAGE_1"));
            }
        }

        // Clear button
        QPushButton* clearButton = receiveCoinsDialog.findChild<QPushButton*>("clearButton");
        clearButton->click();
        QCOMPARE(labelInput->text(), QString(""));
        QCOMPARE(amountInput->value(), CAmount(0));
        QCOMPARE(messageInput->text(), QString(""));

        // Check addition to history
        int currentRowCount = requestTableModel->rowCount({});
        QCOMPARE(currentRowCount, initialRowCount+1);

        // Check addition to wallet
        std::vector<std::string> requests = walletModel.wallet().getAddressReceiveRequests();
        QCOMPARE(requests.size(), size_t{1});
        RecentRequestEntry entry;
        DataStream{MakeUCharSpan(requests[0]), SER_DISK, CLIENT_VERSION} >> entry;
        QCOMPARE(entry.nVersion, int{1});
        QCOMPARE(entry.id, int64_t{1});
        QVERIFY(entry.date.isValid());
        QCOMPARE(entry.recipient.address, address);
        QCOMPARE(entry.recipient.label, QString{"TEST_LABEL_1"});
        QCOMPARE(entry.recipient.amount, CAmount{1});
        QCOMPARE(entry.recipient.message, QString{"TEST_MESSAGE_1"});
        QCOMPARE(entry.recipient.sPaymentRequest, std::string{});
        QCOMPARE(entry.recipient.authenticatedMerchant, QString{});

        // Check Remove button
        QTableView* table = receiveCoinsDialog.findChild<QTableView*>("recentRequestsView");
        table->selectRow(currentRowCount-1);
        QPushButton* removeRequestButton = receiveCoinsDialog.findChild<QPushButton*>("removeRequestButton");
        removeRequestButton->click();
        QCOMPARE(requestTableModel->rowCount({}), currentRowCount-1);

        // Check removal from wallet
        QCOMPARE(walletModel.wallet().getAddressReceiveRequests().size(), size_t{0});
        */
}
