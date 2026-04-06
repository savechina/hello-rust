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
        ClosedBox { contents }
    }

    /// Returns a reference to the contents of this [`ClosedBox<T>`].
    #[allow(dead_code)]
    fn get_contents(&self) -> &T {
        &self.contents
    }
}

/// Visibility sample demonstrating public/private struct fields and methods.
pub fn visiable_sample() {
    // 带有公有字段的公有结构体，可以像平常一样构造
    let open_box = OpenBox {
        contents: "public information",
    };
    println!("The open box contains: {}", open_box.contents);

    // 带有私有字段的结构体可以使用公有的构造器来创建
    let _closed_box = ClosedBox::new("classified information");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visiable_test() {
        visiable_sample();
    }
}
