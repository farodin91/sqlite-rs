extern crate libc;

use std::ptr;
use std::ffi::{CString};
use libc::{c_int, c_uchar, c_char, uint8_t, uint32_t, int32_t};

#[repr(C)]
pub struct sqlite4_num {
    pub sign: c_uchar,
    pub approx: c_uchar,
}

pub enum Sqlite4Env {}
pub enum Sqlite4 {}

#[link(name = "sqlite4")]
extern {
    fn sqlite4_close() -> i32;
    fn sqlite4_env_size() -> i32;

    fn sqlite4_libversion_number() -> i32;
    fn sqlite4_libversion() -> CString;
    fn sqlite4_sourceid() -> CString;

    fn sqlite4_compileoption_used(zOptName: &str) -> i32;
    fn sqlite4_compileoption_get(N: i32) -> &str;

    fn sqlite4_initialize(pEnv: *mut Sqlite4Env) -> i32;
    fn sqlite4_shutdown(pEnv: *mut Sqlite4Env) -> i32;

    fn sqlite4_complete(sql: CString) -> i32;
    fn sqlite4_limit(handle: *mut Sqlite4, id: i32, newVal: i32) -> i32;

    fn sqlite4_errcode(handle: *mut Sqlite4) -> i32;
    fn sqlite4_errmsg(handle: *mut Sqlite4) -> i32;

    fn sqlite4_open(pEnv: *mut Sqlite4Env, filename: CString, handle: *mut Sqlite4) -> i32;
    fn sqlite4_env_default() -> *mut Sqlite4Env;
}

pub fn db_close() -> i32{
    unsafe {
        sqlite4_close()
    }
}

pub fn db_open(pEnv: *mut Sqlite4Env, filename: CString, handle: *mut Sqlite4) -> i32{
    unsafe {
        sqlite4_open(pEnv, filename, handle)
    }
}

fn main() {
    //let mut rust_object = Sqlite4 {};
    let env = unsafe{ sqlite4_env_default() };
    let mut sqlite: *mut Sqlite4 = ptr::null_mut();

    let db_name = CString::new("test.db").unwrap();
    let i = db_open(env, db_name, sqlite);
    let version =  unsafe{sqlite4_libversion_number()};
    println!("status {:?}",version);
}
