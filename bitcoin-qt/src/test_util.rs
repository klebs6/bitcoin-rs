// ---------------- [ File: bitcoin-qt/src/test_util.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/test/util.h]
//-------------------------------------------[.cpp/bitcoin/src/qt/test/util.cpp]

/**
  | Press "Ok" button in message box dialog.
  | 
  | -----------
  | @param text
  | 
  | - Optionally store dialog text.
  | ----------
  | @param msec
  | 
  | - Number of milliseconds to pause before
  | triggering the callback.
  |
  */
pub fn confirm_message(
        text: Option<&mut str>,
        msec: Option<i32>)  {

    let msec: i32 = msec.unwrap_or(0);
    
    todo!();
        /*
            QTimer::singleShot(msec, [text]() {
            for (QWidget* widget : QApplication::topLevelWidgets()) {
                if (widget->inherits("QMessageBox")) {
                    QMessageBox* messageBox = qobject_cast<QMessageBox*>(widget);
                    if (text) *text = messageBox->text();
                    messageBox->defaultButton()->click();
                }
            }
        });
        */
}
