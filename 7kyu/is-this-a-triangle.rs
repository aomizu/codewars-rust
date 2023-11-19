// Implement a function that accepts 3 integer values a, b, c. The function should return true if a triangle can be built with the sides of given length and false in any other case.

// (In this case, all triangles must have surface greater than 0 to be accepted).

// https://www.codewars.com/kata/56606694ec01347ce800001b/train/rust/64358047870c66001af7fa32

fn is_triangle(a: i64, b: i64, c: i64) -> bool {
    if (a+b) > c && (b+c) > a && (a+c) > b {
        true
    } else {
        false
    }
}