#[link(name = "build.c")]
extern "C" {
    fn greet();
    fn add(a: i32, b: i32) -> i32;
}

#[repr(C)] // Ensure memory layout compatibility
struct Point {
    x: i32,
    y: i32,
}

#[link(name = "build.c")]
extern "C" {
    fn get_sum(point: *mut Point) -> i32;
}

#[repr(C)]
struct Points {
    _unused: [u8; 0], // Avoid Rust trying to manage the struct's memory
}

#[link(name = "build.cpp")]
extern "C" {
    fn create_point(x: i32, y: i32) -> *mut Point;
    fn display_point(point: *mut Point);
    fn destroy_point(point: *mut Point);
}

fn main() {
    unsafe {
        greet(); // Calling C function

        let sum = add(3, 4); // Calling C function with arguments
        println!("Sum from C: {}", sum);
    }

    let mut point = Point { x: 10, y: 20 };
    let x_value = unsafe { get_sum(&mut point) };
    println!("sum value from C: {}", x_value);

    unsafe {
        let point = create_point(10, 20);  // Create a C++ object
        display_point(point);              // Call C++ method
        destroy_point(point);              // Free the C++ object
    }
}
