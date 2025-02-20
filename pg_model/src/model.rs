use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};
pub use postgres_from_row::FromRow;

#[cfg(feature = "enum")]
pub use r#enum::*;

#[cfg(feature = "enum")]
mod r#enum {
    use postgres_types::private::BytesMut;
    use postgres_types::{FromSql, IsNull, ToSql, Type, to_sql_checked};
    use std::error::Error;
    use std::fmt::{Display, Formatter};
    use std::str::FromStr;
    use {
        duplicate::duplicate_item,
        num_enum::{IntoPrimitive, TryFromPrimitive},
        serde_repr::{Deserialize_repr, Serialize_repr},
        serde_with::{DeserializeFromStr, SerializeDisplay},
    };

    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        Serialize_repr,
        Deserialize_repr,
        IntoPrimitive,
        TryFromPrimitive,
    )]
    #[repr(i32)]
    pub enum StatusCode {
        Pending = 0,
        Success = 200,
        Fail = -32000,
        Timeout = 500,
        Retrying = 100,
    }

    impl Display for StatusCode {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let v: i32 = self.clone().into();
            write!(f, "{}", v)
        }
    }

    impl FromSql<'_> for StatusCode {
        fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
            let v = i32::from_sql(ty, raw)?;
            Self::try_from(v).map_err(Into::into)
        }
        fn accepts(_ty: &Type) -> bool {
            true
        }
    }

    impl ToSql for StatusCode {
        fn to_sql(
            &self,
            ty: &Type,
            out: &mut BytesMut,
        ) -> Result<IsNull, Box<dyn Error + Sync + Send>>
        where
            Self: Sized,
        {
            let v: i32 = self.clone().into();
            v.to_sql(ty, out)
        }

        fn accepts(_ty: &Type) -> bool
        where
            Self: Sized,
        {
            true
        }

        to_sql_checked!();
    }

    /// serde_with 将strum::Display与serde关联起来。
    #[derive(
        Clone, Debug, SerializeDisplay, DeserializeFromStr, strum::Display, strum::EnumString,
    )]
    #[strum(serialize_all = "UPPERCASE")]
    pub enum TokenCode {
        JPY,
        USD,
        EUR,
        CNY,
        GBP,
        HKD,
        KRW,
    }

    #[derive(
        Clone, Debug, SerializeDisplay, DeserializeFromStr, strum::Display, strum::EnumString,
    )]
    #[strum(serialize_all = "lowercase")]
    pub enum StatusChoice {
        Pending,
        Success,
        Fail,
        Timeout,
        Retrying,
        Suspend,
    }

    #[duplicate_item(type_name; [TokenCode]; [StatusChoice])]
    impl FromSql<'_> for type_name {
        fn from_sql(_ty: &Type, raw: &[u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
            Self::from_str(std::str::from_utf8(raw)?).map_err(Into::into)
        }

        fn accepts(_ty: &Type) -> bool {
            true
        }
    }

    #[duplicate_item(type_name; [TokenCode]; [StatusChoice])]
    impl ToSql for type_name {
        fn to_sql(
            &self,
            ty: &Type,
            out: &mut BytesMut,
        ) -> Result<IsNull, Box<dyn Error + Sync + Send>>
        where
            Self: Sized,
        {
            self.to_string().to_sql(ty, out)
        }

        fn accepts(_ty: &Type) -> bool
        where
            Self: Sized,
        {
            true
        }

        to_sql_checked!();
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct TransactionPool {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub request_time: Option<DateTime<Utc>>,
    pub success_time: Option<DateTime<Utc>>,
    pub block_number: Option<i64>,
    #[cfg(feature = "enum")]
    pub status: StatusChoice,
    #[cfg(not(feature = "enum"))]
    pub status: String,
    pub status_code: i32,
    pub fail_reason: Option<String>,
    pub nonce: Option<i64>,
    pub gas: Option<i64>,
    pub tx_hash: Option<String>,
    pub from_user_id: String,
    pub to_user_id: String,
    pub coin_code: String,
    pub point: f64,
    pub tag_id: String,
    pub store_id: Option<String>,
    pub gen_time: String,
    pub ext_json: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionPoolInsert {
    pub request_time: Option<DateTime<Utc>>,
    pub success_time: Option<DateTime<Utc>>,
    pub block_number: Option<i64>,
    pub status_code: i32,
    pub fail_reason: Option<String>,
    pub nonce: Option<i64>,
    pub gas: Option<i64>,
    pub tx_hash: Option<String>,
    pub from_user_id: String,
    pub to_user_id: String,
    pub coin_code: String,
    pub point: f64,
    pub tag_id: String,
    pub store_id: Option<String>,
    pub gen_time: String,
    pub ext_json: String,
}
