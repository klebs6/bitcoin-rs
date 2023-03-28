# bitcoin-time

The `bitcoin-time` crate is a direct translation
of the Bitcoin codebase from C++ to Rust,
providing a suite of time-related tools and
utilities for the Bitcoin system. This crate
includes functions for converting time between
various formats (`chrono_sanity_check`, `of`,
`format_iso8601date`, `format_iso8601date_time`,
`parse_iso8601date_time`, etc.), as well as
functions for measuring and manipulating time
(`count_microseconds`, `count_milliseconds`,
`count_seconds`, `count_seconds_double`,
`set_mock_time`, `uninterruptible_sleep`, etc.).

While there may not be any specific mathematical
equations or concepts involved in the
`bitcoin-time` crate, the correct handling and
manipulation of time is essential to the proper
functioning of the Bitcoin system. This crate is
currently in the process of translation, and it's
possible that some function bodies are still being
translated.

This crate also includes various helpers and
filters (`MedianFilter`, `median`, `sorted`,
`get_adjusted_datetime`, etc.) to assist with
accurate and precise time
manipulation. Additionally, there are functions
for retrieving and setting system time
(`get_time`, `get_time_micros_since_epoch`,
`get_time_millis_since_epoch`,
`get_time_seconds_since_epoch`,
`get_time_since_epoch`, etc.), as well as
functions for retrieving the maximum Unix
timestamp (`max_unix_timestamp`) and the time
offset (`get_time_offset`).
