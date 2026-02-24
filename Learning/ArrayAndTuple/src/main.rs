
// 有符号整数 i
// 无符号整数 u
// 浮点数 f
// unicode标量类型 char
// 布尔值 bool

// iN, uN 和 fN 占用 N 位，
// isize 和 usize 占用一个指针大小的空间，
// char 占用 32 位空间，
// bool 占用 8 位空间。
fn test_arrays() {
    println!("=====test_arrays=====");

    // A value of the array type [T; N] holds N (a compile-time constant) elements of the same type T. 
    // Note that the length of the array is part of its type, 
    // which means that [u8; 3] and [u8; 4] are considered two different types. 
    // Slices, which have a size determined at runtime, are covered later.
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    
    // {} gives the default output, {:?} gives the debug output
    // Types such as integers and strings implement the default output, 
    // but arrays only implement the debug output. 
    // This means that we must use debug output here.
    
    
    // :?：使用 Debug 格式打印（更适合调试，能看到结构）。
    println!("a use {{a:?}} to output : {a:?}");
    println!("a use {{a:#?}} to output 'pretty printing' : {a:#?}");
    
    // 数组 [i8; 10] 不能用 {}（Display）直接打印
    // println!("a: {}", a);
    println!("a use: {:?}", a);

    // 要把 {:?} 当普通文本打印出来，花括号要转义：
    // { 写成 {{
    // } 写成 }}
    println!("a use {{:?}} to output: {:?}", a);
}

fn test_tuple() {
    println!("=====test_tuples=====");


    // 和数组一样，元组也具有固定的长度。
    // 元组将不同类型的值组成一个复合类型。
    // 元组中的字段可以通过英文句号加上值的下标进行访问比如：t.0, t.1。
    let t: (i8, bool) = (7, true);
    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);

    
    // The empty tuple () is referred to as the “unit type” and 
    // signifies absence of a return value, akin to void in other languages.
    let _tvoid: () = ();
    // println!("_tvoid.0: {}", _tvoid.0);
}

fn test_array_iteration() {
    println!("=====test_array_iteration=====");
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];

    // 这里新增了 assert_ne! 宏。此外，还有 assert_eq! 和 assert! 宏。
    // 系统始终会对这些宏进行检查，而像 debug_assert! 这样的仅调试变体在发布 build 中不会编译成任何代码。
    for prime in primes {
        for i in 2..prime {
            // Rust 的 .. 是左闭右开区间
            // 意思是断言“prime % i 不等于 0”。
            assert_ne!(prime % i, 0);
        }
    }
}


fn check_order(tuple:(i32, i32, i32)) -> bool {
    
    // 左边的 (left, middle, right) 是一个“模式”
    // Rust 按位置把右边元组拆开并绑定到三个新变量：
    // left = tuple.0
    // middle = tuple.1
    // right = tuple.2
    let (left, middle, right) = tuple;
    left < middle && middle < right
    // return left < middle && middle < right;
}

fn test_patterns_destructuring(){
    println!("=====test_patterns_destructuring=====");
    // Rust supports using pattern matching to 
    // destructure a larger value 
    // like a tuple into its constituent part

    let tuple = (1, 5, 3);

    // {tuple:?}：把变量 tuple 用 Debug 格式打印，比如 (1, 5, 3)
    // {}：第二个占位符，接收后面表达式的结果（字符串）
    println!(
        "{tuple:?}: {}",
        if check_order(tuple) {"ordered"} else {"unordered"}
    );
}

// 用数组表示矩阵
// Copy Semantics: Arrays of Copy types (like i32) are themselves Copy. 
// When we pass matrix to the function, 
// it is copied by value. The result variable is a new, separate array.
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    // todo!()

    // Initialization: We initialize result with zeros ([[0; 3]; 3]) before filling it. 
    // Rust requires all variables to be initialized before use; 
    // there is no concept of “uninitialized memory” in safe Rust.
    let mut result = [[0; 3]; 3];
    // 0..3 左闭右开
    for i in 0..3 {
        for j in 0..3 {
            result[i][j] = matrix[j][i];
        }
    }
    result

    // what would happen if they tried to return matrix directly after modifying it 
    // (if they changed the signature to mut matrix). 
    // (Answer: It would work, but it would return a modified copy,
    // leaving the original in main unchanged)
}

fn test_nested_arrays() {
    println!("=====test_nested_arrays=====");

    // Mention that [i32; 3] is a distinct type from [i32; 4]. 
    // Array sizes are part of the type signature. array的size也是类型签名的一部分
    let _vec_3: [i32; 3];
    let _vec_4: [i32; 4];



    let _array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("Original:");
    for row in matrix {
        // 把变量 row 用 Debug 格式打印
        println!("{row:?}"); 
    }

    let transposed = transpose(matrix);
    println!("\nTransposed:");
    for row in transposed {
        println!("{row:?}");
    }
}

fn main() {
    println!("Hello, world!");

    test_arrays();

    test_tuple();

    test_array_iteration();

    test_patterns_destructuring();

    test_nested_arrays(); // 嵌套数组
}
