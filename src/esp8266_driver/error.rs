#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Esp8266Error {
    RxError(RxError),
    TxError(TxError),
    StringConversionError(StringConversionError),
}

impl core::fmt::Debug for Esp8266Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Esp8266Error::RxError(e) => write!(f, "Esp8266Error::RxError({:?})", e),
            Esp8266Error::TxError(e) => write!(f, "Esp8266Error::TxError({:?})", e),
            Esp8266Error::StringConversionError(e) => write!(f, "Esp8266Error::StringConversionError({:?})", e),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RxError {
    ReadError(ch32_hal::usart::Error),
    Timeout,
}

impl core::fmt::Debug for RxError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RxError::ReadError(e) => write!(f, "RxError::ReadError({:?})", e),
            RxError::Timeout => write!(f, "RxError::Timeout"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TxError {
    WriteError(ch32_hal::usart::Error),
}

impl core::fmt::Debug for TxError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TxError::WriteError(e) => write!(f, "TxError::WriteError({:?})", e),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringConversionError {
    Utf8Error,
    BufferConversionError,
}
