// Define error and result types, that's all for this lib :-)

use std::error::Error;

pub type DynError = Box<dyn Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, DynError>;
