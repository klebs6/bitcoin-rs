// ---------------- [ File: bitcoin-qt/src/paymentserver.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/paymentserver.h]

/**
  | This class handles payment requests from
  | clicking on bitcoin: URIs
  |
  | This is somewhat tricky, because we have to
  | deal with the situation where the user clicks
  | on a link during startup/initialization, when
  | the splash-screen is up but the main window
  | (and the Send Coins tab) is not.
  |
  | So, the strategy is:
  |
  | Create the server, and register the event
  | handler, when the application is created. Save
  | any URIs received at or during startup in
  | a list.
  |
  | When startup is finished and the main window is
  | shown, a signal is sent to slot uiReady(),
  | which emits a receivedURI() signal for any
  | payment requests that happened during startup.
  |
  | After startup, receivedURI() happens as usual.
  |
  | This class has one more feature: a static
  | method that finds URIs passed in the command
  | line and, if a server is running in another
  | process, sends them to the server.
  |
  */
#[Q_OBJECT]
pub struct PaymentServer {

    base:          QObject,

    /**
      | true during startup
      |
      */
    save_uris:    bool,

    uri_server:    *mut QLocalServer,
    options_model: *mut OptionsModel,
}

impl PaymentServer {

    /**
      | Fired when a valid payment request is
      | received
      |
      */
    #[Q_SIGNAL]
    pub fn received_payment_request(&mut self, _0: SendCoinsRecipient)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Fired when a message should be reported
      | to the user
      |
      */
    #[Q_SIGNAL]
    pub fn message(&mut self, 
        title:   &String,
        message: &String,
        style:   u32)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Parse URIs on command line
      | 
      | Returns false on error
      |
      | Sending to the server is done synchronously,
      | at startup. If the server isn't already
      | running, startup continues, and the
      | items in savedPaymentRequest will
      | be handled when uiReady() is called.
      | 
      | Warning: ipcSendCommandLine() is
      | called early in init, so don't use "Q_EMIT
      | message()", but "QMessageBox::"!
      |
      */
    pub fn ipc_parse_command_line(&mut self, 
        argc: i32,
        argv: &[*mut u8])  {
        
        todo!();
        /*
            for (int i = 1; i < argc; i++)
        {
            QString arg(argv[i]);
            if (arg.startsWith("-"))
                continue;

            // If the bitcoin: URI contains a payment request, we are not able to detect the
            // network as that would require fetching and parsing the payment request.
            // That means clicking such an URI which contains a testnet payment request
            // will start a mainnet instance and throw a "wrong network" error.
            if (arg.startsWith(BITCOIN_IPC_PREFIX, Qt::CaseInsensitive)) // bitcoin: URI
            {
                if (savedPaymentRequests.contains(arg)) continue;
                savedPaymentRequests.insert(arg);

                SendCoinsRecipient r;
                if (gui_util::parseBitcoinURI(arg, &r) && !r.address.isEmpty())
                {
                    auto tempChainParams = CreateChainParams(gArgs, CBaseChainParams::MAIN);

                    if (IsValidDestinationString(r.address.toStdString(), *tempChainParams)) {
                        SelectParams(CBaseChainParams::MAIN);
                    } else {
                        tempChainParams = CreateChainParams(gArgs, CBaseChainParams::TESTNET);
                        if (IsValidDestinationString(r.address.toStdString(), *tempChainParams)) {
                            SelectParams(CBaseChainParams::TESTNET);
                        }
                    }
                }
            }
        }
        */
    }

    /**
      | Returns true if there were URIs on the
      | command line which were successfully sent
      | to an already-running process.
      |
      | Note: if a payment request is given,
      | SelectParams(MAIN/TESTNET) will be called
      | so we startup in the right mode.
      ===============================
      | Sending to the server is done synchronously, at
      | startup.
      |
      | If the server isn't already running, startup
      | continues, and the items in savedPaymentRequest
      | will be handled when uiReady() is called.
      |
      */
    pub fn ipc_send_command_line(&mut self) -> bool {
        
        todo!();
        /*
            bool fResult = false;
        for (const QString& r : savedPaymentRequests)
        {
            QLocalSocket* socket = new QLocalSocket();
            socket->connectToServer(ipcServerName(), QIODevice::WriteOnly);
            if (!socket->waitForConnected(BITCOIN_IPC_CONNECT_TIMEOUT))
            {
                delete socket;
                socket = nullptr;
                return false;
            }

            QByteArray block;
            QDataStream out(&block, QIODevice::WriteOnly);
            out.setVersion(QDataStream::Qt_4_0);
            out << r;
            out.device()->seek(0);

            socket->write(block);
            socket->flush();
            socket->waitForBytesWritten(BITCOIN_IPC_CONNECT_TIMEOUT);
            socket->disconnectFromServer();

            delete socket;
            socket = nullptr;
            fResult = true;
        }

        return fResult;
        */
    }
    
    /**
      | parent should be QApplication object
      |
      */
    pub fn new(
        parent:             *mut QObject,
        start_local_server: Option<bool>) -> Self {

        let start_local_server: bool =
                 start_local_server.unwrap_or(true);
    
        todo!();
        /*


            :
        QObject(parent),
        saveURIs(true),
        uriServer(nullptr),
        optionsModel(nullptr)
        // Install global event filter to catch QFileOpenEvents
        // on Mac: sent when you click bitcoin: links
        // other OSes: helpful when dealing with payment request files
        if (parent)
            parent->installEventFilter(this);

        QString name = ipcServerName();

        // Clean up old socket leftover from a crash:
        QLocalServer::removeServer(name);

        if (startLocalServer)
        {
            uriServer = new QLocalServer(this);

            if (!uriServer->listen(name)) {
                // constructor is called early in init, so don't use "Q_EMIT message()" here
                QMessageBox::critical(nullptr, tr("Payment request error"),
                    tr("Cannot start bitcoin: click-to-pay handler"));
            }
            else {
                connect(uriServer, &QLocalServer::newConnection, this, &PaymentServer::handleURIConnection);
            }
        }
        */
    }

    /**
      | Constructor registers this on the parent
      | 
      | QApplication to receive QEvent::FileOpen
      | and QEvent:Drop events
      |
      ======================
      | OSX-specific way of handling bitcoin:
      | URIs
      |
      */
    pub fn event_filter(&mut self, 
        object: *mut QObject,
        event:  *mut QEvent) -> bool {
        
        todo!();
        /*
            if (event->type() == QEvent::FileOpen) {
            QFileOpenEvent *fileEvent = static_cast<QFileOpenEvent*>(event);
            if (!fileEvent->file().isEmpty())
                handleURIOrFile(fileEvent->file());
            else if (!fileEvent->url().isEmpty())
                handleURIOrFile(fileEvent->url().toString());

            return true;
        }

        return QObject::eventFilter(object, event);
        */
    }
    
    /**
      | Signal this when the main window's UI
      | is ready to display payment requests
      | to the user
      |
      */
    #[Q_SLOT]
    pub fn ui_ready(&mut self)  {
        
        todo!();
        /*
            saveURIs = false;
        for (const QString& s : savedPaymentRequests)
        {
            handleURIOrFile(s);
        }
        savedPaymentRequests.clear();
        */
    }
    
    /**
      | Handle an incoming URI, URI with local
      | file scheme or file
      |
      */
    #[Q_SLOT]
    pub fn handle_uri_or_file(&mut self, s: &String)  {
        
        todo!();
        /*
            if (saveURIs)
        {
            savedPaymentRequests.insert(s);
            return;
        }

        if (s.startsWith("bitcoin://", Qt::CaseInsensitive))
        {
            Q_EMIT message(tr("URI handling"), tr("'bitcoin://' is not a valid URI. Use 'bitcoin:' instead."),
                CClientUIInterface::MSG_ERROR);
        }
        else if (s.startsWith(BITCOIN_IPC_PREFIX, Qt::CaseInsensitive)) // bitcoin: URI
        {
            QUrlQuery uri((QUrl(s)));
            // normal URI
            {
                SendCoinsRecipient recipient;
                if (gui_util::parseBitcoinURI(s, &recipient))
                {
                    std::string error_msg;
                    const TxDestination dest = DecodeDestination(recipient.address.toStdString(), error_msg);

                    if (!IsValidDestination(dest)) {
                        if (uri.hasQueryItem("r")) {  // payment request
                            Q_EMIT message(tr("URI handling"),
                                tr("Cannot process payment request because BIP70 is not supported.\n"
                                   "Due to widespread security flaws in BIP70 it's strongly recommended that any merchant instructions to switch wallets be ignored.\n"
                                   "If you are receiving this error you should request the merchant provide a BIP21 compatible URI."),
                                CClientUIInterface::ICON_WARNING);
                        }
                        Q_EMIT message(tr("URI handling"), QString::fromStdString(error_msg),
                            CClientUIInterface::MSG_ERROR);
                    }
                    else
                        Q_EMIT receivedPaymentRequest(recipient);
                }
                else
                    Q_EMIT message(tr("URI handling"),
                        tr("URI cannot be parsed! This can be caused by an invalid Bitcoin address or malformed URI parameters."),
                        CClientUIInterface::ICON_WARNING);

                return;
            }
        }

        if (QFile::exists(s)) // payment request file
        {
            Q_EMIT message(tr("Payment request file handling"),
                tr("Cannot process payment request because BIP70 is not supported.\n"
                   "Due to widespread security flaws in BIP70 it's strongly recommended that any merchant instructions to switch wallets be ignored.\n"
                   "If you are receiving this error you should request the merchant provide a BIP21 compatible URI."),
                CClientUIInterface::ICON_WARNING);
        }
        */
    }
    
    #[Q_SLOT]
    pub fn handle_uri_connection(&mut self)  {
        
        todo!();
        /*
            QLocalSocket *clientConnection = uriServer->nextPendingConnection();

        while (clientConnection->bytesAvailable() < (int)sizeof(quint32))
            clientConnection->waitForReadyRead();

        connect(clientConnection, &QLocalSocket::disconnected, clientConnection, &QLocalSocket::deleteLater);

        QDataStream in(clientConnection);
        in.setVersion(QDataStream::Qt_4_0);
        if (clientConnection->bytesAvailable() < (int)sizeof(quint16)) {
            return;
        }
        QString msg;
        in >> msg;

        handleURIOrFile(msg);
        */
    }
    
    /**
      | OptionsModel is used for getting proxy
      | settings and display unit
      |
      */
    pub fn set_options_model(&mut self, options_model: *mut OptionsModel)  {
        
        todo!();
        /*
            this->optionsModel = _optionsModel;
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/paymentserver.cpp]

pub const BITCOIN_IPC_CONNECT_TIMEOUT:           i32 = 1000; // milliseconds
pub const BITCOIN_IPC_PREFIX:           &'static str = "bitcoin:";

/**
  | Create a name that is unique for:
  |  testnet / non-testnet
  |  data directory
  |
  */
pub fn ipc_server_name() -> String {
    
    todo!();
        /*
            QString name("BitcoinQt");

        // Append a simple hash of the datadir
        // Note that gArgs.GetDataDirNet() returns a different path
        // for -testnet versus main net
        QString ddir(gui_util::boostPathToQString(gArgs.GetDataDirNet()));
        name.append(QString::number(qHash(ddir)));

        return name;
        */
}

/**
  | We store payment URIs and requests received
  | before the main GUI window is up and ready
  | to ask the user to send payment.
  |
  */
lazy_static!{
    /*
    static QSet<QString> savedPaymentRequests;
    */
}
