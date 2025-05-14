// ---------------- [ File: bitcoin-qt/src/notificator.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/notificator.h]

/**
  | Cross-platform desktop notification
  | client.
  |
  */
#[Q_OBJECT]
pub struct Notificator {
    base:         QObject,
    parent:       *mut QWidget,
    program_name: String,
    mode:         NotificatorMode,
    tray_icon:    *mut QSystemTrayIcon,

    #[cfg(USE_DBUS)]
    interface:    *mut QDBusInterface,
}

/**
  | Message class
  |
  */
pub enum NotificatorClass
{
    /**
      | Informational message
      |
      */
    Information,    

    /**
      | Notify user of potential problem
      |
      */
    Warning,        

    /**
      | An error occurred
      |
      */
    Critical,        
}

pub enum NotificatorMode {

    /**
      | Ignore informational notifications,
      | and show a modal pop-up dialog for Critical
      | notifications.
      |
      */
    None,                       

    /**
      | Use DBus org.freedesktop.Notifications
      |
      */
    Freedesktop,                

    /**
      | Use QSystemTrayIcon::showMessage()
      |
      */
    QSystemTray,                

    /**
      | Use the 10.8+ User Notification Center
      | (Mac only)
      |
      */
    UserNotificationCenter,      
}

impl Notificator {

    /**
      | Create a new notificator.
      | 
      | -----------
      | @note
      | 
      | Ownership of trayIcon is not transferred
      | to this object.
      |
      */
    pub fn new(
        program_name: &String,
        tray_icon:    *mut QSystemTrayIcon,
        parent:       *mut QWidget) -> Self {
    
        todo!();
        /*


            :
        QObject(_parent),
        parent(_parent),
        programName(_programName),
        mode(None),
        trayIcon(_trayIcon)
    #ifdef USE_DBUS
        ,interface(nullptr)
    #endif

        if(_trayIcon && _trayIcon->supportsMessages())
        {
            mode = QSystemTray;
        }
    #ifdef USE_DBUS
        interface = new QDBusInterface("org.freedesktop.Notifications",
            "/org/freedesktop/Notifications", "org.freedesktop.Notifications");
        if(interface->isValid())
        {
            mode = Freedesktop;
        }
    #endif
    #ifdef Q_OS_MAC
        // check if users OS has support for NSUserNotification
        if( MacNotificationHandler::instance()->hasUserNotificationCenterSupport()) {
            mode = UserNotificationCenter;
        }
    #endif
        */
    }

    #[cfg(USE_DBUS)]
    pub fn notify_dbus(&mut self, 
        cls:            NotificatorClass,
        title:          &String,
        text:           &String,
        icon:           &QIcon,
        millis_timeout: i32)  {
        
        todo!();
        /*
            // https://developer.gnome.org/notification-spec/
        // Arguments for DBus "Notify" call:
        QList<QVariant> args;

        // Program Name:
        args.append(programName);

        // Replaces ID; A value of 0 means that this notification won't replace any existing notifications:
        args.append(0U);

        // Application Icon, empty string
        args.append(QString());

        // Summary
        args.append(title);

        // Body
        args.append(text);

        // Actions (none, actions are deprecated)
        QStringList actions;
        args.append(actions);

        // Hints
        QVariantMap hints;

        // If no icon specified, set icon based on class
        QIcon tmpicon;
        if(icon.isNull())
        {
            QStyle::StandardPixmap sicon = QStyle::SP_MessageBoxQuestion;
            switch(cls)
            {
            case Information: sicon = QStyle::SP_MessageBoxInformation; break;
            case Warning: sicon = QStyle::SP_MessageBoxWarning; break;
            case Critical: sicon = QStyle::SP_MessageBoxCritical; break;
            default: break;
            }
            tmpicon = QApplication::style()->standardIcon(sicon);
        }
        else
        {
            tmpicon = icon;
        }
        hints["icon_data"] = FreedesktopImage::toVariant(tmpicon.pixmap(FREEDESKTOP_NOTIFICATION_ICON_SIZE).toImage());
        args.append(hints);

        // Timeout (in msec)
        args.append(millisTimeout);

        // "Fire and forget"
        interface->callWithArgumentList(QDBus::NoBlock, "Notify", args);
        */
    }
    
    pub fn notify_systray(&mut self, 
        cls:            NotificatorClass,
        title:          &String,
        text:           &String,
        millis_timeout: i32)  {
        
        todo!();
        /*
            QSystemTrayIcon::MessageIcon sicon = QSystemTrayIcon::NoIcon;
        switch(cls) // Set icon based on class
        {
        case Information: sicon = QSystemTrayIcon::Information; break;
        case Warning: sicon = QSystemTrayIcon::Warning; break;
        case Critical: sicon = QSystemTrayIcon::Critical; break;
        }
        trayIcon->showMessage(title, text, sicon, millisTimeout);
        */
    }

    #[cfg(Q_OS_MAC)]
    pub fn notify_mac_user_notification_center(&mut self, 
        title: &String,
        text:  &String)  {
        
        todo!();
        /*
            // icon is not supported by the user notification center yet. OSX will use the app icon.
        MacNotificationHandler::instance()->showNotification(title, text);
        */
    }
    
    /**
      | Show notification message.
      | 
      | -----------
      | @note
      | 
      | Platform implementations are free
      | to ignore any of the provided fields
      | except for \a text.
      | 
      | -----------
      | @param[in] cls
      | 
      | general message class
      | ----------
      | @param[in] title
      | 
      | title shown with message
      | ----------
      | @param[in] text
      | 
      | message content
      | ----------
      | @param[in] icon
      | 
      | optional icon to show with message
      | ----------
      | @param[in] millisTimeout
      | 
      | notification timeout in milliseconds
      | (defaults to 10 seconds)
      |
      */
    #[Q_SLOT]
    pub fn notify(&mut self, 
        cls:            NotificatorClass,
        title:          &String,
        text:           &String,
        icon:           Option<&QIcon>,
        millis_timeout: Option<i32>)  {

        let icon:           &QIcon = unsafe { icon.unwrap_or(&QIcon::new()) };
        let millis_timeout:    i32 = millis_timeout.unwrap_or(10000);
        
        todo!();
        /*
            switch(mode)
        {
    #ifdef USE_DBUS
        case Freedesktop:
            notifyDBus(cls, title, text, icon, millisTimeout);
            break;
    #endif
        case QSystemTray:
            notifySystray(cls, title, text, millisTimeout);
            break;
    #ifdef Q_OS_MAC
        case UserNotificationCenter:
            notifyMacUserNotificationCenter(title, text);
            break;
    #endif
        default:
            if(cls == Critical)
            {
                // Fall back to old fashioned pop-up dialog if critical and no other notification available
                QMessageBox::critical(parent, title, text, QMessageBox::Ok, QMessageBox::Ok);
            }
            break;
        }
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/notificator.cpp]

/**
  | https://wiki.ubuntu.com/NotificationDevelopmentGuidelines
  | recommends at least 128
  |
  */
#[cfg(USE_DBUS)]
pub const FREEDESKTOP_NOTIFICATION_ICON_SIZE: i32 = 128;

impl Drop for Notificator {
    fn drop(&mut self) {
        todo!();
        /*
            #ifdef USE_DBUS
        delete interface;
    #endif
        */
    }
}

/**
  | Loosely based on 
  | https://www.qtcentre.org/archive/index.php/t-25879.html
  |
  */
#[cfg(USE_DBUS)]
#[derive(Default)]
pub struct FreedesktopImage {
    width:           i32,
    height:          i32,
    stride:          i32,
    has_alpha:       bool,
    channels:        i32,
    bits_per_sample: i32,
    image:           QByteArray,
}

#[cfg(USE_DBUS)]
impl FreedesktopImage {
    
    #[cfg(USE_DBUS)]
    pub fn new(img: &QImage) -> Self {
    
        todo!();
        /*


            :
        width(img.width()),
        height(img.height()),
        stride(img.width() * BYTES_PER_PIXEL),
        hasAlpha(true),
        channels(CHANNELS),
        bitsPerSample(BITS_PER_SAMPLE)
        // Convert 00xAARRGGBB to RGBA bytewise (endian-independent) format
        QImage tmp = img.convertToFormat(QImage::Format_ARGB32);
        const uint32_t *data = reinterpret_cast<const uint32_t*>(tmp.bits());

        unsigned int num_pixels = width * height;
        image.resize(num_pixels * BYTES_PER_PIXEL);

        for(unsigned int ptr = 0; ptr < num_pixels; ++ptr)
        {
            image[ptr*BYTES_PER_PIXEL+0] = data[ptr] >> 16; // R
            image[ptr*BYTES_PER_PIXEL+1] = data[ptr] >> 8;  // G
            image[ptr*BYTES_PER_PIXEL+2] = data[ptr];       // B
            image[ptr*BYTES_PER_PIXEL+3] = data[ptr] >> 24; // A
        }
        */
    }

    #[cfg(USE_DBUS)]
    pub fn meta_type(&mut self) -> i32 {
        
        todo!();
        /*
            return qDBusRegisterMetaType<FreedesktopImage>();
        */
    }
    
    /**
      | Image to variant that can be marshalled
      | over DBus
      |
      */
    #[cfg(USE_DBUS)]
    pub fn to_variant(&mut self, img: &QImage) -> QVariant {
        
        todo!();
        /*
            FreedesktopImage fimg(img);
        return QVariant(FreedesktopImage::metaType(), &fimg);
        */
    }
}

#[cfg(USE_DBUS)]
q_declare_metatype!{ FreedesktopImage }

/**
  | Image configuration settings
  |
  */
#[cfg(USE_DBUS)] pub const CHANNELS:        i32 = 4;
#[cfg(USE_DBUS)] pub const BYTES_PER_PIXEL: i32 = 4;
#[cfg(USE_DBUS)] pub const BITS_PER_SAMPLE: i32 = 8;

#[cfg(USE_DBUS)]
impl Shl<&FreedesktopImage> for &mut QDBusArgument {
    type Output = QDBusArgument;
    
    #[inline] fn shl(self, rhs: &FreedesktopImage) -> Self::Output {
        todo!();
        /*
            a.beginStructure();
        a << i.width << i.height << i.stride << i.hasAlpha << i.bitsPerSample << i.channels << i.image;
        a.endStructure();
        return a;
        */
    }
}

#[cfg(USE_DBUS)]
impl Shr<&mut FreedesktopImage> for &QDBusArgument {
    type Output = QDBusArgument;
    
    #[inline] fn shr(self, rhs: &mut FreedesktopImage) -> Self::Output {
        todo!();
        /*
            a.beginStructure();
        a >> i.width >> i.height >> i.stride >> i.hasAlpha >> i.bitsPerSample >> i.channels >> i.image;
        a.endStructure();
        return a;
        */
    }
}
