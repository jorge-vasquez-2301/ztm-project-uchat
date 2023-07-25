use nutype::nutype;

use crate::UserFacingError;

#[nutype(validate(present, max_len = 30))]
#[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Headline(String);

impl UserFacingError for HeadlineError {
    fn formatted_error(&self) -> &'static str {
        match self {
            Self::Missing => "Headline cannot be empty",
            Self::TooLong => "Headline is too long. Must be at most 30 characters.",
        }
    }
}

#[nutype(validate(present, max_len = 100))]
#[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Message(String);

impl UserFacingError for MessageError {
    fn formatted_error(&self) -> &'static str {
        match self {
            Self::Missing => "Message cannot be empty",
            Self::TooLong => "Message is too long. Must be at most 100 characters.",
        }
    }
}