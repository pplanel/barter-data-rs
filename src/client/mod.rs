use std::str::FromStr;
use serde::{de, Deserialize, Deserializer};

/// Binance WebSocket client implementing the ExchangeClient trait.
pub mod binance;

/// Bitstamp WebSocket client implementing the ExchangeClient trait.
pub mod bitstamp;

/// Common client configuration that is likely required by an ExchangeClient trait implementor.
pub struct ClientConfig {
    pub rate_limit_per_second: u64,
}

/// Custom [Deserializer] function to deserialize an input [str] to a [f64].
pub fn de_str_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error> where D: Deserializer<'de> {
    let input_str: &str = Deserialize::deserialize(deserializer)?;
    f64::from_str(input_str).map_err(de::Error::custom)
}