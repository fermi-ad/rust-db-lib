use chrono::{DateTime, TimeZone, Utc};

use crate::{DataStoreError, DataVal};

/// Configurable [`DataVal`] returned by a [`TestRow`](super::TestRow).
///
/// Construct a value directly or use `From` conversions for `bool`, `i8`, `i16`, `i32`,
/// `i64`, `f32`, `f64`, `String`, `&str`, or [`DateTime`] in any time zone (stored in UTC).
/// The [`test_data_store!`](super::test_data_store) macro uses these conversions for values
/// other than literal `None`.
///
/// A non-optional decoder returns the matching populated field or a [`DataStoreError`].
/// An optional decoder returns the matching populated field, `None` when [`is_nullable`](Self::is_nullable)
/// is `true`, or an error otherwise. The default has `is_nullable = false` and no populated
/// fields, so an omitted column errors even when read through an optional decoder.
///
/// A null value has no associated type: when `is_nullable` is true and the matching field
/// is empty, any optional decoder returns `None`.
#[derive(Clone, Debug, Default)]
pub struct TestVal {
    pub is_nullable: bool,
    pub test_bool: Option<bool>,
    pub test_datetime: Option<DateTime<Utc>>,
    pub test_i8: Option<i8>,
    pub test_i16: Option<i16>,
    pub test_i32: Option<i32>,
    pub test_i64: Option<i64>,
    pub test_f32: Option<f32>,
    pub test_f64: Option<f64>,
    pub test_string: Option<String>,
}
impl TestVal {
    fn translate<T>(op: Option<T>) -> Result<T, DataStoreError> {
        op.ok_or_else(generate_error)
    }

    fn translate_optional<T>(&self, op: Option<T>) -> Result<Option<T>, DataStoreError> {
        if self.is_nullable || op.is_some() {
            Ok(op)
        } else {
            Err(generate_error())
        }
    }
}
impl DataVal for TestVal {
    fn to_bool(self) -> Result<bool, DataStoreError> {
        Self::translate(self.test_bool)
    }

    fn to_bool_optional(self) -> Result<Option<bool>, DataStoreError> {
        self.translate_optional(self.test_bool)
    }

    fn to_datetime(self) -> Result<DateTime<Utc>, DataStoreError> {
        Self::translate(self.test_datetime)
    }

    fn to_datetime_optional(self) -> Result<Option<DateTime<Utc>>, DataStoreError> {
        self.translate_optional(self.test_datetime)
    }

    fn to_i8(self) -> Result<i8, DataStoreError> {
        Self::translate(self.test_i8)
    }

    fn to_i8_optional(self) -> Result<Option<i8>, DataStoreError> {
        self.translate_optional(self.test_i8)
    }

    fn to_i16(self) -> Result<i16, DataStoreError> {
        Self::translate(self.test_i16)
    }

    fn to_i16_optional(self) -> Result<Option<i16>, DataStoreError> {
        self.translate_optional(self.test_i16)
    }

    fn to_i32(self) -> Result<i32, DataStoreError> {
        Self::translate(self.test_i32)
    }

    fn to_i32_optional(self) -> Result<Option<i32>, DataStoreError> {
        self.translate_optional(self.test_i32)
    }

    fn to_i64(self) -> Result<i64, DataStoreError> {
        Self::translate(self.test_i64)
    }

    fn to_i64_optional(self) -> Result<Option<i64>, DataStoreError> {
        self.translate_optional(self.test_i64)
    }

    fn to_f32(self) -> Result<f32, DataStoreError> {
        Self::translate(self.test_f32)
    }

    fn to_f32_optional(self) -> Result<Option<f32>, DataStoreError> {
        self.translate_optional(self.test_f32)
    }

    fn to_f64(self) -> Result<f64, DataStoreError> {
        Self::translate(self.test_f64)
    }

    fn to_f64_optional(self) -> Result<Option<f64>, DataStoreError> {
        self.translate_optional(self.test_f64)
    }

    fn to_string(self) -> Result<String, DataStoreError> {
        Self::translate(self.test_string)
    }

    fn to_string_optional(self) -> Result<Option<String>, DataStoreError> {
        let local = self.test_string.clone();
        self.translate_optional(local)
    }
}
impl From<bool> for TestVal {
    fn from(value: bool) -> Self {
        Self {
            test_bool: Some(value),
            ..Default::default()
        }
    }
}
impl<Tz: TimeZone> From<DateTime<Tz>> for TestVal {
    fn from(value: DateTime<Tz>) -> Self {
        Self {
            test_datetime: Some(value.with_timezone(&Utc)),
            ..Default::default()
        }
    }
}
impl From<f32> for TestVal {
    fn from(value: f32) -> Self {
        Self {
            test_f32: Some(value),
            ..Default::default()
        }
    }
}
impl From<f64> for TestVal {
    fn from(value: f64) -> Self {
        Self {
            test_f64: Some(value),
            ..Default::default()
        }
    }
}
impl From<i16> for TestVal {
    fn from(value: i16) -> Self {
        Self {
            test_i16: Some(value),
            ..Default::default()
        }
    }
}
impl From<i32> for TestVal {
    fn from(value: i32) -> Self {
        Self {
            test_i32: Some(value),
            ..Default::default()
        }
    }
}
impl From<i64> for TestVal {
    fn from(value: i64) -> Self {
        Self {
            test_i64: Some(value),
            ..Default::default()
        }
    }
}
impl From<i8> for TestVal {
    fn from(value: i8) -> Self {
        Self {
            test_i8: Some(value),
            ..Default::default()
        }
    }
}
impl From<String> for TestVal {
    fn from(value: String) -> Self {
        Self {
            test_string: Some(value),
            ..Default::default()
        }
    }
}
impl From<&str> for TestVal {
    fn from(value: &str) -> Self {
        Self {
            test_string: Some(value.to_string()),
            ..Default::default()
        }
    }
}
impl PartialEq for TestVal {
    fn eq(&self, other: &Self) -> bool {
        self.is_nullable == other.is_nullable
            && self.test_bool == other.test_bool
            && self.test_datetime == other.test_datetime
            && self.test_f32 == other.test_f32
            && self.test_f64 == other.test_f64
            && self.test_i16 == other.test_i16
            && self.test_i32 == other.test_i32
            && self.test_i64 == other.test_i64
            && self.test_i8 == other.test_i8
            && self.test_string == other.test_string
    }
}

fn generate_error() -> DataStoreError {
    DataStoreError {
        details: "No data found for the requested column".to_string(),
    }
}
