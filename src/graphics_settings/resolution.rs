#[cfg(feature = "json")]
use super::InvalidSetting;

#[derive(Clone, Copy)]
pub struct Resolution(alexandria::Resolution);

impl Resolution {
    #[inline]
    pub fn new(width: usize, height: usize) -> Self {
        Resolution(alexandria::Resolution::new(width, height))
    }

    #[inline]
    pub fn parse(resolution: &str) -> Option<Self> {
        alexandria::Resolution::parse(resolution).map(|resolution| Resolution(resolution))
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.0.width()
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.0.height()
    }

    #[inline]
    pub fn set_width(&mut self, width: usize) {
        self.0.set_width(width)
    }

    #[inline]
    pub fn set_height(&mut self, height: usize) {
        self.0.set_height(height)
    }

    #[inline]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub(crate) fn into_inner(self) -> alexandria::Resolution {
        self.0
    }
}

#[cfg(feature = "json")]
impl json::Serialize for Resolution {
    fn serialize_ref(&self) -> json::Value {
        json::Value::String(self.to_string())
    }
}

#[cfg(feature = "json")]
impl json::Deserialize for Resolution {
    fn deserialize(value: json::Value, key: Option<&str>) -> Result<Self, json::Error> {
        Resolution::parse(&String::deserialize(value, key)?)
            .ok_or(json::Error::Other(Box::new(InvalidSetting("resolution"))))
    }
}
