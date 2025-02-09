//!
//! 包为retangle
//! Rectangle 结构体
//!

/**
 *  Rectangle 矩形结构体定义
 */
pub struct Rectangle {
    /// 矩形宽度
    pub width: u32,
    /// 矩形高度
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

/**
 * Rectangle 结构体构造器，builder模式，用于创建 Rectangle 实例
 *
 */
pub struct RectangleBuilder {
    /// 宽度
    width: u32,
    /// 高度
    height: u32,
}

/**
 *
 * RectangleBuilder 结构体实现
 */
impl RectangleBuilder {
    /// RectangleBuilder 结构体关联函数
    pub fn new() -> RectangleBuilder {
        RectangleBuilder {
            width: 0,
            height: 0,
        }
    }

    /// RectangleBuilder 结构体方法 设置宽度
    pub fn width(&mut self, width: u32) -> &mut RectangleBuilder {
        self.width = width;
        self
    }

    /// RectangleBuilder 结构体方法 设置高度
    pub fn height(&mut self, height: u32) -> &mut RectangleBuilder {
        self.height = height;
        self
    }

    /// RectangleBuilder 结构体方法 构建Rectangle 实例
    pub fn build(&self) -> Rectangle {
        Rectangle {
            width: self.width,
            height: self.height,
        }
    }

    /// RectangleBuilder 结构体方法 构建Rectangle 实例并返回所有权
    /// 该方法会消费RectangleBuilder 实例
    /// 返回Rectangle 实例所有权
    pub fn finalize(&self) -> Rectangle {
        Rectangle {
            width: self.width,
            height: self.height,
        }
    }
}

/**
 *  Rectangle 结构体引用
 */
pub struct RectangleRef<'a> {
    /// 矩形宽度
    pub width: &'a u32,
    /// 矩形高度
    pub height: &'a u32,
}
impl<'a> RectangleRef<'a> {
    /**
     * 构造RectangleRef 实例
     */
    pub fn new(width: &'a u32, height: &'a u32) -> Self {
        RectangleRef { width, height }
    }
}
/**
 *  Rectangle 结构体所有权
 */
pub struct RectangleOwner<'a> {
    /// 矩形宽度
    pub width: &'a mut u32,
    /// 矩形高度
    pub height: &'a mut u32,
}
impl<'a> RectangleOwner<'a> {
    /**
     * 构造RectangleOwner 实例
     */
    pub fn new(width: &'a mut u32, height: &'a mut u32) -> Self {
        RectangleOwner { width, height }
    }
}
/**
 *  Rectangle 结构体借用
 */
pub struct RectangleBorrow<'a> {
    /// 矩形宽度
    pub width: &'a u32,
    /// 矩形高度
    pub height: &'a u32,
}
impl<'a> RectangleBorrow<'a> {
    /**
     * 构造RectangleBorrow 实例
     */
    pub fn new(width: &'a u32, height: &'a u32) -> Self {
        RectangleBorrow { width, height }
    }
}
/**
 *  Rectangle 结构体可变借用
 */
pub struct RectangleBorrowMut<'a> {
    /// 矩形宽度
    pub width: &'a mut u32,
    /// 矩形高度
    pub height: &'a mut u32,
}
impl<'a> RectangleBorrowMut<'a> {
    /**
     * 构造RectangleBorrowMut 实例
     */
    pub fn new(width: &'a mut u32, height: &'a mut u32) -> Self {
        RectangleBorrowMut { width, height }
    }
}
