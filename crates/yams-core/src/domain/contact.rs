use std::convert::TryFrom;

use thiserror::Error;

#[derive(Debug, Error)]
#[error("Invalid email address: {0}")]
pub struct EmailAddressValidationError(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn new<S: AsRef<str>>(s: S) -> Result<Self, EmailAddressValidationError> {
        let s_ref = s.as_ref();
        // Naive email validation: must contain '@' and '.'
        if s_ref.contains('@') && s_ref.contains('.') {
            Ok(Self(s_ref.to_string()))
        } else {
            Err(EmailAddressValidationError(s_ref.to_string()))
        }
    }
}

impl TryFrom<String> for EmailAddress {
    type Error = EmailAddressValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        EmailAddress::new(s)
    }
}

impl TryFrom<&str> for EmailAddress {
    type Error = EmailAddressValidationError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        EmailAddress::new(s)
    }
}

impl AsRef<str> for EmailAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// Mobile number

#[derive(Debug, Error)]
#[error("Invalid mobile number: {0}")]
pub struct MobileNumberValidationError(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MobileNumber(String);

impl MobileNumber {
    pub fn new<S: AsRef<str>>(s: S) -> Result<Self, MobileNumberValidationError> {
        let s_ref = s.as_ref();
        // Naive validation: should be digits, with optional '+' at start, and length between 7 and 15
        let valid = {
            let body = if s_ref.starts_with('+') {
                &s_ref[1..]
            } else {
                s_ref
            };
            body.chars().all(|c| c.is_ascii_digit()) && body.len() >= 7 && body.len() <= 15
        };
        if valid {
            Ok(Self(s_ref.to_string()))
        } else {
            Err(MobileNumberValidationError(s_ref.to_string()))
        }
    }
}

impl TryFrom<String> for MobileNumber {
    type Error = MobileNumberValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        MobileNumber::new(s)
    }
}

impl TryFrom<&str> for MobileNumber {
    type Error = MobileNumberValidationError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        MobileNumber::new(s)
    }
}

impl AsRef<str> for MobileNumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
