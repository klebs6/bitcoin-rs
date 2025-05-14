// ---------------- [ File: bitcoin-imports/src/stdexcept.rs ]
#[derive(Debug,Clone)]
pub enum LogicError {
    InvalidArgument { what: String },
    DomainError     { what: String },
    LengthError     { what: String },
    OutOfRange      { what: String },
    FutureError     { what: String },
    Default         { what: String },
}

#[derive(Debug,Clone)]
pub struct BadOptionalAccess {
    what: String,
}

#[derive(Debug,Clone)]
pub enum SystemError {
    IosBaseFailure  { what: String },
    FilesystemError { what: String },
    Default         { what: String },
}

pub trait RuntimeErrorInterface {}

#[derive(Debug,Clone)]
pub enum RuntimeError {
    RangeError                { what: String },
    OverflowError             { what: String },
    UnderflowError            { what: String },
    RegexError                { what: String },
    SystemError(SystemError),
    TxException               { what: String },
    NonexistentLocalTime      { what: String },
    AmbiguousLocalTime        { what: String },
    FormatError               { what: String },

    /**
     | Exception thrown on connection error.  This
     | error is used to determine when to wait if
     | -rpcwait is given.
     |
     */
    ConnectionFailed          { what: String },
    Default                   { what: String },
}

#[derive(Debug,Clone)]
pub struct BadTypeid {
    what: String,
}

#[derive(Debug,Clone)]
pub enum BadCast {
    BadAnyCast { what: String },
    Default    { what: String },
}

#[derive(Debug,Clone)]
pub struct BadWeakPtr {
    what: String,
}

#[derive(Debug,Clone)]
pub struct BadFunctionCall {
    what: String,
}

#[derive(Debug,Clone)]
pub enum BadAlloc {
    BadArrayNewLength { what: String },
    Default           { what: String },
}

#[derive(Debug,Clone)]
pub struct BadException {
    what: String,
}

#[derive(Debug,Clone)]
pub struct BadVariantAccess {
    what: String,
}

#[derive(Debug,Clone)]
pub enum StdException {
    LogicError(LogicError),
    BadOptionalAccess(BadOptionalAccess),
    RuntimeError(RuntimeError),
    BadTypeid(BadTypeid),
    BadCast(BadCast),
    BadWeakPtr(BadWeakPtr),
    BadFunctionCall(BadFunctionCall),
    BadAlloc(BadAlloc),
    BadException(BadException),
    BadVariantAccess(BadVariantAccess),
    Default { what: String },
}

//this just wraps some c++ exceptions, so we
//retain some semantics
pub fn runtime_error(x: &str) -> StdException {
    StdException::RuntimeError(RuntimeError::Default { what: x.to_string() })
}

pub fn connection_failed(x: &str) -> StdException {
    StdException::RuntimeError(RuntimeError::ConnectionFailed { what: x.to_string() })
}

pub fn invalid_argument(x: &str) -> StdException {
    StdException::LogicError(LogicError::InvalidArgument { what: x.to_string() })
}

pub fn logic_error(x: &str) -> StdException {
    StdException::LogicError(LogicError::Default { what: x.to_string() })
}

pub fn ios_base_failure(x: &str) -> StdException {
    let e = SystemError::IosBaseFailure { what: x.to_string() };
    StdException::RuntimeError(RuntimeError::SystemError(e))
}

/*
match e {
    StdException::LogicError(LogicError::InvalidArgument                              { .. })  => { },
    StdException::LogicError(LogicError::DomainError                                  { .. })  => { },
    StdException::LogicError(LogicError::LengthError                                  { .. })  => { },
    StdException::LogicError(LogicError::OutOfRange                                   { .. })  => { },
    StdException::LogicError(LogicError::FutureError                                  { .. })  => { },
    StdException::LogicError(LogicError::Default                                      { .. })  => { },

    StdException::BadOptionalAccess(BadOptionalAccess                                 { .. })  => { },

    StdException::RuntimeError(RuntimeError::RangeError                               { .. })  => { },
    StdException::RuntimeError(RuntimeError::OverflowError                            { .. })  => { },
    StdException::RuntimeError(RuntimeError::UnderflowError                           { .. })  => { },
    StdException::RuntimeError(RuntimeError::RegexError                               { .. })  => { },
    StdException::RuntimeError(RuntimeError::SystemError(SystemError::IosBaseFailure  { .. })) => { },
    StdException::RuntimeError(RuntimeError::SystemError(SystemError::FilesystemError { .. })) => { },
    StdException::RuntimeError(RuntimeError::SystemError(SystemError::Default         { .. })) => { },
    StdException::RuntimeError(RuntimeError::TxException                              { .. })  => { },
    StdException::RuntimeError(RuntimeError::NonexistentLocalTime                     { .. })  => { },
    StdException::RuntimeError(RuntimeError::AmbiguousLocalTime                       { .. })  => { },
    StdException::RuntimeError(RuntimeError::FormatError                              { .. })  => { },
    StdException::RuntimeError(RuntimeError::Default                                  { .. })  => { },

    StdException::BadTypeid(BadTypeid                                                 { .. })  => { },
    StdException::BadCast(BadCast::BadAnyCast                                         { .. })  => { },
    StdException::BadCast(BadCast::Default                                            { .. })  => { },

    StdException::BadWeakPtr(BadWeakPtr                                               { .. })  => { },
    StdException::BadFunctionCall(BadFunctionCall                                     { .. })  => { },

    StdException::BadAlloc(BadAlloc::BadArrayNewLength                                { .. })  => { },
    StdException::BadAlloc(BadAlloc::Default                                          { .. })  => { },

    StdException::BadException(BadException                                           { .. })  => { },
    StdException::BadVariantAccess(BadVariantAccess                                   { .. })  => { },
    StdException::Default { .. }  => { },
    _                                                                                          => { },
}

match e {
    StdException::LogicError(LogicError::InvalidArgument { .. })  => { },
}
*/

/*
match e {
    StdException::LogicError(LogicError::InvalidArgument                              { .. })  => { },
    StdException::LogicError(LogicError::DomainError                                  { .. })  => { },
    StdException::LogicError(LogicError::LengthError                                  { .. })  => { },
    StdException::LogicError(LogicError::OutOfRange                                   { .. })  => { },
    StdException::LogicError(LogicError::FutureError                                  { .. })  => { },
    StdException::LogicError(LogicError::Default                                      { .. })  => { },

    StdException::BadOptionalAccess(BadOptionalAccess                                 { .. })  => { },

    StdException::RuntimeError(RuntimeError::RangeError                               { .. })  => { },
    StdException::RuntimeError(RuntimeError::OverflowError                            { .. })  => { },
    StdException::RuntimeError(RuntimeError::UnderflowError                           { .. })  => { },
    StdException::RuntimeError(RuntimeError::RegexError                               { .. })  => { },
    StdException::RuntimeError(RuntimeError::SystemError(SystemError::IosBaseFailure  { .. })) => { },
    StdException::RuntimeError(RuntimeError::SystemError(SystemError::FilesystemError { .. })) => { },
    StdException::RuntimeError(RuntimeError::SystemError(SystemError::Default         { .. })) => { },
    StdException::RuntimeError(RuntimeError::TxException                              { .. })  => { },
    StdException::RuntimeError(RuntimeError::NonexistentLocalTime                     { .. })  => { },
    StdException::RuntimeError(RuntimeError::AmbiguousLocalTime                       { .. })  => { },
    StdException::RuntimeError(RuntimeError::FormatError                              { .. })  => { },
    StdException::RuntimeError(RuntimeError::Default                                  { .. })  => { },

    StdException::BadTypeid(BadTypeid                                                 { .. })  => { },
    StdException::BadCast(BadCast::BadAnyCast                                         { .. })  => { },
    StdException::BadCast(BadCast::Default                                            { .. })  => { },

    StdException::BadWeakPtr(BadWeakPtr                                               { .. })  => { },
    StdException::BadFunctionCall(BadFunctionCall                                     { .. })  => { },

    StdException::BadAlloc(BadAlloc::BadArrayNewLength                                { .. })  => { },
    StdException::BadAlloc(BadAlloc::Default                                          { .. })  => { },

    StdException::BadException(BadException                                           { .. })  => { },
    StdException::BadVariantAccess(BadVariantAccess                                   { .. })  => { },
    StdException::Default { .. }  => { },
    _                                                                                          => { },
}

match e {
    StdException::LogicError(LogicError::InvalidArgument { .. })  => { },
}
*/
