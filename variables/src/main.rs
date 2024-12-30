fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The tuple `tup` contains the following values: {x}, {y}, {z}");

    let five_hundred = tup.0;

    println!("The value of first element of `tup` accessed via index: {five_hundred}");

    // array with type and number of elements;
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("First element of a {}", a[0]);

    // array with same element repeated several times;
    let b = [3; 5]; // [5, 5, 5, 5, 5]
    println!("Second element of b {}", b[1]);
}
