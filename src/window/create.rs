pub(super) fn get_display(
    instance: &mut alexandria::Instance,
    adapter: Option<&str>,
    display: Option<&str>,
) -> Result<alexandria::Display, alexandria::Error> {
    let mut adapter = get_adapter(instance, adapter)?;

    let display_name = match display {
        Some(display_name) => display_name,
        None => return adapter.default_display(),
    };

    for display in adapter.enum_displays()? {
        let display = display?;

        if display.name() == display_name {
            return Ok(display);
        }
    }

    adapter.default_display()
}

fn get_adapter(
    instance: &mut alexandria::Instance,
    adapter: Option<&str>,
) -> Result<alexandria::Adapter, alexandria::Error> {
    let adapter_name = match adapter {
        Some(adapter_name) => adapter_name,
        None => return instance.default_adapter(),
    };

    for adapter in instance.enum_adapters()? {
        let adapter = adapter?;

        if adapter.name() == adapter_name {
            return Ok(adapter);
        }
    }

    instance.default_adapter()
}