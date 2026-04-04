use mockall::automock;
use std::sync::Arc;

/// 同步服务 trait
/// 使用 #[automock] 自动生成 Mock 实现
#[automock]
pub trait HmsMonitorService {
    fn monitor(&self) -> bool;
    fn get_status(&self) -> String;
    fn check_health(&self, timeout_ms: u64) -> Result<bool, String>;
}

/// 异步服务 trait
/// 支持 async/await 的 Mock 生成
#[automock]
#[async_trait::async_trait]
pub trait HmsMonitorAsyncService {
    async fn monitor(&self) -> bool;
    async fn get_status(&self) -> String;
    async fn check_health(&self, timeout_ms: u64) -> Result<bool, String>;
}

/// 消息消费者监听器
/// 依赖注入 MonitorService
#[derive(Clone)]
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

/// 消息控制器
/// 依赖注入 AsyncMonitorService
#[derive(Clone)]
pub struct MonitorMessageController {
    monitor_service: Arc<dyn HmsMonitorAsyncService>,
}

impl MonitorMessageController {
    pub fn new(monitor_service: Arc<dyn HmsMonitorAsyncService>) -> Self {
        Self { monitor_service }
    }

    pub async fn check_system_health_async(&self) -> bool {
        self.monitor_service.monitor().await
    }

    pub async fn get_system_status_async(&self) -> String {
        self.monitor_service.get_status().await
    }
}

/// 带参数的 Mock 示例
/// 展示如何 Mock 带参数的方法
#[automock]
pub trait DataProcessor {
    fn process(&self, data: &[u8]) -> Result<Vec<u8>, String>;
    fn transform(&self, input: &str, times: usize) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    /// 测试同步 Mock
    #[test]
    fn test_monitor_sync() {
        let mut mock = MockHmsMonitorService::new();
        mock.expect_monitor().returning(|| true);
        mock.expect_get_status().returning(|| "healthy".to_string());

        let listener = MonitorMessageConsumerListener::new(Arc::new(mock));

        assert!(listener.check_system_health());
        assert_eq!(listener.get_system_status(), "healthy");
    }

    /// 测试异步 Mock
    #[tokio::test]
    async fn test_monitor_async() {
        let mut mock = MockHmsMonitorAsyncService::new();
        mock.expect_monitor().returning(|| true);
        mock.expect_get_status().returning(|| "healthy".to_string());

        let controller = MonitorMessageController::new(Arc::new(mock));

        assert!(controller.check_system_health_async().await);
        assert_eq!(controller.get_system_status_async().await, "healthy");
    }

    /// 测试带参数的 Mock
    #[test]
    fn test_data_processor() {
        let mut mock = MockDataProcessor::new();

        // 设置特定参数的返回值
        mock.expect_process()
            .with(eq(b"hello"))
            .returning(|_| Ok(b"HELLO".to_vec()));

        mock.expect_transform()
            .with(eq("hello"), eq(3))
            .returning(|s, n| s.repeat(n));

        assert_eq!(mock.process(b"hello").unwrap(), b"HELLO");
        assert_eq!(mock.transform("hello", 3), "hellohellohello");
    }

    /// 测试多次调用
    #[test]
    fn test_multiple_calls() {
        let mut mock = MockHmsMonitorService::new();

        // 第一次返回 true，第二次返回 false
        mock.expect_monitor()
            .times(2)
            .returning(|| {
                static mut COUNT: i32 = 0;
                unsafe {
                    COUNT += 1;
                    COUNT == 1
                }
            });

        assert!(mock.monitor());
        assert!(!mock.monitor());
    }

    /// 测试错误返回
    #[test]
    fn test_error_return() {
        let mut mock = MockHmsMonitorAsyncService::new();
        mock.expect_check_health()
            .returning(|_| Err("timeout".to_string()));

        let result = futures::executor::block_on(mock.check_health(1000));
        assert_eq!(result, Err("timeout".to_string()));
    }

    /// 测试调用次数验证
    #[test]
    fn test_call_count_verification() {
        let mut mock = MockHmsMonitorService::new();
        mock.expect_monitor().times(3).returning(|| true);

        mock.monitor();
        mock.monitor();
        mock.monitor();

        // 如果调用次数不匹配，测试会失败
    }
}
