// ---------------- [ File: bitcoinleveldb-posixlogger/src/posix_logger_build_log_body_from_format_and_arguments.rs ]
crate::ix!();

impl PosixLogger {

    pub fn build_log_body_from_format_and_arguments(
        &self,
        format: *const u8,
        arguments: &[&str],
    ) -> Option<String> {

        if format.is_null() {
            error!(
                "PosixLogger::build_log_body_from_format_and_arguments: null format pointer"
            );
            return None;
        }

        unsafe {

            let c_format = std::ffi::CStr::from_ptr(format as *const libc::c_char);
            let template = c_format.to_string_lossy();

            trace!(
                "PosixLogger::build_log_body_from_format_and_arguments: template='{}'",
                template
            );

            let mut body = String::new();
            let mut arg_index: usize = 0;
            let chars: Vec<char> = template.chars().collect();
            let mut i: usize = 0;

            while i < chars.len() {
                if chars[i] == '%' && i + 1 < chars.len() {
                    let next = chars[i + 1];
                    if next == 's' && arg_index < arguments.len() {
                        body.push_str(arguments[arg_index]);
                        arg_index += 1;
                        i += 2;
                        continue;
                    } else if next == '%' {
                        body.push('%');
                        i += 2;
                        continue;
                    }
                }
                body.push(chars[i]);
                i += 1;
            }

            trace!(
                "PosixLogger::build_log_body_from_format_and_arguments: body_len={} args_consumed={}",
                body.len(),
                arg_index
            );

            Some(body)
        }
    }
}

#[cfg(test)]
mod posix_logger_build_log_body_from_format_and_arguments_tests {
    use super::*;

    fn create_logger_with_tmpfile() -> PosixLogger {
        info!("create_logger_with_tmpfile: creating logger backed by tmpfile");
        unsafe {
            let fp = libc::tmpfile();
            if fp.is_null() {
                error!("create_logger_with_tmpfile: tmpfile returned null");
            }
            assert!(!fp.is_null(), "tmpfile should not return null");
            PosixLogger::new(fp)
        }
    }

    #[traced_test]
    fn build_log_body_returns_none_for_null_format_pointer() {
        info!("build_log_body_returns_none_for_null_format_pointer: start");
        let logger = create_logger_with_tmpfile();

        let result =
            logger.build_log_body_from_format_and_arguments(std::ptr::null::<u8>(), &[]);

        debug!(
            "build_log_body_returns_none_for_null_format_pointer: result_is_some={}",
            result.is_some()
        );
        assert!(
            result.is_none(),
            "Expected None when format pointer is null in build_log_body_from_format_and_arguments"
        );
        info!("build_log_body_returns_none_for_null_format_pointer: end");
    }

    #[traced_test]
    fn build_log_body_handles_plain_text_without_placeholders() {
        info!("build_log_body_handles_plain_text_without_placeholders: start");
        let logger = create_logger_with_tmpfile();
        let format_str = "static message";
        let format = CString::new(format_str).expect("CString::new failed");
        let format_ptr = format.as_ptr() as *const u8;

        let body = logger
            .build_log_body_from_format_and_arguments(format_ptr, &[])
            .expect("Body should be constructed for plain text");

        debug!(
            "build_log_body_handles_plain_text_without_placeholders: body='{}'",
            body
        );
        assert_eq!(body, format_str);
        info!("build_log_body_handles_plain_text_without_placeholders: end");
    }

    #[traced_test]
    fn build_log_body_substitutes_single_string_argument() {
        info!("build_log_body_substitutes_single_string_argument: start");
        let logger = create_logger_with_tmpfile();
        let format = CString::new("hello %s").expect("CString::new failed");
        let format_ptr = format.as_ptr() as *const u8;
        let arguments = ["world"];

        let body = logger
            .build_log_body_from_format_and_arguments(format_ptr, &arguments)
            .expect("Body should be constructed for single %s placeholder");

        debug!(
            "build_log_body_substitutes_single_string_argument: body='{}'",
            body
        );
        assert_eq!(body, "hello world");
        info!("build_log_body_substitutes_single_string_argument: end");
    }

    #[traced_test]
    fn build_log_body_handles_escaped_percent_signs() {
        info!("build_log_body_handles_escaped_percent_signs: start");
        let logger = create_logger_with_tmpfile();
        let format =
            CString::new("progress: 100%% complete").expect("CString::new failed");
        let format_ptr = format.as_ptr() as *const u8;

        let body = logger
            .build_log_body_from_format_and_arguments(format_ptr, &[])
            .expect("Body should be constructed for escaped percent");

        debug!(
            "build_log_body_handles_escaped_percent_signs: body='{}'",
            body
        );
        assert_eq!(body, "progress: 100% complete");
        info!("build_log_body_handles_escaped_percent_signs: end");
    }

    #[traced_test]
    fn build_log_body_treats_missing_arguments_as_literal_specifiers() {
        info!("build_log_body_treats_missing_arguments_as_literal_specifiers: start");
        let logger = create_logger_with_tmpfile();
        let format = CString::new("value=%s").expect("CString::new failed");
        let format_ptr = format.as_ptr() as *const u8;

        let body = logger
            .build_log_body_from_format_and_arguments(format_ptr, &[])
            .expect("Body should be constructed even with missing args");

        debug!(
            "build_log_body_treats_missing_arguments_as_literal_specifiers: body='{}'",
            body
        );
        assert_eq!(body, "value=%s");
        info!("build_log_body_treats_missing_arguments_as_literal_specifiers: end");
    }

    #[traced_test]
    fn build_log_body_ignores_extra_arguments_beyond_placeholders() {
        info!("build_log_body_ignores_extra_arguments_beyond_placeholders: start");
        let logger = create_logger_with_tmpfile();
        let format = CString::new("hello %s").expect("CString::new failed");
        let format_ptr = format.as_ptr() as *const u8;
        let arguments = ["world", "ignored-argument"];

        let body = logger
            .build_log_body_from_format_and_arguments(format_ptr, &arguments)
            .expect("Body should be constructed with extra args present");

        debug!(
            "build_log_body_ignores_extra_arguments_beyond_placeholders: body='{}'",
            body
        );
        assert_eq!(body, "hello world");
        info!("build_log_body_ignores_extra_arguments_beyond_placeholders: end");
    }
}
