//!
//! 包为retangle
//! Rectangle 结构体
//!

/**
 *  Rectangle 矩形结构体定义
 */
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

/**
 * 实现矩形结构体方法
 */
impl Rectangle {
    /// 结构体关联函数
    /// 创建矩形结构体,Rust 习惯性使用new 创建结构体实例
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    ///结构体方法
    /// 计算矩形的面积
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    /// 结构体方法
    /// 计算矩形的周长
    pub fn wider(&self, rect: &Rectangle) -> bool {
        self.width > rect.width
    }

    ///结构体关联函数
    /// 创建矩形结构体实例
    pub fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}
