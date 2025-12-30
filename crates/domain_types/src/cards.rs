use hyperswitch_masking::{Strategy, StrongSecret, WithType};

use serde::{Deserialize, Serialize};
use std::fmt;

pub enum CardNumberStrategy {}

impl<T> Strategy<T> for CardNumberStrategy
where
    T: AsRef<str>,
{
    fn fmt(val: &T, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val_str: &str = val.as_ref();

        if val_str.len() < 15 || val_str.len() > 19 {
            return WithType::fmt(val, f);
        }

        if let Some(value) = val_str.get(..6) {
            write!(f, "{}{}", value, "*".repeat(val_str.len() - 6))
        } else {
            WithType::fmt(val, f)
        }
    }
}

/// Card number
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct CardNumber(StrongSecret<String, CardNumberStrategy>);
