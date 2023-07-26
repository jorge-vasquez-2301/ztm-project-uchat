use nutype::nutype;

use crate::UserFacingError;

#[nutype(validate(present, min_len = 3, max_len = 30))]
#[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Username(String);

impl UserFacingError for UsernameError {
    fn formatted_error(&self) -> &'static str {
        match self {
            Self::Missing => "Username cannot be empty",
            Self::TooShort => "Username is too short. Must be at least 3 characters.",
            Self::TooLong => "Username is too long. Must be at most 30 characters.",
        }
    }
}

#[nutype(validate(present, min_len = 8))]
#[derive(AsRef, Clone, Serialize, Deserialize, PartialEq)]
pub struct Password(String);

impl UserFacingError for PasswordError {
    fn formatted_error(&self) -> &'static str {
        match self {
            Self::Missing => "Password cannot be empty",
            Self::TooShort => "Password is too short. Must be at least 8 characters.",
        }
    }
}

#[nutype(validate(max_len = 30))]
#[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DisplayName(String);

impl DisplayName {
    pub const MAX_CHARS: usize = 30;
}

impl UserFacingError for DisplayNameError {
    fn formatted_error(&self) -> &'static str {
        match self {
            Self::TooLong => "Username is too long. Must be at most 30 characters.",
        }
    }
}
