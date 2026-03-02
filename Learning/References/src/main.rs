

//================================================================
// A reference provides a way to access another value 
// without taking ownership of the value, and is also called “borrowing”

// References can never be null in Rust, so null checking is not necessary.

// A reference is said to “borrow” the value it refers to, 
// and this is a good model for students not familiar with pointers: 
// code can use the reference to access the value, 
// but is still “owned” by the original variable. The course will get into more detail on ownership in day 3.

// References are implemented as pointers, and a key advantage is that they can be much smaller than the thing they point to. Students familiar with C or C++ will recognize references as pointers. Later parts of the course will cover how Rust prevents the memory-safety bugs that come from using raw pointers.

// Explicit referencing with & is required, 
// except when invoking methods, 
// where Rust performs automatic referencing and dereferencing.

// Rust will auto-dereference in some cases, in particular when invoking methods (try r.is_ascii()). There is no need for an -> operator like in C++.

// In this example, r is mutable so that it can be reassigned (r = &b). 
// Note that this re-binds r, so that it refers to something else. 
// This is different from C++, where assignment to a reference changes the referenced value.

// A shared reference does not allow modifying the value it refers to, even if that value was mutable. Try *r = 'X'.

// Rust is tracking the lifetimes of all references to ensure they live long enough. 
// Dangling references cannot occur in safe Rust.

// We will talk more about borrowing and preventing dangling references when we get to ownership.


fn test_shared_references(){
    println!("=====test_shared_references=====");
    let a = 'A';
    let b = 'B';
    // let mut r: &char = &a;
    let mut r = &a;
    println!("r: {}", *r);
    dbg!(r);
    r = &b;
    println!("r: {}", *r);
    dbg!(r);

    // `r` is a `&` reference, so the data it refers to cannot be written
    // C++可以修改引用的值，但Rust不允许
    // *r = 'C';

    x_axis(5);
}

// 静态禁止悬垂引用
// fn x_axis(x: i32) -> &(i32, i32) {
//     // point 在函数结束就被销毁，所以这个引用会悬垂，Rust 禁止。
//     let point = (x, 0);
//     return &point;
// }

fn x_axis(x: i32) -> (i32, i32) {
    // point 在函数结束就被销毁，所以这个引用会悬垂，Rust 禁止。
    let point = (x, 0);
    return point;
}

//================================================================

// “独占”意味着只有此引用才能访问该值。不能同时存在其他引用（共享或独占），并且在独占引用存在期间，无法访问被引用的值。
fn test_exclusive_shared_references(){
    println!("=====test_exclusive_references=====");

    // let mut point = (1,2);
    // let x_coord = &mut point.0;
    // *x_coord = 22;
    // println!("point: {point:#?}");
    // // Try making an &point.0 or changing point.0 while x_coord is alive.
    // let testexclusive = &point.0;
    // println!("testexclusive: {testexclusive:#?}");

    // Tips: mut修饰的是变量，不是引用

    let mut _mut_xcoord: &i32;  // 共享引用 - 变量可以重新绑定到新的引用
    let _xcoord: &i32;          // 共享引用 - 绑定后不能改指向   
    let _xcoord_mut: &mut i32;  // 独占引用

    // ---- 1. 独占引用（&mut）：可以修改值 ----
    let mut point = (1, 2);
    println!("原始 point: {point:?}");

    let x_ex = &mut point.0; // &point.0 不加mut只加引用就是共享引用了
    *x_ex = 12;
    println!("修改后 *x_ex = {x_ex}");

    let x_coord = &mut point.0;  // 独占引用
    *x_coord = 22;               // 通过独占引用修改值
    println!("修改后 *x_coord = {x_coord}");
    // x_coord 在这之后不再使用，独占引用结束

    // ---- 2. 共享引用（&）：只读，可以同时存在多个 ----
    // 共享引用是immutable borrow
    let r1 = &point.0;  // 共享引用 1
    let r2 = &point.0;  // 共享引用 2，允许同时存在
    println!("r1 = {r1}, r2 = {r2}");  // 多个共享引用可以同时读

    let mut rm1 = &point.0;
    let rm2 = &point.1;
    println!("rm1 = {rm1}, rm2 = {rm2}");  // 多个共享引用可以同时读
    rm1 = rm2;



    // ---- 3. 共享引用不能修改值 ----
    // *r1 = 99;  // 编译错误：不能通过共享引用修改值

    // ---- 4. 独占引用存在时，不能有共享引用 ----
    // 独占引用是mutable borrow
    let x_mut = &mut point.1; // x_mut 的类型是 &mut i32
    *x_mut = 42;

    // x_mut持有 &mut point.1. 对point.1是独占引用/独占借用
    // 后面还会用x_mut, 独占借用存活期间，不能以任何方式访问point.1

    //cannot borrow `point.1` as immutable because it is also borrowed as mutable
    // let r3 = &point.1;       // 编译错误：独占引用 x_mut 还活着，不能再借用
    // println!("{}", point.1);  // 编译错误：独占引用 x_mut 还活着，不能直接访问 


    // ---- 共享引用和独占引用的copy ----
    // x_mut;   // 错误: &mut 引用不实现 Copy（独占性决定的）, 后面 println! 还想用 x_mut，但值已经被 move 了，所以报错
    // point.1; // 错误: 独占借用存活期间，不能以任何形式访问point.1
    // 关键区别：&（共享引用）实现了 Copy，写 r1; 不会消耗它；
    // 而 &mut（独占引用）不实现 Copy，写 x_mut; 就把它 move 掉了。
 
    // ---- 为什么单独写x_mut会把这个引用move掉？ ----
    // 任何表达式语句都会消耗（move）它的值，除非该类型实现了 Copy。
    // x_mut; 这一行是一个"表达式语句"，它的效果是：
    // 求值 x_mut（即读取这个变量）, 得到的值没有被赋给任何人 → 直接 drop
    // Copy 的语义是"复制一份，原始变量仍然有效"。
    // 但 &mut T 是独占引用，同一时刻只能存在一个 &mut T 指向同一个值。
    // 如果 &mut T 实现了 Copy，就可以同时复制出多个 &mut T，独占性就被破坏了。
    // 所以 Rust 故意不让 &mut T 实现 Copy。


    println!("修改后 *x_mut = {x_mut}");
    // x_mut 在这之后不再使用，独占引用结束

    // ---- 5. 独占引用结束后，又可以正常访问了 ----
    println!("最终 point: {point:?}");
}

//================================================================

struct Point {
    x: u32,
    y: u32,
}

fn modify_coordinates(pnt: &mut Point) {
    let x_ref = &mut pnt.x;
    let y_ref = &mut pnt.y;
    x_ref += 2;
    y_ref += 2;
    println!("pnt after modify: {:?}", (pnt.x, pnt.y));
}

fn test_split_borrowing() {
    println!("=====test_split_borrowing=====");

    let pnt = 
}

fn test_split_borrowing() {
    println!("=====test_split_borrowing=====");

    
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let make_point = |x: i32, y: i32| Point { x, y };
    let modify_coordinates = |pnt: &mut Point| {
        let x_ref = &mut pnt.x;
        let y_ref = &mut pnt.y;
        *x_ref += 1;
        *y_ref += 1;
    };

    let mut pnt = make_point(2, 3);
    modify_coordinates(&mut pnt);
    println!("pnt after modify: {:?}", (pnt.x, pnt.y));
}

//================================================================
fn test_slices() {
    // [T; N] 和 [T] 是两种不同的类型。
    // 数组 [T; N]：长度 N 是类型的一部分，写在类型里。
    // 切片 [T]：类型本身不带长度，长度是运行时附在胖指针里的。

    println!("=====test_slices=====");
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");

    let all: &[i32] = &a[0..a.len()];
    let all2: &[i32] = &a[..a.len()];
    println!("all use {{&a[0..a.len()]}}: {all:?}");
    println!("all2 use {{&a[..a.len()]}}: {all2:?}");
 
    let test_last_Index: &[i32] = &a[3..a.len()];
    let test_last_Index2: &[i32] = &a[3..];
    println!("test_last_Index use {{&a[3..a.len()]}}: {test_last_Index:?}");
    println!("test_last_Index2 use {{&a[3..]}}: {test_last_Index2:?}");


    let aa: [u32; 5] = [3,4,5,6,7];
    // bb[0..5]  类型[u32]  ❌ 切片类型不携带长度
    // &bb[0..5] 类型&[u32] ✅ 引用 = 指针+长度，大小固定

    let _bb: &[u32] = &aa[..aa.len()];
    // _bb 就是一个胖指针： [ aa的起始地址 | 长度=aa的长度 ]
    
    // ===== error 写法分析: let bb = aa[0..5]; 

    // Rust 类型系统做了一个设计决策：用 range .. 对数组取索引，结果永远是 [T]，而不是 [T; N]。
    // 即便写的是常量范围 0..5，编译器也不会把结果当作 [u32; 5] 来处理——那样类型系统会非常复杂。
    // 加上 &，把它变成大小确定的"胖指针"才能使用。

    // 

    // aa[0..5]的类型是[u32], 是一个大小不确定的类型，不能直接放在栈上赋给变量
    // 切片类型不携带长度, 在rust的类型系统里就是unsized(大小不固定)的
    // 所以即使写了0..5这种语法，依然改变不了a[0..5]是切片类型的事实，这里是[u32]
    // rust要求在编译期就知道每个变量占多大内存
    // 必须通过引用来使用切片

    // ==== fat pointer ====
    // 普通指针：只存一个内存地址，占 8 字节（64位系统）。
    // 胖指针(针对普通指针而言)：存两个字段，占 16 字节： 一个内存地址（指向数据）+ 一个额外的元数据
    
    // rust有2种胖指针:
    // 1-切片引用: &[T]
    // [ 指针地址 | 长度 length ]
    //   8字节       8字节       = 16字节
    // 所以 &[u32] 能在运行时知道有几个元素。

    // 2-trait对象: &dyn Trait
    // [ 指针地址 | vtable指针 ]
    // 8字节       8字节       = 16字节
    // vtable 指向这个具体类型实现的方法表。
    // =====================

    // aa[3..5] 还是左闭右开,取的是第3和第4个值
    let ss: &[u32] = &aa[3..5]; // 3..=5 error
    println!("ss: {ss:?}");
}

//================================================================
fn test_strings(){
    println!("=====test_strings=====");

}

//================================================================
fn test_reference_validity(){
    println!("=====test_reference_validity=====");

}

//================================================================
fn test_exercise_geometry(){
    println!("=====test_exercise_geometry=====");

}

//================================================================
fn main() {
    println!("References!");

    test_shared_references();
    test_exclusive_shared_references();
    test_split_borrowing(); // 分割借用


    test_slices();

    test_strings();

    test_reference_validity();

    test_exercise_geometry();
}
