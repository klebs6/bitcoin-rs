// ---------------- [ File: bitcoin-qt/src/test_uritests.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/test/uritests.h]

#[Q_OBJECT]
pub struct URITests {
    base: QObject,
}

//-------------------------------------------[.cpp/bitcoin/src/qt/test/uritests.cpp]
impl URITests {

    #[Q_SLOT]
    pub fn uri_tests(&mut self)  {
        
        todo!();
        /*
            SendCoinsRecipient rv;
        QUrl uri;
        uri.setUrl(QString("bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?req-dontexist="));
        QVERIFY(!gui_util::parseBitcoinURI(uri, &rv));

        uri.setUrl(QString("bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?dontexist="));
        QVERIFY(gui_util::parseBitcoinURI(uri, &rv));
        QVERIFY(rv.address == QString("175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W"));
        QVERIFY(rv.label == QString());
        QVERIFY(rv.amount == 0);

        uri.setUrl(QString("bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?label=Wikipedia Example Address"));
        QVERIFY(gui_util::parseBitcoinURI(uri, &rv));
        QVERIFY(rv.address == QString("175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W"));
        QVERIFY(rv.label == QString("Wikipedia Example Address"));
        QVERIFY(rv.amount == 0);

        uri.setUrl(QString("bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?amount=0.001"));
        QVERIFY(gui_util::parseBitcoinURI(uri, &rv));
        QVERIFY(rv.address == QString("175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W"));
        QVERIFY(rv.label == QString());
        QVERIFY(rv.amount == 100000);

        uri.setUrl(QString("bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?amount=1.001"));
        QVERIFY(gui_util::parseBitcoinURI(uri, &rv));
        QVERIFY(rv.address == QString("175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W"));
        QVERIFY(rv.label == QString());
        QVERIFY(rv.amount == 100100000);

        uri.setUrl(QString("bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?amount=100&label=Wikipedia Example"));
        QVERIFY(gui_util::parseBitcoinURI(uri, &rv));
        QVERIFY(rv.address == QString("175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W"));
        QVERIFY(rv.amount == 10000000000LL);
        QVERIFY(rv.label == QString("Wikipedia Example"));

        uri.setUrl(QString("bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?message=Wikipedia Example Address"));
        QVERIFY(gui_util::parseBitcoinURI(uri, &rv));
        QVERIFY(rv.address == QString("175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W"));
        QVERIFY(rv.label == QString());

        QVERIFY(gui_util::parseBitcoinURI("bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?message=Wikipedia Example Address", &rv));
        QVERIFY(rv.address == QString("175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W"));
        QVERIFY(rv.label == QString());

        uri.setUrl(QString("bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?req-message=Wikipedia Example Address"));
        QVERIFY(gui_util::parseBitcoinURI(uri, &rv));

        uri.setUrl(QString("bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?amount=1,000&label=Wikipedia Example"));
        QVERIFY(!gui_util::parseBitcoinURI(uri, &rv));

        uri.setUrl(QString("bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?amount=1,000.0&label=Wikipedia Example"));
        QVERIFY(!gui_util::parseBitcoinURI(uri, &rv));
        */
    }
}
