
fn read(y: bool)
{
    if y {
        println!("y is true");
    }
}
fn main() {
    println!("Hello, world!");
    let boolVar = true;
    read(boolVar);

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
}
