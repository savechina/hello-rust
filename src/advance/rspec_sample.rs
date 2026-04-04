use mockall::automock;
use std::{fmt::Debug, sync::Arc};

/// 监控服务 trait
/// 用于演示 RSpec 风格的行为驱动测试
#[automock]
pub trait HmsMonitorService: Debug + Send + Sync {
    fn monitor(&self) -> bool;
    fn get_status(&self) -> String;
    fn check_health(&self, timeout_ms: u64) -> Result<bool, String>;
}

/// 消息消费者监听器
/// 依赖注入 MonitorService
#[derive(Clone, Debug)]
pub struct MonitorMessageConsumerListener {
    monitor_service: Arc<dyn HmsMonitorService>,
}

impl MonitorMessageConsumerListener {
    pub fn new(monitor_service: Arc<dyn HmsMonitorService>) -> Self {
        Self { monitor_service }
    }

    pub fn check_system_health(&self) -> bool {
        self.monitor_service.monitor()
    }

    pub fn get_system_status(&self) -> String {
        self.monitor_service.get_status()
    }
}

/// 简单计算器
/// 用于演示基本的 RSpec 测试
pub struct Calculator {
    value: f64,
}

impl Calculator {
    pub fn new() -> Self {
        Self { value: 0.0 }
    }

    pub fn add(&mut self, x: f64) {
        self.value += x;
    }

    pub fn subtract(&mut self, x: f64) {
        self.value -= x;
    }

    pub fn multiply(&mut self, x: f64) {
        self.value *= x;
    }

    pub fn divide(&mut self, x: f64) -> Result<(), String> {
        if x == 0.0 {
            Err("除数不能为 0".to_string())
        } else {
            self.value /= x;
            Ok(())
        }
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn reset(&mut self) {
        self.value = 0.0;
    }
}

/// 用户结构体
/// 用于演示数据验证测试
#[derive(Debug, PartialEq)]
pub struct User {
    pub name: String,
    pub email: String,
    pub age: u8,
}

impl User {
    pub fn new(name: &str, email: &str, age: u8) -> Result<Self, String> {
        if name.is_empty() {
            return Err("姓名不能为空".to_string());
        }
        if !email.contains('@') {
            return Err("邮箱格式不正确".to_string());
        }
        if age < 0 || age > 150 {
            return Err("年龄必须在 0-150 之间".to_string());
        }
        Ok(Self {
            name: name.to_string(),
            email: email.to_string(),
            age,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspec::describe;

    /// RSpec 风格的测试套件
    /// 使用 describe/it 语法组织测试
    #[test]
    fn test_rspec_monitor_suite() {
        rspec::run(&describe("MonitorMessageConsumerListener", (), |ctx| {
            ctx.describe("when monitor service returns true", |ctx| {
                ctx.it("should report healthy", |()| {
                    let mut mock = MockHmsMonitorService::new();
                    mock.expect_monitor().returning(|| true);
                    mock.expect_get_status().returning(|| "healthy".to_string());

                    let listener = MonitorMessageConsumerListener::new(Arc::new(mock));

                    assert!(listener.check_system_health());
                    assert_eq!(listener.get_system_status(), "healthy");
                    true
                });
            });

            ctx.describe("when monitor service returns false", |ctx| {
                ctx.it("should report unhealthy", |()| {
                    let mut mock = MockHmsMonitorService::new();
                    mock.expect_monitor().returning(|| false);

                    let listener = MonitorMessageConsumerListener::new(Arc::new(mock));

                    assert!(!listener.check_system_health());
                    true
                });
            });
        }));
    }

    /// 计算器 RSpec 测试
    #[test]
    fn test_rspec_calculator_suite() {
        rspec::run(&describe("Calculator", Calculator::new(), |ctx| {
            ctx.before(|calc| {
                calc.reset();
            });

            ctx.describe("addition", |ctx| {
                ctx.it("should add positive numbers", |calc| {
                    calc.add(5.0);
                    calc.add(3.0);
                    assert_eq!(calc.get_value(), 8.0);
                    true
                });

                ctx.it("should add negative numbers", |calc| {
                    calc.add(-5.0);
                    calc.add(-3.0);
                    assert_eq!(calc.get_value(), -8.0);
                    true
                });
            });

            ctx.describe("subtraction", |ctx| {
                ctx.it("should subtract numbers", |calc| {
                    calc.add(10.0);
                    calc.subtract(4.0);
                    assert_eq!(calc.get_value(), 6.0);
                    true
                });
            });

            ctx.describe("multiplication", |ctx| {
                ctx.it("should multiply numbers", |calc| {
                    calc.add(5.0);
                    calc.multiply(3.0);
                    assert_eq!(calc.get_value(), 15.0);
                    true
                });
            });

            ctx.describe("division", |ctx| {
                ctx.it("should divide numbers", |calc| {
                    calc.add(10.0);
                    calc.divide(2.0).unwrap();
                    assert_eq!(calc.get_value(), 5.0);
                    true
                });

                ctx.it("should return error for division by zero", |calc| {
                    calc.add(10.0);
                    let result = calc.divide(0.0);
                    assert!(result.is_err());
                    true
                });
            });
        }));
    }

    /// 用户验证 RSpec 测试
    #[test]
    fn test_rspec_user_validation_suite() {
        rspec::run(&describe("User Validation", (), |ctx| {
            ctx.describe("valid user creation", |ctx| {
                ctx.it("should create user with valid data", |()| {
                    let user = User::new("Alice", "alice@example.com", 30);
                    assert!(user.is_ok());
                    let user = user.unwrap();
                    assert_eq!(user.name, "Alice");
                    assert_eq!(user.email, "alice@example.com");
                    assert_eq!(user.age, 30);
                    true
                });
            });

            ctx.describe("invalid user creation", |ctx| {
                ctx.it("should reject empty name", |()| {
                    let user = User::new("", "test@example.com", 25);
                    assert!(user.is_err());
                    assert_eq!(user.unwrap_err(), "姓名不能为空");
                    true
                });

                ctx.it("should reject invalid email", |()| {
                    let user = User::new("Bob", "invalid-email", 25);
                    assert!(user.is_err());
                    assert_eq!(user.unwrap_err(), "邮箱格式不正确");
                    true
                });

                ctx.it("should reject invalid age", |()| {
                    let user = User::new("Charlie", "charlie@example.com", 200);
                    assert!(user.is_err());
                    assert_eq!(user.unwrap_err(), "年龄必须在 0-150 之间");
                    true
                });
            });
        }));
    }

    /// 传统测试风格对比
    #[test]
    fn test_monitor_listener() {
        let mut mock = MockHmsMonitorService::new();
        mock.expect_monitor().returning(|| true);
        let listener = MonitorMessageConsumerListener::new(Arc::new(mock));
        assert!(listener.monitor_service.monitor());
    }

    #[test]
    fn test_monitor_listener_call_count() {
        let mut mock = MockHmsMonitorService::new();
        mock.expect_monitor().times(1).returning(|| false);
        let listener = MonitorMessageConsumerListener::new(Arc::new(mock));
        assert_eq!(listener.monitor_service.monitor(), false);
    }
}
