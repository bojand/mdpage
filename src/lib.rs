#[macro_use]
extern crate handlebars;

#[macro_use]
extern crate log;

mod content;
mod data;
mod utils;
mod writer;

pub use content::Content;
pub use data::build;
pub use data::Data;
pub use writer::write_data;
