// The rgb function is incomplete. Complete it so that passing in RGB decimal values will result in a hexadecimal representation being returned. Valid decimal values for RGB are 0 - 255. Any values that fall out of that range must be rounded to the closest valid value.

// Note: Your answer should always be 6 characters long, the shorthand with 3 will not work here.

// Examples (input --> output):

// 255, 255, 255 --> "FFFFFF"
// 255, 255, 300 --> "FFFFFF"
// 0, 0, 0       --> "000000"
// 148, 0, 211   --> "9400D3"

// https://www.codewars.com/kata/513e08acc600c94f01000001/train/rust/6440f640ef815386523899d0

fn rgb(mut r: i32, mut g: i32, mut b: i32) -> String {
    if r<0 {r=0} else if r>255 {r=255} 
    if g<0 {g=0} else if g>255 {g=255}
    if b<0 {b=0} else if b>255 {b=255}
    
    format!("{:02X?}{:02X?}{:02X?}", r,g,b)
  }