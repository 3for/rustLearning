
use std::convert::{From, Into, TryFrom, TryInto};

#[derive(Copy, Clone, Debug)]
pub struct TryIntoConvertError(());

#[derive(Copy, Clone, Debug)]
pub struct TryFromConvertError(());

#[derive(Clone, Debug)]
pub struct Message {
    raw: Vec<u8>,
}

impl TryFrom<Vec<u8>> for Message {
    type Error = TryFromConvertError;
    fn try_from(v: Vec<u8>) -> Result<Self, TryFromConvertError> {
        if v.len() < 8 {
            Err(TryFromConvertError(()))
        } else {
            Ok(Message { raw: v })
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for Message {
    type Error = TryFromConvertError;
    fn try_from(v: &[u8]) -> Result<Self, TryFromConvertError> {
        if v.len() < 8 {
            Err(TryFromConvertError(()))
        } else {
            Ok(Message { raw: v.to_vec() })
        }
    }
}

impl<'a> TryFrom<&'a Vec<u8>> for Message {
    type Error = TryFromConvertError;
    fn try_from(v: &Vec<u8>) -> Result<Self, TryFromConvertError> {
        if v.len() < 8 {
            Err(TryFromConvertError(()))
        } else {
            Ok(Message { raw: v.clone() })
        }
    }
}

impl TryInto<Vec<u8>> for Message {
    type Error = TryIntoConvertError;
    fn try_into(self) -> Result<Vec<u8>, TryIntoConvertError> {
        if self.raw.len() < 8 {
            Err(TryIntoConvertError(()))
        } else {
            Ok(self.raw)
        }
    }
}

impl<'a> TryInto<Vec<u8>> for &'a Message {
    type Error = TryIntoConvertError;
    fn try_into(self) -> Result<Vec<u8>, TryIntoConvertError> {
        if self.raw.len() < 8 {
            Err(TryIntoConvertError(()))
        } else {
            Ok(self.raw.clone())
        }
    }
}