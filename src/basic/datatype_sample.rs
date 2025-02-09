//!
//!
//! Basic DataType Sample
//!
//!
use bigdecimal::num_bigint::{BigUint, RandBigInt, ToBigInt};
use bigdecimal::num_traits::{One, Zero};
use bigdecimal::{BigDecimal, RoundingMode};
use chrono::prelude::*;
use serde::*;
use serde_json;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList};
use std::str::FromStr;
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime};
use std::vec;
///
/// 字符串
///
pub(crate) fn string_sample() {
    println!("datatype::string_sample ...... start");

    //普通字符串字面量
    let simple_string = "Hello, world!";

    println!("{}", simple_string);

    //转义字符 反斜杠 \ 来转义特殊字符
    let escaped_string = "This is a \"quoted\" string with a newline\nand a tab\t.";

    println!("{}", escaped_string);

    let multi_line_string = "This is a long string \
that spans multiple lines.";
    println!("{}", multi_line_string);

    // 普通字符串字面量，需要转义特殊字符
    let normal_string = "This is a \"normal\" string with a newline\n and a backslash \\.";

    // 原始字符串字面量，不需要转义特殊字符
    let raw_string = r#"This is a "raw" string with a newline
 and a backslash \."#;

    println!("{}", normal_string);
    println!("{}", raw_string);

    //字符串切片
    let string_slice: &str = "Hello, world!";

    let hello = &string_slice[0..5];
    let world = &string_slice[7..12];

    println!(
        "{}, slice[0..5]:{},slice[7..12]:{}",
        string_slice, hello, world
    );

    //字符串连接
    let hello = String::from("Hello");
    let world = " world!";
    let combined = hello + world; // 注意：hello 被移动，不能再使用

    let mut greet = String::from("Hello");
    greet.push_str(", world!");

    //字符串查找和替换
    let s = "Hello, world!";
    if let Some(index) = s.find("world") {
        println!("Found 'world' at index {}", index);
    }

    let replaced = s.replace("world", "Rust");
    println!("{}", replaced);

    //字符串分割
    let s = "Hello, world!";
    let parts: Vec<&str> = s.split(", ").collect();
    for part in parts {
        println!("{}", part);
    }

    println!("datatype::string_sample ...... end\n");
}

/*
 *Account sample
 */
#[derive(Debug, Serialize, Deserialize)]
struct MyAccount {
    name: String,
    // this will be written to json as string
    value: BigDecimal,
    // this will be written to json as number
    #[serde(with = "bigdecimal::serde::json_num")]
    number: BigDecimal,
}

///
/// decimal_sample
///
pub(crate) fn decimal_sample() {
    let json_src = r#"
    { "name": "foo", "value": 1234567e-3, "number": 3.14159 }
"#;

    let account: MyAccount = serde_json::from_str(&json_src).unwrap();

    dbg!(&account);
    // MyStruct { name: "foo", value: BigDecimal("1234.567"), BigDecimal("3.1459") }

    println!("{:?}", serde_json::to_string(&account));
    // {"name":"foo","value":"1234.567","number":3.1459}

    //sqrt
    let two = BigDecimal::from(2);
    println!("sqrt(2) = {}", two.sqrt().unwrap());

    //
    let n = BigDecimal::from(700);
    println!("1/{n} = {}", n.inverse().round(6));

    //new
    let input = "0.8";
    let dec = BigDecimal::from_str(&input).unwrap();
    let float = f32::from_str(&input).unwrap();

    println!("Input ({}) with 10 decimals: {} vs {})", input, dec, float);

    // rounding
    let n: BigDecimal = "129.41675".parse().unwrap();

    assert_eq!(
        n.with_scale_round(2, RoundingMode::Up),
        "129.42".parse().unwrap()
    );
    assert_eq!(
        n.with_scale_round(-1, RoundingMode::Down),
        "120".parse().unwrap()
    );
    assert_eq!(
        n.with_scale_round(4, RoundingMode::HalfEven),
        "129.4168".parse().unwrap()
    );

    //abs
    let n: BigDecimal = "123.45".parse().unwrap();
    assert_eq!(n.abs(), "123.45".parse().unwrap());

    let n: BigDecimal = "-123.45".parse().unwrap();
    assert_eq!(n.abs(), "123.45".parse().unwrap());

    //cube
    let n: BigDecimal = "1.1156024145937225657484".parse().unwrap();
    assert_eq!(
        n.cube(),
        "1.388443899780141911774491376394890472130000455312878627147979955904"
            .parse()
            .unwrap()
    );

    let n: BigDecimal = "-9.238597585E+84".parse().unwrap();
    assert_eq!(
        n.cube(),
        "-7.88529874035334084567570176625E+254".parse().unwrap()
    );
}

// Calculate large fibonacci numbers.
fn fib(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = f1;
        f1 = f2;
    }
    f0
}

pub(crate) fn bigint_sample() {
    // This is a very large number.
    println!("fib(1000) = {}", fib(1000));

    let mut rng = rand::thread_rng();
    let a = rng.gen_bigint(1000);

    let low = -10000.to_bigint().unwrap();
    let high = 10000.to_bigint().unwrap();
    let b = rng.gen_bigint_range(&low, &high);

    // Probably an even larger number.
    println!("{}", a * b);
}

///
/// Array Sample
/// 数组
///
pub(crate) fn array_sample() {
    println!("datatype::array_sampe ...... start");

    //定义 12个月 数组
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

    //获取数组中元素
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

    //创建一个空的Vec
    let mut empty_vec = Vec::new();
    //push item to vec
    empty_vec.push(123);
    empty_vec.push(456);
    empty_vec.push(789);
    println!("empty vec is {:?}", empty_vec);

    //创建一个包含5个元素的Vec，每个元素都是0
    let zero_vec = vec![0; 5];
    println!("zero vec is {:?}", zero_vec);

    //创建一个包含5个元素的Vec，每个元素都是0
    let mut zero_vec = Vec::with_capacity(5);
    //push item to vec
    zero_vec.push(0);
    zero_vec.push(0);
    zero_vec.push(0);
    zero_vec.push(0);
    zero_vec.push(0);
    println!("zero vec is {:?}", zero_vec);

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

    //使用dot(.) 获取元组数值，offset 从0开始。
    //获取元组第1个值
    let five_hundred = x.0;

    //获取元组 第2个值
    let six_point_four: f64 = x.1;
    //获取元组 第3个值
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

///
///集合 HashMap
///
pub(crate) fn hashmap_example() {
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

    //通过 contains_key 判断是否存在 Key
    if map.contains_key(&"pony".to_string()) {
        //借用 map 权限，获取key 的 val. &map[&key]
        let val = &map[&"pony".to_string()];

        println!("val:{}", val);
    }

    //insert key value
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

/**
 * 集合 HashSet
 * basic/datatype_sample.rs
 */
pub(crate) fn hashset_sample() {
    println!("datatype hashset_sample ..... start");
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    println!("hashset is {:?}", set);
    println!("hashset size:{}", set.len());
    println!("hashset contains 2:{}", set.contains(&2));

    set.remove(&2);
    println!("hashset contains 2:{}", set.contains(&2));

    //集合迭代器
    let set1: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let set2: HashSet<_> = [4, 2, 3, 4].iter().cloned().collect();
    println!("hashset1: {:?}", set1);
    println!("hashset2: {:?}", set2);

    //并集
    let union: HashSet<_> = set1.union(&set2).collect();
    println!("union: {:?}", union);

    //交集
    let intersection: HashSet<_> = set1.intersection(&set2).collect();
    println!("intersection: {:?}", intersection);

    //差集
    let difference: HashSet<_> = set1.difference(&set2).collect();
    println!("difference: {:?}", difference);

    //对称差集
    let symmetric_difference: HashSet<_> = set1.symmetric_difference(&set2).collect();
    println!("symmetric_difference: {:?}", symmetric_difference);

    //空集
    let empty: HashSet<i32> = HashSet::new();
    println!("empty: {:?}", empty);

    //全集
    let full: HashSet<i32> = (0..=10).collect();
    println!("full: {:?}", full);

    //集合长度
    let length: usize = set1.len();
    println!("length: {:?}", length);

    //包含
    let contains: bool = set1.contains(&3);
    println!("contains: {:?}", contains);

    //空集合
    let is_empty: bool = set1.is_empty();
    println!("is_empty: {:?}", is_empty);

    let mut set3: HashSet<i32> = HashSet::new();
    //插入
    set3.insert(1);
    set3.insert(2);
    set3.insert(3);
    println!("set3: {:?}", set3);

    //删除
    set3.remove(&3);
    println!("remove: {:?}", set3);

    //清空
    set3.clear();
    println!("clear: {:?}", set3);

    println!("datatype hashset_sample ..... end\n");
}

/**
 * 集合 BTreeMap
 */
pub(crate) fn btree_map_sample() {
    println!("datatype btree_map_sample ..... start");

    //初始化 BTreeMap 并设置值 有序插入
    let mut map = BTreeMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    map.insert("e", 5);
    map.insert("d", 4);
    map.insert("f", 6);
    map.insert("h", 8);
    map.insert("g", 7);
    map.insert("i", 9);
    map.insert("j", 10);

    println!("map: {:?}", map);

    //集合迭代器
    for (key, value) in map.iter() {
        println!("key: {}, value: {}", key, value);
    }

    //集合长度
    let length: usize = map.len();
    println!("length: {:?}", length);

    //查找
    let find = map.get("a");
    println!("find: {:?}", find);

    //更新
    map.insert("a", 11);
    println!("update: {:?}", map);

    //包含
    let contains: bool = map.contains_key("a");
    println!("contains: {:?}", contains);

    //倒序
    for (key, value) in map.iter().rev() {
        println!("key: {}, value: {}", key, value);
    }

    //删除
    map.remove("b");
    println!("remove: {:?}", map);

    //清空
    map.clear();
    println!("clear: {:?}", map);

    println!("datatype btree_map_sample ..... end\n");
}

/**
 * 集合 BTreeSet
 */
pub(crate) fn btree_set_sample() {
    println!("datatype btree_set_sample ..... start");
    let mut set = BTreeSet::new();
    set.insert("a");
    set.insert("b");
    println!("set: {:?}", set);
    //删除
    set.remove("b");
    println!("remove: {:?}", set);
    //清空
    set.clear();
    println!("clear: {:?}", set);

    //集合迭代器
    for item in set.iter() {
        println!("{}", item);
    }

    //集合长度
    let length: usize = set.len();
    println!("length: {:?}", length);

    //包含
    let contains: bool = set.contains("a");
    println!("contains: {:?}", contains);

    //两个集合
    let set1: BTreeSet<_> = [1, 2, 3].iter().cloned().collect();
    let set2: BTreeSet<_> = [4, 2, 3, 4].iter().cloned().collect();

    println!("set1: {:?}", set1);
    println!("set2: {:?}", set2);

    //交集
    let intersection: BTreeSet<_> = set1.intersection(&set2).cloned().collect();
    println!("intersection: {:?}", intersection);

    //差集
    let difference: BTreeSet<_> = set1.difference(&set2).cloned().collect();
    println!("difference: {:?}", difference);

    //对称差集
    let symmetric_difference: BTreeSet<_> = set1.symmetric_difference(&set2).cloned().collect();
    println!("symmetric_difference: {:?}", symmetric_difference);

    //空集合
    let is_empty: bool = set1.is_empty();
    println!("is_empty: {:?}", is_empty);

    //全集
    let full: BTreeSet<i32> = (0..=10).collect();
    println!("full: {:?}", full);

    println!("datatype btree_set_sample ..... end\n");
}

/**
 * 集合 链表 LinkedList
 */
pub(crate) fn linkedlist_sample() {
    println!("datatype linkedlist_sample ..... start");

    let mut list = LinkedList::from([1, 2, 3]);

    println!("linkelist is {:?}", list);
    // 链表头部增加 值
    list.push_front(0);
    list.push_front(-1);
    //链表尾部追加 值
    list.push_back(4);
    list.push_back(5);

    println!("linkelist is {:?}", list);

    println!("datatype linkedlist_sample ..... end\n");
}

pub fn time_sample() {
    //Instant
    let now = Instant::now();
    // 执行一些代码
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed); // 输出经过的时间

    //Duration
    let timeout = Duration::from_secs(1);
    let start = Instant::now();

    sleep(Duration::from_millis(500));

    if start.elapsed() > timeout {
        println!("Timeout!");
    } else {
        println!("Operation completed within timeout.");
    }

    //SystemTime
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(SystemTime::UNIX_EPOCH);

    println!(
        "Seconds since the epoch: {}",
        since_the_epoch.unwrap().as_secs()
    );
}

fn date_sample() {
    // 使用 from_ymd_opt 创建 NaiveDate
    let date = NaiveDate::from_ymd_opt(2024, 10, 26).unwrap();
    println!("Date: {}", date);

    // 使用 from_hms_opt 创建 NaiveTime
    let time = NaiveTime::from_hms_opt(12, 30, 0).unwrap();
    println!("Time: {}", time);

    // 使用 new 创建 NaiveDateTime
    let datetime = NaiveDateTime::new(date, time);
    println!("DateTime: {}", datetime);

    // 使用 with_ymd_and_hms 创建 DateTime<Utc>
    let utc_datetime = Utc.with_ymd_and_hms(2024, 10, 26, 12, 30, 0).unwrap();
    println!("UTC DateTime: {}", utc_datetime);

    // 使用 with_ymd_and_hms 创建 DateTime<Local>
    let local_datetime = Local.with_ymd_and_hms(2024, 10, 26, 12, 30, 0).unwrap();
    println!("Local DateTime: {}", local_datetime);

    // 获取当前 UTC 时间
    let now_utc = Utc::now();
    println!("Now (UTC): {}", now_utc);

    // 获取当前本地时间
    let now_local = Local::now();
    println!("Now (Local): {}", now_local);

    //日期格式化
    let now = Utc::now();

    // 常用格式
    println!("ISO 8601 / RFC 3339: {}", now.to_rfc3339()); // 推荐的格式
    println!(
        "Year-Month-Day Hour:Minute:Second: {}",
        now.format("%Y-%m-%d %H:%M:%S")
    );
    println!(
        "Day/Month/Year Hour:Minute:Second: {}",
        now.format("%d/%m/%Y %H:%M:%S")
    );
    println!("Month Day, Year: {}", now.format("%B %d, %Y"));
    println!("Weekday, Day Month Year: {}", now.format("%A, %d %B %Y"));

    // 自定义格式
    println!("Custom format: {}", now.format("%a %b %e %T %Y"));

    // 时间戳 (Unix timestamp)
    println!("Timestamp (seconds): {}", now.timestamp());
    println!("Timestamp (milliseconds): {}", now.timestamp_millis());

    //日期解析
    let datetime_str = "2024-10-26 12:30:00";
    let datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S").unwrap();
    println!("Parsed DateTime: {}", datetime);

    let date_str = "2024-10-26";
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
    println!("Parsed Date: {}", date);

    let rfc3339_str = "2024-10-26T12:30:00Z";
    let rfc3339_datetime = DateTime::parse_from_rfc3339(rfc3339_str).unwrap();
    println!("Parsed RFC3339 DateTime: {}", rfc3339_datetime);

    //错误处理
    let invalid_date_str = "2024-13-26";
    let invalid_date = NaiveDate::parse_from_str(invalid_date_str, "%Y-%m-%d");
    match invalid_date {
        Ok(_) => println!("Parsed Date: {:?}", invalid_date),
        Err(e) => println!("Error parsing date: {}", e),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn string_test() {
        string_sample();
    }

    #[test]
    fn decimal_test() {
        decimal_sample();
    }

    #[test]
    fn bigint_test() {
        bigint_sample();
    }

    #[test]
    fn vet_test() {
        vet_sample();
    }

    #[test]
    fn poker_test() {
        enum_sample();
    }

    #[test]
    fn test_user_struct() {
        struct_sample();
    }

    #[test]
    fn array_test() {
        array_sample();
    }

    #[test]
    fn test_hashmap() {
        hashmap_example();
    }

    #[test]
    fn test_hashset() {
        hashset_sample();
    }

    #[test]
    fn test_btree_map() {
        btree_map_sample();
    }

    #[test]
    fn test_btree_set() {
        btree_set_sample();
    }

    #[test]
    fn test_linkedlist() {
        let list = LinkedList::from([1, 2, 3]);

        println!("linkelist is {:?}", list);

        linkedlist_sample();
    }

    #[test]
    fn test_time() {
        time_sample();
    }

    #[test]
    fn test_date() {
        date_sample();
    }
}
