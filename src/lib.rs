#![feature(macro_rules)]

extern crate libc;

use std::c_str::CString;
use std::default::Default;
use std::fmt::Show;

#[repr(C)]
type uuid_t = [libc::c_char, ..16];

#[repr(C)]
type r_uuid_t = [libc::c_char, ..37];

#[deriving(PartialEq)]
pub struct Uuid(uuid_t);

fn uuid_t() -> uuid_t {
    [0, ..16]
}

fn r_uuid_t() -> r_uuid_t {
    [0, ..37]
}

impl Show for Uuid {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let &Uuid(uuid) = self;
        try!(write!(fmt, "Uuid("));
        for c in uuid.iter() {
            try!(write!(fmt, "{:02x}", c.clone() as u8));
        }
        try!(write!(fmt, ")"));
        Ok(())
    }
}

impl Default for Uuid {
    fn default() -> Uuid {
        Uuid(uuid_t())
    }
}

impl Clone for Uuid {
    fn clone(&self) -> Uuid {
        let mut uuid = uuid_t();
        let &Uuid(old) = self;

        for i in range(0, 16) {
            uuid[i] = old[i];
        }
        Uuid(uuid)
    }

    fn clone_from(&mut self, source: &Uuid) {
        let &Uuid(old) = source;
        let &Uuid(ref mut new) = self;

        for i in range(0, 16) {
            new[i] = old[i];
        }
    }
}

mod c {
    use super::libc::{c_char, c_int};

    #[link(name = "uuid")]
    extern {
        pub fn uuid_generate(out: *mut c_char);
        pub fn uuid_generate_random(out: *mut c_char);
        pub fn uuid_generate_time(out: *mut c_char);
        pub fn uuid_generate_time_safe(out: *mut c_char) -> c_int;

        pub fn uuid_parse(inp: *const c_char, out: *mut c_char) -> c_int;

        pub fn uuid_unparse(uu: *const c_char, out: *mut c_char);
        pub fn uuid_unparse_lower(uu: *const c_char, out: *mut c_char);
        pub fn uuid_unparse_upper(uu: *const c_char, out: *mut c_char);
    }
}

macro_rules! gen (
    ($f:ident) => ({
        let mut out = uuid_t();
        unsafe { c::$f(out.as_mut_ptr()) };
        Uuid(out)
    })
)
macro_rules! unparse (
    ($f:ident, $uu:ident) => ({
        let mut out = r_uuid_t();
        let Uuid(inp) = $uu;
        unsafe {
            c::$f(inp.as_ptr(), out.as_mut_ptr());
        }
        String::from_str(unsafe{
            CString::new(out.as_ptr(), false).as_str().unwrap()
        })
    })
)

pub fn uuid_generate() -> Uuid {
    gen!(uuid_generate)
}

pub fn uuid_generate_random() -> Uuid {
    gen!(uuid_generate_random)
}

pub fn uuid_generate_time() -> Uuid {
    gen!(uuid_generate_time)
}

pub fn uuid_generate_time_safe() -> (Uuid, bool) {
    let mut out = uuid_t();
    let res = unsafe { c::uuid_generate_time_safe(out.as_mut_ptr()) };
    (Uuid(out), (res == 0))
}

pub fn uuid_parse(s: &str) -> Option<Uuid> {
    let mut out = uuid_t();
    match unsafe {c::uuid_parse(s.as_ptr() as *const libc::c_char, out.as_mut_ptr())} {
        0 => Some(Uuid(out)),
        _ => None,
    }
}

pub fn uuid_unparse(s: Uuid) -> String {
    unparse!(uuid_unparse, s)
}

pub fn uuid_unparse_lower(s: Uuid) -> String {
    unparse!(uuid_unparse_lower, s)
}

pub fn uuid_unparse_upper(s: Uuid) -> String {
    unparse!(uuid_unparse_upper, s)
}

#[cfg(test)]
mod test {
    use super::{uuid_generate, uuid_generate_random, uuid_generate_time, uuid_generate_time_safe};
    use super::{uuid_parse, uuid_unparse, uuid_unparse_lower, uuid_unparse_upper};
    use std::ascii::OwnedAsciiExt;

    #[test]
    fn test_gen() {
        let uuid1 = uuid_generate();
        let uuid2 = uuid_generate();
        assert!(uuid1 != uuid2);

        let uuid3 = uuid1.clone();
        assert_eq!(uuid1, uuid3);

        let uuid4 = uuid_generate_random();
        assert!(uuid4 != uuid1);

        let uuid5 = uuid_generate_time();
        assert!(uuid5 != uuid4);

        let (uuid6, _) = uuid_generate_time_safe();
        assert!(uuid6 != uuid4);
    }

    #[test]
    fn test_parse() {
        let uuid = uuid_generate();
        let u1 = uuid_unparse(uuid);
        assert_eq!(uuid_parse(u1.as_slice()).unwrap(), uuid);

        let u2 = uuid_unparse_lower(uuid);
        assert_eq!(uuid_parse(u2.as_slice()).unwrap(), uuid);
        let u1_copy = u1.clone();
        assert_eq!(u1_copy.into_ascii_lower(), u2.into_ascii_lower());

        let u3 = uuid_unparse_upper(uuid);
        assert_eq!(uuid_parse(u3.as_slice()).unwrap(), uuid);
        assert_eq!(u1.into_ascii_lower(), u3.into_ascii_lower());

    }
}
