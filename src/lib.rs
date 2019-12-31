#[macro_use] 
extern crate handlebars;

#[macro_use]
extern crate log;

mod content;
mod utils;
mod data;
mod writer;

pub use content::Content;
pub use data::Data;
pub use data::build;
pub use writer::write_data;