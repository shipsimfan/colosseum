#[cfg_attr(feature = "json", derive(json::Deserialize, json::Serialize))]
pub struct GraphicsSettings {
    adapter: Option<String>,
    display: Option<String>,
}

impl GraphicsSettings {
    pub fn new(adapter: Option<String>, display: Option<String>) -> Self {
        GraphicsSettings { adapter, display }
    }

    pub fn adapter(&self) -> Option<&str> {
        self.adapter.as_ref().map(|adapter| adapter.as_str())
    }

    pub fn display(&self) -> Option<&str> {
        self.display.as_ref().map(|display| display.as_str())
    }

    pub fn set_adapter(&mut self, adapter: Option<String>) {
        self.adapter = adapter;
    }

    pub fn set_display(&mut self, display: Option<String>) {
        self.display = display;
    }
}
