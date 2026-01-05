fn main()
{
    let x = 5;
    let mut y = 5;
    println!("Hello, world!");
    println!("x is {}", x);
    println!("x is {x}");

    // x = 6;
    const GLOBALCHAR: u32 = 2;

    print_labeled_measurement(5, 'h');

    branches();

    looptest();
    () // 相当于隐式的执行了(), 最后1个表达式的值是()
}

fn print_labeled_measurement(value: i32, unit_label:char)
{
    println!("the measurement is {value} {unit_label}");

    let y = {
        let x = 3;
        x + 1
    };

    let x = plus_one(5);
}

fn plus_one(x:i32) -> i32
{
    x+1
}

fn branches(){
    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("the num is {num}");
}

fn looptest()
{
    loop {
        println!("loop");
    }
}