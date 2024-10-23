mod c;
mod cpp;
mod structs;

use crate::{c::*, cpp::*, structs::*};

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
        let point = create_point(10, 20); // Create a C++ object
        display_point(point); // Call C++ method
        destroy_point(point); // Free the C++ object
    }
}
