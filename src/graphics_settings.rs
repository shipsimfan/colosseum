#[cfg_attr(feature = "json", derive(json::Deserialize, json::Serialize))]
pub struct GraphicsSettings {
    adapter: Option<String>,
    display: Option<String>,
    resolution: Option<Resolution>,
    refresh_rate: Option<RefreshRate>,
}

#[derive(Clone, Copy)]
pub struct RefreshRate(alexandria::RefreshRate);

#[derive(Clone, Copy)]
pub struct Resolution(alexandria::Resolution);

#[cfg(feature = "json")]
struct InvalidSetting(&'static str);

impl GraphicsSettings {
    pub fn new(
        adapter: Option<String>,
        display: Option<String>,
        resolution: Option<Resolution>,
        refresh_rate: Option<RefreshRate>,
    ) -> Self {
        GraphicsSettings {
            adapter,
            display,
            resolution,
            refresh_rate,
        }
    }

    pub fn adapter(&self) -> Option<&str> {
        self.adapter.as_ref().map(|adapter| adapter.as_str())
    }

    pub fn display(&self) -> Option<&str> {
        self.display.as_ref().map(|display| display.as_str())
    }

    pub fn resolution(&self) -> Option<Resolution> {
        self.resolution
    }

    pub fn refresh_rate(&self) -> Option<RefreshRate> {
        self.refresh_rate
    }

    pub fn set_adapter(&mut self, adapter: Option<String>) {
        self.adapter = adapter;
    }

    pub fn set_display(&mut self, display: Option<String>) {
        self.display = display;
    }

    pub fn set_resolution(&mut self, resolution: Option<Resolution>) {
        self.resolution = resolution;
    }

    pub fn set_refresh_rate(&mut self, refresh_rate: Option<RefreshRate>) {
        self.refresh_rate = refresh_rate;
    }

    pub fn resolution_mut(&mut self) -> Option<&mut Resolution> {
        self.resolution.as_mut()
    }
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        GraphicsSettings {
            adapter: None,
            display: None,
            resolution: None,
            refresh_rate: None,
        }
    }
}

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

impl RefreshRate {
    #[inline]
    pub fn parse(refresh_rate: &str) -> Option<Self> {
        alexandria::RefreshRate::parse(refresh_rate).map(|refresh_rate| RefreshRate(refresh_rate))
    }

    #[inline]
    pub fn to_string(&self) -> String {
        self.0.to_string()
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

#[cfg(feature = "json")]
impl std::error::Error for InvalidSetting {}

#[cfg(feature = "json")]
impl std::fmt::Debug for InvalidSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

#[cfg(feature = "json")]
impl std::fmt::Display for InvalidSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid {}", self.0)
    }
}
