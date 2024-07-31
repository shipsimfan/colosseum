use crate::LoadSettingsError;
use alexandria::{
    DeviceCreateError, EnumeratePhysicalDevicesError, InstanceCreateError, WindowCreationError,
};

/// An error that occurred while graphics was initializing
#[derive(Debug)]
pub enum GraphicsInitError {
    LoadSettings(LoadSettingsError),
    InstanceCreation(InstanceCreateError),
    WindowCreation(WindowCreationError),
    EnumeratePhysicalDevices(EnumeratePhysicalDevicesError),
    NoSupportedPhysicalDevices,
    DeviceCreation(DeviceCreateError),
}

impl alexandria::Error for GraphicsInitError {
    fn title(&self) -> &'static str {
        "Graphics Initialization Error"
    }
}

impl std::error::Error for GraphicsInitError {}

impl std::fmt::Display for GraphicsInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphicsInitError::LoadSettings(error) => error.fmt(f),
            GraphicsInitError::InstanceCreation(error) => error.fmt(f),
            GraphicsInitError::WindowCreation(error) => error.fmt(f),
            GraphicsInitError::EnumeratePhysicalDevices(error) => error.fmt(f),
            GraphicsInitError::NoSupportedPhysicalDevices => {
                f.write_str("No supported physical devices found")
            }
            GraphicsInitError::DeviceCreation(error) => error.fmt(f),
        }
    }
}

impl From<LoadSettingsError> for GraphicsInitError {
    fn from(error: LoadSettingsError) -> Self {
        GraphicsInitError::LoadSettings(error)
    }
}

impl From<InstanceCreateError> for GraphicsInitError {
    fn from(error: InstanceCreateError) -> Self {
        GraphicsInitError::InstanceCreation(error)
    }
}

impl From<WindowCreationError> for GraphicsInitError {
    fn from(error: WindowCreationError) -> Self {
        GraphicsInitError::WindowCreation(error)
    }
}

impl From<EnumeratePhysicalDevicesError> for GraphicsInitError {
    fn from(error: EnumeratePhysicalDevicesError) -> Self {
        GraphicsInitError::EnumeratePhysicalDevices(error)
    }
}

impl From<DeviceCreateError> for GraphicsInitError {
    fn from(error: DeviceCreateError) -> Self {
        GraphicsInitError::DeviceCreation(error)
    }
}
