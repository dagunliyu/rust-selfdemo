
fn read(y: bool)
{
    if y {
        println!("y is true");
    }
}

fn test_data_in_stack(){
    let a = [0; 1_000_000];
    let b = a; // copy 
}

fn test_data_in_heap() {
    // Stack保存与特定函数关联的数据，而Heap保存可能比函数存活更长的数据

    //• 堆是独立的内存区域，数据可以在堆里无限期的存活；
    //• 堆的数据不依赖于函数的stack frame；

    // 使用Box来创建，将数据放在堆上；
    // stack内存里有个data_in_heap的stack frame，里面有个变量a，就是指针，存的是Heap的地址；在这个地址里存的是真正的数据；
    let a = Box::new([0; 1_000_000]); 

    // 将a复制给b，复制的是这个指针，这个数组在堆内存里的地址；
    let b = a;
    // 赋值完成后，a被移动了；
}

fn test_memory_management() {

    // • Rust不允许手动内存管理
    // • Stack Frame由Rust 自动管理：
    // • 当调用一个函数时，Rust为被调用的函数分配一个Stack Frame。
    // 当调用结束时，Rust 释放该Stack Frame
    //

    // • Box的所有者来管理内存释放
    //  ◦ Rust会自动释放Box的堆(heap)内存。
    // • Box内存释放原则(几乎正确):
    //  ◦ 如果一个变量绑定到一个Box,当Rust释放变量的frame时，Rust也会释放Box的堆(heap)内存。


}

//=====================================
fn implicit_and_explicit_unref() {
    let x = Box::new(-1);       // x是地址    
    let x_abs1 = i32::abs(*x);  // 显式解引用
    let x_abs2 = x.abs();       // 隐式解引用
    assert_eq!(x_abs1, x_abs2);

    let r = &x; // 共享引用
    // let r1: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // 显式解引用2次
    let r_abs2 = r.abs();       // 隐式解引用2次
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s);  // 显式引用  len期待的是个引用类型
    let s_len2 = s.len();       // 隐式引用  s是拥有数据的，这里插入了一个借用的符号，做成了一个s的引用
    assert_eq!(s_len1, s_len2);

    // 本例中，关于隐式解引用
    // x.abs() 在方法调用时也会发生解引用
    // 隐式转换支持多层
    // 反向亦可
}

fn ieu_prob1() {
    println!("=====test_ieu_prob1=====");
    let x = Box::new(3);
    let y = Box::new(&x);

    // 如果想复制出通过y得到的数字3，需要使用多少次解引用?
    // 真坑，要三次解引用才行，因为要“拆Box<>“????
}

fn ieu_prob2() {
    
    println!("=====test_ieu_prob2=====");
    let get_first = |vr: &Vec<i32>| -> i32{
        vr[0]
    };

    let mut v = vec![0,1,2];
    let n = get_first(&v);
    println!("{} {}", n, v[1]);

    // 在调用get_first后, v是否被释放了? 没有
    
    // 原因？下面哪个？
    // v在调用get_first后没有被释放
    // v在调用get_first后再print中被使用
    // v是一个引用，不拥有它指向的向量
    // get_first返回的事类型为i32的值，而不是向量本身

    // v是一个引用，没有所有权
}

fn test_ieu() {
    println!("=====test_split_borrowing=====");
    implicit_and_explicit_unref();

    ieu_prob1();

    ieu_prob2();
}

//=====================================

fn test_alias_and_mutation()
{
    
}

//=====================================
fn main() {
    println!("Hello, world!");
    let bool_var = true;
    read(bool_var);

    let x = 1;
    let x = Box::new(x);
    let y = x; // 所有权转移给了y，x释放了？

    // println!("x is {}", x); // value borrowed here after move
    println!("y is {}", y);

    // 使用引用來賦值，指向1块地址
    let r1 = &y;
    let r2 = &y;
    println!("r1 is {r1}, r2: {r2}");

    // 总结：Box不允许别名，但是引用可以创建1个临时的别名
    // 多个引用可以访问同一块数据，间接的访问到Box的数据

    // 如何绕过Box，直接指向数据“1”？？ 怎么写
    //  let r3 = &x; // 此时x已经被move了

    let x0 = 0;
    let mut x0_ref = &x0;
    let y1 = 1;

    // *x0_ref += 1; // immutable borrowed context 不可变借用上下文->其借用的东西是不可变的
    x0_ref = &y1; // xref可以指向其他变量

    test_data_in_stack();
    test_data_in_heap();

    test_memory_management();

    test_ieu();
}
