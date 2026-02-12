
fn interproduct(a: i32, b: i32, c: i32) -> i32{
    return a*b + b*c +c*a;
}

fn interproduct_i16(a: i16, b: i16, c: i16) -> i16{
    return a*b + b*c +c*a;
}

fn take_i8(a: i8)
{
    println!("i8 = {a}");
}

fn fib(i: u32) -> u32 {
    if i <= 2 {
        return 1;
    }
    else { 
        return fib(i-1) + fib(i-2);
    }
}

fn if_test(){
    println!("=====if_test=====");
    let x = 10;
    
    let sizestr = if x < 10 {"small"} else {"large"};
    println!("number size: {}", sizestr);
}

fn while_test() {
    println!("=====while_test=====");
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }
    println!("finial x = {}", x);
} 

fn for_test() {

    println!("=====for_test=====");
    for x in 1..5 {
        // println!("x = {}", x);
        println!("x = {x}");
    }
    println!("stop at 4");

    for _elem in [1, 2, 3, 4, 5] {
        // println!("elem: {elem}");
    }

    for x in 1..=5 {
        println!("x: {x}");
    }
    println!("stop at 5");
}

fn loop_test() {
    println!("=====loop_test=====");
    let mut i = 0;
    loop {
        i += 1;
        println!("{i}");
        if i > 5 {
            println!("exit loop_test");
            break;
        }
    }
}

fn label_test() {
    println!("=====label_test=====");
    // 请注意，loop 是唯一返回有意义的值的循环结构。 
    // 这是因为它保证至少被输入一次（与 while 和 for 循环不同）。
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }
    print!("print!: elements searched: {elements_searched}");
    println!("println!: elements searched: {elements_searched}");
    
    let mut test_num = 1;
    'forlabel: for j2 in 0..=10{
        test_num += 1;
        if test_num == 10 {
            println!("break forlabel, j2 = {j2}");
            break 'forlabel;
        }
    }
}

fn test_break_continue() {
    println!("=====test_break_continue=====");
    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            break;
        }
        if i % 2 == 0 {
            continue;
        }
        println!("{}", i);
    }
}

fn test_block() {
    println!("=====test_block=====");
    let z = 13;
    let x = {
        let y = 10;
        println!("y: {y}");
        z - y
    };
    println!("x: {x}");
}

fn main() {
    println!("Hello, world!");

    let x: i32 = 32;
    println!("x = {x}");

    // integer overflow
    println!("interproduct res = {}", interproduct(2,3,4));
    println!("interproduct res = {}", interproduct(120, 100, 2480));
    println!("interproduct_I16 res = {}", interproduct_i16(120, 10, 24)); // 120, 100, 2480

    let _a = 3.14;
    let _b = "string";
    let c = 123;
    let d = 123;

    take_i8(c);
    // assert_eq!(a, b);
    assert_eq!(d, c);

    let n = 20;
    fib(n);
    println!("fib({n}) = {}", fib(n));

    if_test();

    // loop and 
    while_test();
    for_test();
    loop_test();
    label_test();
    test_break_continue();
    
    // block\
    test_block();
}
