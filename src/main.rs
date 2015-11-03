extern crate libc;


pub enum Sqlite4Env {}
pub enum Sqlite4 {}

#[link(name = "sqlite4")]
extern {
    fn sqlite4_close() -> i32;
    fn sqlite4_open(pEnv: *mut Sqlite4Env, filename: &str, handle: *mut Sqlite4) -> i32;
    fn sqlite4_env_default() -> *mut Sqlite4Env;
}

pub fn db_close() -> i32{
    unsafe {
        sqlite4_close()
    }
}

pub fn db_open(pEnv: *mut Sqlite4Env, filename: &str, handle: *mut Sqlite4) -> i32{
    unsafe {
        sqlite4_open(env, filename, handle)
    }
}

fn main() {
    let mut rust_object = Box::new(Sqlite4);
    let env = unsafe{ sqlite4_env_default() };
    println!("status {:?}",db_open(env,"test.db",rust_object));
}
