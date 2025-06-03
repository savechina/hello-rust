use derive_more::Display;

#[derive(Getters, Setters, Default, Display)]
#[display("Catetory{{first_category_id:{first_category_id},first_category_name:{first_category_name},second_category_id:{second_category_id},second_category_name:{second_category_name},three_category_id:{three_category_id},three_categroy_name:{three_categroy_name}}}")]
pub struct Category {
    #[getset(get = "pub", set = "pub")]
    first_category_id: u64,
    #[getset(get = "pub", set = "pub")]
    first_category_name: String,
    #[getset(get = "pub", set = "pub")]
    second_category_id: u64,
    #[getset(get = "pub", set = "pub")]
    second_category_name: String,
    #[getset(get = "pub", set = "pub")]
    three_category_id: u64,
    #[getset(get = "pub", set = "pub")]
    three_categroy_name: String,
}

impl Category {
    /// 创建一个新的 builder 实例
    pub fn builder() -> CategoryBuilder {
        CategoryBuilder::new()
    }

    ///setting first category
    pub fn with_first_category(mut self, id: u64, name: String) -> Self {
        self.first_category_id = id;
        self.first_category_name = name;
        self
    }
    ///setting second category
    pub fn with_second_category(mut self, id: u64, name: String) -> Self {
        self.second_category_id = id;
        self.second_category_name = name;
        self
    }

    ///setting three category
    pub fn with_three_category(mut self, id: u64, name: String) -> Self {
        self.three_category_id = id;
        self.three_categroy_name = name;
        self
    }
}

///Category独立的 Builder 结构体
pub struct CategoryBuilder {
    inner: Category,
}

impl CategoryBuilder {
    pub fn new() -> Self {
        Self {
            inner: Category::default(),
        }
    }

    pub fn with_first_category(mut self, id: u64, name: String) -> Self {
        self.inner.set_first_category_id(id);
        self.inner.set_first_category_name(name);
        self
    }

    pub fn with_second_category(mut self, id: u64, name: String) -> Self {
        self.inner.set_second_category_id(id);
        self.inner.set_second_category_name(name);
        self
    }

    pub fn with_three_category(mut self, id: u64, name: String) -> Self {
        self.inner.set_three_category_id(id);
        self.inner.set_three_categroy_name(name);
        self
    }

    pub fn build(self) -> Category {
        self.inner
    }
}

/// Define Product model struct with getset annotations
/// Display formatter String with derive_more annotations
#[derive(Getters, Setters, MutGetters, CopyGetters, Default, Display)]
#[display(
    "Product(id={}, name={}, price={},in_stock={},category={})",
    id,
    name,
    price,
    in_stock,
    category
)]
pub struct Product {
    /// The unique identifier for the product
    #[getset(get = "pub", set = "pub")]
    id: u64,

    /// The name of the product
    #[getset(get = "pub", set = "pub")]
    name: String,

    /// The price of the product in cents
    #[getset(get = "pub", set = "pub")]
    price: u32,

    /// Whether the product is in stock
    #[getset(get_copy = "pub", set = "pub")] // get_copy for primitive types
    in_stock: bool,

    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    category: Category,
}

/// getset an sample . auto generate Struct getter and setter method.
/// create an instance setting it field from getter and setter sample
fn getset_sample() {
    // Create a new product instance
    let mut product = Product {
        id: 1,
        name: "Rust Programming Book".to_string(),
        price: 3999, // $39.99
        in_stock: true,
        category: Category {
            first_category_id: 1,
            first_category_name: "水果".to_string(),
            second_category_id: 10,
            second_category_name: "苹果梨".to_string(),
            three_category_id: 100,
            three_categroy_name: "苹果".to_string(),
        },
    };

    // Use the generated getter methods
    println!("Product ID: {}", product.id());
    println!("Product Name: {}", product.name());
    println!(
        "Price: ${}.{:02}",
        product.price() / 100,
        product.price() % 100
    );
    println!("In Stock: {}", product.in_stock());

    // Use the generated setter methods
    product.set_name("Advanced Rust Programming".to_string());
    product.set_price(4999); // $49.99
    product.set_in_stock(false);

    println!("\nAfter modification:");
    println!("New Name: {}", product.name());
    println!(
        "New Price: ${}.{:02}",
        product.price() / 100,
        product.price() % 100
    );
    println!("Stock Status: {}", product.in_stock());

    println!("Catetory Display:{}", product.category());

    //after cateogry
    let category = product.category_mut();
    category.set_first_category_id(2);
    category.set_first_category_name("蔬菜".to_string());

    println!("After Catetory:{}", product.category());

    println!("After Product:{}", product);
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
    fn test_getset_sample() {
        getset_sample();
    }

    #[test]
    fn test_display_sample() {
        //new Product instance
        let mut product = Product::default();
        product
            .set_id(1)
            .set_name("Rust Programming Book".to_string())
            .set_price(3668)
            .set_in_stock(true);

        println!("{}", product);

        //builder pattern create instance use Category
        let category = Category::default()
            .with_first_category(1, "水果".to_string())
            .with_second_category(10, "苹果梨".to_string())
            .with_three_category(100, "苹果".to_string());

        println!("{}", category);

        //builder pattern create instance use CategoryBuilder
        let category = Category::builder()
            .with_first_category(2, "水果".to_string())
            .with_second_category(21, "苹果梨".to_string())
            .with_three_category(201, "苹果".to_string())
            .build();

        println!("{}", category);
    }
}
