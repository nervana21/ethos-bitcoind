//! Generated version-specific RPC response types
//!
//! Generated for Bitcoin Core v30.2
//!
//! These types are version-specific and may not match other versions.
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Response for the `AbandonTransaction` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AbandonTransactionResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for AbandonTransactionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = AbandonTransactionResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbandonTransactionResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbandonTransactionResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbandonTransactionResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbandonTransactionResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbandonTransactionResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbandonTransactionResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbandonTransactionResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(AbandonTransactionResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for AbandonTransactionResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for AbandonTransactionResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for AbandonTransactionResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for AbandonTransactionResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<AbandonTransactionResponse> for () {
    fn from(wrapper: AbandonTransactionResponse) -> Self { wrapper.value }
}

/// Response for the `AbortRescan` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AbortRescanResponse {
    /// Wrapped primitive value
    pub value: bool,
}

impl<'de> serde::Deserialize<'de> for AbortRescanResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = AbortRescanResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbortRescanResponse { value: v != 0 })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbortRescanResponse { value: v != 0 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbortRescanResponse { value: v != 0.0 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<bool>().map_err(de::Error::custom)?;
                Ok(AbortRescanResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AbortRescanResponse { value: v })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(AbortRescanResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for AbortRescanResponse {
    type Target = bool;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for AbortRescanResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<bool> for AbortRescanResponse {
    fn as_ref(&self) -> &bool { &self.value }
}

impl From<bool> for AbortRescanResponse {
    fn from(value: bool) -> Self { Self { value } }
}

impl From<AbortRescanResponse> for bool {
    fn from(wrapper: AbortRescanResponse) -> Self { wrapper.value }
}

/// Response for the `AddConnection` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AddConnectionResponse {
    /// Address of newly added connection.
    pub address: String,
    /// Type of connection opened.
    pub connection_type: String,
}

/// Response for the `AddNode` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AddNodeResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for AddNodeResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = AddNodeResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AddNodeResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AddNodeResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AddNodeResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AddNodeResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AddNodeResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AddNodeResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AddNodeResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(AddNodeResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for AddNodeResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for AddNodeResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for AddNodeResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for AddNodeResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<AddNodeResponse> for () {
    fn from(wrapper: AddNodeResponse) -> Self { wrapper.value }
}

/// Response for the `AddPeerAddress` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AddPeerAddressResponse {
    /// error description, if the address could not be added
    pub error: Option<String>,
    /// whether the peer address was successfully added to the address manager table
    pub success: bool,
}

/// Response for the `AnalyzePsbt` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AnalyzePsbtResponse {
    /// Error message (if there is one)
    pub error: Option<String>,
    /// Estimated feerate of the final signed transaction in BTC/kvB. Shown only if all UTXO slots in the PSBT have been filled
    pub estimated_feerate: Option<f64>,
    /// Estimated vsize of the final signed transaction
    pub estimated_vsize: Option<u64>,
    /// The transaction fee paid. Shown only if all UTXO slots in the PSBT have been filled
    #[serde(default)]
    #[serde(deserialize_with = "option_amount_from_btc_float")]
    pub fee: Option<bitcoin::Amount>,
    pub inputs: Option<serde_json::Value>,
    /// Role of the next person that this psbt needs to go to
    pub next: String,
}

/// Response for the `BackupWallet` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BackupWalletResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for BackupWalletResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = BackupWalletResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(BackupWalletResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(BackupWalletResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(BackupWalletResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(BackupWalletResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(BackupWalletResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(BackupWalletResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(BackupWalletResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(BackupWalletResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for BackupWalletResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for BackupWalletResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for BackupWalletResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for BackupWalletResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<BackupWalletResponse> for () {
    fn from(wrapper: BackupWalletResponse) -> Self { wrapper.value }
}

/// Response for the `BumpFee` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct BumpFeeResponse {
    /// Errors encountered during processing (may be empty).
    pub errors: serde_json::Value,
    /// The fee of the new transaction.
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub fee: bitcoin::Amount,
    /// The fee of the replaced transaction.
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub origfee: bitcoin::Amount,
    /// The id of the new transaction.
    pub txid: bitcoin::Txid,
}

/// Response for the `ClearBanned` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ClearBannedResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for ClearBannedResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = ClearBannedResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ClearBannedResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ClearBannedResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ClearBannedResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ClearBannedResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ClearBannedResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ClearBannedResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ClearBannedResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(ClearBannedResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for ClearBannedResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for ClearBannedResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for ClearBannedResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for ClearBannedResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<ClearBannedResponse> for () {
    fn from(wrapper: ClearBannedResponse) -> Self { wrapper.value }
}

/// Response for the `CombinePsbt` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CombinePsbtResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for CombinePsbtResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = CombinePsbtResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombinePsbtResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombinePsbtResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombinePsbtResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombinePsbtResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombinePsbtResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(CombinePsbtResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for CombinePsbtResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for CombinePsbtResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for CombinePsbtResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for CombinePsbtResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<CombinePsbtResponse> for String {
    fn from(wrapper: CombinePsbtResponse) -> Self { wrapper.value }
}

/// Response for the `CombineRawTransaction` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CombineRawTransactionResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for CombineRawTransactionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = CombineRawTransactionResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombineRawTransactionResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombineRawTransactionResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombineRawTransactionResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombineRawTransactionResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CombineRawTransactionResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(CombineRawTransactionResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for CombineRawTransactionResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for CombineRawTransactionResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for CombineRawTransactionResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for CombineRawTransactionResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<CombineRawTransactionResponse> for String {
    fn from(wrapper: CombineRawTransactionResponse) -> Self { wrapper.value }
}

/// Response for the `ConvertToPsbt` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ConvertToPsbtResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for ConvertToPsbtResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = ConvertToPsbtResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ConvertToPsbtResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ConvertToPsbtResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ConvertToPsbtResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ConvertToPsbtResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ConvertToPsbtResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(ConvertToPsbtResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for ConvertToPsbtResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for ConvertToPsbtResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for ConvertToPsbtResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for ConvertToPsbtResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<ConvertToPsbtResponse> for String {
    fn from(wrapper: ConvertToPsbtResponse) -> Self { wrapper.value }
}

/// Response for the `CreateMultisig` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreateMultisigResponse {
    /// The value of the new multisig address.
    pub address: String,
    /// The descriptor for this multisig
    pub descriptor: String,
    /// The string value of the hex-encoded redemption script.
    pub redeemScript: bitcoin::ScriptBuf,
    /// Any warnings resulting from the creation of this multisig
    pub warnings: Option<serde_json::Value>,
}

/// Response for the `CreatePsbt` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CreatePsbtResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for CreatePsbtResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = CreatePsbtResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreatePsbtResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreatePsbtResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreatePsbtResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreatePsbtResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreatePsbtResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(CreatePsbtResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for CreatePsbtResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for CreatePsbtResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for CreatePsbtResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for CreatePsbtResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<CreatePsbtResponse> for String {
    fn from(wrapper: CreatePsbtResponse) -> Self { wrapper.value }
}

/// Response for the `CreateRawTransaction` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CreateRawTransactionResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for CreateRawTransactionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = CreateRawTransactionResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreateRawTransactionResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreateRawTransactionResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreateRawTransactionResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreateRawTransactionResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(CreateRawTransactionResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(CreateRawTransactionResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for CreateRawTransactionResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for CreateRawTransactionResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for CreateRawTransactionResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for CreateRawTransactionResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<CreateRawTransactionResponse> for String {
    fn from(wrapper: CreateRawTransactionResponse) -> Self { wrapper.value }
}

/// Response for the `CreateWallet` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreateWalletResponse {
    /// The wallet name if created successfully. If the wallet was created using a full path, the wallet_name will be the full path.
    pub name: String,
    /// Warning messages, if any, related to creating and loading the wallet.
    pub warnings: Option<serde_json::Value>,
}

/// Response for the `CreateWalletDescriptor` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreateWalletDescriptorResponse {
    /// The public descriptors that were added to the wallet
    pub descs: serde_json::Value,
}

/// Response for the `DecodePsbt` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtResponse {
    /// The transaction fee paid if all UTXOs slots in the PSBT have been filled.
    #[serde(default)]
    #[serde(deserialize_with = "option_amount_from_btc_float")]
    pub fee: Option<bitcoin::Amount>,
    pub global_xpubs: serde_json::Value,
    pub inputs: serde_json::Value,
    pub outputs: serde_json::Value,
    /// The global proprietary map
    pub proprietary: serde_json::Value,
    /// The PSBT version number. Not to be confused with the unsigned transaction version
    pub psbt_version: u64,
    /// The decoded network-serialized unsigned transaction.
    pub tx: serde_json::Value,
    /// The unknown global fields
    pub unknown: serde_json::Value,
}

/// Response for the `DecodeRawTransaction` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeRawTransactionResponse {
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The lock time
    pub locktime: u64,
    /// The serialized transaction size
    pub size: u64,
    /// The transaction id
    pub txid: bitcoin::Txid,
    /// The version
    pub version: u32,
    pub vin: serde_json::Value,
    pub vout: serde_json::Value,
    /// The virtual transaction size (differs from size for witness transactions)
    pub vsize: u64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: u64,
}

/// Response for the `DecodeScript` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeScriptResponse {
    /// The Bitcoin address (only if a well-defined address exists)
    pub address: Option<String>,
    /// Disassembly of the script
    pub asm: String,
    /// Inferred descriptor for the script
    pub desc: String,
    /// address of P2SH script wrapping this redeem script (not returned for types that should not be wrapped)
    pub p2sh: Option<String>,
    /// Result of a witness output script wrapping this redeem script (not returned for types that should not be wrapped)
    pub segwit: Option<serde_json::Value>,
    /// The output type (e.g. nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Response for the `DeriveAddresses` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DeriveAddressesResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for DeriveAddressesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for DeriveAddressesResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<DeriveAddressesResponse> for Vec<serde_json::Value> {
    fn from(wrapper: DeriveAddressesResponse) -> Self { wrapper.value }
}

/// Response for the `DescriptorProcessPsbt` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DescriptorProcessPsbtResponse {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if complete
    pub hex: Option<String>,
    /// The base64-encoded partially signed transaction
    pub psbt: String,
}

/// Response for the `DisconnectNode` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DisconnectNodeResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for DisconnectNodeResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = DisconnectNodeResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DisconnectNodeResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DisconnectNodeResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DisconnectNodeResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DisconnectNodeResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DisconnectNodeResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DisconnectNodeResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DisconnectNodeResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(DisconnectNodeResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for DisconnectNodeResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for DisconnectNodeResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for DisconnectNodeResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for DisconnectNodeResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<DisconnectNodeResponse> for () {
    fn from(wrapper: DisconnectNodeResponse) -> Self { wrapper.value }
}

/// Response for the `DumpTxOutSet` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DumpTxOutSetResponse {
    /// the hash of the base of the snapshot
    pub base_hash: String,
    /// the height of the base of the snapshot
    pub base_height: u64,
    /// the number of coins written in the snapshot
    pub coins_written: u64,
    /// the number of transactions in the chain up to and including the base block
    pub nchaintx: u64,
    /// the absolute path that the snapshot was written to
    pub path: String,
    /// the hash of the UTXO set contents
    pub txoutset_hash: String,
}

/// Response for the `Echo` RPC method
///
/// This method returns arbitrary JSON (e.g. string, object, array) as a single value.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EchoResponse {
    /// Wrapped JSON value
    pub value: serde_json::Value,
}

impl<'de> serde::Deserialize<'de> for EchoResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<serde_json::Value> for EchoResponse {
    fn from(value: serde_json::Value) -> Self { Self { value } }
}

/// Response for the `Echoipc` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EchoipcResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for EchoipcResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = EchoipcResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EchoipcResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EchoipcResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EchoipcResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EchoipcResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EchoipcResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(EchoipcResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for EchoipcResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for EchoipcResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for EchoipcResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for EchoipcResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<EchoipcResponse> for String {
    fn from(wrapper: EchoipcResponse) -> Self { wrapper.value }
}

/// Response for the `Echojson` RPC method
///
/// This method returns arbitrary JSON (e.g. string, object, array) as a single value.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EchojsonResponse {
    /// Wrapped JSON value
    pub value: serde_json::Value,
}

impl<'de> serde::Deserialize<'de> for EchojsonResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<serde_json::Value> for EchojsonResponse {
    fn from(value: serde_json::Value) -> Self { Self { value } }
}

/// Response for the `EncryptWallet` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EncryptWalletResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for EncryptWalletResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = EncryptWalletResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EncryptWalletResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EncryptWalletResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EncryptWalletResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EncryptWalletResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EncryptWalletResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(EncryptWalletResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for EncryptWalletResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for EncryptWalletResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for EncryptWalletResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for EncryptWalletResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<EncryptWalletResponse> for String {
    fn from(wrapper: EncryptWalletResponse) -> Self { wrapper.value }
}

/// Response for the `EnumerateSigners` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EnumerateSignersResponse {
    pub signers: serde_json::Value,
}

/// Response for the `EstimateRawFee` RPC method
///
/// Results are returned for any horizon which tracks blocks up to the confirmation target
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EstimateRawFeeResponse {
    /// estimate for long time horizon
    pub long: Option<serde_json::Value>,
    /// estimate for medium time horizon
    pub medium: Option<serde_json::Value>,
    /// estimate for short time horizon
    pub short: Option<serde_json::Value>,
}
impl<'de> serde::Deserialize<'de> for EstimateRawFeeResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct ConditionalResponseVisitor;

        #[allow(clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for ConditionalResponseVisitor {
            type Value = EstimateRawFeeResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or object")
            }

            fn visit_str<E>(self, _v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(EstimateRawFeeResponse { long: None, medium: None, short: None })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut long = None;
                let mut medium = None;
                let mut short = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "long" {
                        if long.is_some() {
                            return Err(de::Error::duplicate_field("long"));
                        }
                        long = Some(map.next_value::<serde_json::Value>()?);
                    }
                    if key == "medium" {
                        if medium.is_some() {
                            return Err(de::Error::duplicate_field("medium"));
                        }
                        medium = Some(map.next_value::<serde_json::Value>()?);
                    }
                    if key == "short" {
                        if short.is_some() {
                            return Err(de::Error::duplicate_field("short"));
                        }
                        short = Some(map.next_value::<serde_json::Value>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                Ok(EstimateRawFeeResponse { long, medium, short })
            }
        }

        deserializer.deserialize_any(ConditionalResponseVisitor)
    }
}

/// Response for the `EstimateSmartFee` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateSmartFeeResponse {
    /// block number where estimate was found
    /// The request target will be clamped between 2 and the highest target
    /// fee estimation is able to return based on how long it has been running.
    /// An error is returned if not enough transactions and blocks
    /// have been observed to make an estimate for any number of blocks.
    pub blocks: u64,
    /// Errors encountered during processing (if there are any)
    pub errors: Option<serde_json::Value>,
    /// estimate fee rate in BTC/kvB (only present if no errors were encountered)
    pub feerate: Option<f64>,
}

/// Response for the `FinalizePsbt` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FinalizePsbtResponse {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if extracted
    pub hex: Option<String>,
    /// The base64-encoded partially signed transaction if not extracted
    pub psbt: Option<String>,
}

/// Response for the `FundRawTransaction` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FundRawTransactionResponse {
    /// The position of the added change output, or -1
    pub changepos: i64,
    /// Fee in BTC the resulting transaction pays
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub fee: bitcoin::Amount,
    /// The resulting raw transaction (hex-encoded string)
    pub hex: String,
}

/// Response for the `Generate` RPC method
///
/// This method returns no meaningful data.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GenerateResponse;

/// Response for the `GenerateBlock` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GenerateBlockResponse {
    /// hash of generated block
    pub hash: String,
    /// hex of generated block, only present when submit=false
    pub hex: Option<String>,
}

/// Response for the `GenerateToAddress` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GenerateToAddressResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for GenerateToAddressResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for GenerateToAddressResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<GenerateToAddressResponse> for Vec<serde_json::Value> {
    fn from(wrapper: GenerateToAddressResponse) -> Self { wrapper.value }
}

/// Response for the `GenerateToDescriptor` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GenerateToDescriptorResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for GenerateToDescriptorResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for GenerateToDescriptorResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<GenerateToDescriptorResponse> for Vec<serde_json::Value> {
    fn from(wrapper: GenerateToDescriptorResponse) -> Self { wrapper.value }
}

/// Response for the `GetAddedNodeInfo` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetAddedNodeInfoResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for GetAddedNodeInfoResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for GetAddedNodeInfoResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<GetAddedNodeInfoResponse> for Vec<serde_json::Value> {
    fn from(wrapper: GetAddedNodeInfoResponse) -> Self { wrapper.value }
}

/// Response for the `GetAddressesByLabel` RPC method
///
/// json object with addresses as keys
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddressesByLabelResponse {
    /// json object with information about address
    pub address: serde_json::Value,
}

/// Response for the `GetAddressInfo` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddressInfoResponse {
    /// The bitcoin address validated.
    pub address: String,
    /// A descriptor for spending coins sent to this address (only when solvable).
    pub desc: Option<String>,
    /// Information about the address embedded in P2SH or P2WSH, if relevant and known.
    pub embedded: Option<serde_json::Value>,
    /// The HD keypath, if the key is HD and available.
    pub hdkeypath: Option<String>,
    /// The fingerprint of the master key.
    pub hdmasterfingerprint: Option<String>,
    /// The Hash160 of the HD seed.
    pub hdseedid: Option<String>,
    /// The redeemscript for the p2sh address.
    pub hex: Option<String>,
    /// If the address was used for change output.
    pub ischange: bool,
    /// If the pubkey is compressed.
    pub iscompressed: Option<bool>,
    /// If the address is yours.
    pub ismine: bool,
    /// If the key is a script.
    pub isscript: Option<bool>,
    /// (DEPRECATED) Always false.
    pub iswatchonly: bool,
    /// If the address is a witness address.
    pub iswitness: bool,
    /// Array of labels associated with the address. Currently limited to one label but returned
    /// as an array to keep the API stable if multiple labels are enabled in the future.
    pub labels: serde_json::Value,
    /// The descriptor used to derive this address if this is a descriptor wallet
    pub parent_desc: Option<String>,
    /// The hex value of the raw public key for single-key addresses (possibly embedded in P2SH or P2WSH).
    pub pubkey: Option<String>,
    /// Array of pubkeys associated with the known redeemscript (only if script is multisig).
    pub pubkeys: Option<serde_json::Value>,
    /// The output script type. Only if isscript is true and the redeemscript is known. Possible
    /// types: nonstandard, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_keyhash,
    /// witness_v0_scripthash, witness_unknown.
    pub script: Option<bitcoin::ScriptBuf>,
    /// The hex-encoded output script generated by the address.
    pub scriptPubKey: bitcoin::ScriptBuf,
    /// The number of signatures required to spend multisig output (only if script is multisig).
    pub sigsrequired: Option<u64>,
    /// If we know how to spend coins sent to this address, ignoring the possible lack of private keys.
    pub solvable: bool,
    /// The creation time of the key, if available, expressed in UNIX epoch time.
    pub timestamp: Option<u64>,
    /// The hex value of the witness program.
    pub witness_program: Option<String>,
    /// The version number of the witness program.
    pub witness_version: Option<u64>,
}

/// Response for the `GetAddrManInfo` RPC method
///
/// json object with network type as keys
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAddrManInfoResponse {
    /// the network (ipv4, ipv6, onion, i2p, cjdns, all_networks)
    #[serde(default)]
    pub network: Option<serde_json::Value>,
}

/// Response for the `GetBalance` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetBalanceResponse {
    /// Wrapped primitive value
    pub value: bitcoin::Amount,
}

impl<'de> serde::Deserialize<'de> for GetBalanceResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetBalanceResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBalanceResponse { value: bitcoin::Amount::from_sat(v) })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v < 0 {
                    return Err(de::Error::custom(format!("Amount cannot be negative: {}", v)));
                }
                Ok(GetBalanceResponse { value: bitcoin::Amount::from_sat(v as u64) })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let amount = bitcoin::Amount::from_btc(v)
                    .map_err(|e| de::Error::custom(format!("Invalid BTC amount: {}", e)))?;
                Ok(GetBalanceResponse { value: amount })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<bitcoin::Amount>().map_err(de::Error::custom)?;
                Ok(GetBalanceResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Err(de::Error::custom("cannot convert bool to bitcoin::Amount"))
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetBalanceResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetBalanceResponse {
    type Target = bitcoin::Amount;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetBalanceResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<bitcoin::Amount> for GetBalanceResponse {
    fn as_ref(&self) -> &bitcoin::Amount { &self.value }
}

impl From<bitcoin::Amount> for GetBalanceResponse {
    fn from(value: bitcoin::Amount) -> Self { Self { value } }
}

impl From<GetBalanceResponse> for bitcoin::Amount {
    fn from(wrapper: GetBalanceResponse) -> Self { wrapper.value }
}

/// Response for the `GetBalances` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBalancesResponse {
    /// hash and height of the block this information was generated on
    pub lastprocessedblock: serde_json::Value,
    /// balances from outputs that the wallet can sign
    pub mine: serde_json::Value,
}

/// Response for the `GetBestBlockHash` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetBestBlockHashResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for GetBestBlockHashResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetBestBlockHashResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBestBlockHashResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBestBlockHashResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBestBlockHashResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBestBlockHashResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBestBlockHashResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetBestBlockHashResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetBestBlockHashResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetBestBlockHashResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for GetBestBlockHashResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for GetBestBlockHashResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<GetBestBlockHashResponse> for String {
    fn from(wrapper: GetBestBlockHashResponse) -> Self { wrapper.value }
}

/// Response for the `GetBlock` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockResponse {
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// Expected number of hashes required to produce the chain up to this block (in hex)
    pub chainwork: String,
    /// The number of confirmations, or -1 if the block is not on the main chain
    pub confirmations: i64,
    /// The difficulty
    pub difficulty: f64,
    /// the block hash (same as provided)
    pub hash: String,
    /// The block height or index
    pub height: u64,
    /// The median block time expressed in UNIX epoch time
    pub mediantime: u64,
    /// The merkle root
    pub merkleroot: String,
    /// The number of transactions in the block
    pub nTx: u64,
    /// The hash of the next block (if available)
    pub nextblockhash: Option<String>,
    /// The nonce
    pub nonce: u64,
    /// The hash of the previous block (if available)
    pub previousblockhash: Option<String>,
    /// The block size
    pub size: u64,
    /// The block size excluding witness data
    pub strippedsize: u64,
    /// The difficulty target
    pub target: String,
    /// The block time expressed in UNIX epoch time
    pub time: u64,
    /// The transaction ids
    pub tx: serde_json::Value,
    pub tx_1: serde_json::Value,
    pub tx_2: serde_json::Value,
    /// The block version
    pub version: u32,
    /// The block version formatted in hexadecimal
    pub versionHex: String,
    /// The block weight as defined in BIP 141
    pub weight: u64,
}

/// Response for the `GetBlockchainInfo` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockchainInfoResponse {
    /// whether automatic pruning is enabled (only present if pruning is enabled)
    pub automatic_pruning: Option<bool>,
    /// the hash of the currently best block
    pub bestblockhash: String,
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// the height of the most-work fully-validated chain. The genesis block has height 0
    pub blocks: u64,
    /// current network name (main, test, testnet4, signet, regtest)
    pub chain: String,
    /// total amount of work in active chain, in hexadecimal
    pub chainwork: String,
    /// the current difficulty
    pub difficulty: f64,
    /// the current number of headers we have validated
    pub headers: u64,
    /// (debug information) estimate of whether this node is in Initial Block Download mode
    pub initialblockdownload: bool,
    /// The median block time expressed in UNIX epoch time
    pub mediantime: u64,
    /// the target size used by pruning (only present if automatic pruning is enabled)
    pub prune_target_size: Option<u64>,
    /// if the blocks are subject to pruning
    pub pruned: bool,
    /// height of the last block pruned, plus one (only present if pruning is enabled)
    pub pruneheight: Option<u64>,
    /// the block challenge (aka. block script), in hexadecimal (only present if the current network is a signet)
    pub signet_challenge: Option<String>,
    /// the estimated size of the block and undo files on disk
    pub size_on_disk: u64,
    /// The difficulty target
    pub target: String,
    /// The block time expressed in UNIX epoch time
    pub time: u64,
    /// estimate of verification progress \[0..1\]
    pub verificationprogress: f64,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: serde_json::Value,
}

/// Response for the `GetBlockCount` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetBlockCountResponse {
    /// Wrapped primitive value
    pub value: u64,
}

impl<'de> serde::Deserialize<'de> for GetBlockCountResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetBlockCountResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockCountResponse { value: v })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockCountResponse { value: v as u64 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockCountResponse { value: v as u64 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<u64>().map_err(de::Error::custom)?;
                Ok(GetBlockCountResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockCountResponse { value: v as u64 })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetBlockCountResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetBlockCountResponse {
    type Target = u64;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetBlockCountResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<u64> for GetBlockCountResponse {
    fn as_ref(&self) -> &u64 { &self.value }
}

impl From<u64> for GetBlockCountResponse {
    fn from(value: u64) -> Self { Self { value } }
}

impl From<GetBlockCountResponse> for u64 {
    fn from(wrapper: GetBlockCountResponse) -> Self { wrapper.value }
}

/// Response for the `GetBlockFilter` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockFilterResponse {
    /// the hex-encoded filter data
    pub filter: String,
    /// the hex-encoded filter header
    pub header: String,
}

/// Response for the `GetBlockFromPeer` RPC method
///
/// This method returns no meaningful data.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GetBlockFromPeerResponse;

/// Response for the `GetBlockHash` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetBlockHashResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for GetBlockHashResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetBlockHashResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockHashResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockHashResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockHashResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockHashResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetBlockHashResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetBlockHashResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetBlockHashResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetBlockHashResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for GetBlockHashResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for GetBlockHashResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<GetBlockHashResponse> for String {
    fn from(wrapper: GetBlockHashResponse) -> Self { wrapper.value }
}

/// Response for the `GetBlockHeader` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockHeaderResponse {
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// Expected number of hashes required to produce the current chain
    pub chainwork: String,
    /// The number of confirmations, or -1 if the block is not on the main chain
    pub confirmations: i64,
    /// The difficulty
    pub difficulty: f64,
    /// the block hash (same as provided)
    pub hash: String,
    /// The block height or index
    pub height: u64,
    /// The median block time expressed in UNIX epoch time
    pub mediantime: u64,
    /// The merkle root
    pub merkleroot: String,
    /// The number of transactions in the block
    pub nTx: u64,
    /// The hash of the next block (if available)
    pub nextblockhash: Option<String>,
    /// The nonce
    pub nonce: u64,
    /// The hash of the previous block (if available)
    pub previousblockhash: Option<String>,
    /// The difficulty target
    pub target: String,
    /// The block time expressed in UNIX epoch time
    pub time: u64,
    /// The block version
    pub version: u32,
    /// The block version formatted in hexadecimal
    pub versionHex: String,
}

/// Response for the `GetBlockStats` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockStatsResponse {
    /// Average fee in the block
    pub avgfee: Option<u64>,
    /// Average feerate (in satoshis per virtual byte)
    pub avgfeerate: Option<u64>,
    /// Average transaction size
    pub avgtxsize: Option<u64>,
    /// The block hash (to check for potential reorgs)
    pub blockhash: Option<bitcoin::BlockHash>,
    /// Feerates at the 10th, 25th, 50th, 75th, and 90th percentile weight unit (in satoshis per virtual byte)
    pub feerate_percentiles: Option<serde_json::Value>,
    /// The height of the block
    pub height: Option<u64>,
    /// The number of inputs (excluding coinbase)
    pub ins: Option<u64>,
    /// Maximum fee in the block
    pub maxfee: Option<u64>,
    /// Maximum feerate (in satoshis per virtual byte)
    pub maxfeerate: Option<f64>,
    /// Maximum transaction size
    pub maxtxsize: Option<u64>,
    /// Truncated median fee in the block
    pub medianfee: Option<u64>,
    /// The block median time past
    pub mediantime: Option<u64>,
    /// Truncated median transaction size
    pub mediantxsize: Option<u64>,
    /// Minimum fee in the block
    pub minfee: Option<u64>,
    /// Minimum feerate (in satoshis per virtual byte)
    pub minfeerate: Option<u64>,
    /// Minimum transaction size
    pub mintxsize: Option<u64>,
    /// The number of outputs
    pub outs: Option<u64>,
    /// The block subsidy
    pub subsidy: Option<u64>,
    /// Total size of all segwit transactions
    pub swtotal_size: Option<u64>,
    /// Total weight of all segwit transactions
    pub swtotal_weight: Option<u64>,
    /// The number of segwit transactions
    pub swtxs: Option<u64>,
    /// The block time
    pub time: Option<u64>,
    /// Total amount in all outputs (excluding coinbase and thus reward \[ie subsidy + totalfee\])
    pub total_out: Option<u64>,
    /// Total size of all non-coinbase transactions
    pub total_size: Option<u64>,
    /// Total weight of all non-coinbase transactions
    pub total_weight: Option<u64>,
    /// The fee total
    pub totalfee: Option<u64>,
    /// The number of transactions (including coinbase)
    pub txs: Option<u64>,
    /// The increase/decrease in the number of unspent outputs (not discounting op_return and similar)
    pub utxo_increase: Option<u64>,
    /// The increase/decrease in the number of unspent outputs, not counting unspendables
    pub utxo_increase_actual: Option<u64>,
    /// The increase/decrease in size for the utxo index (not discounting op_return and similar)
    pub utxo_size_inc: Option<u64>,
    /// The increase/decrease in size for the utxo index, not counting unspendables
    pub utxo_size_inc_actual: Option<u64>,
}

/// Response for the `GetBlockTemplate` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockTemplateResponse {
    /// compressed target of next block
    pub bits: String,
    pub capabilities: serde_json::Value,
    /// data that should be included in the coinbase's scriptSig content
    pub coinbaseaux: serde_json::Value,
    /// maximum allowable input to coinbase transaction, including the generation award and transaction fees (in satoshis)
    pub coinbasevalue: u64,
    /// current timestamp in UNIX epoch time. Adjusted for the proposed BIP94 timewarp rule.
    pub curtime: u64,
    /// a valid witness commitment for the unmodified block template
    pub default_witness_commitment: Option<String>,
    #[serde(default)]
    pub field_0: Option<()>,
    /// The height of the next block
    pub height: u64,
    /// an id to include with a request to longpoll on an update to this template
    pub longpollid: String,
    /// The minimum timestamp appropriate for the next block time, expressed in UNIX epoch time. Adjusted for the proposed BIP94 timewarp rule.
    pub mintime: u64,
    /// list of ways the block template may be changed
    pub mutable: serde_json::Value,
    /// A range of valid nonces
    pub noncerange: String,
    /// The hash of current highest block
    pub previousblockhash: String,
    /// specific block rules that are to be enforced
    pub rules: serde_json::Value,
    /// Only on signet
    pub signet_challenge: Option<String>,
    /// limit of sigops in blocks
    pub sigoplimit: u64,
    /// limit of block size
    pub sizelimit: u64,
    /// The hash target
    pub target: String,
    /// contents of non-coinbase transactions that should be included in the next block
    pub transactions: serde_json::Value,
    /// set of pending, supported versionbit (BIP 9) softfork deployments
    pub vbavailable: serde_json::Value,
    /// bit mask of versionbits the server requires set in submissions
    pub vbrequired: u64,
    /// The preferred block version
    pub version: u32,
    /// limit of block weight
    pub weightlimit: Option<u64>,
}

/// Response for the `GetChainStates` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainStatesResponse {
    /// list of the chainstates ordered by work, with the most-work (active) chainstate last
    pub chainstates: serde_json::Value,
    /// the number of headers seen so far
    pub headers: u64,
}

/// Response for the `GetChainTips` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainTipsResponse {
    pub field: serde_json::Value,
}

/// Response for the `GetChainTxStats` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainTxStatsResponse {
    /// The timestamp for the final block in the window, expressed in UNIX epoch time
    pub time: u64,
    /// The total number of transactions in the chain up to that point, if known. It may be unknown when using assumeutxo.
    pub txcount: Option<u64>,
    /// The average rate of transactions per second in the window. Only returned if "window_interval" is &gt; 0 and if window_tx_count exists.
    pub txrate: Option<u64>,
    /// Size of the window in number of blocks
    pub window_block_count: u64,
    /// The hash of the final block in the window
    pub window_final_block_hash: String,
    /// The height of the final block in the window.
    pub window_final_block_height: u64,
    /// The elapsed time in the window in seconds. Only returned if "window_block_count" is &gt; 0
    pub window_interval: Option<u64>,
    /// The number of transactions in the window. Only returned if "window_block_count" is &gt; 0 and if txcount exists for the start and end of the window.
    pub window_tx_count: Option<u64>,
}

/// Response for the `GetConnectionCount` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetConnectionCountResponse {
    /// Wrapped primitive value
    pub value: u64,
}

impl<'de> serde::Deserialize<'de> for GetConnectionCountResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetConnectionCountResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetConnectionCountResponse { value: v })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetConnectionCountResponse { value: v as u64 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetConnectionCountResponse { value: v as u64 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<u64>().map_err(de::Error::custom)?;
                Ok(GetConnectionCountResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetConnectionCountResponse { value: v as u64 })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetConnectionCountResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetConnectionCountResponse {
    type Target = u64;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetConnectionCountResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<u64> for GetConnectionCountResponse {
    fn as_ref(&self) -> &u64 { &self.value }
}

impl From<u64> for GetConnectionCountResponse {
    fn from(value: u64) -> Self { Self { value } }
}

impl From<GetConnectionCountResponse> for u64 {
    fn from(wrapper: GetConnectionCountResponse) -> Self { wrapper.value }
}

/// Response for the `GetDeploymentInfo` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDeploymentInfoResponse {
    pub deployments: serde_json::Value,
    /// requested block hash (or tip)
    pub hash: String,
    /// requested block height (or tip)
    pub height: u64,
    /// script verify flags for the block
    pub script_flags: serde_json::Value,
}

/// Response for the `GetDescriptorActivity` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorActivityResponse {
    /// events
    pub activity: serde_json::Value,
}

/// Response for the `GetDescriptorInfo` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorInfoResponse {
    /// The checksum for the input descriptor
    pub checksum: String,
    /// The descriptor in canonical form, without private keys. For a multipath descriptor, only the first will be returned.
    pub descriptor: String,
    /// Whether the input descriptor contained at least one private key
    pub hasprivatekeys: bool,
    /// Whether the descriptor is ranged
    pub isrange: bool,
    /// Whether the descriptor is solvable
    pub issolvable: bool,
    /// All descriptors produced by expanding multipath derivation elements. Only if the provided descriptor specifies multipath derivation elements.
    pub multipath_expansion: Option<serde_json::Value>,
}

/// Response for the `GetDifficulty` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetDifficultyResponse {
    /// Wrapped primitive value
    pub value: u64,
}

impl<'de> serde::Deserialize<'de> for GetDifficultyResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetDifficultyResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetDifficultyResponse { value: v })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetDifficultyResponse { value: v as u64 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetDifficultyResponse { value: v as u64 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<u64>().map_err(de::Error::custom)?;
                Ok(GetDifficultyResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetDifficultyResponse { value: v as u64 })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetDifficultyResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetDifficultyResponse {
    type Target = u64;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetDifficultyResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<u64> for GetDifficultyResponse {
    fn as_ref(&self) -> &u64 { &self.value }
}

impl From<u64> for GetDifficultyResponse {
    fn from(value: u64) -> Self { Self { value } }
}

impl From<GetDifficultyResponse> for u64 {
    fn from(wrapper: GetDifficultyResponse) -> Self { wrapper.value }
}

/// Response for the `GetHdKeys` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetHdKeysResponse {
    pub field: serde_json::Value,
}

/// Response for the `GetIndexInfo` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetIndexInfoResponse {
    /// The name of the index
    #[serde(default)]
    pub name: Option<serde_json::Value>,
}

/// Response for the `GetMemoryInfo` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMemoryInfoResponse {
    /// Information about locked memory manager
    pub locked: serde_json::Value,
}

/// Response for the `GetMempoolAncestors` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolAncestorsResponse {
    pub field_0: serde_json::Value,
    pub transactionid: serde_json::Value,
}

/// Response for the `GetMempoolCluster` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolClusterResponse {
    /// chunks in this cluster (in mining order)
    pub chunks: serde_json::Value,
    /// total sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop')
    pub clusterweight: u64,
    /// number of transactions
    pub txcount: u64,
}

/// Response for the `GetMempoolDescendants` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolDescendantsResponse {
    pub field_0: serde_json::Value,
    pub transactionid: serde_json::Value,
}

/// Response for the `GetMempoolEntry` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolEntryResponse {
    /// number of in-mempool ancestor transactions (including this one)
    pub ancestorcount: u64,
    /// virtual transaction size of in-mempool ancestors (including this one)
    pub ancestorsize: u64,
    /// Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability. (DEPRECATED)
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: bool,
    /// sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop') of this transaction's chunk
    pub chunkweight: u64,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: serde_json::Value,
    /// number of in-mempool descendant transactions (including this one)
    pub descendantcount: u64,
    /// virtual transaction size of in-mempool descendants (including this one)
    pub descendantsize: u64,
    pub fees: serde_json::Value,
    /// block height when transaction entered pool
    pub height: u64,
    /// unconfirmed transactions spending outputs from this transaction
    pub spentby: serde_json::Value,
    /// local time transaction entered pool in seconds since 1 Jan 1970 GMT
    pub time: u64,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
    pub unbroadcast: bool,
    /// virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize: u64,
    /// transaction weight as defined in BIP 141.
    pub weight: u64,
    /// hash of serialized transaction, including witness data
    pub wtxid: String,
}

/// Response for the `GetMempoolFeeRateDiagram` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolFeeRateDiagramResponse {
    pub field: serde_json::Value,
}

/// Response for the `GetMempoolInfo` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolInfoResponse {
    /// Sum of all virtual transaction sizes as defined in BIP 141. Differs from actual serialized size because witness data is discounted
    pub bytes: u64,
    /// True if the mempool accepts RBF without replaceability signaling inspection (DEPRECATED)
    pub fullrbf: bool,
    /// minimum fee rate increment for mempool limiting or replacement in BTC/kvB
    pub incrementalrelayfee: f64,
    /// Maximum number of transactions that can be in a cluster (configured by -limitclustercount)
    pub limitclustercount: Option<u64>,
    /// Maximum size of a cluster in virtual bytes (configured by -limitclustersize)
    pub limitclustersize: Option<u64>,
    /// True if the initial load attempt of the persisted mempool finished
    pub loaded: bool,
    /// Maximum number of bytes that can be used by OP_RETURN outputs in the mempool
    pub maxdatacarriersize: Option<u64>,
    /// Maximum memory usage for the mempool
    pub maxmempool: u64,
    /// Minimum fee rate in BTC/kvB for tx to be accepted. Is the maximum of minrelaytxfee and minimum mempool fee
    pub mempoolminfee: f64,
    /// Current minimum relay fee for transactions
    pub minrelaytxfee: f64,
    /// True if the mempool accepts transactions with bare multisig outputs
    pub permitbaremultisig: Option<bool>,
    /// Current tx count
    pub size: u64,
    /// Total fees for the mempool in BTC, ignoring modified fees through prioritisetransaction
    pub total_fee: f64,
    /// Current number of transactions that haven't passed initial broadcast yet
    pub unbroadcastcount: u64,
    /// Total memory usage for the mempool
    pub usage: u64,
}

/// Response for the `GetMiningInfo` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMiningInfoResponse {
    /// The current nBits, compact representation of the block difficulty target
    pub bits: String,
    /// Minimum feerate of packages selected for block inclusion in BTC/kvB
    pub blockmintxfee: Option<f64>,
    /// The current block
    pub blocks: u64,
    /// current network name (main, test, testnet4, signet, regtest)
    pub chain: String,
    /// The number of block transactions (excluding coinbase) of the last assembled block (only present if a block was ever assembled)
    pub currentblocktx: Option<u64>,
    /// The block weight (including reserved weight for block header, txs count and coinbase tx) of the last assembled block (only present if a block was ever assembled)
    pub currentblockweight: Option<u64>,
    /// The current difficulty
    pub difficulty: f64,
    /// The network hashes per second
    pub networkhashps: f64,
    /// The next block
    pub next: serde_json::Value,
    /// The size of the mempool
    pub pooledtx: u64,
    /// The block challenge (aka. block script), in hexadecimal (only present if the current network is a signet)
    pub signet_challenge: Option<String>,
    /// The current target
    pub target: String,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: serde_json::Value,
}

/// Response for the `GetNetTotals` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetTotalsResponse {
    /// Current system UNIX epoch time in milliseconds
    pub timemillis: u64,
    /// Total bytes received
    pub totalbytesrecv: u64,
    /// Total bytes sent
    pub totalbytessent: u64,
    pub uploadtarget: serde_json::Value,
}

/// Response for the `GetNetworkHashPs` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetNetworkHashPsResponse {
    /// Wrapped primitive value
    pub value: u64,
}

impl<'de> serde::Deserialize<'de> for GetNetworkHashPsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetNetworkHashPsResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNetworkHashPsResponse { value: v })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNetworkHashPsResponse { value: v as u64 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNetworkHashPsResponse { value: v as u64 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<u64>().map_err(de::Error::custom)?;
                Ok(GetNetworkHashPsResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNetworkHashPsResponse { value: v as u64 })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetNetworkHashPsResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetNetworkHashPsResponse {
    type Target = u64;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetNetworkHashPsResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<u64> for GetNetworkHashPsResponse {
    fn as_ref(&self) -> &u64 { &self.value }
}

impl From<u64> for GetNetworkHashPsResponse {
    fn from(value: u64) -> Self { Self { value } }
}

impl From<GetNetworkHashPsResponse> for u64 {
    fn from(wrapper: GetNetworkHashPsResponse) -> Self { wrapper.value }
}

/// Response for the `GetNetworkInfo` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetNetworkInfoResponse {
    /// the total number of connections
    pub connections: u64,
    /// the number of inbound connections
    pub connections_in: u64,
    /// the number of outbound connections
    pub connections_out: u64,
    /// minimum fee rate increment for mempool limiting or replacement in BTC/kvB
    pub incrementalfee: f64,
    /// list of local addresses
    pub localaddresses: serde_json::Value,
    /// true if transaction relay is requested from peers
    pub localrelay: bool,
    /// the services we offer to the network
    pub localservices: String,
    /// the services we offer to the network, in human-readable form
    pub localservicesnames: serde_json::Value,
    /// whether p2p networking is enabled
    pub networkactive: bool,
    /// information per network
    pub networks: serde_json::Value,
    /// the protocol version
    pub protocolversion: u64,
    /// minimum relay fee rate for transactions in BTC/kvB
    pub relayfee: f64,
    /// the server subversion string
    pub subversion: String,
    /// the time offset
    pub timeoffset: u64,
    /// the server version
    pub version: u32,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: serde_json::Value,
}

/// Response for the `GetNewAddress` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetNewAddressResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for GetNewAddressResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetNewAddressResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNewAddressResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNewAddressResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNewAddressResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNewAddressResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetNewAddressResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetNewAddressResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetNewAddressResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetNewAddressResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for GetNewAddressResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for GetNewAddressResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<GetNewAddressResponse> for String {
    fn from(wrapper: GetNewAddressResponse) -> Self { wrapper.value }
}

/// Response for the `GetNodeAddresses` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetNodeAddressesResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for GetNodeAddressesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for GetNodeAddressesResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<GetNodeAddressesResponse> for Vec<serde_json::Value> {
    fn from(wrapper: GetNodeAddressesResponse) -> Self { wrapper.value }
}

/// Response for the `GetOrphanTxs` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetOrphanTxsResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for GetOrphanTxsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for GetOrphanTxsResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<GetOrphanTxsResponse> for Vec<serde_json::Value> {
    fn from(wrapper: GetOrphanTxsResponse) -> Self { wrapper.value }
}

/// Response for the `GetPeerInfo` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetPeerInfoResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for GetPeerInfoResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for GetPeerInfoResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<GetPeerInfoResponse> for Vec<serde_json::Value> {
    fn from(wrapper: GetPeerInfoResponse) -> Self { wrapper.value }
}

/// Response for the `GetPrioritisedTransactions` RPC method
///
/// prioritisation keyed by txid
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetPrioritisedTransactionsResponse {
    #[serde(rename = "<transactionid>")]
    pub transactionid: serde_json::Value,
}

/// Response for the `GetRawAddrMan` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawAddrManResponse {
    /// buckets with addresses in the address manager table ( new, tried )
    #[serde(default)]
    pub table: Option<serde_json::Value>,
}

/// Response for the `GetRawChangeAddress` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetRawChangeAddressResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for GetRawChangeAddressResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetRawChangeAddressResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetRawChangeAddressResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetRawChangeAddressResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetRawChangeAddressResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetRawChangeAddressResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetRawChangeAddressResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetRawChangeAddressResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetRawChangeAddressResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetRawChangeAddressResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for GetRawChangeAddressResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for GetRawChangeAddressResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<GetRawChangeAddressResponse> for String {
    fn from(wrapper: GetRawChangeAddressResponse) -> Self { wrapper.value }
}

/// Response for the `GetRawMempool` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetRawMempoolResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for GetRawMempoolResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for GetRawMempoolResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<GetRawMempoolResponse> for Vec<serde_json::Value> {
    fn from(wrapper: GetRawMempoolResponse) -> Self { wrapper.value }
}

/// Response for the `GetRawTransaction` RPC method
///
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetRawTransactionResponse {
    /// the block hash
    pub blockhash: Option<bitcoin::BlockHash>,
    /// The block time expressed in UNIX epoch time
    pub blocktime: Option<u64>,
    /// The confirmations
    pub confirmations: Option<i64>,
    /// The serialized transaction as a hex-encoded string for 'txid'
    pub data: Option<String>,
    /// transaction fee in BTC, omitted if block undo data is not available
    pub fee: Option<f64>,
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: Option<String>,
    /// The serialized, hex-encoded data for 'txid'
    pub hex: Option<String>,
    /// Whether specified block is in the active chain or not (only present with explicit "blockhash" argument)
    pub in_active_chain: Option<bool>,
    /// The lock time
    pub locktime: Option<u64>,
    /// The serialized transaction size
    pub size: Option<u64>,
    /// Same as "blocktime"
    pub time: Option<u64>,
    /// The transaction id (same as provided)
    pub txid: Option<bitcoin::Txid>,
    /// The version
    pub version: Option<u32>,
    pub vin: Option<serde_json::Value>,
    pub vin_1: Option<serde_json::Value>,
    pub vout: Option<serde_json::Value>,
    /// The virtual transaction size (differs from size for witness transactions)
    pub vsize: Option<u64>,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: Option<u64>,
}
impl<'de> serde::Deserialize<'de> for GetRawTransactionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct ConditionalResponseVisitor;

        #[allow(clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for ConditionalResponseVisitor {
            type Value = GetRawTransactionResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or object")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let data = v.to_string();
                Ok(GetRawTransactionResponse {
                    blockhash: None,
                    blocktime: None,
                    confirmations: None,
                    data: Some(data),
                    fee: None,
                    hash: None,
                    hex: None,
                    in_active_chain: None,
                    locktime: None,
                    size: None,
                    time: None,
                    txid: None,
                    version: None,
                    vin: None,
                    vin_1: None,
                    vout: None,
                    vsize: None,
                    weight: None,
                })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut blockhash = None;
                let mut blocktime = None;
                let mut confirmations = None;
                let mut data = None;
                let mut fee = None;
                let mut hash = None;
                let mut hex = None;
                let mut in_active_chain = None;
                let mut locktime = None;
                let mut size = None;
                let mut time = None;
                let mut txid = None;
                let mut version = None;
                let mut vin = None;
                let mut vin_1 = None;
                let mut vout = None;
                let mut vsize = None;
                let mut weight = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "blockhash" {
                        if blockhash.is_some() {
                            return Err(de::Error::duplicate_field("blockhash"));
                        }
                        blockhash = Some(map.next_value::<bitcoin::BlockHash>()?);
                    }
                    if key == "blocktime" {
                        if blocktime.is_some() {
                            return Err(de::Error::duplicate_field("blocktime"));
                        }
                        blocktime = Some(map.next_value::<u64>()?);
                    }
                    if key == "confirmations" {
                        if confirmations.is_some() {
                            return Err(de::Error::duplicate_field("confirmations"));
                        }
                        confirmations = Some(map.next_value::<i64>()?);
                    }
                    if key == "data" {
                        if data.is_some() {
                            return Err(de::Error::duplicate_field("data"));
                        }
                        data = Some(map.next_value::<String>()?);
                    }
                    if key == "fee" {
                        if fee.is_some() {
                            return Err(de::Error::duplicate_field("fee"));
                        }
                        fee = Some(map.next_value::<f64>()?);
                    }
                    if key == "hash" {
                        if hash.is_some() {
                            return Err(de::Error::duplicate_field("hash"));
                        }
                        hash = Some(map.next_value::<String>()?);
                    }
                    if key == "hex" {
                        if hex.is_some() {
                            return Err(de::Error::duplicate_field("hex"));
                        }
                        hex = Some(map.next_value::<String>()?);
                    }
                    if key == "in_active_chain" {
                        if in_active_chain.is_some() {
                            return Err(de::Error::duplicate_field("in_active_chain"));
                        }
                        in_active_chain = Some(map.next_value::<bool>()?);
                    }
                    if key == "locktime" {
                        if locktime.is_some() {
                            return Err(de::Error::duplicate_field("locktime"));
                        }
                        locktime = Some(map.next_value::<u64>()?);
                    }
                    if key == "size" {
                        if size.is_some() {
                            return Err(de::Error::duplicate_field("size"));
                        }
                        size = Some(map.next_value::<u64>()?);
                    }
                    if key == "time" {
                        if time.is_some() {
                            return Err(de::Error::duplicate_field("time"));
                        }
                        time = Some(map.next_value::<u64>()?);
                    }
                    if key == "txid" {
                        if txid.is_some() {
                            return Err(de::Error::duplicate_field("txid"));
                        }
                        txid = Some(map.next_value::<bitcoin::Txid>()?);
                    }
                    if key == "version" {
                        if version.is_some() {
                            return Err(de::Error::duplicate_field("version"));
                        }
                        version = Some(map.next_value::<u32>()?);
                    }
                    if key == "vin" {
                        if vin.is_some() {
                            return Err(de::Error::duplicate_field("vin"));
                        }
                        vin = Some(map.next_value::<serde_json::Value>()?);
                    }
                    if key == "vin_1" {
                        if vin_1.is_some() {
                            return Err(de::Error::duplicate_field("vin_1"));
                        }
                        vin_1 = Some(map.next_value::<serde_json::Value>()?);
                    }
                    if key == "vout" {
                        if vout.is_some() {
                            return Err(de::Error::duplicate_field("vout"));
                        }
                        vout = Some(map.next_value::<serde_json::Value>()?);
                    }
                    if key == "vsize" {
                        if vsize.is_some() {
                            return Err(de::Error::duplicate_field("vsize"));
                        }
                        vsize = Some(map.next_value::<u64>()?);
                    }
                    if key == "weight" {
                        if weight.is_some() {
                            return Err(de::Error::duplicate_field("weight"));
                        }
                        weight = Some(map.next_value::<u64>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                Ok(GetRawTransactionResponse {
                    blockhash,
                    blocktime,
                    confirmations,
                    data,
                    fee,
                    hash,
                    hex,
                    in_active_chain,
                    locktime,
                    size,
                    time,
                    txid,
                    version,
                    vin,
                    vin_1,
                    vout,
                    vsize,
                    weight,
                })
            }
        }

        deserializer.deserialize_any(ConditionalResponseVisitor)
    }
}

/// Response for the `GetReceivedByAddress` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetReceivedByAddressResponse {
    /// Wrapped primitive value
    pub value: bitcoin::Amount,
}

impl<'de> serde::Deserialize<'de> for GetReceivedByAddressResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetReceivedByAddressResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetReceivedByAddressResponse { value: bitcoin::Amount::from_sat(v) })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v < 0 {
                    return Err(de::Error::custom(format!("Amount cannot be negative: {}", v)));
                }
                Ok(GetReceivedByAddressResponse { value: bitcoin::Amount::from_sat(v as u64) })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let amount = bitcoin::Amount::from_btc(v)
                    .map_err(|e| de::Error::custom(format!("Invalid BTC amount: {}", e)))?;
                Ok(GetReceivedByAddressResponse { value: amount })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<bitcoin::Amount>().map_err(de::Error::custom)?;
                Ok(GetReceivedByAddressResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Err(de::Error::custom("cannot convert bool to bitcoin::Amount"))
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetReceivedByAddressResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetReceivedByAddressResponse {
    type Target = bitcoin::Amount;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetReceivedByAddressResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<bitcoin::Amount> for GetReceivedByAddressResponse {
    fn as_ref(&self) -> &bitcoin::Amount { &self.value }
}

impl From<bitcoin::Amount> for GetReceivedByAddressResponse {
    fn from(value: bitcoin::Amount) -> Self { Self { value } }
}

impl From<GetReceivedByAddressResponse> for bitcoin::Amount {
    fn from(wrapper: GetReceivedByAddressResponse) -> Self { wrapper.value }
}

/// Response for the `GetReceivedByLabel` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetReceivedByLabelResponse {
    /// Wrapped primitive value
    pub value: bitcoin::Amount,
}

impl<'de> serde::Deserialize<'de> for GetReceivedByLabelResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetReceivedByLabelResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetReceivedByLabelResponse { value: bitcoin::Amount::from_sat(v) })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v < 0 {
                    return Err(de::Error::custom(format!("Amount cannot be negative: {}", v)));
                }
                Ok(GetReceivedByLabelResponse { value: bitcoin::Amount::from_sat(v as u64) })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let amount = bitcoin::Amount::from_btc(v)
                    .map_err(|e| de::Error::custom(format!("Invalid BTC amount: {}", e)))?;
                Ok(GetReceivedByLabelResponse { value: amount })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<bitcoin::Amount>().map_err(de::Error::custom)?;
                Ok(GetReceivedByLabelResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Err(de::Error::custom("cannot convert bool to bitcoin::Amount"))
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetReceivedByLabelResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetReceivedByLabelResponse {
    type Target = bitcoin::Amount;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetReceivedByLabelResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<bitcoin::Amount> for GetReceivedByLabelResponse {
    fn as_ref(&self) -> &bitcoin::Amount { &self.value }
}

impl From<bitcoin::Amount> for GetReceivedByLabelResponse {
    fn from(value: bitcoin::Amount) -> Self { Self { value } }
}

impl From<GetReceivedByLabelResponse> for bitcoin::Amount {
    fn from(wrapper: GetReceivedByLabelResponse) -> Self { wrapper.value }
}

/// Response for the `GetRpcInfo` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRpcInfoResponse {
    /// All active commands
    pub active_commands: serde_json::Value,
    /// The complete file path to the debug log
    pub logpath: String,
}

/// Response for the `GetTransaction` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTransactionResponse {
    /// The amount in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub amount: bitcoin::Amount,
    /// ("yes|no|unknown") Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability.
    /// May be unknown for unconfirmed transactions not in the mempool because their unconfirmed ancestors are unknown.
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: String,
    /// The block hash containing the transaction.
    pub blockhash: Option<bitcoin::BlockHash>,
    /// The block height containing the transaction.
    pub blockheight: Option<u64>,
    /// The index of the transaction in the block that includes it.
    pub blockindex: Option<u64>,
    /// The block time expressed in UNIX epoch time.
    pub blocktime: Option<u64>,
    /// If a comment is associated with the transaction, only present if not empty.
    pub comment: Option<String>,
    /// The number of confirmations for the transaction. Negative confirmations means the
    /// transaction conflicted that many blocks ago.
    pub confirmations: i64,
    /// The decoded transaction (only present when `verbose` is passed)
    pub decoded: Option<serde_json::Value>,
    pub details: serde_json::Value,
    /// The amount of the fee in BTC. This is negative and only available for the
    /// 'send' category of transactions.
    #[serde(deserialize_with = "option_amount_from_btc_float")]
    pub fee: Option<bitcoin::Amount>,
    /// Only present if the transaction's only input is a coinbase one.
    pub generated: Option<bool>,
    /// Raw data for transaction
    pub hex: String,
    /// hash and height of the block this information was generated on
    pub lastprocessedblock: serde_json::Value,
    /// Transactions in the mempool that directly conflict with either this transaction or an ancestor transaction
    pub mempoolconflicts: serde_json::Value,
    /// Only if 'category' is 'received'. List of parent descriptors for the output script of this coin.
    pub parent_descs: Option<serde_json::Value>,
    /// Only if 'category' is 'send'. The txid if this tx was replaced.
    pub replaced_by_txid: Option<String>,
    /// Only if 'category' is 'send'. The txid if this tx replaces another.
    pub replaces_txid: Option<String>,
    /// The transaction time expressed in UNIX epoch time.
    pub time: u64,
    /// The time received expressed in UNIX epoch time.
    pub timereceived: u64,
    /// If a comment to is associated with the transaction.
    pub to: Option<String>,
    /// Whether we consider the transaction to be trusted and safe to spend from.
    /// Only present when the transaction has 0 confirmations (or negative confirmations, if conflicted).
    pub trusted: Option<bool>,
    /// The transaction id.
    pub txid: bitcoin::Txid,
    /// Confirmed transactions that have been detected by the wallet to conflict with this transaction.
    pub walletconflicts: serde_json::Value,
    /// The hash of serialized transaction, including witness data.
    pub wtxid: String,
}

/// Response for the `GetTxOut` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOutResponse {
    /// The hash of the block at the tip of the chain
    pub bestblock: String,
    /// Coinbase or not
    pub coinbase: bool,
    /// The number of confirmations
    pub confirmations: i64,
    #[serde(default)]
    pub field_0: Option<()>,
    pub scriptPubKey: serde_json::Value,
    /// The transaction value in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub value: bitcoin::Amount,
}

/// Response for the `GetTxOutProof` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetTxOutProofResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for GetTxOutProofResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = GetTxOutProofResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetTxOutProofResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetTxOutProofResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetTxOutProofResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetTxOutProofResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GetTxOutProofResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(GetTxOutProofResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for GetTxOutProofResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for GetTxOutProofResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for GetTxOutProofResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for GetTxOutProofResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<GetTxOutProofResponse> for String {
    fn from(wrapper: GetTxOutProofResponse) -> Self { wrapper.value }
}

/// Response for the `GetTxOutSetInfo` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOutSetInfoResponse {
    /// The hash of the block at which these statistics are calculated
    pub bestblock: String,
    /// Info on amounts in the block at this block height (only available if coinstatsindex is used)
    pub block_info: Option<serde_json::Value>,
    /// Database-independent, meaningless metric indicating the UTXO set size
    pub bogosize: u64,
    /// The estimated size of the chainstate on disk (not available when coinstatsindex is used)
    pub disk_size: Option<u64>,
    /// The serialized hash (only present if 'hash_serialized_3' hash_type is chosen)
    pub hash_serialized_3: Option<String>,
    /// The block height (index) of the returned statistics
    pub height: u64,
    /// The serialized hash (only present if 'muhash' hash_type is chosen)
    pub muhash: Option<String>,
    /// The total amount of coins in the UTXO set
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub total_amount: bitcoin::Amount,
    /// The total amount of coins permanently excluded from the UTXO set (only available if coinstatsindex is used)
    #[serde(default)]
    #[serde(deserialize_with = "option_amount_from_btc_float")]
    pub total_unspendable_amount: Option<bitcoin::Amount>,
    /// The number of transactions with unspent outputs (not available when coinstatsindex is used)
    pub transactions: Option<u64>,
    /// The number of unspent transaction outputs
    pub txouts: u64,
}

/// Response for the `GetTxSpendingPrevOut` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxSpendingPrevOutResponse {
    pub field: serde_json::Value,
}

/// Response for the `GetWalletInfo` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetWalletInfoResponse {
    /// whether this wallet tracks clean/dirty coins in terms of reuse
    pub avoid_reuse: bool,
    /// The start time for blocks scanning. It could be modified by (re)importing any descriptor with an earlier timestamp.
    pub birthtime: Option<u64>,
    /// Whether this wallet intentionally does not contain any keys, scripts, or descriptors
    pub blank: bool,
    /// whether this wallet uses descriptors for output script management
    pub descriptors: bool,
    /// whether this wallet is configured to use an external signer such as a hardware wallet
    pub external_signer: bool,
    /// The flags currently set on the wallet
    pub flags: serde_json::Value,
    /// the database format (only sqlite)
    pub format: String,
    /// how many new keys are pre-generated (only counts external keys)
    pub keypoolsize: u64,
    /// how many new keys are pre-generated for internal use (used for change outputs, only appears if the wallet is using this feature, otherwise external keys are used)
    pub keypoolsize_hd_internal: Option<u64>,
    /// hash and height of the block this information was generated on
    pub lastprocessedblock: serde_json::Value,
    /// false if privatekeys are disabled for this wallet (enforced watch-only wallet)
    pub private_keys_enabled: bool,
    /// current scanning details, or false if no scan is in progress
    pub scanning: serde_json::Value,
    /// the total number of transactions in the wallet
    pub txcount: u64,
    /// the UNIX epoch time until which the wallet is unlocked for transfers, or 0 if the wallet is locked (only present for passphrase-encrypted wallets)
    pub unlocked_until: Option<u64>,
    /// the wallet name
    pub walletname: String,
    /// (DEPRECATED) only related to unsupported legacy wallet, returns the latest version 169900 for backwards compatibility
    pub walletversion: u64,
}

/// Response for the `Help` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct HelpResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for HelpResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = HelpResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(HelpResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(HelpResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(HelpResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(HelpResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(HelpResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(HelpResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for HelpResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for HelpResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for HelpResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for HelpResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<HelpResponse> for String {
    fn from(wrapper: HelpResponse) -> Self { wrapper.value }
}

/// Response for the `ImportDescriptors` RPC method
///
/// Response is an array with the same size as the input that has the execution result
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ImportDescriptorsResponse {
    pub field: serde_json::Value,
}

/// Response for the `ImportMempool` RPC method
///
/// This method returns no meaningful data.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ImportMempoolResponse;

/// Response for the `ImportPrunedFunds` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ImportPrunedFundsResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for ImportPrunedFundsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = ImportPrunedFundsResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ImportPrunedFundsResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ImportPrunedFundsResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ImportPrunedFundsResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ImportPrunedFundsResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ImportPrunedFundsResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ImportPrunedFundsResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ImportPrunedFundsResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(ImportPrunedFundsResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for ImportPrunedFundsResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for ImportPrunedFundsResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for ImportPrunedFundsResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for ImportPrunedFundsResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<ImportPrunedFundsResponse> for () {
    fn from(wrapper: ImportPrunedFundsResponse) -> Self { wrapper.value }
}

/// Response for the `InvalidateBlock` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct InvalidateBlockResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for InvalidateBlockResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = InvalidateBlockResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(InvalidateBlockResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(InvalidateBlockResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(InvalidateBlockResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(InvalidateBlockResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(InvalidateBlockResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(InvalidateBlockResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(InvalidateBlockResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(InvalidateBlockResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for InvalidateBlockResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for InvalidateBlockResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for InvalidateBlockResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for InvalidateBlockResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<InvalidateBlockResponse> for () {
    fn from(wrapper: InvalidateBlockResponse) -> Self { wrapper.value }
}

/// Response for the `JoinPsbts` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct JoinPsbtsResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for JoinPsbtsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = JoinPsbtsResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(JoinPsbtsResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(JoinPsbtsResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(JoinPsbtsResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(JoinPsbtsResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(JoinPsbtsResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(JoinPsbtsResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for JoinPsbtsResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for JoinPsbtsResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for JoinPsbtsResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for JoinPsbtsResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<JoinPsbtsResponse> for String {
    fn from(wrapper: JoinPsbtsResponse) -> Self { wrapper.value }
}

/// Response for the `KeypoolRefill` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct KeypoolRefillResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for KeypoolRefillResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = KeypoolRefillResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(KeypoolRefillResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(KeypoolRefillResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(KeypoolRefillResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(KeypoolRefillResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(KeypoolRefillResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(KeypoolRefillResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(KeypoolRefillResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(KeypoolRefillResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for KeypoolRefillResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for KeypoolRefillResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for KeypoolRefillResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for KeypoolRefillResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<KeypoolRefillResponse> for () {
    fn from(wrapper: KeypoolRefillResponse) -> Self { wrapper.value }
}

/// Response for the `ListAddressGroupings` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListAddressGroupingsResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for ListAddressGroupingsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for ListAddressGroupingsResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<ListAddressGroupingsResponse> for Vec<serde_json::Value> {
    fn from(wrapper: ListAddressGroupingsResponse) -> Self { wrapper.value }
}

/// Response for the `ListBanned` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListBannedResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for ListBannedResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for ListBannedResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<ListBannedResponse> for Vec<serde_json::Value> {
    fn from(wrapper: ListBannedResponse) -> Self { wrapper.value }
}

/// Response for the `ListDescriptors` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListDescriptorsResponse {
    /// Array of descriptor objects (sorted by descriptor string representation)
    pub descriptors: serde_json::Value,
    /// Name of wallet this operation was performed on
    pub wallet_name: String,
}

/// Response for the `ListLabels` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListLabelsResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for ListLabelsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for ListLabelsResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<ListLabelsResponse> for Vec<serde_json::Value> {
    fn from(wrapper: ListLabelsResponse) -> Self { wrapper.value }
}

/// Response for the `ListLockUnspent` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListLockUnspentResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for ListLockUnspentResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for ListLockUnspentResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<ListLockUnspentResponse> for Vec<serde_json::Value> {
    fn from(wrapper: ListLockUnspentResponse) -> Self { wrapper.value }
}

/// Response for the `ListReceivedByAddress` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListReceivedByAddressResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for ListReceivedByAddressResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for ListReceivedByAddressResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<ListReceivedByAddressResponse> for Vec<serde_json::Value> {
    fn from(wrapper: ListReceivedByAddressResponse) -> Self { wrapper.value }
}

/// Response for the `ListReceivedByLabel` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListReceivedByLabelResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for ListReceivedByLabelResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for ListReceivedByLabelResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<ListReceivedByLabelResponse> for Vec<serde_json::Value> {
    fn from(wrapper: ListReceivedByLabelResponse) -> Self { wrapper.value }
}

/// Response for the `ListSinceBlock` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListSinceBlockResponse {
    /// The hash of the block (target_confirmations-1) from the best block on the main chain, or the genesis hash if the referenced block does not exist yet. This is typically used to feed back into listsinceblock the next time you call it. So you would generally use a target_confirmations of say 6, so you will be continually re-notified of transactions until they've reached 6 confirmations plus any new ones
    pub lastblock: String,
    /// &lt;structure is the same as "transactions" above, only present if include_removed=true&gt;
    /// Note: transactions that were re-added in the active chain will appear as-is in this array, and may thus have a positive confirmation count.
    pub removed: Option<serde_json::Value>,
    pub transactions: serde_json::Value,
}

/// Response for the `ListTransactions` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListTransactionsResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for ListTransactionsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for ListTransactionsResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<ListTransactionsResponse> for Vec<serde_json::Value> {
    fn from(wrapper: ListTransactionsResponse) -> Self { wrapper.value }
}

/// Response for the `ListUnspent` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListUnspentResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for ListUnspentResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for ListUnspentResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<ListUnspentResponse> for Vec<serde_json::Value> {
    fn from(wrapper: ListUnspentResponse) -> Self { wrapper.value }
}

/// Response for the `ListWalletDir` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListWalletDirResponse {
    pub wallets: serde_json::Value,
}

/// Response for the `ListWallets` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ListWalletsResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for ListWalletsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for ListWalletsResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<ListWalletsResponse> for Vec<serde_json::Value> {
    fn from(wrapper: ListWalletsResponse) -> Self { wrapper.value }
}

/// Response for the `LoadTxOutSet` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LoadTxOutSetResponse {
    /// the height of the base of the snapshot
    pub base_height: u64,
    /// the number of coins loaded from the snapshot
    pub coins_loaded: u64,
    /// the absolute path that the snapshot was loaded from
    pub path: String,
    /// the hash of the base of the snapshot
    pub tip_hash: String,
}

/// Response for the `LoadWallet` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LoadWalletResponse {
    /// The wallet name if loaded successfully.
    pub name: String,
    /// Warning messages, if any, related to loading the wallet.
    pub warnings: Option<serde_json::Value>,
}

/// Response for the `LockUnspent` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LockUnspentResponse {
    /// Wrapped primitive value
    pub value: bool,
}

impl<'de> serde::Deserialize<'de> for LockUnspentResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = LockUnspentResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(LockUnspentResponse { value: v != 0 })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(LockUnspentResponse { value: v != 0 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(LockUnspentResponse { value: v != 0.0 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<bool>().map_err(de::Error::custom)?;
                Ok(LockUnspentResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(LockUnspentResponse { value: v })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(LockUnspentResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for LockUnspentResponse {
    type Target = bool;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for LockUnspentResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<bool> for LockUnspentResponse {
    fn as_ref(&self) -> &bool { &self.value }
}

impl From<bool> for LockUnspentResponse {
    fn from(value: bool) -> Self { Self { value } }
}

impl From<LockUnspentResponse> for bool {
    fn from(wrapper: LockUnspentResponse) -> Self { wrapper.value }
}

/// Response for the `Logging` RPC method
///
/// keys are the logging categories, and values indicates its status
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LoggingResponse {
    /// if being debug logged or not. false:inactive, true:active
    #[serde(default)]
    pub category: Option<bool>,
}

/// Response for the `MigrateWallet` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct MigrateWalletResponse {
    /// The location of the backup of the original wallet
    pub backup_path: String,
    /// The name of the migrated wallet containing solvable but not watched scripts
    pub solvables_name: Option<String>,
    /// The name of the primary migrated wallet
    pub wallet_name: String,
    /// The name of the migrated wallet containing the watchonly scripts
    pub watchonly_name: Option<String>,
}

/// Response for the `MockScheduler` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MockSchedulerResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for MockSchedulerResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = MockSchedulerResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MockSchedulerResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MockSchedulerResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MockSchedulerResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MockSchedulerResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MockSchedulerResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MockSchedulerResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(MockSchedulerResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(MockSchedulerResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for MockSchedulerResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for MockSchedulerResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for MockSchedulerResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for MockSchedulerResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<MockSchedulerResponse> for () {
    fn from(wrapper: MockSchedulerResponse) -> Self { wrapper.value }
}

/// Response for the `Ping` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PingResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for PingResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = PingResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PingResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PingResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PingResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PingResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PingResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PingResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PingResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(PingResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for PingResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for PingResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for PingResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for PingResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<PingResponse> for () {
    fn from(wrapper: PingResponse) -> Self { wrapper.value }
}

/// Response for the `PreciousBlock` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PreciousBlockResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for PreciousBlockResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = PreciousBlockResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PreciousBlockResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PreciousBlockResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PreciousBlockResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PreciousBlockResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PreciousBlockResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PreciousBlockResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PreciousBlockResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(PreciousBlockResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for PreciousBlockResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for PreciousBlockResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for PreciousBlockResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for PreciousBlockResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<PreciousBlockResponse> for () {
    fn from(wrapper: PreciousBlockResponse) -> Self { wrapper.value }
}

/// Response for the `PrioritiseTransaction` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PrioritiseTransactionResponse {
    /// Wrapped primitive value
    pub value: bool,
}

impl<'de> serde::Deserialize<'de> for PrioritiseTransactionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = PrioritiseTransactionResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PrioritiseTransactionResponse { value: v != 0 })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PrioritiseTransactionResponse { value: v != 0 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PrioritiseTransactionResponse { value: v != 0.0 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<bool>().map_err(de::Error::custom)?;
                Ok(PrioritiseTransactionResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PrioritiseTransactionResponse { value: v })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(PrioritiseTransactionResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for PrioritiseTransactionResponse {
    type Target = bool;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for PrioritiseTransactionResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<bool> for PrioritiseTransactionResponse {
    fn as_ref(&self) -> &bool { &self.value }
}

impl From<bool> for PrioritiseTransactionResponse {
    fn from(value: bool) -> Self { Self { value } }
}

impl From<PrioritiseTransactionResponse> for bool {
    fn from(wrapper: PrioritiseTransactionResponse) -> Self { wrapper.value }
}

/// Response for the `PruneBlockchain` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PruneBlockchainResponse {
    /// Wrapped primitive value
    pub value: u64,
}

impl<'de> serde::Deserialize<'de> for PruneBlockchainResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = PruneBlockchainResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PruneBlockchainResponse { value: v })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PruneBlockchainResponse { value: v as u64 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PruneBlockchainResponse { value: v as u64 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<u64>().map_err(de::Error::custom)?;
                Ok(PruneBlockchainResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(PruneBlockchainResponse { value: v as u64 })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(PruneBlockchainResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for PruneBlockchainResponse {
    type Target = u64;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for PruneBlockchainResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<u64> for PruneBlockchainResponse {
    fn as_ref(&self) -> &u64 { &self.value }
}

impl From<u64> for PruneBlockchainResponse {
    fn from(value: u64) -> Self { Self { value } }
}

impl From<PruneBlockchainResponse> for u64 {
    fn from(wrapper: PruneBlockchainResponse) -> Self { wrapper.value }
}

/// Response for the `PsbtBumpFee` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PsbtBumpFeeResponse {
    /// Errors encountered during processing (may be empty).
    pub errors: serde_json::Value,
    /// The fee of the new transaction.
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub fee: bitcoin::Amount,
    /// The fee of the replaced transaction.
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub origfee: bitcoin::Amount,
    /// The base64-encoded unsigned PSBT of the new transaction.
    pub psbt: String,
}

/// Response for the `ReconsiderBlock` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ReconsiderBlockResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for ReconsiderBlockResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = ReconsiderBlockResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ReconsiderBlockResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ReconsiderBlockResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ReconsiderBlockResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ReconsiderBlockResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ReconsiderBlockResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ReconsiderBlockResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ReconsiderBlockResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(ReconsiderBlockResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for ReconsiderBlockResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for ReconsiderBlockResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for ReconsiderBlockResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for ReconsiderBlockResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<ReconsiderBlockResponse> for () {
    fn from(wrapper: ReconsiderBlockResponse) -> Self { wrapper.value }
}

/// Response for the `RemovePrunedFunds` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RemovePrunedFundsResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for RemovePrunedFundsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = RemovePrunedFundsResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(RemovePrunedFundsResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(RemovePrunedFundsResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(RemovePrunedFundsResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(RemovePrunedFundsResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(RemovePrunedFundsResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(RemovePrunedFundsResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(RemovePrunedFundsResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(RemovePrunedFundsResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for RemovePrunedFundsResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for RemovePrunedFundsResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for RemovePrunedFundsResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for RemovePrunedFundsResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<RemovePrunedFundsResponse> for () {
    fn from(wrapper: RemovePrunedFundsResponse) -> Self { wrapper.value }
}

/// Response for the `RescanBlockchain` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct RescanBlockchainResponse {
    /// The block height where the rescan started (the requested height or 0)
    pub start_height: u64,
    /// The height of the last rescanned block. May be null in rare cases if there was a reorg and the call didn't scan any blocks because they were already scanned in the background.
    pub stop_height: u64,
}

/// Response for the `RestoreWallet` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct RestoreWalletResponse {
    /// The wallet name if restored successfully.
    pub name: String,
    /// Warning messages, if any, related to restoring and loading the wallet.
    pub warnings: Option<serde_json::Value>,
}

/// Response for the `SaveMempool` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SaveMempoolResponse {
    /// the directory and file where the mempool was saved
    pub filename: String,
}

/// Response for the `ScanBlocks` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanBlocksResponse {
    /// true if the scan process was not aborted
    pub completed: bool,
    /// Height of the block currently being scanned
    pub current_height: u64,
    pub field_0: (),
    /// The height we started the scan from
    pub from_height: u64,
    /// Approximate percent complete
    pub progress: u64,
    /// Blocks that may have matched a scanobject.
    pub relevant_blocks: serde_json::Value,
    /// True if scan will be aborted (not necessarily before this RPC returns), or false if there is no scan to abort
    pub success: bool,
    /// The height we ended the scan at
    pub to_height: u64,
}

/// Response for the `ScanTxOutSet` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanTxOutSetResponse {
    /// The hash of the block at the tip of the chain
    pub bestblock: String,
    pub field_3: (),
    /// The block height at which the scan was done
    pub height: u64,
    /// Approximate percent complete
    pub progress: u64,
    /// Whether the scan was completed
    pub success: bool,
    /// True if scan will be aborted (not necessarily before this RPC returns), or false if there is no scan to abort
    pub success_1: bool,
    /// The total amount of all found unspent outputs in BTC
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub total_amount: bitcoin::Amount,
    /// The number of unspent transaction outputs scanned
    pub txouts: u64,
    pub unspents: serde_json::Value,
}

/// Response for the `Schema` RPC method
///
/// This method returns no meaningful data.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SchemaResponse;

/// Response for the `Send` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendResponse {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// If add_to_wallet is false, the hex-encoded raw transaction with signature(s)
    pub hex: Option<String>,
    /// If more signatures are needed, or if add_to_wallet is false, the base64-encoded (partially) signed transaction
    pub psbt: Option<String>,
    /// The transaction id for the send. Only 1 transaction is created regardless of the number of addresses.
    pub txid: Option<bitcoin::Txid>,
}

/// Response for the `SendAll` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendAllResponse {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// If add_to_wallet is false, the hex-encoded raw transaction with signature(s)
    pub hex: Option<String>,
    /// If more signatures are needed, or if add_to_wallet is false, the base64-encoded (partially) signed transaction
    pub psbt: Option<String>,
    /// The transaction id for the send. Only 1 transaction is created regardless of the number of addresses.
    pub txid: Option<bitcoin::Txid>,
}

/// Response for the `SendMany` RPC method
///
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SendManyResponse {
    /// The transaction fee reason.
    pub fee_reason: Option<String>,
    /// The transaction id for the send. Only 1 transaction is created regardless of
    /// the number of addresses.
    pub txid: Option<bitcoin::Txid>,
}
impl<'de> serde::Deserialize<'de> for SendManyResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct ConditionalResponseVisitor;

        #[allow(clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for ConditionalResponseVisitor {
            type Value = SendManyResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or object")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let txid = bitcoin::Txid::from_str(v).map_err(de::Error::custom)?;
                Ok(SendManyResponse { fee_reason: None, txid: Some(txid) })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut fee_reason = None;
                let mut txid = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "fee_reason" {
                        if fee_reason.is_some() {
                            return Err(de::Error::duplicate_field("fee_reason"));
                        }
                        fee_reason = Some(map.next_value::<String>()?);
                    }
                    if key == "txid" {
                        if txid.is_some() {
                            return Err(de::Error::duplicate_field("txid"));
                        }
                        txid = Some(map.next_value::<bitcoin::Txid>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                Ok(SendManyResponse { fee_reason, txid })
            }
        }

        deserializer.deserialize_any(ConditionalResponseVisitor)
    }
}

/// Response for the `SendMsgToPeer` RPC method
///
/// This method returns no meaningful data.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SendMsgToPeerResponse;

/// Response for the `SendRawTransaction` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SendRawTransactionResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for SendRawTransactionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = SendRawTransactionResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SendRawTransactionResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SendRawTransactionResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SendRawTransactionResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SendRawTransactionResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SendRawTransactionResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(SendRawTransactionResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for SendRawTransactionResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for SendRawTransactionResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for SendRawTransactionResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for SendRawTransactionResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<SendRawTransactionResponse> for String {
    fn from(wrapper: SendRawTransactionResponse) -> Self { wrapper.value }
}

/// Response for the `SendToAddress` RPC method
///
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SendToAddressResponse {
    /// The transaction fee reason.
    pub fee_reason: Option<String>,
    /// The transaction id.
    pub txid: Option<bitcoin::Txid>,
}
impl<'de> serde::Deserialize<'de> for SendToAddressResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct ConditionalResponseVisitor;

        #[allow(clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for ConditionalResponseVisitor {
            type Value = SendToAddressResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or object")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let txid = bitcoin::Txid::from_str(v).map_err(de::Error::custom)?;
                Ok(SendToAddressResponse { fee_reason: None, txid: Some(txid) })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut fee_reason = None;
                let mut txid = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "fee_reason" {
                        if fee_reason.is_some() {
                            return Err(de::Error::duplicate_field("fee_reason"));
                        }
                        fee_reason = Some(map.next_value::<String>()?);
                    }
                    if key == "txid" {
                        if txid.is_some() {
                            return Err(de::Error::duplicate_field("txid"));
                        }
                        txid = Some(map.next_value::<bitcoin::Txid>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                Ok(SendToAddressResponse { fee_reason, txid })
            }
        }

        deserializer.deserialize_any(ConditionalResponseVisitor)
    }
}

/// Response for the `SetBan` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SetBanResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for SetBanResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = SetBanResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetBanResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetBanResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetBanResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetBanResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetBanResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetBanResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetBanResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(SetBanResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for SetBanResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for SetBanResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for SetBanResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for SetBanResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<SetBanResponse> for () {
    fn from(wrapper: SetBanResponse) -> Self { wrapper.value }
}

/// Response for the `SetLabel` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SetLabelResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for SetLabelResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = SetLabelResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetLabelResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetLabelResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetLabelResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetLabelResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetLabelResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetLabelResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetLabelResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(SetLabelResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for SetLabelResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for SetLabelResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for SetLabelResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for SetLabelResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<SetLabelResponse> for () {
    fn from(wrapper: SetLabelResponse) -> Self { wrapper.value }
}

/// Response for the `SetMockTime` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SetMockTimeResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for SetMockTimeResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = SetMockTimeResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetMockTimeResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetMockTimeResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetMockTimeResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetMockTimeResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetMockTimeResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetMockTimeResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetMockTimeResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(SetMockTimeResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for SetMockTimeResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for SetMockTimeResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for SetMockTimeResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for SetMockTimeResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<SetMockTimeResponse> for () {
    fn from(wrapper: SetMockTimeResponse) -> Self { wrapper.value }
}

/// Response for the `SetNetworkActive` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SetNetworkActiveResponse {
    /// Wrapped primitive value
    pub value: bool,
}

impl<'de> serde::Deserialize<'de> for SetNetworkActiveResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = SetNetworkActiveResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetNetworkActiveResponse { value: v != 0 })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetNetworkActiveResponse { value: v != 0 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetNetworkActiveResponse { value: v != 0.0 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<bool>().map_err(de::Error::custom)?;
                Ok(SetNetworkActiveResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SetNetworkActiveResponse { value: v })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(SetNetworkActiveResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for SetNetworkActiveResponse {
    type Target = bool;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for SetNetworkActiveResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<bool> for SetNetworkActiveResponse {
    fn as_ref(&self) -> &bool { &self.value }
}

impl From<bool> for SetNetworkActiveResponse {
    fn from(value: bool) -> Self { Self { value } }
}

impl From<SetNetworkActiveResponse> for bool {
    fn from(wrapper: SetNetworkActiveResponse) -> Self { wrapper.value }
}

/// Response for the `SetWalletFlag` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SetWalletFlagResponse {
    /// The name of the flag that was modified
    pub flag_name: String,
    /// The new state of the flag
    pub flag_state: bool,
    /// Any warnings associated with the change
    pub warnings: Option<String>,
}

/// Response for the `SignMessage` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SignMessageResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for SignMessageResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = SignMessageResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(SignMessageResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for SignMessageResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for SignMessageResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for SignMessageResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for SignMessageResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<SignMessageResponse> for String {
    fn from(wrapper: SignMessageResponse) -> Self { wrapper.value }
}

/// Response for the `SignMessageWithPrivKey` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SignMessageWithPrivKeyResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for SignMessageWithPrivKeyResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = SignMessageWithPrivKeyResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageWithPrivKeyResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageWithPrivKeyResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageWithPrivKeyResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageWithPrivKeyResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SignMessageWithPrivKeyResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(SignMessageWithPrivKeyResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for SignMessageWithPrivKeyResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for SignMessageWithPrivKeyResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for SignMessageWithPrivKeyResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for SignMessageWithPrivKeyResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<SignMessageWithPrivKeyResponse> for String {
    fn from(wrapper: SignMessageWithPrivKeyResponse) -> Self { wrapper.value }
}

/// Response for the `SignRawTransactionWithKey` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignRawTransactionWithKeyResponse {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// Script verification errors (if there are any)
    pub errors: Option<serde_json::Value>,
    /// The hex-encoded raw transaction with signature(s)
    pub hex: String,
}

/// Response for the `SignRawTransactionWithWallet` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignRawTransactionWithWalletResponse {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// Script verification errors (if there are any)
    pub errors: Option<serde_json::Value>,
    /// The hex-encoded raw transaction with signature(s)
    pub hex: String,
}

/// Response for the `SimulateRawTransaction` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SimulateRawTransactionResponse {
    /// The wallet balance change (negative means decrease).
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub balance_change: bitcoin::Amount,
}

/// Response for the `Stop` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StopResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for StopResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = StopResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StopResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StopResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StopResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StopResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StopResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(StopResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for StopResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for StopResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for StopResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for StopResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<StopResponse> for String {
    fn from(wrapper: StopResponse) -> Self { wrapper.value }
}

/// Response for the `SubmitBlock` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubmitBlockResponse {
    pub field_0: (),
    /// According to BIP22
    pub field_1: String,
}

/// Response for the `SubmitHeader` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SubmitHeaderResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for SubmitHeaderResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = SubmitHeaderResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SubmitHeaderResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SubmitHeaderResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SubmitHeaderResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SubmitHeaderResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SubmitHeaderResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SubmitHeaderResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SubmitHeaderResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(SubmitHeaderResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for SubmitHeaderResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for SubmitHeaderResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for SubmitHeaderResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for SubmitHeaderResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<SubmitHeaderResponse> for () {
    fn from(wrapper: SubmitHeaderResponse) -> Self { wrapper.value }
}

/// Response for the `SubmitPackage` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubmitPackageResponse {
    /// The transaction package result message. "success" indicates all transactions were accepted into or are already in the mempool.
    pub package_msg: String,
    /// List of txids of replaced transactions
    #[serde(rename = "replaced-transactions")]
    pub replaced_transactions: Option<serde_json::Value>,
    /// The transaction results keyed by wtxid. An entry is returned for every submitted wtxid.
    #[serde(rename = "tx-results")]
    pub tx_results: serde_json::Value,
}

/// Response for the `SyncWithValidationInterfaceQueue` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SyncWithValidationInterfaceQueueResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for SyncWithValidationInterfaceQueueResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = SyncWithValidationInterfaceQueueResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SyncWithValidationInterfaceQueueResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SyncWithValidationInterfaceQueueResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SyncWithValidationInterfaceQueueResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SyncWithValidationInterfaceQueueResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SyncWithValidationInterfaceQueueResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SyncWithValidationInterfaceQueueResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SyncWithValidationInterfaceQueueResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(SyncWithValidationInterfaceQueueResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for SyncWithValidationInterfaceQueueResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for SyncWithValidationInterfaceQueueResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for SyncWithValidationInterfaceQueueResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for SyncWithValidationInterfaceQueueResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<SyncWithValidationInterfaceQueueResponse> for () {
    fn from(wrapper: SyncWithValidationInterfaceQueueResponse) -> Self { wrapper.value }
}

/// Response for the `TestMempoolAccept` RPC method
///
/// The result of the mempool acceptance test for each raw transaction in the input array.
/// Returns results for each transaction in the same order they were passed in.
/// Transactions that cannot be fully validated due to failures in other transactions will not contain an 'allowed' result.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct TestMempoolAcceptResponse {
    pub field: serde_json::Value,
}

/// Response for the `UnloadWallet` RPC method
///
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UnloadWalletResponse {
    /// Warning messages, if any, related to unloading the wallet.
    pub warnings: Option<serde_json::Value>,
}
impl<'de> serde::Deserialize<'de> for UnloadWalletResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct ConditionalResponseVisitor;

        #[allow(clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for ConditionalResponseVisitor {
            type Value = UnloadWalletResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or object")
            }

            fn visit_str<E>(self, _v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UnloadWalletResponse { warnings: None })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut warnings = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "warnings" {
                        if warnings.is_some() {
                            return Err(de::Error::duplicate_field("warnings"));
                        }
                        warnings = Some(map.next_value::<serde_json::Value>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                Ok(UnloadWalletResponse { warnings })
            }
        }

        deserializer.deserialize_any(ConditionalResponseVisitor)
    }
}

/// Response for the `Uptime` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UptimeResponse {
    /// Wrapped primitive value
    pub value: u64,
}

impl<'de> serde::Deserialize<'de> for UptimeResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = UptimeResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UptimeResponse { value: v })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UptimeResponse { value: v as u64 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UptimeResponse { value: v as u64 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<u64>().map_err(de::Error::custom)?;
                Ok(UptimeResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UptimeResponse { value: v as u64 })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(UptimeResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for UptimeResponse {
    type Target = u64;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for UptimeResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<u64> for UptimeResponse {
    fn as_ref(&self) -> &u64 { &self.value }
}

impl From<u64> for UptimeResponse {
    fn from(value: u64) -> Self { Self { value } }
}

impl From<UptimeResponse> for u64 {
    fn from(wrapper: UptimeResponse) -> Self { wrapper.value }
}

/// Response for the `UtxoUpdatePsbt` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UtxoUpdatePsbtResponse {
    /// Wrapped primitive value
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for UtxoUpdatePsbtResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = UtxoUpdatePsbtResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UtxoUpdatePsbtResponse { value: v.to_string() })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UtxoUpdatePsbtResponse { value: v.to_string() })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UtxoUpdatePsbtResponse { value: v.to_string() })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UtxoUpdatePsbtResponse { value: v.to_string() })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(UtxoUpdatePsbtResponse { value: v.to_string() })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(UtxoUpdatePsbtResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for UtxoUpdatePsbtResponse {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for UtxoUpdatePsbtResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<String> for UtxoUpdatePsbtResponse {
    fn as_ref(&self) -> &String { &self.value }
}

impl From<String> for UtxoUpdatePsbtResponse {
    fn from(value: String) -> Self { Self { value } }
}

impl From<UtxoUpdatePsbtResponse> for String {
    fn from(wrapper: UtxoUpdatePsbtResponse) -> Self { wrapper.value }
}

/// Response for the `ValidateAddress` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ValidateAddressResponse {
    /// The bitcoin address validated
    pub address: Option<String>,
    /// Error message, if any
    pub error: Option<String>,
    /// Indices of likely error locations in address, if known (e.g. Bech32 errors)
    pub error_locations: Option<serde_json::Value>,
    /// If the key is a script
    pub isscript: Option<bool>,
    /// If the address is valid or not
    pub isvalid: bool,
    /// If the address is a witness address
    pub iswitness: Option<bool>,
    /// The hex-encoded output script generated by the address
    pub scriptPubKey: Option<bitcoin::ScriptBuf>,
    /// The hex value of the witness program
    pub witness_program: Option<String>,
    /// The version number of the witness program
    pub witness_version: Option<u64>,
}

/// Response for the `VerifyChain` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VerifyChainResponse {
    /// Wrapped primitive value
    pub value: bool,
}

impl<'de> serde::Deserialize<'de> for VerifyChainResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = VerifyChainResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(VerifyChainResponse { value: v != 0 })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(VerifyChainResponse { value: v != 0 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(VerifyChainResponse { value: v != 0.0 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<bool>().map_err(de::Error::custom)?;
                Ok(VerifyChainResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(VerifyChainResponse { value: v })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(VerifyChainResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for VerifyChainResponse {
    type Target = bool;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for VerifyChainResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<bool> for VerifyChainResponse {
    fn as_ref(&self) -> &bool { &self.value }
}

impl From<bool> for VerifyChainResponse {
    fn from(value: bool) -> Self { Self { value } }
}

impl From<VerifyChainResponse> for bool {
    fn from(wrapper: VerifyChainResponse) -> Self { wrapper.value }
}

/// Response for the `VerifyMessage` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VerifyMessageResponse {
    /// Wrapped primitive value
    pub value: bool,
}

impl<'de> serde::Deserialize<'de> for VerifyMessageResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = VerifyMessageResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(VerifyMessageResponse { value: v != 0 })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(VerifyMessageResponse { value: v != 0 })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(VerifyMessageResponse { value: v != 0.0 })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = v.parse::<bool>().map_err(de::Error::custom)?;
                Ok(VerifyMessageResponse { value })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(VerifyMessageResponse { value: v })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(VerifyMessageResponse { value })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for VerifyMessageResponse {
    type Target = bool;
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for VerifyMessageResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<bool> for VerifyMessageResponse {
    fn as_ref(&self) -> &bool { &self.value }
}

impl From<bool> for VerifyMessageResponse {
    fn from(value: bool) -> Self { Self { value } }
}

impl From<VerifyMessageResponse> for bool {
    fn from(wrapper: VerifyMessageResponse) -> Self { wrapper.value }
}

/// Response for the `VerifyTxOutProof` RPC method
///
/// This method returns an array wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VerifyTxOutProofResponse {
    /// Wrapped array value
    pub value: Vec<serde_json::Value>,
}

impl<'de> serde::Deserialize<'de> for VerifyTxOutProofResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Vec::<serde_json::Value>::deserialize(deserializer)?;
        Ok(Self { value })
    }
}

impl From<Vec<serde_json::Value>> for VerifyTxOutProofResponse {
    fn from(value: Vec<serde_json::Value>) -> Self { Self { value } }
}

impl From<VerifyTxOutProofResponse> for Vec<serde_json::Value> {
    fn from(wrapper: VerifyTxOutProofResponse) -> Self { wrapper.value }
}

/// Response for the `WaitForBlock` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WaitForBlockResponse {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: u64,
}

/// Response for the `WaitForBlockHeight` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WaitForBlockHeightResponse {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: u64,
}

/// Response for the `WaitForNewBlock` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WaitForNewBlockResponse {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: u64,
}

/// Response for the `WalletCreateFundedPsbt` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WalletCreateFundedPsbtResponse {
    /// The position of the added change output, or -1
    pub changepos: i64,
    /// Fee in BTC the resulting transaction pays
    #[serde(deserialize_with = "amount_from_btc_float")]
    pub fee: bitcoin::Amount,
    /// The resulting raw transaction (base64-encoded string)
    pub psbt: String,
}

/// Response for the `WalletDisplayAddress` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WalletDisplayAddressResponse {
    /// The address as confirmed by the signer
    pub address: String,
}

/// Response for the `WalletLock` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WalletLockResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for WalletLockResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = WalletLockResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletLockResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletLockResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletLockResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletLockResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletLockResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletLockResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletLockResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(WalletLockResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for WalletLockResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for WalletLockResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for WalletLockResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for WalletLockResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<WalletLockResponse> for () {
    fn from(wrapper: WalletLockResponse) -> Self { wrapper.value }
}

/// Response for the `WalletPassphrase` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WalletPassphraseResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for WalletPassphraseResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = WalletPassphraseResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(WalletPassphraseResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for WalletPassphraseResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for WalletPassphraseResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for WalletPassphraseResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for WalletPassphraseResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<WalletPassphraseResponse> for () {
    fn from(wrapper: WalletPassphraseResponse) -> Self { wrapper.value }
}

/// Response for the `WalletPassphraseChange` RPC method
///
/// This method returns a primitive value wrapped in a transparent struct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WalletPassphraseChangeResponse {
    /// Wrapped primitive value
    pub value: (),
}

impl<'de> serde::Deserialize<'de> for WalletPassphraseChangeResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, Visitor};

        struct PrimitiveWrapperVisitor;

        #[allow(unused_variables, clippy::needless_lifetimes)]
        impl<'de> Visitor<'de> for PrimitiveWrapperVisitor {
            type Value = WalletPassphraseChangeResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a primitive value or an object with 'value' field")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseChangeResponse { value: () })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseChangeResponse { value: () })
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseChangeResponse { value: () })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseChangeResponse { value: () })
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseChangeResponse { value: () })
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseChangeResponse { value: () })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WalletPassphraseChangeResponse { value: () })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "value" {
                        if value.is_some() {
                            return Err(de::Error::duplicate_field("value"));
                        }
                        value = Some(map.next_value::<()>()?);
                    } else {
                        let _ = map.next_value::<de::IgnoredAny>()?;
                    }
                }
                value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(WalletPassphraseChangeResponse { value: () })
            }
        }

        deserializer.deserialize_any(PrimitiveWrapperVisitor)
    }
}

impl std::ops::Deref for WalletPassphraseChangeResponse {
    type Target = ();
    fn deref(&self) -> &Self::Target { &self.value }
}

impl std::ops::DerefMut for WalletPassphraseChangeResponse {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl AsRef<()> for WalletPassphraseChangeResponse {
    fn as_ref(&self) -> &() { &self.value }
}

impl From<()> for WalletPassphraseChangeResponse {
    fn from(value: ()) -> Self { Self { value } }
}

impl From<WalletPassphraseChangeResponse> for () {
    fn from(wrapper: WalletPassphraseChangeResponse) -> Self { wrapper.value }
}

/// Response for the `WalletProcessPsbt` RPC method
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WalletProcessPsbtResponse {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if complete
    pub hex: Option<String>,
    /// The base64-encoded partially signed transaction
    pub psbt: String,
}

/// Deserializer for bitcoin::Amount that handles both float (BTC) and integer (satoshis) formats
/// Bitcoin Core returns amounts as floats in BTC, but some fields may be integers in satoshis
fn amount_from_btc_float<'de, D>(deserializer: D) -> Result<bitcoin::Amount, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use std::fmt;

    use serde::de::{self, Visitor};

    struct AmountVisitor;

    impl Visitor<'_> for AmountVisitor {
        type Value = bitcoin::Amount;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number (float BTC or integer satoshis)")
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            bitcoin::Amount::from_btc(v)
                .map_err(|e| E::custom(format!("Invalid BTC amount: {}", e)))
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(bitcoin::Amount::from_sat(v))
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v < 0 {
                return Err(E::custom(format!("Amount cannot be negative: {}", v)));
            }
            Ok(bitcoin::Amount::from_sat(v as u64))
        }
    }

    deserializer.deserialize_any(AmountVisitor)
}

/// Deserializer for Option<bitcoin::Amount> that handles both float (BTC) and integer (satoshis) formats
/// Bitcoin Core returns amounts as floats in BTC, but some fields may be integers in satoshis
/// This deserializer also handles null/None values
fn option_amount_from_btc_float<'de, D>(
    deserializer: D,
) -> Result<Option<bitcoin::Amount>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use std::fmt;

    use serde::de::{self, Visitor};

    struct OptionAmountVisitor;

    #[allow(clippy::needless_lifetimes)]
    impl<'de> Visitor<'de> for OptionAmountVisitor {
        type Value = Option<bitcoin::Amount>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an optional number (float BTC or integer satoshis)")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            amount_from_btc_float(deserializer).map(Some)
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            bitcoin::Amount::from_btc(v)
                .map_err(|e| E::custom(format!("Invalid BTC amount: {}", e)))
                .map(Some)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(bitcoin::Amount::from_sat(v)))
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v < 0 {
                return Err(E::custom(format!("Amount cannot be negative: {}", v)));
            }
            Ok(Some(bitcoin::Amount::from_sat(v as u64)))
        }
    }

    deserializer.deserialize_any(OptionAmountVisitor)
}
