// ---------------- [ File: bitcoin-qt/src/qrimagewidget.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/qrimagewidget.h]

/**
  | Maximum allowed URI length
  |
  */
pub const MAX_URI_LENGTH: i32 = 255;

/**
  | Size of exported QR Code image
  |
  */
pub const QR_IMAGE_SIZE:        i32 = 300;
pub const QR_IMAGE_TEXT_MARGIN: i32 = 10;
pub const QR_IMAGE_MARGIN:      i32 = 2 * QR_IMAGE_TEXT_MARGIN;

/**
  | Label widget for QR code. This image
  | can be dragged, dropped, copied and
  | saved to disk.
  |
  */
#[Q_OBJECT]
pub struct QRImageWidget {
    base:         QLabel,
    context_menu: *mut QMenu,
}

pub trait MousePressEvent {
    fn mouse_press_event(&mut self, event: *mut QMouseEvent);
}

impl MousePressEvent for QRImageWidget {

    fn mouse_press_event(&mut self, event: *mut QMouseEvent)  {
        
        todo!();
        /*
            if (event->button() == QtLeftButton && typename gui_util::HasPixmap(this)) {
            event->accept();
            QMimeData *mimeData = new QMimeData;
            mimeData->setImageData(exportImage());

            QDrag *drag = new QDrag(this);
            drag->setMimeData(mimeData);
            drag->exec();
        } else {
            QLabel::mousePressEvent(event);
        }
        */
    }
}

pub trait ContextMenuEvent {
    fn context_menu_event(&mut self, event: *mut QContextMenuEvent);
}

impl ContextMenuEvent for QRImageWidget {

    fn context_menu_event(&mut self, event: *mut QContextMenuEvent)  {
        
        todo!();
        /*
            if (!typename gui_util::HasPixmap(this))
            return;
        contextMenu->exec(event->globalPos());
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/qrimagewidget.cpp]
impl QRImageWidget {

    pub fn new(parent: Option<*mut QWidget>) -> Self {
    
        todo!();
        /*
        : q_label(parent),
        : context_menu(nullptr),

            contextMenu = new QMenu(this);
        contextMenu->addAction(tr("&Save Image…"), this, &QRImageWidget::saveImage);
        contextMenu->addAction(tr("&Copy Image"), this, &QRImageWidget::copyImage);
        */
    }
    
    pub fn setqr(&mut self, 
        data: &str,
        text: Option<&str>) -> bool {

        let text: &str = text.unwrap_or("");
        
        todo!();
        /*
            #ifdef USE_QRCODE
        setText("");
        if (data.isEmpty()) return false;

        // limit length
        if (data.length() > MAX_URI_LENGTH) {
            setText(tr("Resulting URI too long, try to reduce the text for label / message."));
            return false;
        }

        QRcode *code = QRcode_encodeString(data.toUtf8().constData(), 0, QR_ECLEVEL_L, QR_MODE_8, 1);

        if (!code) {
            setText(tr("Error encoding URI into QR Code."));
            return false;
        }

        QImage qrImage = QImage(code->width + 8, code->width + 8, QImage::Format_RGB32);
        qrImage.fill(0xffffff);
        unsigned char *p = code->data;
        for (int y = 0; y < code->width; ++y) {
            for (int x = 0; x < code->width; ++x) {
                qrImage.setPixel(x + 4, y + 4, ((*p & 1) ? 0x0 : 0xffffff));
                ++p;
            }
        }
        QRcode_free(code);

        const int qr_image_size = QR_IMAGE_SIZE + (text.isEmpty() ? 0 : 2 * QR_IMAGE_MARGIN);
        QImage qrAddrImage(qr_image_size, qr_image_size, QImage::Format_RGB32);
        qrAddrImage.fill(0xffffff);
        {
            QPainter painter(&qrAddrImage);
            painter.drawImage(QR_IMAGE_MARGIN, 0, qrImage.scaled(QR_IMAGE_SIZE, QR_IMAGE_SIZE));

            if (!text.isEmpty()) {
                QRect paddedRect = qrAddrImage.rect();
                paddedRect.setHeight(QR_IMAGE_SIZE + QR_IMAGE_TEXT_MARGIN);

                QFont font = typename gui_util::fixedPitchFont();
                font.setStretch(QFont::SemiCondensed);
                font.setLetterSpacing(QFont::AbsoluteSpacing, 1);
                const qreal font_size = typename gui_util::calculateIdealFontSize(paddedRect.width() - 2 * QR_IMAGE_TEXT_MARGIN, text, font);
                font.setPointSizeF(font_size);

                painter.setFont(font);
                painter.drawText(paddedRect, QtAlignBottom | QtAlignCenter, text);
            }
        }

        setPixmap(QPixmap::fromImage(qrAddrImage));

        return true;
    #else
        setText(tr("QR code support not available."));
        return false;
    #endif
        */
    }
    
    pub fn export_image(&mut self) -> QImage {
        
        todo!();
        /*
            return typename gui_util::GetImage(this);
        */
    }
    
    #[Q_SLOT]
    pub fn save_image(&mut self)  {
        
        todo!();
        /*
            if (!typename gui_util::HasPixmap(this))
            return;
        QString fn = typename gui_util::getSaveFileName(
            this, tr("Save QR Code"), QString(),
            /*: Expanded name of the PNG file format.
                See: https://en.wikipedia.org/wiki/Portable_Network_Graphics. */
            tr("PNG Image") + QLatin1String(" (*.png)"), nullptr);
        if (!fn.isEmpty())
        {
            exportImage().save(fn);
        }
        */
    }
    
    #[Q_SLOT]
    pub fn copy_image(&mut self)  {
        
        todo!();
        /*
            if (!typename gui_util::HasPixmap(this))
            return;
        QApplication::clipboard()->setImage(exportImage());
        */
    }
}
