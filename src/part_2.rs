pub fn f1() {
    let age: i32 = 123;
    let name = "Tom";
    println!("Name: {}, Age: {}", name, age);
}

pub fn f2() {
    let mut a = 52;
    println!("a: {}", a);
    a = 25;
    println!("a: {}", a);

    let a = 5;
    println!("a: {}", a);
    let a = a + 5;
    println!("a: {}", a);
    let a = 'a';
    println!("a: {}", a);


    let b: i8 = 127;
    let b: i32 = 1_000_000_000;
    let b: isize = 1;
    let b = 1_456_i16;
    let f: f32 = 2.0004;
    let b: bool = false;
}

pub fn f3() {
    let num = 123;
    if num < 0 {
        println!("number < 0");
    }
    else {
        println!("number >= 0");
    }

    let sign: i8 =
        if num < 0 { -1 }
        else if num == 0 { 0 }
        else { 1 };

    match sign {
        -1 => println!("negative number"),
        0 => println!("zero"),
        1 => println!("positive number"),
        _ => println!("ERROR"),
    }
}

pub fn f4() {
    // loop {
    //     println!("Hello, world!");
    //     println!("Hello, ANDREY!");
    // }

    let mut n = 1;
    while n < 10 {
        println!("while {0}", n);
        n += 1;
    }

    for var in 1..10 {
        println!("for {0}", var);
    }

}

pub fn f5() {
    fn square(n: i32) -> i32
    {
        let square = n * n;
        return square;
    }

    let a = square(2);
    println!("square {} => {}", a, square(a));


    fn sum1(a: i32, b: i32) -> i32
    {
        a + b
    }
    let sum2 = |a, b| -> i32 {
        let result = sum1(a, b);
        println!("sum({},{}) = {}", a, b, result);
        result };
    let sum3 = |a, b|  a + b;

    let num1 = sum1(5, 2);
    sum2(5, 2);
    let num3 = sum3(5, 2);
    // let num3 = sum3(5.02, 2.34);


    let hello = || println!("Hello, world!");
    hello();
}

pub fn f6() {
    const PI: f32 = 3.141592653589793;  // std::f32::consts::PI
    println!("PI: {}", PI);
}

pub fn f7() {
    fn do_operation(a: i32, b: i32, operation: fn(i32, i32) -> i32) {
        let result = operation(a, b);
        println!("result: {}", result);
    }
    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
    let sum: fn(i32, i32) -> i32  = |a, b| a + b;

    do_operation(6, 4, sum);
    do_operation(6, 4, multiply);


    fn mul(a: i32, b: i32) { println!("{}", a * b); }
    fn chose_operation(n:i32) -> fn(i32, i32){
        match n{
            1 => |a, b|{println!("{}", a + b);},
            2 => mul,
            _ => |a, b|{println!("a={}  b={}", a, b);},
        }
    }
    let operation1 = chose_operation(1);
    operation1(5, 4);
    let operation2 = chose_operation(2);
    operation2(5, 4);
    let operation3 = chose_operation(3);
    operation3(5, 4);
}
