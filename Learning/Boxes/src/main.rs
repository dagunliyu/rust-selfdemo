
use List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("{}", b);
    println!("Hello, world!");
    // 离开作用的时候，box也会被释放
    // box本身就是存在stack上，数据是存在heap上；它们都会被释放

    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(Nil))))));
}

enum List {
    // 这样写，各个数据是挨着放的，而不是嵌套
    Cons(i32, Box<List>), // 此时的List就是间接存储，Box里放了list
    // Box是指针，指针就是间接的，它自己的大小是固定的.

    // 这种写法是1个套1个，把1个数据套在另一个里面
    // Cons(i32, List), // Cons变体可以存i32类型，也可以存list类型
    Nil,
}
//eg. (1, (2, (3, Nil)))