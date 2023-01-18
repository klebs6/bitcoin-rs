crate::ix!();

pub struct Logger {

    /**
      | Can not use Mutex from sync.h because
      | in debug mode it would cause a deadlock
      | when a potential deadlock was detected
      |
      | TODO: does this still happen with the rust Mutex?
      */
    cs:                  RefCell<Mutex<LoggerInner>>,

    /**
      | m_started_new_line is a state variable
      | that will suppress printing of the timestamp
      | when multiple calls are made that don't
      | end in a newline.
      |
      */
    started_new_line:    AtomicBool, // default = { true }

    /**
      | Log categories bitfield.
      |
      */
    categories:          AtomicU32, // default = { 0 }

    print_to_console:    bool, // default = false
    print_to_file:       bool, // default = false
    log_timestamps:      bool, // default = DEFAULT_LOGTIMESTAMPS
    log_time_micros:     bool, // default = DEFAULT_LOGTIMEMICROS
    log_threadnames:     bool, // default = DEFAULT_LOGTHREADNAMES
    log_sourcelocations: bool, // default = DEFAULT_LOGSOURCELOCATIONS
    file_path:           Box<Path>,
    reopen_file:         AtomicBool, // default = { false }

}

pub struct LoggerInner {

    fileout:             *mut libc::FILE, // default = nullptr

    msgs_before_open:    LinkedList<String>,

    /**
      | Buffer messages before logging can
      | be started.
      |
      */
    buffering:           bool, // default = true

    /**
      | Slots that connect to the print signal
      |
      */
    print_callbacks:     LinkedList<fn(_0: &String) -> ()>,
}

impl Logger {

    /**
      | Returns whether logs will be written
      | to any output
      |
      */
    pub fn enabled(&self) -> bool {
        
        todo!();
        /*
            StdLockGuard scoped_lock(m_cs);
                return m_buffering || m_print_to_console || m_print_to_file || !m_print_callbacks.empty();
        */
    }

    /**
      | Connect a slot to the print signal and
      | return the connection
      |
      */
    pub fn push_back_callback(&mut self, fun: fn(_0: &String) -> ()) -> linked_list::Iter::<fn(_0: &String) -> ()> {
        
        todo!();
        /*
            StdLockGuard scoped_lock(m_cs);
                m_print_callbacks.push_back(std::move(fun));
                return --m_print_callbacks.end();
        */
    }

    /**
      | Delete a connection
      |
      */
    pub fn delete_callback(&mut self, it: linked_list::Iter::<fn(_0: &String) -> ()>)  {
        
        todo!();
        /*
            StdLockGuard scoped_lock(m_cs);
                m_print_callbacks.erase(it);
        */
    }

    pub fn get_category_mask(&self) -> u32 {
        
        todo!();
        /*
            return m_categories.load();
        */
    }
    
    /**
      | Returns a string with the log categories
      | in alphabetical order.
      |
      */
    pub fn log_categories_string(&self) -> String {
        
        todo!();
        /*
            return Join(LogCategoriesList(), ", ", [&](const LogCategory& i) { return i.category; });
            }{
        */
    }
    
    /**
      | Start logging (and flush all buffered
      | messages)
      |
      */
    pub fn start_logging(&mut self) -> bool {
        
        todo!();
        /*
            StdLockGuard scoped_lock(m_cs);

        assert(m_buffering);
        assert(m_fileout == nullptr);

        if (m_print_to_file) {
            assert(!m_file_path.empty());
            m_fileout = fsbridge::fopen(m_file_path, "a");
            if (!m_fileout) {
                return false;
            }

            setbuf(m_fileout, nullptr); // unbuffered

            // Add newlines to the logfile to distinguish this execution from the
            // last one.
            FileWriteStr("\n\n\n\n\n", m_fileout);
        }

        // dump buffered messages from before we opened the log
        m_buffering = false;
        while (!m_msgs_before_open.empty()) {
            const std::string& s = m_msgs_before_open.front();

            if (m_print_to_file) FileWriteStr(s, m_fileout);
            if (m_print_to_console) fwrite(s.data(), 1, s.size(), stdout);
            for (const auto& cb : m_print_callbacks) {
                cb(s);
            }

            m_msgs_before_open.pop_front();
        }
        if (m_print_to_console) fflush(stdout);

        return true;
        */
    }
    
    /**
      | Only for testing
      |
      */
    pub fn disconnect_test_logger(&mut self)  {
        
        todo!();
        /*
            StdLockGuard scoped_lock(m_cs);
        m_buffering = true;
        if (m_fileout != nullptr) fclose(m_fileout);
        m_fileout = nullptr;
        m_print_callbacks.clear();
        */
    }
    
    pub fn enable_category_with_flags(&mut self, flag: LogFlags)  {
        
        todo!();
        /*
            m_categories |= flag;
        */
    }
    
    pub fn enable_category(&mut self, str_: &String) -> bool {
        
        todo!();
        /*
            BCLog::LogFlags flag;
        if (!GetLogCategory(flag, str)) return false;
        EnableCategory(flag);
        return true;
        */
    }
    
    pub fn disable_category_with_flags(&mut self, flag: LogFlags)  {
        
        todo!();
        /*
            m_categories &= ~flag;
        */
    }
    
    pub fn disable_category(&mut self, str_: &String) -> bool {
        
        todo!();
        /*
            BCLog::LogFlags flag;
        if (!GetLogCategory(flag, str)) return false;
        DisableCategory(flag);
        return true;
        */
    }
    
    pub fn will_log_category(&self, category: LogFlags) -> bool {
        
        todo!();
        /*
            return (m_categories.load(std::memory_order_relaxed) & category) != 0;
        */
    }
    
    pub fn default_shrink_debug_file(&self) -> bool {
        
        todo!();
        /*
            return m_categories == BCLog::NONE;
        */
    }
    
    /**
      | Returns a vector of the log categories
      | in alphabetical order.
      |
      */
    pub fn log_categories_list(&self) -> Vec<LogCategory> {
        
        todo!();
        /*
            // Sort log categories by alphabetical order.
        std::array<CLogCategoryDesc, std::size(LogCategories)> categories;
        std::copy(std::begin(LogCategories), std::end(LogCategories), categories.begin());
        std::sort(categories.begin(), categories.end(), [](auto a, auto b) { return a.category < b.category; });

        std::vector<LogCategory> ret;
        for (const CLogCategoryDesc& category_desc : categories) {
            if (category_desc.flag == BCLog::NONE || category_desc.flag == BCLog::ALL) continue;
            LogCategory catActive;
            catActive.category = category_desc.category;
            catActive.active = WillLogCategory(category_desc.flag);
            ret.push_back(catActive);
        }
        return ret;
        */
    }
    
    pub fn log_timestamp_str(&mut self, str_: &String) -> String {
        
        todo!();
        /*
            std::string strStamped;

        if (!m_log_timestamps)
            return str;

        if (m_started_new_line) {
            int64_t nTimeMicros = GetTimeMicros();
            strStamped = FormatISO8601DateTime(nTimeMicros/1000000);
            if (m_log_time_micros) {
                strStamped.pop_back();
                strStamped += strprintf(".%06dZ", nTimeMicros%1000000);
            }
            std::chrono::seconds mocktime = GetMockTime();
            if (mocktime > 0s) {
                strStamped += " (mocktime: " + FormatISO8601DateTime(count_seconds(mocktime)) + ")";
            }
            strStamped += ' ' + str;
        } else
            strStamped = str;

        return strStamped;
        */
    }
    
    /**
      | Send a string to the log output
      |
      */
    pub fn log_print_str(&mut self, 
        str_:             &String,
        logging_function: &String,
        source_file:      &String,
        source_line:      i32)  {
        
        todo!();
        /*
            StdLockGuard scoped_lock(m_cs);
        std::string str_prefixed = LogEscapeMessage(str);

        if (m_log_sourcelocations && m_started_new_line) {
            str_prefixed.insert(0, "[" + RemovePrefix(source_file, "./") + ":" + ToString(source_line) + "] [" + logging_function + "] ");
        }

        if (m_log_threadnames && m_started_new_line) {
            str_prefixed.insert(0, "[" + util::ThreadGetInternalName() + "] ");
        }

        str_prefixed = LogTimestampStr(str_prefixed);

        m_started_new_line = !str.empty() && str[str.size()-1] == '\n';

        if (m_buffering) {
            // buffer if we haven't started logging yet
            m_msgs_before_open.push_back(str_prefixed);
            return;
        }

        if (m_print_to_console) {
            // print to console
            fwrite(str_prefixed.data(), 1, str_prefixed.size(), stdout);
            fflush(stdout);
        }
        for (const auto& cb : m_print_callbacks) {
            cb(str_prefixed);
        }
        if (m_print_to_file) {
            assert(m_fileout != nullptr);

            // reopen the log file, if requested
            if (m_reopen_file) {
                m_reopen_file = false;
                FILE* new_fileout = fsbridge::fopen(m_file_path, "a");
                if (new_fileout) {
                    setbuf(new_fileout, nullptr); // unbuffered
                    fclose(m_fileout);
                    m_fileout = new_fileout;
                }
            }
            FileWriteStr(str_prefixed, m_fileout);
        }
        */
    }
    
    pub fn shrink_debug_file(&mut self)  {
        
        todo!();
        /*
            // Amount of debug.log to save at end when shrinking (must fit in memory)
        constexpr size_t RECENT_DEBUG_HISTORY_SIZE = 10 * 1000000;

        assert(!m_file_path.empty());

        // Scroll debug.log if it's getting too big
        FILE* file = fsbridge::fopen(m_file_path, "r");

        // Special files (e.g. device nodes) may not have a size.
        size_t log_size = 0;
        try {
            log_size = fs::file_size(m_file_path);
        } catch (const fs::filesystem_error&) {}

        // If debug.log file is more than 10% bigger the RECENT_DEBUG_HISTORY_SIZE
        // trim it down by saving only the last RECENT_DEBUG_HISTORY_SIZE bytes
        if (file && log_size > 11 * (RECENT_DEBUG_HISTORY_SIZE / 10))
        {
            // Restart the file with some of the end
            std::vector<char> vch(RECENT_DEBUG_HISTORY_SIZE, 0);
            if (fseek(file, -((long)vch.size()), SEEK_END)) {
                LogPrintf("Failed to shrink debug log file: fseek(...) failed\n");
                fclose(file);
                return;
            }
            int nBytes = fread(vch.data(), 1, vch.size(), file);
            fclose(file);

            file = fsbridge::fopen(m_file_path, "w");
            if (file)
            {
                fwrite(vch.data(), 1, nBytes, file);
                fclose(file);
            }
        }
        else if (file != nullptr)
            fclose(file);
        */
    }
}
