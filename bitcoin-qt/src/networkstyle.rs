// ---------------- [ File: bitcoin-qt/src/networkstyle.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/networkstyle.h]

/**
  | Coin network-specific GUI style information
  |
  */
pub struct NetworkStyle {
    app_name:             String,
    app_icon:             QIcon,
    tray_and_window_icon: QIcon,
    title_add_text:       String,
}

impl NetworkStyle {

    pub fn get_app_name(&self) -> &String {
        
        todo!();
        /*
            return appName;
        */
    }
    
    pub fn get_app_icon(&self) -> &QIcon {
        
        todo!();
        /*
            return appIcon;
        */
    }
    
    pub fn get_tray_and_window_icon(&self) -> &QIcon {
        
        todo!();
        /*
            return trayAndWindowIcon;
        */
    }
    
    pub fn get_title_add_text(&self) -> &String {
        
        todo!();
        /*
            return titleAddText;
        */
    }
    
    /**
      | titleAddText needs to be const char*
      | for tr()
      |
      */
    pub fn new(
        app_name:                        &String,
        icon_color_hue_shift:            i32,
        icon_color_saturation_reduction: i32,
        title_add_text:                  *const u8) -> Self {
    
        todo!();
        /*
            :
        appName(_appName),
        titleAddText(qApp->translate("SplashScreen", _titleAddText))

        // load pixmap
        QPixmap pixmap(":/icons/bitcoin");

        if(iconColorHueShift != 0 && iconColorSaturationReduction != 0)
        {
            // generate QImage from QPixmap
            QImage img = pixmap.toImage();

            int h,s,l,a;

            // traverse though lines
            for(int y=0;y<img.height();y++)
            {
                QRgb *scL = reinterpret_cast< QRgb *>( img.scanLine( y ) );

                // loop through pixels
                for(int x=0;x<img.width();x++)
                {
                    // preserve alpha because QColor::getHsl doesn't return the alpha value
                    a = qAlpha(scL[x]);
                    QColor col(scL[x]);

                    // get hue value
                    col.getHsl(&h,&s,&l);

                    // rotate color on RGB color circle
                    // 70° should end up with the typical "testnet" green
                    h+=iconColorHueShift;

                    // change saturation value
                    if(s>iconColorSaturationReduction)
                    {
                        s -= iconColorSaturationReduction;
                    }
                    col.setHsl(h,s,l,a);

                    // set the pixel
                    scL[x] = col.rgba();
                }
            }

            //convert back to QPixmap
            pixmap.convertFromImage(img);
        }

        appIcon             = QIcon(pixmap);
        trayAndWindowIcon   = QIcon(pixmap.scaled(QSize(256,256)));
        */
    }
    
    /**
      | Get style associated with provided
      | network id, or 0 if not known
      |
      */
    pub fn instantiate(&mut self, network_id: &String) -> *const NetworkStyle {
        
        todo!();
        /*
            std::string titleAddText = networkId == CBaseChainParams::MAIN ? "" : strprintf("[%s]", networkId);
        for (const auto& network_style : network_styles) {
            if (networkId == network_style.networkId) {
                return new NetworkStyle(
                        network_style.appName,
                        network_style.iconColorHueShift,
                        network_style.iconColorSaturationReduction,
                        titleAddText.c_str());
            }
        }
        return nullptr;
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/networkstyle.cpp]

pub struct NetworkStyleDescriptor {
    network_id:                      *const u8,
    app_name:                        *const u8,
    icon_color_hue_shift:            i32,
    icon_color_saturation_reduction: i32,
} 

lazy_static!{
    /*
    network_styles[] = {
        {"main", QAPP_APP_NAME_DEFAULT, 0, 0},
        {"test", QAPP_APP_NAME_TESTNET, 70, 30},
        {"signet", QAPP_APP_NAME_SIGNET, 35, 15},
        {"regtest", QAPP_APP_NAME_REGTEST, 160, 30},
    };
    */
}
