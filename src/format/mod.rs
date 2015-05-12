pub use ::util::sample_format::SampleFormat as Sample;
pub use ::util::pixel_format::PixelFormat as Pixel;

use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::*;

pub fn version() -> u32 {
	unsafe {
		avformat_version()
	}
}

pub fn configuration() -> &'static str {
	unsafe {
		from_utf8_unchecked(CStr::from_ptr(avformat_configuration()).to_bytes())
	}
}

pub fn license() -> &'static str {
	unsafe {
		from_utf8_unchecked(CStr::from_ptr(avformat_license()).to_bytes())
	}
}
