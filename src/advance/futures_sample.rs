//!
//! futures Sample
//!

use core::fmt;

// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
use futures::{executor::block_on, Future};

///
/// async hello
async fn hello_world() {
    println!("hello, world!");
}

///
/// future async block call
pub(crate) fn futures_block_sample() {
    let future = hello_world(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
}

/// Song
///
#[derive(Debug)]
struct Song;

///
///learn song async future
async fn learn_song() -> Song {
    /* ...async do thing ... */
    Song
}
async fn sing_song(song: Song) {
    /* async do thing... */
    println!("async sing :{:?}", song)
}
async fn dance() {
    /* async do thing... */
    println!("async dance ...")
}

///
/// manul block handle async future
pub(crate) fn futures_block_handle_sample() {
    let song = block_on(learn_song());

    block_on(sing_song(song));

    block_on(dance());
}

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song().await;
    sing_song(song).await;
}

///futures join! async handle
///  
async fn futures_async_handle_main() {
    //future sing
    let f1 = learn_and_sing();

    //future dance
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}

///
pub(crate) fn futures_await_main() {
    block_on(futures_async_handle_main());
}

// `foo()` returns a type that implements `Future<Output = u8>`.
// `foo().await` will result in a value of type `u8`.
async fn foo() -> u8 {
    5
}

//
// async block rsult
fn bar() -> impl Future<Output = u8> {
    // This `async` block results in a type that implements
    // `Future<Output = u8>`.
    async {
        let x: u8 = foo().await;
        x + 5
    }
}

/// `async` block:
///
/// Multiple different `async` blocks can access the same local variable
/// so long as they're executed within the variable's scope
async fn blocks() {
    let my_string = "foo".to_string();

    let future_one = async {
        // ...
        println!("{my_string}");
    };

    let future_two = async {
        // ...
        println!("{my_string}");
    };

    // Run both futures to completion, printing "foo" twice:
    let ((), ()) = futures::join!(future_one, future_two);
}

/// `async move` block:
///
/// Only one `async move` block can access the same captured variable, since
/// captures are moved into the `Future` generated by the `async move` block.
/// However, this allows the `Future` to outlive the original scope of the
/// variable:
fn move_block() -> impl Future<Output = ()> {
    let my_string = "foo".to_string();
    async move {
        // ...
        println!("{my_string}");
    }
}

///
pub(crate) fn futures_async_block_main() {
    let r = block_on(bar());

    println!("result : {}", r);

    let r1 = block_on(blocks());

    let r2 = block_on(move_block());
}
///
/// 单元测试
/// #[cfg(test)]
///
#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_fetures_hello() {
        futures_block_sample();
    }

    #[test]
    fn test_fetures_main() {
        futures_block_handle_sample();
    }

    #[test]
    fn test_fetures_await_main() {
        futures_await_main();
    }

    #[test]
    fn test_fetures_async_block() {
        futures_async_block_main();
    }
}