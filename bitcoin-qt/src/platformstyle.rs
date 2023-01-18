crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/platformstyle.h]

/**
  | Coin network-specific GUI style information
  |
  */
pub struct PlatformStyle {
    name:              String,
    images_on_buttons: bool,
    colorize_icons:    bool,
    use_extra_spacing: bool,
}

impl PlatformStyle {

    pub fn get_name(&self) -> &String {
        
        todo!();
        /*
            return name;
        */
    }
    
    pub fn get_images_on_buttons(&self) -> bool {
        
        todo!();
        /*
            return imagesOnButtons;
        */
    }
    
    pub fn get_use_extra_spacing(&self) -> bool {
        
        todo!();
        /*
            return useExtraSpacing;
        */
    }
    
    pub fn new(
        name:              &String,
        images_on_buttons: bool,
        colorize_icons:    bool,
        use_extra_spacing: bool) -> Self {
    
        todo!();
        /*


            :
        name(_name),
        imagesOnButtons(_imagesOnButtons),
        colorizeIcons(_colorizeIcons),
        useExtraSpacing(_useExtraSpacing)
        */
    }
    
    pub fn text_color(&self) -> QColor {
        
        todo!();
        /*
            return QApplication::palette().color(QPalette::WindowText);
        */
    }
    
    pub fn single_color(&self) -> QColor {
        
        todo!();
        /*
            if (colorizeIcons) {
            const QColor colorHighlightBg(QApplication::palette().color(QPalette::Highlight));
            const QColor colorHighlightFg(QApplication::palette().color(QPalette::HighlightedText));
            const QColor colorText(QApplication::palette().color(QPalette::WindowText));
            const int colorTextLightness = colorText.lightness();
            if (abs(colorHighlightBg.lightness() - colorTextLightness) < abs(colorHighlightFg.lightness() - colorTextLightness)) {
                return colorHighlightBg;
            }
            return colorHighlightFg;
        }
        return {0, 0, 0};
        */
    }
    
    /**
      | Colorize an image (given filename)
      | with the icon color
      |
      */
    pub fn single_color_image(&self, filename: &String) -> QImage {
        
        todo!();
        /*
            if (!colorizeIcons)
            return QImage(filename);
        return ColorizeImage(filename, SingleColor());
        */
    }
    
    /**
      | Colorize an icon (given filename) with
      | the icon color
      |
      */
    pub fn single_color_icon_with_filename(&self, filename: &String) -> QIcon {
        
        todo!();
        /*
            if (!colorizeIcons)
            return QIcon(filename);
        return ColorizeIcon(filename, SingleColor());
        */
    }
    
    /**
      | Colorize an icon (given object) with
      | the icon color
      |
      */
    pub fn single_color_icon(&self, icon: &QIcon) -> QIcon {
        
        todo!();
        /*
            if (!colorizeIcons)
            return icon;
        return ColorizeIcon(icon, SingleColor());
        */
    }
    
    /**
      | Colorize an icon (given object) with
      | the text color
      |
      */
    pub fn text_color_icon(&self, icon: &QIcon) -> QIcon {
        
        todo!();
        /*
            return ColorizeIcon(icon, TextColor());
        */
    }
    
    /**
      | Get style associated with provided
      | platform name, or 0 if not known
      |
      */
    pub fn instantiate(&mut self, platform_id: &String) -> *const PlatformStyle {
        
        todo!();
        /*
            for (const auto& platform_style : platform_styles) {
            if (platformId == platform_style.platformId) {
                return new PlatformStyle(
                        platform_style.platformId,
                        platform_style.imagesOnButtons,
                        platform_style.colorizeIcons,
                        platform_style.useExtraSpacing);
            }
        }
        return nullptr;
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/platformstyle.cpp]

pub struct PlatformStyleDescriptor {
    platform_id:       &'static str,

    /**
      | Show images on push buttons
      |
      */
    images_on_buttons: bool,


    /**
      | Colorize single-color icons
      |
      */
    colorize_icons:    bool,


    /**
      | Extra padding/spacing in transactionview
      |
      */
    use_extra_spacing: bool,

} 

pub const PLATFORM_STYLES: &[PlatformStyleDescriptor] = &[
    PlatformStyleDescriptor {
        platform_id:        "macosx",
        images_on_buttons:  false,
        colorize_icons:     true,
        use_extra_spacing:  true,
    },
    PlatformStyleDescriptor {
        platform_id:        "windows",
        images_on_buttons:  true,
        colorize_icons:     false,
        use_extra_spacing:  false,
    },

    /**
      | Other: linux, unix, ...
      |
      */
    PlatformStyleDescriptor {
        platform_id:        "other",
        images_on_buttons:  true,
        colorize_icons:     true,
        use_extra_spacing:  false,
    }
];

/*
  | Local functions for colorizing single-color
  | images
  |
  */
pub fn make_single_color_image(
        img:       &mut QImage,
        colorbase: &QColor)  {
    
    todo!();
        /*
            img = img.convertToFormat(QImage::Format_ARGB32);
        for (int x = img.width(); x--; )
        {
            for (int y = img.height(); y--; )
            {
                const QRgb rgb = img.pixel(x, y);
                img.setPixel(x, y, qRgba(colorbase.red(), colorbase.green(), colorbase.blue(), qAlpha(rgb)));
            }
        }
        */
}

pub fn colorize_icon(
        ico:       &QIcon,
        colorbase: &QColor) -> QIcon {
    
    todo!();
        /*
            QIcon new_ico;
        for (const QSize& sz : ico.availableSizes())
        {
            QImage img(ico.pixmap(sz).toImage());
            MakeSingleColorImage(img, colorbase);
            new_ico.addPixmap(QPixmap::fromImage(img));
        }
        return new_ico;
        */
}

pub fn colorize_image(
        filename:  &String,
        colorbase: &QColor) -> QImage {
    
    todo!();
        /*
            QImage img(filename);
        MakeSingleColorImage(img, colorbase);
        return img;
        */
}

pub fn colorize_icon_with_filename(
        filename:  &String,
        colorbase: &QColor) -> QIcon {
    
    todo!();
        /*
            return QIcon(QPixmap::fromImage(ColorizeImage(filename, colorbase)));
        */
}
