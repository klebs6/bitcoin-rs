// ---------------- [ File: bitcoin-qt/src/transactiondesc.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/transactiondesc.h]

/**
  | Provide a human-readable extended
  | HTML description of a transaction.
  |
  */
#[Q_OBJECT]
pub struct TransactionDesc {
    base: QObject,
}

//-------------------------------------------[.cpp/bitcoin/src/qt/transactiondesc.cpp]
impl TransactionDesc {
    
    pub fn format_tx_status(&mut self, 
        wtx:        &WalletTx,
        status:     &WalletTxStatus,
        in_mempool: bool,
        num_blocks: i32) -> String {
        
        todo!();
        /*
            if (!status.is_final)
        {
            if (wtx.tx->nLockTime < LOCKTIME_THRESHOLD)
                return tr("Open for %n more block(s)", "", wtx.tx->nLockTime - numBlocks);
            else
                return tr("Open until %1").arg(typename gui_util::dateTimeStr(wtx.tx->nLockTime));
        }
        else
        {
            int nDepth = status.depth_in_main_chain;
            if (nDepth < 0) {
                return tr("conflicted with a transaction with %1 confirmations").arg(-nDepth);
            } else if (nDepth == 0) {
                const QString abandoned{status.is_abandoned ? QLatin1String(", ") + tr("abandoned") : QString()};
                return tr("0/unconfirmed, %1").arg(inMempool ? tr("in memory pool") : tr("not in memory pool")) + abandoned;
            } else if (nDepth < 6) {
                return tr("%1/unconfirmed").arg(nDepth);
            } else {
                return tr("%1 confirmations").arg(nDepth);
            }
        }
        */
    }
    
    pub fn tohtml(&mut self, 
        node:   Rc<RefCell<dyn NodeInterface>>,
        wallet: Rc<RefCell<dyn WalletInterface>>,
        rec:    *mut TransactionRecord,
        unit:   i32) -> String {
        
        todo!();
        /*
            int numBlocks;
        typename interfaces::WalletTxStatus status;
        typename interfaces::WalletOrderForm orderForm;
        bool inMempool;
        typename interfaces::WalletTx wtx = wallet.getWalletTxDetails(rec->hash, status, orderForm, inMempool, numBlocks);

        QString strHTML;

        strHTML.reserve(4000);
        strHTML += "<html><font face='verdana, arial, helvetica, sans-serif'>";

        int64_t nTime = wtx.time;
        CAmount nCredit = wtx.credit;
        CAmount nDebit = wtx.debit;
        CAmount nNet = nCredit - nDebit;

        strHTML += "<b>" + tr("Status") + ":</b> " + FormatTxStatus(wtx, status, inMempool, numBlocks);
        strHTML += "<br>";

        strHTML += "<b>" + tr("Date") + ":</b> " + (nTime ? typename gui_util::dateTimeStr(nTime) : "") + "<br>";

        //
        // From
        //
        if (wtx.is_coinbase)
        {
            strHTML += "<b>" + tr("Source") + ":</b> " + tr("Generated") + "<br>";
        }
        else if (wtx.value_map.count("from") && !wtx.value_map["from"].empty())
        {
            // Online transaction
            strHTML += "<b>" + tr("From") + ":</b> " + typename gui_util::HtmlEscape(wtx.value_map["from"]) + "<br>";
        }
        else
        {
            // Offline transaction
            if (nNet > 0)
            {
                // Credit
                TxDestination address = DecodeDestination(rec->address);
                if (IsValidDestination(address)) {
                    std::string name;
                    isminetype ismine;
                    if (wallet.getAddress(address, &name, &ismine, /* purpose= */ nullptr))
                    {
                        strHTML += "<b>" + tr("From") + ":</b> " + tr("unknown") + "<br>";
                        strHTML += "<b>" + tr("To") + ":</b> ";
                        strHTML += typename gui_util::HtmlEscape(rec->address);
                        QString addressOwned = ismine == ISMINE_SPENDABLE ? tr("own address") : tr("watch-only");
                        if (!name.empty())
                            strHTML += " (" + addressOwned + ", " + tr("label") + ": " + typename gui_util::HtmlEscape(name) + ")";
                        else
                            strHTML += " (" + addressOwned + ")";
                        strHTML += "<br>";
                    }
                }
            }
        }

        //
        // To
        //
        if (wtx.value_map.count("to") && !wtx.value_map["to"].empty())
        {
            // Online transaction
            std::string strAddress = wtx.value_map["to"];
            strHTML += "<b>" + tr("To") + ":</b> ";
            TxDestination dest = DecodeDestination(strAddress);
            std::string name;
            if (wallet.getAddress(
                    dest, &name, /* is_mine= */ nullptr, /* purpose= */ nullptr) && !name.empty())
                strHTML += typename gui_util::HtmlEscape(name) + " ";
            strHTML += typename gui_util::HtmlEscape(strAddress) + "<br>";
        }

        //
        // Amount
        //
        if (wtx.is_coinbase && nCredit == 0)
        {
            //
            // Coinbase
            //
            CAmount nUnmatured = 0;
            for (const CTxOut& txout : wtx.tx->vout)
                nUnmatured += wallet.getCredit(txout, ISMINE_ALL);
            strHTML += "<b>" + tr("Credit") + ":</b> ";
            if (status.is_in_main_chain)
                strHTML += BitcoinUnits::formatHtmlWithUnit(unit, nUnmatured)+ " (" + tr("matures in %n more block(s)", "", status.blocks_to_maturity) + ")";
            else
                strHTML += "(" + tr("not accepted") + ")";
            strHTML += "<br>";
        }
        else if (nNet > 0)
        {
            //
            // Credit
            //
            strHTML += "<b>" + tr("Credit") + ":</b> " + BitcoinUnits::formatHtmlWithUnit(unit, nNet) + "<br>";
        }
        else
        {
            isminetype fAllFromMe = ISMINE_SPENDABLE;
            for (const isminetype mine : wtx.txin_is_mine)
            {
                if(fAllFromMe > mine) fAllFromMe = mine;
            }

            isminetype fAllToMe = ISMINE_SPENDABLE;
            for (const isminetype mine : wtx.txout_is_mine)
            {
                if(fAllToMe > mine) fAllToMe = mine;
            }

            if (fAllFromMe)
            {
                if(fAllFromMe & ISMINE_WATCH_ONLY)
                    strHTML += "<b>" + tr("From") + ":</b> " + tr("watch-only") + "<br>";

                //
                // Debit
                //
                auto mine = wtx.txout_is_mine.begin();
                for (const CTxOut& txout : wtx.tx->vout)
                {
                    // Ignore change
                    isminetype toSelf = *(mine++);
                    if ((toSelf == ISMINE_SPENDABLE) && (fAllFromMe == ISMINE_SPENDABLE))
                        continue;

                    if (!wtx.value_map.count("to") || wtx.value_map["to"].empty())
                    {
                        // Offline transaction
                        TxDestination address;
                        if (ExtractDestination(txout.scriptPubKey, address))
                        {
                            strHTML += "<b>" + tr("To") + ":</b> ";
                            std::string name;
                            if (wallet.getAddress(
                                    address, &name, /* is_mine= */ nullptr, /* purpose= */ nullptr) && !name.empty())
                                strHTML += typename gui_util::HtmlEscape(name) + " ";
                            strHTML += typename gui_util::HtmlEscape(EncodeDestination(address));
                            if(toSelf == ISMINE_SPENDABLE)
                                strHTML += " (own address)";
                            else if(toSelf & ISMINE_WATCH_ONLY)
                                strHTML += " (watch-only)";
                            strHTML += "<br>";
                        }
                    }

                    strHTML += "<b>" + tr("Debit") + ":</b> " + BitcoinUnits::formatHtmlWithUnit(unit, -txout.nValue) + "<br>";
                    if(toSelf)
                        strHTML += "<b>" + tr("Credit") + ":</b> " + BitcoinUnits::formatHtmlWithUnit(unit, txout.nValue) + "<br>";
                }

                if (fAllToMe)
                {
                    // Payment to self
                    CAmount nChange = wtx.change;
                    CAmount nValue = nCredit - nChange;
                    strHTML += "<b>" + tr("Total debit") + ":</b> " + BitcoinUnits::formatHtmlWithUnit(unit, -nValue) + "<br>";
                    strHTML += "<b>" + tr("Total credit") + ":</b> " + BitcoinUnits::formatHtmlWithUnit(unit, nValue) + "<br>";
                }

                CAmount nTxFee = nDebit - wtx.tx->GetValueOut();
                if (nTxFee > 0)
                    strHTML += "<b>" + tr("Transaction fee") + ":</b> " + BitcoinUnits::formatHtmlWithUnit(unit, -nTxFee) + "<br>";
            }
            else
            {
                //
                // Mixed debit transaction
                //
                auto mine = wtx.txin_is_mine.begin();
                for (const CTxIn& txin : wtx.tx->vin) {
                    if (*(mine++)) {
                        strHTML += "<b>" + tr("Debit") + ":</b> " + BitcoinUnits::formatHtmlWithUnit(unit, -wallet.getDebit(txin, ISMINE_ALL)) + "<br>";
                    }
                }
                mine = wtx.txout_is_mine.begin();
                for (const CTxOut& txout : wtx.tx->vout) {
                    if (*(mine++)) {
                        strHTML += "<b>" + tr("Credit") + ":</b> " + BitcoinUnits::formatHtmlWithUnit(unit, wallet.getCredit(txout, ISMINE_ALL)) + "<br>";
                    }
                }
            }
        }

        strHTML += "<b>" + tr("Net amount") + ":</b> " + BitcoinUnits::formatHtmlWithUnit(unit, nNet, true) + "<br>";

        //
        // Message
        //
        if (wtx.value_map.count("message") && !wtx.value_map["message"].empty())
            strHTML += "<br><b>" + tr("Message") + ":</b><br>" + typename gui_util::HtmlEscape(wtx.value_map["message"], true) + "<br>";
        if (wtx.value_map.count("comment") && !wtx.value_map["comment"].empty())
            strHTML += "<br><b>" + tr("Comment") + ":</b><br>" + typename gui_util::HtmlEscape(wtx.value_map["comment"], true) + "<br>";

        strHTML += "<b>" + tr("Transaction ID") + ":</b> " + rec->getTxHash() + "<br>";
        strHTML += "<b>" + tr("Transaction total size") + ":</b> " + QString::number(wtx.tx->GetTotalSize()) + " bytes<br>";
        strHTML += "<b>" + tr("Transaction virtual size") + ":</b> " + QString::number(GetVirtualTransactionSize(*wtx.tx)) + " bytes<br>";
        strHTML += "<b>" + tr("Output index") + ":</b> " + QString::number(rec->getOutputIndex()) + "<br>";

        // Message from normal bitcoin:URI (bitcoin:123...?message=example)
        for (const std::pair<std::string, std::string>& r : orderForm) {
            if (r.first == "Message")
                strHTML += "<br><b>" + tr("Message") + ":</b><br>" + typename gui_util::HtmlEscape(r.second, true) + "<br>";

            //
            // PaymentRequest info:
            //
            if (r.first == "PaymentRequest")
            {
                QString merchant;
                if (!GetPaymentRequestMerchant(r.second, merchant)) {
                    merchant.clear();
                } else {
                    merchant += tr(" (Certificate was not verified)");
                }
                if (!merchant.isNull()) {
                    strHTML += "<b>" + tr("Merchant") + ":</b> " + typename gui_util::HtmlEscape(merchant) + "<br>";
                }
            }
        }

        if (wtx.is_coinbase)
        {
            quint32 numBlocksToMaturity = COINBASE_MATURITY +  1;
            strHTML += "<br>" + tr("Generated coins must mature %1 blocks before they can be spent. When you generated this block, it was broadcast to the network to be added to the block chain. If it fails to get into the chain, its state will change to \"not accepted\" and it won't be spendable. This may occasionally happen if another node generates a block within a few seconds of yours.").arg(QString::number(numBlocksToMaturity)) + "<br>";
        }

        //
        // Debug view
        //
        if (node.getLogCategories() != BCLog::NONE)
        {
            strHTML += "<hr><br>" + tr("Debug information") + "<br><br>";
            for (const CTxIn& txin : wtx.tx->vin)
                if(wallet.txinIsMine(txin))
                    strHTML += "<b>" + tr("Debit") + ":</b> " + BitcoinUnits::formatHtmlWithUnit(unit, -wallet.getDebit(txin, ISMINE_ALL)) + "<br>";
            for (const CTxOut& txout : wtx.tx->vout)
                if(wallet.txoutIsMine(txout))
                    strHTML += "<b>" + tr("Credit") + ":</b> " + BitcoinUnits::formatHtmlWithUnit(unit, wallet.getCredit(txout, ISMINE_ALL)) + "<br>";

            strHTML += "<br><b>" + tr("Transaction") + ":</b><br>";
            strHTML += typename gui_util::HtmlEscape(wtx.tx->ToString(), true);

            strHTML += "<br><b>" + tr("Inputs") + ":</b>";
            strHTML += "<ul>";

            for (const CTxIn& txin : wtx.tx->vin)
            {
                OutPoint prevout = txin.prevout;

                Coin prev;
                if(node.getUnspentOutput(prevout, prev))
                {
                    {
                        strHTML += "<li>";
                        const CTxOut &vout = prev.out;
                        TxDestination address;
                        if (ExtractDestination(vout.scriptPubKey, address))
                        {
                            std::string name;
                            if (wallet.getAddress(address, &name, /* is_mine= */ nullptr, /* purpose= */ nullptr) && !name.empty())
                                strHTML += typename gui_util::HtmlEscape(name) + " ";
                            strHTML += QString::fromStdString(EncodeDestination(address));
                        }
                        strHTML = strHTML + " " + tr("Amount") + "=" + BitcoinUnits::formatHtmlWithUnit(unit, vout.nValue);
                        strHTML = strHTML + " IsMine=" + (wallet.txoutIsMine(vout) & ISMINE_SPENDABLE ? tr("true") : tr("false")) + "</li>";
                        strHTML = strHTML + " IsWatchOnly=" + (wallet.txoutIsMine(vout) & ISMINE_WATCH_ONLY ? tr("true") : tr("false")) + "</li>";
                    }
                }
            }

            strHTML += "</ul>";
        }

        strHTML += "</font></html>";
        return strHTML;
        */
    }
}

/**
  | Takes an encoded PaymentRequest as
  | a string and tries to find the Common
  | Name of the X.509 certificate used to
  | sign the PaymentRequest.
  |
  */
pub fn get_payment_request_merchant(
        pr:       &String,
        merchant: &mut String) -> bool {
    
    todo!();
        /*
            // Search for the supported pki type strings
        if (pr.find(std::string({0x12, 0x0b}) + "x509+sha256") != std::string::npos || pr.find(std::string({0x12, 0x09}) + "x509+sha1") != std::string::npos) {
            // We want the common name of the Subject of the cert. This should be the second occurrence
            // of the bytes 0x0603550403. The first occurrence of those is the common name of the issuer.
            // After those bytes will be either 0x13 or 0x0C, then length, then either the ascii or utf8
            // string with the common name which is the merchant name
            size_t cn_pos = pr.find({0x06, 0x03, 0x55, 0x04, 0x03});
            if (cn_pos != std::string::npos) {
                cn_pos = pr.find({0x06, 0x03, 0x55, 0x04, 0x03}, cn_pos + 5);
                if (cn_pos != std::string::npos) {
                    cn_pos += 5;
                    if (pr[cn_pos] == 0x13 || pr[cn_pos] == 0x0c) {
                        cn_pos++; // Consume the type
                        int str_len = pr[cn_pos];
                        cn_pos++; // Consume the string length
                        merchant = QString::fromUtf8(pr.data() + cn_pos, str_len);
                        return true;
                    }
                }
            }
        }
        return false;
        */
}
