

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
    // C++可以修改引用的值
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
fn test_exclusive_references(){
    println!("=====test_exclusive_references=====");
    let mut point = (1,2);
    let x_coord = &mut point.0;
    *x_coord = 22;
    println!("point: {point:#?}");


    // Try making an &point.0 or changing point.0 while x_coord is alive.
    let testexclusive = &point.0;
    println!("testexclusive: {testexclusive:#?}");

    let mut x_coord: &i32;  // 共享引用
    let x_coord: &mut i32;  // 独占引用
}

//================================================================
fn test_slices(){
    println!("=====test_slices=====");

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

    test_exclusive_references();

    test_slices();

    test_strings();

    test_reference_validity();

    test_exercise_geometry();
}
