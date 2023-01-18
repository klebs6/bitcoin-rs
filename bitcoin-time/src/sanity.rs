crate::ix!();

/**
  | Sanity check epoch match normal Unix
  | epoch
  |
  */
pub fn chrono_sanity_check() -> bool {
    
    todo!();
        /*
            // system_clock.time_since_epoch and time_t(0) are not guaranteed
        // to use the Unix epoch timestamp, prior to C++20, but in practice they almost
        // certainly will. Any differing behavior will be assumed to be an error, unless
        // certain platforms prove to consistently deviate, at which point we'll cope
        // with it by adding offsets.

        // Create a new clock from time_t(0) and make sure that it represents 0
        // seconds from the system_clock's time_since_epoch. Then convert that back
        // to a time_t and verify that it's the same as before.
        const time_t time_t_epoch{};
        auto clock = system_clock::from_time_t(time_t_epoch);
        if (duration_cast<seconds>(clock.time_since_epoch()).count() != 0) {
            return false;
        }

        time_t time_val = system_clock::to_time_t(clock);
        if (time_val != time_t_epoch) {
            return false;
        }

        // Check that the above zero time is actually equal to the known unix timestamp.
        struct tm epoch;
    #ifdef HAVE_GMTIME_R
        if (gmtime_r(&time_val, &epoch) == nullptr) {
    #else
        if (gmtime_s(&epoch, &time_val) != 0) {
    #endif
            return false;
        }

        if ((epoch.tm_sec != 0)  ||
           (epoch.tm_min  != 0)  ||
           (epoch.tm_hour != 0)  ||
           (epoch.tm_mday != 1)  ||
           (epoch.tm_mon  != 0)  ||
           (epoch.tm_year != 70)) {
            return false;
        }
        return true;
        */
}

