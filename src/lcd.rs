//! This module implements various functions and methods to interact with
//! an LCD character display, starting with the low-level [`Driver`].
//!
//! # Glossary
//!
//! Throughout the docs some terms will be used to refer to components of the
//! LCD controller, here is a list:
//! - **DDROM**: the Driver Data ROM is the read-only memory that contains
//!   all of the predefined charater bitmaps
//! - **DDRAM**: the Driver Data RAM is the part of memory of the display that
//!   stores what is shown as addresses to both the **DDROM** and **CGRAM**
//! - **CGRAM**: the Character Generator RAM is the memory that contains
//!   the user generated characters as bitmaps
//! - **AC**: the Address Counter is a special register of the display which
//!   stores the current address for reading and writing operation happening
//!   on the **DDRAM** and **CGRAM**

use esp_idf_sys::EspError;

mod ddrom;

mod driver;
pub use driver::bus;
pub use driver::cmd;
pub use driver::Driver;
pub use driver::Pins;

pub mod canvas;

mod bitmap;
pub use bitmap::Bitmap;

type Result<T> = core::result::Result<T, EspError>;
