use std::fs::File;

#[no_mangle]
pub unsafe extern "C" fn close(file: *mut File) {
    Box::from_raw(file);
}
