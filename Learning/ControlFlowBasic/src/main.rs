
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
    let x2_bracket = {
        let y = 10;
        let _ = z - y; // use `let _ = ...` to ignore the resulting value
        // type is ()
        // Rust 将其视为一条语句（Statement），而不是表达式（Expression）。
        // 这意味着这个代码块执行完后没有返回具体数值，而是返回了 ()（所谓的
        //  Unit Type，类似其他语言的 void）
    };
    
    let x2 = {
        let y = 10;
        y
        // return z - y;
    };
    // 无分号（expr）：表示代码块的值（用于赋值给变量）。
    // 有分号（stmt;）：表示语句结束，代码块返回 ()。
    // return：直接结束整个函数，不仅仅是赋值。
    println!("x: {x}, x2: {x2}, x2_bracket: {:?}", x2_bracket);
}

fn test_shadowing()
{
    let a = 10;
    println!("before: {a}");
    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");
}

// greatest common Divisor
fn gcd(a: u32, b: u32) -> u32 {
    // b作为余数
    // 代码块（{}）的最后一个表达式默认作为返回值（隐式返回），
    // 但也可以使用 return 关键字进行显式返回。这两种写法是等价的
    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

fn gcd_semicolon(a:u32, b:u32) -> u32{
    if b == 0 {
        return a;
    } else {
        // 不带分号的行是一个表达式 (Expression)，它的结果会被返回。
        // 带分号的行是一个语句 (Statement)，它的返回值被丢弃，总是返回 () (空元组/Unit type)。
        // 所以不加return的话，这样会错:
        // gcd1(b, a % b); // 一直返回的是()
        return gcd_semicolon(b, a % b);
    }
}

fn gcd_without_semicolon(a:u32, b:u32) -> u32{
    if b == 0 {
        return a;
    } else {
        // 不带分号的行是一个表达式 (Expression)，它的结果会被返回。
        // 带分号的行是一个语句 (Statement)，它的返回值被丢弃，总是返回 () (空元组/Unit type)。
        gcd_without_semicolon(b, a % b)
    }
}

fn gcd_iterative(mut a:u32, mut b:u32) -> u32{
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
    // return a;
}

fn test_gcd() {
    println!("=====test_gcd=====");
    println!("gcd: {}", gcd(143, 52));
    println!("gcd_semicolon: {}", gcd_semicolon(143, 52));
    println!("gcd_withoutSemicolon: {}", gcd_without_semicolon(143, 52));
    println!("gcd_iterative: {}", gcd_iterative(143, 52));
}

fn test_macro_factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

fn test_macro_fizzbuzz(_n: u32) -> u32 {
    todo!()
}

fn test_macro() {
    // println!(format, ..) prints a line to standard output, applying formatting described in std::fmt.
    // format!(format, ..) 的用法与 println! 类似，但它以字符串形式返回结果。
    // dbg!(expression) 会记录表达式的值并返回该值。
    // todo!() 用于标记尚未实现的代码段。如果执行该代码段，则会触发 panic。
    // unreachable!() 用于标记无法访问的代码段。如果执行该代码段，则会触发 panic。
    let n = 4;
    println!("{n}! = {}", test_macro_factorial(n));

    // test_macro_fizzbuzz();
}

// 考拉兹序列（Collatz Sequence），也被称为 3n + 1 问题、冰雹猜想（Hailstone Sequence）或 奇偶归一猜想
// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
//   todo!("Implement this")
    let mut len = 1;
    while n > 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3*n + 1;
        }
        len += 1;
    }
    return len;
}

fn collatz_length2(mut n: i32) -> u32 {
//   todo!("Implement this")
    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}

#[test]
fn test_collatz_length(){
    assert_eq!(collatz_length(11), 15);
}

fn test_collatz_sequence()
{
    println!("=====test_collatz_sequence=====");
    println!("Length: {}", collatz_length(11));
    println!("Length2: {}", collatz_length2(11));
}
 


/////////////////////////////////////////////////////
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

    test_shadowing();

    test_gcd();

    test_macro();

    test_collatz_sequence();
}
