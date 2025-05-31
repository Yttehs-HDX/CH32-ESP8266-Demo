#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Esp8266Error {
    Rx(RxError),
    Tx(TxError),
    StringConversion(StringConversionError),
}

impl core::fmt::Debug for Esp8266Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Esp8266Error::Rx(e) => write!(f, "Esp8266Error::Rx({e:?})"),
            Esp8266Error::Tx(e) => write!(f, "Esp8266Error::Tx({e:?})"),
            Esp8266Error::StringConversion(e) => {
                write!(f, "Esp8266Error::StringConversion({e:?})")
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RxError {
    Read(ch32_hal::usart::Error),
    Timeout,
}

impl core::fmt::Debug for RxError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RxError::Read(e) => write!(f, "RxError::Read({e:?})"),
            RxError::Timeout => write!(f, "RxError::Timeout"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TxError {
    Write(ch32_hal::usart::Error),
}

impl core::fmt::Debug for TxError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TxError::Write(e) => write!(f, "TxError::Write({e:?})"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringConversionError {
    Utf8Conversion,
    BufferConversion,
}
