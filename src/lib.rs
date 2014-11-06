extern crate libc;

use std::c_str::CString;

#[repr(C)]
type uuid_t = [libc::c_char, ..16];

#[repr(C)]
type r_uuid_t = [libc::c_char, ..37];

fn uuid_t() -> uuid_t {
    [0, ..16]
}

fn r_uuid_t() -> r_uuid_t {
    [0, ..37]
}

#[link(name = "uuid")]
extern {
    fn uuid_generate(out: *mut libc::c_char);
    fn uuid_unparse(uu: *const libc::c_char, out: *mut libc::c_char);
}

pub fn gen_uuid() -> String {
    let mut tmp = uuid_t();
    let mut out = r_uuid_t();
    unsafe {
        uuid_generate(tmp.as_mut_ptr());
        uuid_unparse(tmp.as_ptr(), out.as_mut_ptr());
    }
    String::from_str(unsafe{
        CString::new(out.as_ptr(), false).as_str().unwrap()
    })
}
