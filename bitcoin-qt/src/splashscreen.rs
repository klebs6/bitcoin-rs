crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/splashscreen.h]

/**
  | Class for the splashscreen with information
  | of the running client.
  | 
  | -----------
  | @note
  | 
  | this is intentionally not a QSplashScreen.
  | Bitcoin Core initialization can take
  | a long time, and in that case a progress
  | window that cannot be moved around and
  | minimized has turned out to be frustrating
  | to the user.
  |
  */
#[Q_OBJECT]
pub struct SplashScreen {
    base:                      QWidget,
    pixmap:                    QPixmap,
    cur_message:               String,
    cur_color:                 QColor,
    cur_alignment:             i32,
    node:                      Rc<RefCell<dyn NodeInterface>>, // default = nullptr
    shutdown:                  bool, // default = false
    handler_init_message:      Box<dyn Handler>,
    handler_show_progress:     Box<dyn Handler>,
    handler_load_wallet:       Box<dyn Handler>,
    connected_wallets:         LinkedList<Box<dyn WalletInterface>>,
    connected_wallet_handlers: LinkedList<Box<dyn Handler>>,
}

//-------------------------------------------[.cpp/bitcoin/src/qt/splashscreen.cpp]
impl Drop for SplashScreen {
    fn drop(&mut self) {
        todo!();
        /*
            if (m_node) unsubscribeFromCoreSignals();
        */
    }
}

impl SplashScreen {
    
    pub fn new(network_style: *const NetworkStyle) -> Self {
    
        todo!();
        /*
        : q_widget(),
        : cur_alignment(0),

            // set reference point, paddings
        int paddingRight            = 50;
        int paddingTop              = 50;
        int titleVersionVSpace      = 17;
        int titleCopyrightVSpace    = 40;

        float fontFactor            = 1.0;
        float devicePixelRatio      = 1.0;
        devicePixelRatio = static_cast<QGuiApplication*>(QCoreApplication::instance())->devicePixelRatio();

        // define text to place
        QString titleText       = PACKAGE_NAME;
        QString versionText     = QString("Version %1").arg(QString::fromStdString(FormatFullVersion()));
        QString copyrightText   = QString::fromUtf8(CopyrightHolders(strprintf("\xc2\xA9 %u-%u ", 2009, COPYRIGHT_YEAR)).c_str());
        QString titleAddText    = networkStyle->getTitleAddText();

        QString font            = QApplication::font().toString();

        // create a bitmap according to device pixelratio
        QSize splashSize(480*devicePixelRatio,320*devicePixelRatio);
        pixmap = QPixmap(splashSize);

        // change to HiDPI if it makes sense
        pixmap.setDevicePixelRatio(devicePixelRatio);

        QPainter pixPaint(&pixmap);
        pixPaint.setPen(QColor(100,100,100));

        // draw a slightly radial gradient
        QRadialGradient gradient(QPoint(0,0), splashSize.width()/devicePixelRatio);
        gradient.setColorAt(0, Qtwhite);
        gradient.setColorAt(1, QColor(247,247,247));
        QRect rGradient(QPoint(0,0), splashSize);
        pixPaint.fillRect(rGradient, gradient);

        // draw the bitcoin icon, expected size of PNG: 1024x1024
        QRect rectIcon(QPoint(-150,-122), QSize(430,430));

        const QSize requiredSize(1024,1024);
        QPixmap icon(networkStyle->getAppIcon().pixmap(requiredSize));

        pixPaint.drawPixmap(rectIcon, icon);

        // check font size and drawing with
        pixPaint.setFont(QFont(font, 33*fontFactor));
        QFontMetrics fm = pixPaint.fontMetrics();
        int titleTextWidth = typename gui_util::TextWidth(fm, titleText);
        if (titleTextWidth > 176) {
            fontFactor = fontFactor * 176 / titleTextWidth;
        }

        pixPaint.setFont(QFont(font, 33*fontFactor));
        fm = pixPaint.fontMetrics();
        titleTextWidth  = typename gui_util::TextWidth(fm, titleText);
        pixPaint.drawText(pixmap.width()/devicePixelRatio-titleTextWidth-paddingRight,paddingTop,titleText);

        pixPaint.setFont(QFont(font, 15*fontFactor));

        // if the version string is too long, reduce size
        fm = pixPaint.fontMetrics();
        int versionTextWidth  = typename gui_util::TextWidth(fm, versionText);
        if(versionTextWidth > titleTextWidth+paddingRight-10) {
            pixPaint.setFont(QFont(font, 10*fontFactor));
            titleVersionVSpace -= 5;
        }
        pixPaint.drawText(pixmap.width()/devicePixelRatio-titleTextWidth-paddingRight+2,paddingTop+titleVersionVSpace,versionText);

        // draw copyright stuff
        {
            pixPaint.setFont(QFont(font, 10*fontFactor));
            const int x = pixmap.width()/devicePixelRatio-titleTextWidth-paddingRight;
            const int y = paddingTop+titleCopyrightVSpace;
            QRect copyrightRect(x, y, pixmap.width() - x - paddingRight, pixmap.height() - y);
            pixPaint.drawText(copyrightRect, QtAlignLeft | QtAlignTop | QtTextWordWrap, copyrightText);
        }

        // draw additional text if special network
        if(!titleAddText.isEmpty()) {
            QFont boldFont = QFont(font, 10*fontFactor);
            boldFont.setWeight(QFont::Bold);
            pixPaint.setFont(boldFont);
            fm = pixPaint.fontMetrics();
            int titleAddTextWidth  = typename gui_util::TextWidth(fm, titleAddText);
            pixPaint.drawText(pixmap.width()/devicePixelRatio-titleAddTextWidth-10,15,titleAddText);
        }

        pixPaint.end();

        // Set window title
        setWindowTitle(titleText + " " + titleAddText);

        // Resize window and move to center of desktop, disallow resizing
        QRect r(QPoint(), QSize(pixmap.size().width()/devicePixelRatio,pixmap.size().height()/devicePixelRatio));
        resize(r.size());
        setFixedSize(r.size());
        move(QGuiApplication::primaryScreen()->geometry().center() - r.center());

        installEventFilter(this);

        typename gui_util::handleCloseWindowShortcut(this);
        */
    }
    
    pub fn set_node(&mut self, node: Rc<RefCell<dyn NodeInterface>>)  {
        
        todo!();
        /*
            assert(!m_node);
        m_node = &node;
        subscribeToCoreSignals();
        if (m_shutdown) m_node->startShutdown();
        */
    }
    
    /**
      | Initiate shutdown
      |
      */
    #[Q_SLOT]
    pub fn shutdown(&mut self)  {
        
        todo!();
        /*
            m_shutdown = true;
        if (m_node) m_node->startShutdown();
        */
    }
    
    #[Q_SLOT]
    pub fn event_filter(&mut self, 
        obj: *mut QObject,
        ev:  *mut QEvent) -> bool {
        
        todo!();
        /*
            if (ev->type() == QEvent::KeyPress) {
            QKeyEvent *keyEvent = static_cast<QKeyEvent *>(ev);
            if (keyEvent->key() == QtKey_Q) {
                shutdown();
            }
        }
        return QObject::eventFilter(obj, ev);
        */
    }
    
    /**
      | Hide the splash screen window and schedule
      | the splash screen object for deletion
      |
      */
    #[Q_SLOT]
    pub fn finish(&mut self)  {
        
        todo!();
        /*
            /* If the window is minimized, hide() will be ignored. */
        /* Make sure we de-minimize the splashscreen window before hiding */
        if (isMinimized())
            showNormal();
        hide();
        deleteLater(); // No more need for this
        */
    }
    
    /**
      | Connect core signals to splash screen
      |
      */
    #[Q_SLOT]
    pub fn subscribe_to_core_signals(&mut self)  {
        
        todo!();
        /*
            // Connect signals to client
        m_handler_init_message = m_node->handleInitMessage(std::bind(InitMessage, this, std::placeholders::_1));
        m_handler_show_progress = m_node->handleShowProgress(std::bind(ShowProgress, this, std::placeholders::_1, std::placeholders::_2, std::placeholders::_3));
        */
    }
    
    /**
      | Handle wallet load notifications.
      |
      */
    #[Q_SLOT]
    pub fn handle_load_wallet(&mut self)  {
        
        todo!();
        /*
            #ifdef ENABLE_WALLET
        if (!WalletModel::isWalletEnabled()) return;
        m_handler_load_wallet = m_node->walletClient().handleLoadWallet([this](std::unique_ptr<typename interfaces::Wallet> wallet) {
            m_connected_wallet_handlers.emplace_back(wallet->handleShowProgress(std::bind(ShowProgress, this, std::placeholders::_1, std::placeholders::_2, false)));
            m_connected_wallets.emplace_back(std::move(wallet));
        });
    #endif
        */
    }
    
    /**
      | Disconnect core signals to splash screen
      |
      */
    #[Q_SLOT]
    pub fn unsubscribe_from_core_signals(&mut self)  {
        
        todo!();
        /*
            // Disconnect signals from client
        m_handler_init_message->disconnect();
        m_handler_show_progress->disconnect();
        for (const auto& handler : m_connected_wallet_handlers) {
            handler->disconnect();
        }
        m_connected_wallet_handlers.clear();
        m_connected_wallets.clear();
        */
    }
    
    /**
      | Show message and progress
      |
      */
    #[Q_SLOT]
    pub fn show_message(&mut self, 
        message:   &String,
        alignment: i32,
        color:     &QColor)  {
        
        todo!();
        /*
            curMessage = message;
        curAlignment = alignment;
        curColor = color;
        update();
        */
    }
    
    pub fn paint_event(&mut self, event: *mut QPaintEvent)  {
        
        todo!();
        /*
            QPainter painter(this);
        painter.drawPixmap(0, 0, pixmap);
        QRect r = rect().adjusted(5, 5, -5, -5);
        painter.setPen(curColor);
        painter.drawText(r, curAlignment, curMessage);
        */
    }
    
    pub fn close_event(&mut self, event: *mut QCloseEvent)  {
        
        todo!();
        /*
            shutdown(); // allows an "emergency" shutdown during startup
        event->ignore();
        */
    }
}

pub fn init_message(
        splash:  *mut SplashScreen,
        message: &String)  {
    
    todo!();
        /*
            bool invoked = QMetaObject::invokeMethod(splash, "showMessage",
            QtQueuedConnection,
            Q_ARG(QString, QString::fromStdString(message)),
            Q_ARG(int, QtAlignBottom|QtAlignHCenter),
            Q_ARG(QColor, QColor(55,55,55)));
        assert(invoked);
        */
}

pub fn show_progress(
        splash:          *mut SplashScreen,
        title:           &String,
        n_progress:      i32,
        resume_possible: bool)  {
    
    todo!();
        /*
            InitMessage(splash, title + std::string("\n") +
                (resume_possible ? SplashScreen::tr("(press q to shutdown and continue later)").toStdString()
                                    : SplashScreen::tr("press q to shutdown").toStdString()) +
                strprintf("\n%d", nProgress) + "%");
        */
}
