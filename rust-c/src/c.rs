use super::*;

#[link(name = "build.c")]
extern "C" {
    pub fn greet();
    pub fn add(a: i32, b: i32) -> i32;
}

#[link(name = "build.c")]
extern "C" {
    pub fn get_sum(point: *mut Point) -> i32;
}
