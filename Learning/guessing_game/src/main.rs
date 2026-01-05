/*
 * @Description: 
 * @Author: lhc
 * @Date: 2025-01-22 22:32:14
 * @LastEditTime: 2025-02-14 01:31:15
 * @LastEditors: lhc
 * @Reference: 
 */

use std::io; //prelude
use rand::Rng; //trait 可以看成是其他语言里的接口
use std::cmp::Ordering; //引入Ordering枚举类型

fn main() {
    println!("guess!");
    println!("guess a number!");

    // 没加mut，默认是不可变的
    let secret_number = rand::thread_rng().gen_range(1..101); // i32 u32 i64
    println!("The secret number is: {}", secret_number);

    let x = 5;
    let _i = x; // immutable
    // i = 6; // error

    let mut y = 5;
    y    = 6; // mutable


    // 无限循环
    // loop {}

    loop{
        // let 声明变量
        // mut 表示可变的
        let mut guess = String::new();

        // rust的优势，可以保证我们简单并安全的使用引用
        // 引用在rust里也是默认不可变的，所以这里也加上了mut
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        // std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        // io::Result 一个枚举类型，有两个值，Ok和Err 
        // expect就是io::Result的一个方法，如果Result的值是Err，expect会使程序崩溃，并显示expect里的信息

        println!("You guessed: {}", guess);

        // shadow机制
        // Rust允许我们用一个新值来隐藏一个旧值(上面String类型的guess)，这样我们可以重用guess变量名而不是创建两个不同的变量
        // trim去掉空白
        // parse也有返回值，其返回值Result类型也是个枚举类型
        // parse方法可以将字符串转换成数字，但是它会返回一个Result类型，这个类型是一个枚举类型，有两个值，Ok和Err
        // 处理parse返回的Result类型，使用expect方法: 如果Result的值是Err，expect会使程序崩溃，并显示expect里的信息
        let guess:u32 = guess.trim().parse().expect( "Please type a number");
        // trim也会把/n去掉，parse会把字符串转换成某种数值类型；
        // 具体用parse能解析成什么类型，这里用:u32指定，显式声明类型；
    
        let guess = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        // 比较
        // guess这个变量上有一个cmp方法，它可以与secret_number进行比较
        // cmp的返回类型就是Ordering
        // 这是rust的优点之一：强制让你匹配每一种情况，代码更加安全，还可以发现到潜在的错误
        // match表达式，可以根据cmp返回的值来执行不同的代码. rust里的重要功能，用的也很多
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),   //arm
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }


  

    // rust是一门静态的强类型的语言，此外也有一些类型推断的能力
}
