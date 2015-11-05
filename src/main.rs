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
pub enum Sqlite4Stmt {}
pub enum Sqlite4Context {}

#[link(name = "sqlite4")]
extern {
    fn sqlite4_libversion_number() -> i32;
    fn sqlite4_libversion() -> CString;
    fn sqlite4_sourceid() -> CString;

    fn sqlite4_compileoption_used(zOptName: CString) -> i32;
    fn sqlite4_compileoption_get(N: i32) -> CString;

    fn sqlite4_initialize(p_env: *mut Sqlite4Env) -> i32;
    fn sqlite4_shutdown(p_env: *mut Sqlite4Env) -> i32;
    fn sqlite4_threadsafe(p_env: *mut Sqlite4Env) -> i32;

    fn sqlite4_complete(sql: CString) -> i32;
    fn sqlite4_limit(handle: *mut Sqlite4, id: i32, newVal: i32) -> i32;

    fn sqlite4_errcode(handle: *mut Sqlite4) -> i32;
    fn sqlite4_errmsg(handle: *mut Sqlite4) -> CString;
    fn sqlite4_db_config(handle: *mut Sqlite4, op: i32) -> i32;
    fn sqlite4_db_env(handle: *mut Sqlite4) ->  *mut Sqlite4Env;
    fn sqlite4_changes(handle: *mut Sqlite4) -> i32;
    fn sqlite4_total_changes(handle: *mut Sqlite4) -> i32;
    fn sqlite4_interrupt(handle: *mut Sqlite4);

    fn sqlite4_stmt_sql(handle: *mut Sqlite4Stmt) -> CString;
    fn sqlite4_stmt_readonly(handle: *mut Sqlite4Stmt) -> i32;
    fn sqlite4_stmt_busy(handle: *mut Sqlite4Stmt) -> i32;

    //SQLITE4_API int sqlite4_bind_blob(sqlite4_stmt*, int, const void*, int n, void(*)(void*,void*),void*);
    //SQLITE4_API int sqlite4_bind_text(sqlite4_stmt*, int, const char*, int n, void(*)(void*,void*),void*);
    //SQLITE4_API int sqlite4_bind_text16(sqlite4_stmt*, int, const void*, int, void(*)(void*,void*),void*);
    //SQLITE4_API int sqlite4_bind_value(sqlite4_stmt*, int, const sqlite4_value*);
    fn sqlite4_bind_double(handle: *mut Sqlite4Stmt, a: i32, b: f64) -> i32;
    fn sqlite4_bind_int(handle: *mut Sqlite4Stmt, a: i32, b: i32) -> i32;
    fn sqlite4_bind_int64(handle: *mut Sqlite4Stmt, a: i32, b: i64) -> i32;
    fn sqlite4_bind_null(handle: *mut Sqlite4Stmt, a: i32) -> i32;
    fn sqlite4_bind_parameter_count(handle: *mut Sqlite4Stmt) -> i32;
    fn sqlite4_bind_parameter_name(handle: *mut Sqlite4Stmt, a: i32) -> CString;
    fn sqlite4_bind_parameter_index(handle: *mut Sqlite4Stmt, name: CString) -> i32;
    fn sqlite4_clear_bindings(handle: *mut Sqlite4Stmt) -> i32;

    fn sqlite4_column_count(handle: *mut Sqlite4Stmt) -> i32;
    fn sqlite4_column_name(handle: *mut Sqlite4Stmt, n: i32) -> CString;
    fn sqlite4_column_database_name(handle: *mut Sqlite4Stmt, n: i32) -> CString;
    fn sqlite4_column_table_name(handle: *mut Sqlite4Stmt, n: i32) -> CString;
    fn sqlite4_column_origin_name(handle: *mut Sqlite4Stmt, n: i32) -> CString;
    fn sqlite4_column_decltype(handle: *mut Sqlite4Stmt, n: i32) -> CString;
    fn sqlite4_step(handle: *mut Sqlite4Stmt) -> i32;
    fn sqlite4_data_count(handle: *mut Sqlite4Stmt) -> i32;

    fn sqlite4_uri_parameter(filename: CString, param: CString) -> CString;
    fn sqlite4_uri_boolean(filename: CString, param: CString, default: i32) -> i32;
    fn sqlite4_uri_int64(filename: CString, param: CString, default: i64) -> i64;

    fn sqlite4_close(handle: *mut Sqlite4, flags: uint32_t) -> i32;
    fn sqlite4_open(p_env: *mut Sqlite4Env, filename: CString, handle: *mut *mut Sqlite4, flag: i32) -> i32;

    fn sqlite4_env_default() -> *mut Sqlite4Env;
    fn sqlite4_env_size() -> i32;
    fn sqlite4_env_config(p_env: *mut Sqlite4Env, op: i32) -> i32;

    fn sqlite4_mprintf(p_env: *mut Sqlite4Env, data: CString) -> CString;
}


pub fn db_close(handle: *mut Sqlite4) -> i32{
    unsafe {
        sqlite4_close(handle, 0)
    }
}

pub fn db_open(p_env: *mut Sqlite4Env, filename: CString, handle: *mut *mut Sqlite4) -> i32{
    unsafe {
        sqlite4_open(p_env, filename, handle, 0)
    }
}

fn main() {
    let version =  unsafe { sqlite4_libversion_number() };
    println!("version {:?}",version);
    let env = unsafe { sqlite4_env_default() };
    let test = unsafe { sqlite4_env_config(env,0) };
    println!("status {:?}",test);
    let mut sqlite = ptr::null_mut();
    let db_name = CString::new(":memory:").unwrap();
    let i = db_open(env, db_name, &mut sqlite);
    println!("status {:?}", i);
    //db_close(*sqlite);
}
