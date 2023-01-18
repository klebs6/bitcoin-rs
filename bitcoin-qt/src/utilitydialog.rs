crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/utilitydialog.h]
//-------------------------------------------[.cpp/bitcoin/src/qt/utilitydialog.cpp]

/**
  | "Help message" dialog box
  |
  */
#[Q_OBJECT]
pub struct HelpMessageDialog {
    base: QDialog,
    ui:   *mut UiHelpMessageDialog,
    text: String,
}

impl Drop for HelpMessageDialog {
    fn drop(&mut self) {
        todo!();
        /*
            delete ui;
        */
    }
}

impl HelpMessageDialog {
    
    /**
      | "Help message" or "About" dialog box
      |
      */
    pub fn new(
        parent: *mut QWidget,
        about:  bool) -> Self {
    
        todo!();
        /*


            :
        QDialog(parent, typename gui_util::dialog_flags),
        ui(new UiHelpMessageDialog)

        ui->setupUi(this);

        QString version = QString{PACKAGE_NAME} + " " + tr("version") + " " + QString::fromStdString(FormatFullVersion());

        if (about)
        {
            setWindowTitle(tr("About %1").arg(PACKAGE_NAME));

            std::string licenseInfo = LicenseInfo();
            /// HTML-format the license message from the core
            QString licenseInfoHTML = QString::fromStdString(LicenseInfo());
            // Make URLs clickable
            QRegExp uri("<(.*)>", QtCaseSensitive, QRegExp::RegExp2);
            uri.setMinimal(true); // use non-greedy matching
            licenseInfoHTML.replace(uri, "<a href=\"\\1\">\\1</a>");
            // Replace newlines with HTML breaks
            licenseInfoHTML.replace("\n", "<br>");

            ui->aboutMessage->setTextFormat(QtRichText);
            ui->scrollArea->setVerticalScrollBarPolicy(QtScrollBarAsNeeded);
            text = version + "\n" + QString::fromStdString(FormatParagraph(licenseInfo));
            ui->aboutMessage->setText(version + "<br><br>" + licenseInfoHTML);
            ui->aboutMessage->setWordWrap(true);
            ui->helpMessage->setVisible(false);
        } else {
            setWindowTitle(tr("Command-line options"));
            QString header = "Usage:  bitcoin-qt [command-line options]                     \n";
            QTextCursor cursor(ui->helpMessage->document());
            cursor.insertText(version);
            cursor.insertBlock();
            cursor.insertText(header);
            cursor.insertBlock();

            std::string strUsage = gArgs.GetHelpMessage();
            QString coreOptions = QString::fromStdString(strUsage);
            text = version + "\n\n" + header + "\n" + coreOptions;

            QTextTableFormat tf;
            tf.setBorderStyle(QTextFrameFormat::BorderStyle_None);
            tf.setCellPadding(2);
            QVector<QTextLength> widths;
            widths << QTextLength(QTextLength::PercentageLength, 35);
            widths << QTextLength(QTextLength::PercentageLength, 65);
            tf.setColumnWidthConstraints(widths);

            QTextCharFormat bold;
            bold.setFontWeight(QFont::Bold);

            for (const QString &line : coreOptions.split("\n")) {
                if (line.startsWith("  -"))
                {
                    cursor.currentTable()->appendRows(1);
                    cursor.movePosition(QTextCursor::PreviousCell);
                    cursor.movePosition(QTextCursor::NextRow);
                    cursor.insertText(line.trimmed());
                    cursor.movePosition(QTextCursor::NextCell);
                } else if (line.startsWith("   ")) {
                    cursor.insertText(line.trimmed()+' ');
                } else if (line.size() > 0) {
                    //Title of a group
                    if (cursor.currentTable())
                        cursor.currentTable()->appendRows(1);
                    cursor.movePosition(QTextCursor::Down);
                    cursor.insertText(line.trimmed(), bold);
                    cursor.insertTable(1, 2, tf);
                }
            }

            ui->helpMessage->moveCursor(QTextCursor::Start);
            ui->scrollArea->setVisible(false);
            ui->aboutLogo->setVisible(false);
        }

        typename gui_util::handleCloseWindowShortcut(this);
        */
    }
    
    pub fn print_to_console(&mut self)  {
        
        todo!();
        /*
            // On other operating systems, the expected action is to print the message to the console.
        tfm::format(std::cout, "%s\n", qPrintable(text));
        */
    }
    
    pub fn show_or_print(&mut self)  {
        
        todo!();
        /*
            #if defined(WIN32)
        // On Windows, show a message box, as there is no stderr/stdout in windowed applications
        exec();
    #else
        // On other operating systems, print help text to console
        printToConsole();
    #endif
        */
    }
    
    #[Q_SLOT]
    pub fn on_ok_button_accepted(&mut self)  {
        
        todo!();
        /*
            close();
        */
    }
}

/**
  | "Shutdown" window
  |
  */
#[Q_OBJECT]
pub struct ShutdownWindow {
    base: QWidget,
}

impl ShutdownWindow {

    /**
      | "Shutdown" window
      |
      */
    pub fn new(
        parent: *mut QWidget,
        f:      Option<QtWindowFlags>) -> Self {

        let f: QtWindowFlags = f.unwrap_or(QtWidgetDefault);
    
        todo!();
        /*
        : q_widget(parent, f),

            QVBoxLayout *layout = new QVBoxLayout();
        layout->addWidget(new QLabel(
            tr("%1 is shutting down…").arg(PACKAGE_NAME) + "<br /><br />" +
            tr("Do not shut down the computer until this window disappears.")));
        setLayout(layout);

        typename gui_util::handleCloseWindowShortcut(this);
        */
    }
    
    pub fn show_shutdown_window(&mut self, window: *mut QMainWindow) -> *mut QWidget {
        
        todo!();
        /*
            assert(window != nullptr);

        // Show a simple window indicating shutdown status
        QWidget *shutdownWindow = new ShutdownWindow();
        shutdownWindow->setWindowTitle(window->windowTitle());

        // Center shutdown window at where main window was
        const QPoint global = window->mapToGlobal(window->rect().center());
        shutdownWindow->move(global.x() - shutdownWindow->width() / 2, global.y() - shutdownWindow->height() / 2);
        shutdownWindow->show();
        return shutdownWindow;
        */
    }
    
    pub fn close_event(&mut self, event: *mut QCloseEvent)  {
        
        todo!();
        /*
            event->ignore();
        */
    }
}
