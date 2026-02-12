struct Person{
    name:String,
    age:u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point{
    x : f32,
    y:f32
}

struct Rectangle{
    top_left:Point,
    bottom_right:Point
}

fn main() {
    println!("Hello, world!");
}
