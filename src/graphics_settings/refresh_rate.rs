#[cfg(feature = "json")]
use super::InvalidSetting;

#[derive(Clone, Copy)]
pub struct RefreshRate(alexandria::RefreshRate);

impl RefreshRate {
    #[inline]
    pub fn parse(refresh_rate: &str) -> Option<Self> {
        alexandria::RefreshRate::parse(refresh_rate).map(|refresh_rate| RefreshRate(refresh_rate))
    }

    #[inline]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub(crate) fn into_inner(self) -> alexandria::RefreshRate {
        self.0
    }
}

#[cfg(feature = "json")]
impl json::Serialize for RefreshRate {
    fn serialize_ref(&self) -> json::Value {
        json::Value::String(self.to_string())
    }
}

#[cfg(feature = "json")]
impl json::Deserialize for RefreshRate {
    fn deserialize(value: json::Value, key: Option<&str>) -> Result<Self, json::Error> {
        RefreshRate::parse(&String::deserialize(value, key)?)
            .ok_or(json::Error::Other(Box::new(InvalidSetting("refresh rate"))))
    }
}
