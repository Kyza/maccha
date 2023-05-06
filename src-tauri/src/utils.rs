use std::ffi::{c_char, CStr, CString};

pub fn ptr_to_string(c_buf: *const c_char) -> String {
	let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
	let str_slice: &str =
		c_str.to_str().expect("CStr was not a valid string");
	let str_buf: String = str_slice.to_owned();
	str_buf
}

pub fn str_to_ptr(str: &str) -> *const c_char {
	CString::new(str)
		.expect("Failed to convert &str to CString pointer")
		.into_raw()
}
