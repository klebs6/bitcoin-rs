crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/modaloverlay.h]

/**
  | The required delta of headers to the
  | estimated number of available headers
  | until we show the IBD progress
  |
  */
pub const HEADER_HEIGHT_DELTA_SYNC: i32 = 24;

/**
  | Modal overlay to display information
  | about the chain-sync state
  |
  */
#[Q_OBJECT]
pub struct ModalOverlay {
    base:               QWidget,
    ui:                 *mut UiModalOverlay,

    /**
      | best known height (based on the headers)
      |
      */
    best_header_height: i32,

    best_header_date:   QDateTime,
    block_process_time: Vec<(i64,f64)>,
    layer_is_visible:   bool,
    user_closed:        bool,
    animation:          QPropertyAnimation,
}

//-------------------------------------------[.cpp/bitcoin/src/qt/modaloverlay.cpp]
impl Drop for ModalOverlay {
    fn drop(&mut self) {
        todo!();
        /*
            delete ui;
        */
    }
}

impl ModalOverlay {

    pub fn is_layer_visible(&self) -> bool {
        
        todo!();
        /*
            return layerIsVisible;
        */
    }

    #[Q_SIGNAL]
    pub fn triggered(&mut self, hidden: bool)  { }
    
    pub fn new(
        enable_wallet: bool,
        parent:        *mut QWidget) -> Self {
    
        todo!();
        /*
            :
            QWidget(parent),
            ui(new UiModalOverlay),
            bestHeaderHeight(0),
            bestHeaderDate(QDateTime()),
            layerIsVisible(false),
            userClosed(false)

        ui->setupUi(this);
        connect(ui->closeButton, &QPushButton::clicked, this, &ModalOverlay::closeClicked);
        if (parent) {
            parent->installEventFilter(this);
            raise();
        }

        blockProcessTime.clear();
        setVisible(false);
        if (!enable_wallet) {
            ui->infoText->setVisible(false);
            ui->infoTextStrong->setText(tr("%1 is currently syncing.  It will download headers and blocks from peers and validate them until reaching the tip of the block chain.").arg(PACKAGE_NAME));
        }

        m_animation.setTargetObject(this);
        m_animation.setPropertyName("pos");
        m_animation.setDuration(300 /* ms */);
        m_animation.setEasingCurve(QEasingCurve::OutQuad);
        */
    }
    
    #[Q_SIGNAL]
    pub fn event_filter(&mut self, 
        obj: *mut QObject,
        ev:  *mut QEvent) -> bool {
        
        todo!();
        /*
            if (obj == parent()) {
            if (ev->type() == QEvent::Resize) {
                QResizeEvent * rev = static_cast<QResizeEvent*>(ev);
                resize(rev->size());
                if (!layerIsVisible)
                    setGeometry(0, height(), width(), height());

                if (m_animation.endValue().toPoint().y() > 0) {
                    m_animation.setEndValue(QPoint(0, height()));
                }
            }
            else if (ev->type() == QEvent::ChildAdded) {
                raise();
            }
        }
        return QWidget::eventFilter(obj, ev);
        */
    }

    /**
      | Tracks parent widget changes
      |
      */
    #[Q_SIGNAL]
    pub fn event(&mut self, ev: *mut QEvent) -> bool {
        
        todo!();
        /*
            if (ev->type() == QEvent::ParentAboutToChange) {
            if (parent()) parent()->removeEventFilter(this);
        }
        else if (ev->type() == QEvent::ParentChange) {
            if (parent()) {
                parent()->installEventFilter(this);
                raise();
            }
        }
        return QWidget::event(ev);
        */
    }
    
    pub fn set_known_best_height(&mut self, 
        count:      i32,
        block_date: &QDateTime)  {
        
        todo!();
        /*
            if (count > bestHeaderHeight) {
            bestHeaderHeight = count;
            bestHeaderDate = blockDate;
            UpdateHeaderSyncLabel();
        }
        */
    }
    
    pub fn tip_update(&mut self, 
        count:                   i32,
        block_date:              &QDateTime,
        n_verification_progress: f64)  {
        
        todo!();
        /*
            QDateTime currentDate = QDateTime::currentDateTime();

        // keep a vector of samples of verification progress at height
        blockProcessTime.push_front(qMakePair(currentDate.toMSecsSinceEpoch(), nVerificationProgress));

        // show progress speed if we have more than one sample
        if (blockProcessTime.size() >= 2) {
            double progressDelta = 0;
            double progressPerHour = 0;
            i64 timeDelta = 0;
            i64 remainingMSecs = 0;
            double remainingProgress = 1.0 - nVerificationProgress;
            for (int i = 1; i < blockProcessTime.size(); i++) {
                QPair<i64, double> sample = blockProcessTime[i];

                // take first sample after 500 seconds or last available one
                if (sample.first < (currentDate.toMSecsSinceEpoch() - 500 * 1000) || i == blockProcessTime.size() - 1) {
                    progressDelta = blockProcessTime[0].second - sample.second;
                    timeDelta = blockProcessTime[0].first - sample.first;
                    progressPerHour = progressDelta / (double) timeDelta * 1000 * 3600;
                    remainingMSecs = (progressDelta > 0) ? remainingProgress / progressDelta * timeDelta : -1;
                    break;
                }
            }
            // show progress increase per hour
            ui->progressIncreasePerH->setText(QString::number(progressPerHour * 100, 'f', 2)+"%");

            // show expected remaining time
            if(remainingMSecs >= 0) {
                ui->expectedTimeLeft->setText(typename gui_util::formatNiceTimeOffset(remainingMSecs / 1000.0));
            } else {
                ui->expectedTimeLeft->setText(QObject::tr("unknown"));
            }

            static const int MAX_SAMPLES = 5000;
            if (blockProcessTime.count() > MAX_SAMPLES) {
                blockProcessTime.remove(MAX_SAMPLES, blockProcessTime.count() - MAX_SAMPLES);
            }
        }

        // show the last block date
        ui->newestBlockDate->setText(blockDate.toString());

        // show the percentage done according to nVerificationProgress
        ui->percentageProgress->setText(QString::number(nVerificationProgress*100, 'f', 2)+"%");

        if (!bestHeaderDate.isValid())
            // not syncing
            return;

        // estimate the number of headers left based on nPowTargetSpacing
        // and check if the gui is not aware of the best header (happens rarely)
        int estimateNumHeadersLeft = bestHeaderDate.secsTo(currentDate) / Params().GetConsensus().nPowTargetSpacing;
        bool hasBestHeader = bestHeaderHeight >= count;

        // show remaining number of blocks
        if (estimateNumHeadersLeft < HEADER_HEIGHT_DELTA_SYNC && hasBestHeader) {
            ui->numberOfBlocksLeft->setText(QString::number(bestHeaderHeight - count));
        } else {
            UpdateHeaderSyncLabel();
            ui->expectedTimeLeft->setText(tr("Unknown…"));
        }
        */
    }
    
    pub fn update_header_sync_label(&mut self)  {
        
        todo!();
        /*
            int est_headers_left = bestHeaderDate.secsTo(QDateTime::currentDateTime()) / Params().GetConsensus().nPowTargetSpacing;
        ui->numberOfBlocksLeft->setText(tr("Unknown. Syncing Headers (%1, %2%)…").arg(bestHeaderHeight).arg(QString::number(100.0 / (bestHeaderHeight + est_headers_left) * bestHeaderHeight, 'f', 1)));
        */
    }
    
    #[Q_SLOT]
    pub fn toggle_visibility(&mut self)  {
        
        todo!();
        /*
            showHide(layerIsVisible, true);
        if (!layerIsVisible)
            userClosed = true;
        */
    }
    
    /**
      | will show or hide the modal layer
      |
      */
    pub fn show_hide(&mut self, 
        hide:           Option<bool>,
        user_requested: Option<bool>)  {

        let hide:           bool = hide.unwrap_or(false);
        let user_requested: bool = user_requested.unwrap_or(false);
        
        todo!();
        /*
            if ( (layerIsVisible && !hide) || (!layerIsVisible && hide) || (!hide && userClosed && !userRequested))
            return;

        Q_EMIT triggered(hide);

        if (!isVisible() && !hide)
            setVisible(true);

        m_animation.setStartValue(QPoint(0, hide ? 0 : height()));
        // The eventFilter() updates the endValue if it is required for QEvent::Resize.
        m_animation.setEndValue(QPoint(0, hide ? height() : 0));
        m_animation.start(QAbstractAnimation::KeepWhenStopped);
        layerIsVisible = !hide;
        */
    }
    
    #[Q_SLOT]
    pub fn close_clicked(&mut self)  {
        
        todo!();
        /*
            showHide(true);
        userClosed = true;
        */
    }
}
