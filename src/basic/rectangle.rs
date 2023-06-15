/**
 * 矩形结构体定义
 */
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

/**
 * 实现矩形结构体方法
 */
impl Rectangle {
    //结构体方法
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    //结构体方法
    pub fn wider(&self, rect: &Rectangle) -> bool {
        self.width > rect.width
    }

    //结构体关联函数
    pub fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}
