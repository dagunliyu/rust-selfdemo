#![allow(unused)]

use futures::executor::block_on; //

struct Song {}

async fn learn_song() -> Song  {
    Song {}
}

async fn sing_song(song: Song) {

}

async fn dance(){

}

async fn learn_and_sing(){
    // 使用await就不容易造成线程的阻塞
    let song = learn_song().await;
    sing_song(song).await;
}

// 可以这样理解 futures内部异步无阻  然后futuer进行阻塞
async fn async_main() {
    let f1 = learn_and_sing(); // 本已异步
    let f2 = dance();
    // 如果阻塞在f1，f2就会接管线程，反之亦然；
    // 如果f1和f2都阻塞了，就可以认为async_main这个函数也阻塞了
    futures::join!(f1, f2);
}
async fn hello_world(){
    println!("hello world");
}

enum Poll<T>{
    Ready(T),
    Pending,
}

trait SimpleFuture{
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
    // 返回Poll类型，其内部模板T由自己定义，定义啥返回啥
}

fn main() {
    async fn do_something() {
        
    }
    // println!("Hello, world!");
    let future = hello_world();
    // block阻塞当前线程，直到当前的future执行完成
    block_on(future); // future 在执行者上才能运行

    // await
    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());

    // await2
    block_on(async_main()); // block_on作为执行者，这行async_main返回的future
}

