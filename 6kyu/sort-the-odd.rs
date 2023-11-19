// Task

// You will be given an array of numbers. You have to sort the odd numbers in ascending order while leaving the even numbers at their original positions.

// Examples

// [7, 1]  =>  [1, 7]
// [5, 8, 6, 3, 4]  =>  [3, 8, 6, 5, 4]
// [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]  =>  [1, 8, 3, 6, 5, 4, 7, 2, 9, 0]

// https://www.codewars.com/kata/578aa45ee9fd15ff4600090d/train/rust/6437d0d2193b9b005db7e28c

fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut odd_nums:Vec<i32> = vec![];
    let mut sorted:Vec<i32> = vec![];
    for i in arr {
        if !(i % 2 == 0) {
            odd_nums.push(*i);
        }
    }
    odd_nums.sort();
    odd_nums.reverse();
    for i in arr {
        if i % 2 == 0 {
            sorted.push(*i);
        } else {
            sorted.push(odd_nums.pop().unwrap());
        }
    }
    sorted
}