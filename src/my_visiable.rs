#[allow(dead_code)]
mod my_visiable {
    // 一个公有的结构体，带有一个公有的字段（类型为泛型 `T`）
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // 一个公有的结构体，带有一个私有的字段（类型为泛型 `T`）
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // 一个公有的构造器方法
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents: contents }
        }

        /// Returns a reference to the get contentss of this [`ClosedBox<T>`].
        fn get_contents(&self) -> &T {
            return &self.contents;
        }
    }
}

#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn main_test() {
        // 带有公有字段的公有结构体，可以像平常一样构造
        let open_box = my_visiable::OpenBox {
            contents: "public information",
        };

        // 并且它们的字段可以正常访问到。
        println!("The open box contains: {}", open_box.contents);

        // 带有私有字段的公有结构体不能使用字段名来构造。
        // 报错！`ClosedBox` 含有私有字段。
        //let closed_box = my::ClosedBox { contents: "classified information" };
        // 试一试 ^ 取消此行注释

        // 不过带有私有字段的结构体可以使用公有的构造器来创建。
        let _closed_box = my_visiable::ClosedBox::new("classified information");

        // 并且一个结构体中的私有字段不能访问到。
        // 报错！`content` 字段是私有的。
        //println!("The closed box contains: {}", _closed_box.contents);
        // 试一试 ^ 取消此行注释

        // 并且一个结构体中的私有方法不能访问到。
        // 访问！`get_contents` 方法是私有的。
        //println!("The closed box contains: {}", _closed_box.get_contents());
        // 试一试 ^ 取消此行注释
    }
}
