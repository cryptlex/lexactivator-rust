use std::ffi::{ c_char, CStr, CString, NulError};

// --------------------------- String operations ------------------------

pub fn string_to_cstring(mut rust_string: String) -> Result<CString, NulError> {
    if rust_string.contains('\0') {
        rust_string = rust_string.replace('\0', "");
    }
    let c_string = CString::new(rust_string)?;
    Ok(c_string)
}

pub fn to_utf16(rust_string: String) -> Vec<u16> {
    let utf16: Vec<u16> = rust_string
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    utf16
}

pub fn utf16_to_string(buffer: &[u16]) -> String {
    let string = String::from_utf16_lossy(buffer);
    string.trim_end_matches('\0').to_owned()
}
pub fn c_char_to_string(buffer: &[c_char]) -> String {
    let c_str = unsafe { CStr::from_ptr(buffer.as_ptr()) };
    c_str.to_string_lossy().into_owned()
}

pub fn u32_to_bool(value: u32) -> bool {
    value != 0
}
