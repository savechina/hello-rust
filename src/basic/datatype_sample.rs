//!
//!
//! Basic DataType Sample
//!
//!
use std::array;
use std::borrow::Borrow;
use std::collections::{HashMap, LinkedList};
use std::vec;

use crate::leetcode::Solution;

///
///集合 HashMap
///
pub(crate) fn collections_example() {
    println!("datatype_sample::collections_example ... start");

    //初始化HashMap 并设置值
    let mut map: HashMap<String, String> = HashMap::new();

    map.insert("jack".to_string(), "1334567896".to_string());

    map.insert("pony".to_string(), "1342356755".to_string());

    map.insert("tony".to_string(), "1324567891".to_string());

    println!("collection example hashmap: {:?}", map);

    //获取 Key by entry
    let entry = map.entry("jack".to_string());

    println!(
        "map is entry: key:{},value:{:?}",
        "jack",
        &entry.or_default()
    );

    //通过 get_key_value 获取 Map 的值
    let kv = map.get_key_value(&"pony".to_string());

    match kv {
        Some(val) => println!("k:{},v:{}", val.0, val.1),
        None => println!("panic"),
    }

    if map.contains_key(&"pony".to_string()) {
        //借用 map 权限，获取key 的 val. &map[&key]
        let val = &map[&"pony".to_string()];

        println!("val:{}", val);
    }

    map.insert("key".to_string(), "val".to_string());

    //HashMap 迭代器
    for (key, val) in map.iter() {
        println!("itertor key:{}, val:{} ", key, val);
    }

    println!("remove before get key:k,val: {:?}", map.get("key").unwrap());

    map.remove("key");

    let k = map.get("key");
    println!("remove after get key:k,val: {:?}", k);

    println!("map is empty:{}", map.is_empty());

    println!("datatype_sample::collections_example ... end\n");
}

///
/// Array Sample
/// 数组
///

pub(crate) fn array_sample() {
    println!("datatype::array_sampe ...... start");

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("Months array first is {:?}", months[0]);
    println!("Months array second  is {:?}", months[1]);

    println!("Months array all is {:?}", months);

    // 编译器自动推导出one的类型
    let one = [1, 2, 3];
    // 显式类型标注
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

    // 借用arrays的元素用作循环中
    for a in &arrays {
        print!("{:?}: ", a);
        // 将a变成一个迭代器，用于循环
        // 你也可以直接用for n in a {}来进行循环
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }

    println!("array_sampe ...... end \n");
}

///
/// Vec Samle
/// Vector 为动态数组
///
pub(crate) fn vet_sample() {
    println!("vet_sample ......start");

    let v = vec![12, 34, 56, 78];

    let first = v.first();

    println!("ver fist is {:?}", first);

    //fist unwrap option is 12
    println!("ver fist is {}", first.unwrap());

    let mut sum = 0;
    //iter every item
    for n in v {
        println!("vet print item is {}", n);
        sum += n;
    }

    println!("vet all sum is {}", sum);

    println!("vet_sample ......end\n");
}

/**
 * 定义扑克花色枚举
 */
#[derive(Debug)]
enum PokerSuit {
    ///
    /// 黑桃 ♠️
    ///
    Clubs,
    ///
    ///梅花 ♣️
    ///
    Spades,
    ///
    /// 方块 ♦️
    ///
    Diamonds,
    ///
    ///红心 ♥️
    ///
    Hearts,
}

/**
 * Poker
 */
#[derive(Debug)]
struct PokerCard {
    suit: PokerSuit,
    value: u8,
}

/**
 * PokerCard implement 方法
 */
impl PokerCard {
    ///
    /// 格式化显示Poker Card Value 方法
    ///
    pub fn view(&self) -> String {
        match self.suit {
            PokerSuit::Clubs => format!("♠️{}", Self::format_value(self.value)),
            PokerSuit::Spades => format!("♣️{}", Self::format_value(self.value)),
            PokerSuit::Diamonds => format!("♦️{}", Self::format_value(self.value)),
            PokerSuit::Hearts => format!("♥️{}", Self::format_value(self.value)),
        }
    }

    /**
     * 格式化Poker value 为字符
     */
    fn format_value(value: u8) -> String {
        if value == 1 {
            return "A".to_string();
        } else if value > 1 && value < 10 {
            return value.to_string();
        } else if value == 10 {
            return String::from("T");
        } else if value == 11 {
            return String::from("J");
        } else if value == 12 {
            return String::from("Q");
        } else if value == 13 {
            return String::from("K");
        } else {
            panic!("value is incorrect");
        }
    }
}

/**
 * enum 样例
 */
pub(crate) fn enum_sample() {
    println!("enum_sample ...... start");
    let c1 = PokerCard {
        suit: PokerSuit::Clubs,
        value: 1,
    };
    let c2: PokerCard = PokerCard {
        suit: PokerSuit::Diamonds,
        value: 12,
    };

    println!("PokerCard A is {:?},B is {:?}", c1, c2);

    println!("PokerCard view A is {},B is {}", c1.view(), c2.view());

    let mut poker_cards: Vec<PokerCard> = Vec::new();

    for n in 1..14 {
        let p1 = PokerCard {
            suit: PokerSuit::Clubs,
            value: n,
        };
        poker_cards.push(p1);

        let p2 = PokerCard {
            suit: PokerSuit::Spades,
            value: n,
        };
        poker_cards.push(p2);

        let p3 = PokerCard {
            suit: PokerSuit::Diamonds,
            value: n,
        };
        poker_cards.push(p3);

        let p4 = PokerCard {
            suit: PokerSuit::Hearts,
            value: n,
        };
        poker_cards.push(p4);
    }

    for i in 0..poker_cards.len() {
        let card = &poker_cards[i];

        let view = card.view();

        if i % 4 == 0 {
            print!("\nPokerCards is {} ", card.value)
        }
        print!("\t{} ", view);
    }
    println!();

    println!("enum_sample ...... end\n");
}

/**
 * tupl_sample
 */
pub(crate) fn tupl_sample() {
    println!("datatype tupl_sample .....start");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four: f64 = x.1;

    let one: u8 = x.2;

    println!("tupl:({},{},{})", five_hundred, six_point_four, one);

    let s1 = String::from("hello");

    let (s2, len) = calc_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    println!("datatype tupl_sample .....end\n");
}

/**
 * calc length
 */
fn calc_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}

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

    // print!("user is {:?}", user1);

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

/**
 * 链表 LinkedList
 */
pub(crate) fn linkedlist_sample() {
    let mut list = LinkedList::from([1, 2, 3]);

    println!("linkelist is {:?}", list);

    list.push_front(0);
    list.push_front(-1);

    list.push_back(4);
    list.push_back(5);

    println!("linkelist is {:?}", list);
 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn vet_test() {
        vet_sample();
    }

    #[test]
    fn poker_test() {
        enum_sample();
    }

    #[test]
    fn test_linkedlist() {
        let list = LinkedList::from([1, 2, 3]);
    
        println!("linkelist is {:?}", list);

        linkedlist_sample();
    }
}
