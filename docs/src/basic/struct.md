# 结构体



结构体是 Rust 中的一种数据结构，它允许你将多个值组合在一起，并且可以定义方法来操作这些值。你可以使用结构体来定义数据类型，这些数据类型可以包含其他值。


结构体样例代码如下：
```rust
/**
 * 结构体 sample
 */
pub(crate) fn struct_sample() {
    println!("datatype sample struct_sample .....start");

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("Struct update filed value by other struct result.");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    //可以尝试注释以下语句，会报错误，因为user1 所有权已经被借用了，
    //^ print!("user is {:?}", user1);

    //所以user2 拥有所有权 ，可以正常打印
    println!(
        "根据已有的结构体实例，创建新的结构体实例.user2: {:?}",
        user2
    );

    let user3 = build_user(
        String::from("another@example.com"),
        String::from("someusername456"),
    );

    let user4: User = User {
        active: user3.active,
        username: user3.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    println!("user3 user.emal:{}", user3.email);

    //以下语句报：
    //borrow of moved value: `user3.username`
    //move occurs because `user3.username` has type `String`, which does not implement the `Copy`
    //可以尝试删除注释
    //^ println!("user3 user.username:{}",user3.username);

    println!("user build result user4 is {:?}", user4);

    println!("datatype sample struct_sample .....end\n");
}

///
///  用户信息 结构体
///
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}



```