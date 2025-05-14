// ---------------- [ File: bitcoin-cli/src/progress.rs ]
crate::ix!();

/**
  | GetProgressBar constructs a progress
  | bar with 5% intervals.
  | 
  | -----------
  | @param[in] progress
  | 
  | The proportion of the progress bar to
  | be filled between 0 and 1.
  | ----------
  | @param[out] progress_bar
  | 
  | String representation of the progress
  | bar.
  |
  */
pub fn get_progress_bar(
        progress:     f64,
        progress_bar: &mut String)  {

    if progress < 0.0 || progress > 1.0 {
        return;
    }

    lazy_static!{
        static ref INCREMENT: f64 = 0.05;

        static ref COMPLETE_BAR:   String = "\u{2592}".to_string();
        static ref INCOMPLETE_BAR: String = "\u{2591}".to_string();
    }

    for i in 0..(progress / *INCREMENT) as usize {
        *progress_bar += &*COMPLETE_BAR;
    }

    for i in 0..((1.0 - progress) / *INCREMENT) as usize {
        *progress_bar += &*INCOMPLETE_BAR;
    }
}
