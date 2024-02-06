//!
//! module_sample.rs 文件就是module_sample 模块
//!
// 此处定义：一个名为 `supper_mod` 的模块，与文件模块重复。
//因模块不是公开的，无法在super 模块访问
//!
//!

#[allow(dead_code)]
pub mod supper_mod {
    // 模块中的项默认具有私有的可见性
    fn private_function() {
        println!("called `supper_mod::private_function()`");
    }

    // 使用 `pub` 修饰语来改变默认可见性。
    pub fn function() {
        println!("called `supper_mod::function()`");
    }

    // 在同一模块中，项可以访问其它项，即使它是私有的。
    pub fn indirect_access() {
        print!("called `supper_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // 模块也可以嵌套
    pub mod nested {
        pub fn function() {
            println!("called `supper_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `supper_mod::nested::private_function()`");
        }

        // 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
        // `path` 必须是父模块（parent module）或祖先模块（ancestor module）
        pub(in crate::basic::module_sample) fn public_function_in_supper_mod() {
            print!("called `supper_mod::nested::public_function_in_supper_mod()`, that\n > ");
            public_function_in_nested()
        }

        // 使用 `pub(self)` 语法定义的函数则只在当前模块中可见。
        pub(self) fn public_function_in_nested() {
            println!("called `supper_mod::nested::public_function_in_nested");
        }

        // 使用 `pub(super)` 语法定义的函数只在父模块中可见。
        pub(super) fn public_function_in_super_mod() {
            println!("called supper_mod::nested::public_function_in_super_mod");
        }
    }

    pub fn call_public_function_in_supper_mod() {
        print!("called `supper_mod::call_public_funcion_in_supper_mod()`, that\n> ");

        nested::public_function_in_supper_mod();

        print!("> ");

        nested::public_function_in_super_mod();
    }

    // `pub(crate)` 使得函数只在当前 crate 中可见
    pub(crate) fn public_function_in_crate() {
        println!("called `supper_mod::public_function_in_crate()");
    }

    // 嵌套模块的可见性遵循相同的规则
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `supper_mod::private_nested::function()`");
        }
    }
}

#[allow(dead_code)]
pub fn function() {
    println!("called `function()`");
}

#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn main_test() {
        // 模块机制消除了相同名字的项之间的歧义。
        function();
        supper_mod::function();

        // 公有项，包括嵌套模块内的，都可以在父模块外部访问。
        supper_mod::indirect_access();
        supper_mod::nested::function();
        supper_mod::call_public_function_in_supper_mod();

        // pub(crate) 项可以在同一个 crate 中的任何地方访问
        supper_mod::public_function_in_crate();

        // pub(in path) 项只能在指定的模块中访问
        // 报错！函数 `public_function_in_supper_mod` 是私有的
        supper_mod::nested::public_function_in_supper_mod();
        // 试一试 ^ 取消该行的注释

        // 模块的私有项不能直接访问，即便它是嵌套在公有模块内部的

        // 报错！`private_function` 是私有的
        //supper_mod::private_function();
        // 试一试 ^ 取消此行注释

        // 报错！`private_function` 是私有的
        //supper_mod::nested::private_function();
        // 试一试 ^ 取消此行的注释

        // Error! `private_nested` is a private module
        //supper_mod::private_nested::function();
        // 试一试 ^ 取消此行的注释
    }
}
