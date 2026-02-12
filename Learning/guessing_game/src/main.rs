/*
 * @Description: 
 * @Author: lhc
 * @Date: 2025-01-22 22:32:14
 * @LastEditTime: 2026-01-06 14:40:50
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

    let mut _y = 5;
    _y    = 6; // mutable


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
        // 即允许用新变量“遮蔽”旧变量，注意这里用了2次let guess，而不是像C++里第二次声明同名变量旧会报错
        // 这里第二次声明guess时，依靠shadow机制把第1个guess给隐藏了；可以理解为覆盖吗？
        
        // trim去掉空白
        // parse也有返回值，其返回值Result类型也是个枚举类型
        // parse方法可以将字符串转换成数字，但是它会返回一个Result类型，这个类型是一个枚举类型，有两个值，Ok和Err
        // 处理parse返回的Result类型，使用expect方法: 如果Result的值是Err，expect会使程序崩溃，并显示expect里的信息
        // let guess:u32 = guess.trim().parse().expect("");
        

        // 把用户输入的东西变成数字给我。如果成功了，就接着往下跑；
        // 如果那根本不是个数字，别报错崩溃，直接装作没发生，让他重输一遍。
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // trim(): 去掉字符串首尾的空白字符和回车换行符（用户输入后按回车会带入 \n）trim也会把/n去掉
        // parse(): 会把字符串转换成某种数值类型；具体用parse能解析成什么类型，这里用:u32指定，显式声明类型;
        // 这里尝试将字符串解析成数字。 因为前面声明了 : u32，所以它知道要解析成 u32 类型。
        // match ... { ... } (错误处理)
        // parse() 不会直接返回数字，而是返回一个 Result 枚举（可能是 Ok 也可能是 Err）。
        // Ok(num) => num: 如果解析成功（比如用户输入 "50"），parse 返回 Ok(50)，这里就把 50 取出来赋值给 guess。
        // Err(_) => continue: 如果解析失败（比如用户输入 "abc"），parse 返回 Err。continue 会告诉程序直接跳过本次循环剩余的代码，回到 loop 的开头重新让用户输入。
        
        // match 需要被匹配的值 {
        //     模式1 => 代码块1,
        //     模式2 => 代码块2,
        //     _    => 默认代码块,
        // }

        // ------------------------------------
        // --- 通配符示例 (Wildcard Example) ---
        match guess {
            100 => println!("Wow! A perfect hundred!"), // 匹配具体值 100
            0 => println!("Zero? Really?"),             // 匹配具体值 0
            _ => {}, // 通配符 `_`：表示“其余所有值”。因为我们只关心 0 和 100，其他的什么都不做，所以用空花括号 {}
        }
        // ------------------------------------

        // 比较
        // guess这个变量上有一个cmp方法，它可以与secret_number进行比较
        // cmp的返回类型就是Ordering
        // 这是rust的优点之一：强制让你匹配每一种情况，代码更加安全，还可以发现到潜在的错误
        // match表达式，可以根据cmp返回的值来执行不同的代码. rust里的重要功能，用的也很多
        // 不需要分号的情况：当它作为独立的控制流语句时
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
