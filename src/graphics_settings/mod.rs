mod refresh_rate;
mod resolution;

pub use refresh_rate::*;
pub use resolution::*;

#[cfg_attr(feature = "json", derive(json::Deserialize, json::Serialize))]
pub struct GraphicsSettings {
    adapter: Option<String>,
    display: Option<String>,
    resolution: Option<Resolution>,
    refresh_rate: Option<RefreshRate>,
}

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
