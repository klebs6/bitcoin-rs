// ---------------- [ File: bitcoin-qt/src/lib.rs ]
#![feature(test)]

#[macro_use] mod imports; use imports::*;

x!{unlinked}

x!{addressbookpage}
x!{addresstablemodel}
x!{askpassphrasedialog}
x!{bantablemodel}
x!{bitcoinaddressvalidator}
x!{bitcoinamountfield}
x!{bitcoingui}
x!{bitcoinstrings}
x!{bitcoinunits}
x!{clientmodel}
x!{coincontroldialog}
x!{coincontroltreewidget}
x!{createwalletdialog}
x!{csvmodelwriter}
x!{editaddressdialog}
x!{guiconstants}
x!{guiutil}
x!{initexecutor}
x!{intro}
x!{macdockiconhandler}
x!{macnotificationhandler}
x!{macos_appnap}
x!{run_main}
x!{modaloverlay}
x!{networkstyle}
x!{notificator}
x!{openuridialog}
x!{optionsdialog}
x!{optionsmodel}
x!{overviewpage}
x!{paymentserver}
x!{peertablemodel}
x!{peertablesortproxy}
x!{platformstyle}
x!{psbtoperationsdialog}
x!{qrimagewidget}
x!{qt}
x!{qvalidatedlineedit}
x!{qvaluecombobox}
x!{receivecoinsdialog}
x!{receiverequestdialog}
x!{recentrequeststablemodel}
x!{rpcconsole}
x!{sendcoinsdialog}
x!{sendcoinsentry}
x!{sendcoinsrecipient}
x!{signverifymessagedialog}
x!{splashscreen}
x!{test_addressbooktests}
x!{test_apptests}
x!{test_rpcnestedtests}
x!{test_test_main}
x!{test_uritests}
x!{test_util}
x!{test_wallettests}
x!{trafficgraphwidget}
x!{transactiondescdialog}
x!{transactiondesc}
x!{transactionfilterproxy}
x!{transactionoverviewwidget}
x!{transactiontablemodel}
x!{transactionview}
x!{utilitydialog}
x!{walletcontroller}
x!{walletframe}
x!{walletmodeltransaction}
x!{walletmodel}
x!{walletview}
x!{winshutdownmonitor}
x!{txn}

#[macro_export] macro_rules! q_declare_metatype {
    ($mt:ty) => {
        /*TODO*/
    }
}

q_declare_metatype!{u256}
