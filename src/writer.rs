use std::error::Error;
use std::str;

use crate::data::Data;

use voca_rs::*;

use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};

pub static INDEX: &[u8] = include_bytes!("templates/index.hbs");
pub static BODY_MULTI: &[u8] = include_bytes!("templates/multi.hbs");
pub static BODY_FULL: &[u8] = include_bytes!("templates/full.hbs");
pub static CSS: &[u8] = include_bytes!("templates/css.hbs");

handlebars_helper!(capitalize: |s: str| case::capitalize(s, false));
handlebars_helper!(upper: |s: str| s.to_uppercase());

pub fn array_length_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let length = h
        .param(0)
        .as_ref()
        .and_then(|v| v.value().as_array())
        .map(|arr| arr.len())
        .ok_or(RenderError::new(
            "Param 0 with 'array' type is required for array_length helper",
        ))?;

    out.write(length.to_string().as_ref())?;

    Ok(())
}

pub fn write_data(mut writer: impl std::io::Write, data: &Data) -> Result<(), Box<dyn Error>> {
    let mut hb = Handlebars::new();

    hb.register_helper("capitalize", Box::new(capitalize));
    hb.register_helper("upper", Box::new(upper));
    hb.register_helper("array_length", Box::new(array_length_helper));

    let index = str::from_utf8(INDEX)?;
    let body_multi = str::from_utf8(BODY_MULTI)?;
    let body_full = str::from_utf8(BODY_FULL)?;
    let css = str::from_utf8(CSS)?;

    hb.register_template_string("index", index)?;
    hb.register_template_string("multi", body_multi)?;
    hb.register_template_string("full", body_full)?;
    hb.register_template_string("css", css)?;

    let contents = hb.render("index", &data)?;
    let data = contents.as_bytes();
    let mut pos = 0;

    while pos < data.len() {
        let bytes_written = writer.write(&data[pos..])?;
        pos += bytes_written;
    }

    return Ok(());
}
