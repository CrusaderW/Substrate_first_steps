fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.2 / 32.8;
    //println!("Quotient is: {}", quotient);

    // remainder
    let remainder = 43 % 5;

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // magic with tuples
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    //array
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}

