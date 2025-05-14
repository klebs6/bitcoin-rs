// ---------------- [ File: bitcoin-qt/src/trafficgraphwidget.rs ]
crate::ix!();

pub const DESIRED_SAMPLES: usize = 800;
pub const XMARGIN:         usize = 10;
pub const YMARGIN:         usize = 10;

//-------------------------------------------[.cpp/bitcoin/src/qt/trafficgraphwidget.h]

#[Q_OBJECT]
pub struct TrafficGraphWidget {
    base:             QWidget,
    timer:            *mut QTimer,
    max:              f32,
    n_mins:           i32,
    samples_in:       QQueue<f32>,
    samples_out:      QQueue<f32>,
    n_last_bytes_in:  u64,
    n_last_bytes_out: u64,
    client_model:     *mut ClientModel,
}

//-------------------------------------------[.cpp/bitcoin/src/qt/trafficgraphwidget.cpp]
impl TrafficGraphWidget {

    pub fn new(parent: *mut QWidget) -> Self {
    
        todo!();
        /*
            :
        QWidget(parent),
        timer(nullptr),
        fMax(0.0f),
        nMins(0),
        vSamplesIn(),
        vSamplesOut(),
        nLastBytesIn(0),
        nLastBytesOut(0),
        clientModel(nullptr)
        timer = new QTimer(this);
        connect(timer, &QTimer::timeout, this, &TrafficGraphWidget::updateRates);
        */
    }
    
    pub fn set_client_model(&mut self, model: *mut ClientModel)  {
        
        todo!();
        /*
            clientModel = model;
        if(model) {
            nLastBytesIn = model->node().getTotalBytesRecv();
            nLastBytesOut = model->node().getTotalBytesSent();
        }
        */
    }
    
    pub fn get_graph_range_mins(&self) -> i32 {
        
        todo!();
        /*
            return nMins;
        */
    }
    
    #[Q_SLOT]
    pub fn paint_path(&mut self, 
        path:    &mut QPainterPath,
        samples: &mut QQueue<f32>)  {
        
        todo!();
        /*
            int sampleCount = samples.size();
        if(sampleCount > 0) {
            int h = height() - YMARGIN * 2, w = width() - XMARGIN * 2;
            int x = XMARGIN + w;
            path.moveTo(x, YMARGIN + h);
            for(int i = 0; i < sampleCount; ++i) {
                x = XMARGIN + w - w * i / DESIRED_SAMPLES;
                int y = YMARGIN + h - (int)(h * samples.at(i) / fMax);
                path.lineTo(x, y);
            }
            path.lineTo(x, YMARGIN + h);
        }
        */
    }
    
    pub fn paint_event(&mut self, _0: *mut QPaintEvent)  {
        
        todo!();
        /*
            QPainter painter(this);
        painter.fillRect(rect(), Qtblack);

        if(fMax <= 0.0f) return;

        QColor axisCol(Qtgray);
        int h = height() - YMARGIN * 2;
        painter.setPen(axisCol);
        painter.drawLine(XMARGIN, YMARGIN + h, width() - XMARGIN, YMARGIN + h);

        // decide what order of magnitude we are
        int base = floor(log10(fMax));
        float val = pow(10.0f, base);

        const QString units = tr("kB/s");
        const float yMarginText = 2.0;

        // draw lines
        painter.setPen(axisCol);
        painter.drawText(XMARGIN, YMARGIN + h - h * val / fMax-yMarginText, QString("%1 %2").arg(val).arg(units));
        for(float y = val; y < fMax; y += val) {
            int yy = YMARGIN + h - h * y / fMax;
            painter.drawLine(XMARGIN, yy, width() - XMARGIN, yy);
        }
        // if we drew 3 or fewer lines, break them up at the next lower order of magnitude
        if(fMax / val <= 3.0f) {
            axisCol = axisCol.darker();
            val = pow(10.0f, base - 1);
            painter.setPen(axisCol);
            painter.drawText(XMARGIN, YMARGIN + h - h * val / fMax-yMarginText, QString("%1 %2").arg(val).arg(units));
            int count = 1;
            for(float y = val; y < fMax; y += val, count++) {
                // don't overwrite lines drawn above
                if(count % 10 == 0)
                    continue;
                int yy = YMARGIN + h - h * y / fMax;
                painter.drawLine(XMARGIN, yy, width() - XMARGIN, yy);
            }
        }

        painter.setRenderHint(QPainter::Antialiasing);
        if(!vSamplesIn.empty()) {
            QPainterPath p;
            paintPath(p, vSamplesIn);
            painter.fillPath(p, QColor(0, 255, 0, 128));
            painter.setPen(Qtgreen);
            painter.drawPath(p);
        }
        if(!vSamplesOut.empty()) {
            QPainterPath p;
            paintPath(p, vSamplesOut);
            painter.fillPath(p, QColor(255, 0, 0, 128));
            painter.setPen(Qtred);
            painter.drawPath(p);
        }
        */
    }
    
    #[Q_SLOT]
    pub fn update_rates(&mut self)  {
        
        todo!();
        /*
            if(!clientModel) return;

        u64 bytesIn = clientModel->node().getTotalBytesRecv(),
                bytesOut = clientModel->node().getTotalBytesSent();
        float in_rate_kilobytes_per_sec = static_cast<float>(bytesIn - nLastBytesIn) / timer->interval();
        float out_rate_kilobytes_per_sec = static_cast<float>(bytesOut - nLastBytesOut) / timer->interval();
        vSamplesIn.push_front(in_rate_kilobytes_per_sec);
        vSamplesOut.push_front(out_rate_kilobytes_per_sec);
        nLastBytesIn = bytesIn;
        nLastBytesOut = bytesOut;

        while(vSamplesIn.size() > DESIRED_SAMPLES) {
            vSamplesIn.pop_back();
        }
        while(vSamplesOut.size() > DESIRED_SAMPLES) {
            vSamplesOut.pop_back();
        }

        float tmax = 0.0f;
        for (const float f : vSamplesIn) {
            if(f > tmax) tmax = f;
        }
        for (const float f : vSamplesOut) {
            if(f > tmax) tmax = f;
        }
        fMax = tmax;
        update();
        */
    }
    
    #[Q_SLOT]
    pub fn set_graph_range_mins(&mut self, mins: i32)  {
        
        todo!();
        /*
            nMins = mins;
        int msecsPerSample = nMins * 60 * 1000 / DESIRED_SAMPLES;
        timer->stop();
        timer->setInterval(msecsPerSample);

        clear();
        */
    }
    
    #[Q_SLOT]
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            timer->stop();

        vSamplesOut.clear();
        vSamplesIn.clear();
        fMax = 0.0f;

        if(clientModel) {
            nLastBytesIn = clientModel->node().getTotalBytesRecv();
            nLastBytesOut = clientModel->node().getTotalBytesSent();
        }
        timer->start();
        */
    }
}
