//!
//!
//! Basic DataType Sample
//!
//!

use std::array;
use std::collections::HashMap;
use std::vec;

///
///集合 HashMap
///
pub(crate) fn collections_example() {
    println!(" collections_example ... start");

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

    println!(" collections_example ... end\n");
}

///
/// Array Sample
/// 数组
///

pub(crate) fn array_sample() {
    println!("array_sampe ...... start");
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

    println!("Months is {:?}", months);

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

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn vet_test() {
        vet_sample();
    }
}
