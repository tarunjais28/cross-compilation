use super::*;

#[allow(unused)]
#[repr(C)]
pub struct Points {
    _unused: [u8; 0], // Avoid Rust trying to manage the struct's memory
}

#[link(name = "build.cpp")]
extern "C" {
    pub fn create_point(x: i32, y: i32) -> *mut Point;
    pub fn display_point(point: *mut Point);
    pub fn destroy_point(point: *mut Point);
}
