use crate::{
    Error, Result,
    graphics::{OutputResolution, context::SWAPCHAIN_FORMAT},
    math::{Rational, Vector2u, number::IntoF32},
};
use std::{cmp::Ordering, ptr::null_mut};
use win32::{ComPtr, dxgi1_2::IDXGIOutput1, try_hresult};

impl OutputResolution {
    /// Enumerates the available resolutions for `output`
    pub(in crate::graphics::adapter::output) fn enumerate(
        output: &mut ComPtr<IDXGIOutput1>,
    ) -> Result<Vec<Self>> {
        // Get the list of formats
        let mut num_formats = 0;
        try_hresult!(output.get_display_mode_list1(
            SWAPCHAIN_FORMAT,
            0,
            &mut num_formats,
            null_mut()
        ))
        .map_err(|os| Error::new_inner("unable to enumerate output resolutions", os))?;

        let mut formats = Vec::with_capacity(num_formats as _);
        try_hresult!(output.get_display_mode_list1(
            SWAPCHAIN_FORMAT,
            0,
            &mut num_formats,
            formats.as_mut_ptr()
        ))
        .map_err(|os| Error::new_inner("unable to enumerate output resolutions", os))?;
        unsafe { formats.set_len(num_formats as _) };

        // Process the formats
        let mut resolutions: Vec<OutputResolution> = Vec::new();
        'format_loop: for format in formats {
            let size = Vector2u::new(format.width, format.height);
            let refresh_rate = Rational::new(
                format.refresh_rate.numerator,
                format.refresh_rate.denominator,
            );

            for resolution in &mut resolutions {
                if resolution.size != size {
                    continue;
                }

                for old_refresh_rates in &resolution.refresh_rates {
                    if old_refresh_rates.numerator == refresh_rate.numerator
                        && old_refresh_rates.denominator == refresh_rate.denominator
                    {
                        continue 'format_loop;
                    }
                }

                resolution.refresh_rates.push(refresh_rate);
                continue 'format_loop;
            }

            resolutions.push(OutputResolution {
                size,
                refresh_rates: vec![refresh_rate],
            })
        }

        // Sort refresh rates and resolutions
        for resolution in &mut resolutions {
            resolution
                .refresh_rates
                .sort_by(|a, b| b.into_f32().partial_cmp(&a.into_f32()).unwrap());
        }

        resolutions.sort_by(|a, b| match b.width().cmp(&a.width()) {
            Ordering::Equal => b.height().cmp(&a.height()),
            ordering => ordering,
        });

        Ok(resolutions)
    }
}
