fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let spaces = "    ";
    let spaces = spaces.len();

    /*
    scalar types represent a single value, rust has 4 primary scalar types:
    - Integers
    - Floating point
    - Booleans
    - Characters
    */

    let my_iint = 10;
    let my_float = 10.0;
    let my_bool = false;
    let my_char = 'z';

    // compound types

    let tup = (500, 6.4, true);
    let (x, y, z) = tup;
    let y = tup.1;

    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // [3,3,3,3,3]

    another_function(x);

    // statements do not return a value
    // expressions return a value

    let y = {
        let x = 3;
        x + 1
    };

    let five = five();

    if y < five {
        println!("y > five");
    } else {
        println!("five >= y");
    }

    let mut number = if true { 5 } else { 6 };

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function(x: i32) {
    println!("the value of x is {}", x);
}

fn five() -> i32 {
    5
}
