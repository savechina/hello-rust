# 测试


## Mock 测试

Rust 中有类似Java Mockito 的测试Mock框架，mockall 框架。

下面是一个完成Mock 测试样例代码：

```rust

use mockall::automock;
use std::sync::Arc;

/// trait mock sample
#[automock]
trait HmsMonitorService {
    fn monitor(&self) -> bool;
}

#[derive(Clone)]
pub struct MonitorMessageConsumerListener {
    monitor_service: Arc<dyn HmsMonitorService>,
}

/// async trait sample
#[automock]
#[async_trait::async_trait]
trait HmsMonitorAsyncService {
    async fn monitor(&self) -> bool;
}

#[derive(Clone)]
pub struct MonitorMessageController {
    monitor_service: Arc<dyn HmsMonitorAsyncService>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor() {
        let mut mock = MockHmsMonitorService::new();
        mock.expect_monitor().returning(|| true);
        let listener = MonitorMessageConsumerListener {
            monitor_service: Arc::new(mock),
        };
        assert!(listener.monitor_service.monitor());
    }

    #[tokio::test]
    async fn test_monitor_async() {
        let mut mock = MockHmsMonitorAsyncService::new();
        mock.expect_monitor().returning(|| true);
        let listener = MonitorMessageController {
            monitor_service: Arc::new(mock),
        };
        assert!(listener.monitor_service.monitor().await);
    }
}


```


## Rspec 测试
