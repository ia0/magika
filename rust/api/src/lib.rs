use ort as _; // we just need it to download the binaries

#[no_mangle]
pub extern "C" fn magika_all_in_one(ptr: *const u8, len: i32, code: *mut u8, cap: *mut i32) -> i32 {
    let data = unsafe { std::slice::from_raw_parts(ptr, len as usize) };
    let path = unsafe { std::ffi::OsStr::from_encoded_bytes_unchecked(data) };
    let out = unsafe { std::slice::from_raw_parts_mut(code, *cap as usize) };
    fn rust(path: &std::ffi::OsStr) -> Option<&'static str> {
        let session = magika::Session::new().ok()?;
        let result = session.identify_file_sync(path).ok()?;
        Some(result.content_type()?.info().label)
    }
    match rust(path) {
        Some(x) if x.len() <= out.len() => {
            out[..x.len()].copy_from_slice(x.as_bytes());
            unsafe { *cap = x.len() as i32 };
            0
        }
        _ => -1,
    }
}
