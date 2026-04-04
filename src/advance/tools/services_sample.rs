// 服务框架示例
// 完整示例：https://github.com/savechina/hello-rust/blob/main/src/advance/tools/services_sample.rs

use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;
use thiserror::Error;

/// 服务错误类型
#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("启动失败：{0}")]
    StartupFailed(String),
    
    #[error("停止失败：{0}")]
    ShutdownFailed(String),
    
    #[error("健康检查失败：{0}")]
    HealthCheckFailed(String),
    
    #[error("依赖缺失：{0}")]
    DependencyMissing(String),
}

/// 服务生命周期状态
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Stopping,
}

/// 服务 trait - 定义服务接口
#[async_trait]
pub trait Service: Send + Sync {
    /// 服务名称
    fn name(&self) -> &str;
    
    /// 启动服务
    async fn start(&self) -> Result<(), ServiceError>;
    
    /// 停止服务
    async fn stop(&self) -> Result<(), ServiceError>;
    
    /// 健康检查
    async fn health_check(&self) -> bool;
    
    /// 获取服务状态
    fn state(&self) -> ServiceState;
}

/// 日志服务 trait - 用于依赖注入
#[async_trait]
pub trait Logger: Send + Sync {
    async fn log(&self, message: &str);
    async fn flush(&self);
}

/// 控制台日志实现
pub struct ConsoleLogger {
    prefix: String,
}

impl ConsoleLogger {
    pub fn new(prefix: &str) -> Self {
        ConsoleLogger {
            prefix: prefix.to_string(),
        }
    }
}

#[async_trait]
impl Logger for ConsoleLogger {
    async fn log(&self, message: &str) {
        println!("[{}][{}] {}", 
            chrono::Local::now().format("%H:%M:%S"),
            self.prefix,
            message
        );
    }
    
    async fn flush(&self) {
        // 控制台不需要 flush
    }
}

/// 用户服务 - 演示依赖注入
pub struct UserService<L: Logger> {
    logger: L,
    user_count: RwLock<usize>,
}

impl<L: Logger> UserService<L> {
    pub fn new(logger: L) -> Self {
        UserService {
            logger,
            user_count: RwLock::new(0),
        }
    }
    
    pub async fn get_user_count(&self) -> usize {
        *self.user_count.read().await
    }
    
    pub async fn add_user(&self, name: &str) -> Result<(), ServiceError> {
        self.logger.log(&format!("添加用户：{}", name)).await;
        let mut count = self.user_count.write().await;
        *count += 1;
        Ok(())
    }
}

#[async_trait]
impl<L: Logger> Service for UserService<L> {
    fn name(&self) -> &str {
        "UserService"
    }
    
    async fn start(&self) -> Result<(), ServiceError> {
        self.logger.log("启动用户服务").await;
        Ok(())
    }
    
    async fn stop(&self) -> Result<(), ServiceError> {
        self.logger.log("停止用户服务").await;
        Ok(())
    }
    
    async fn health_check(&self) -> bool {
        true  // 简单实现，总是健康
    }
    
    fn state(&self) -> ServiceState {
        ServiceState::Running
    }
}

/// 数据库服务 - 另一个服务示例
pub struct DatabaseService<L: Logger> {
    logger: L,
    connection_string: String,
    connected: RwLock<bool>,
}

impl<L: Logger> DatabaseService<L> {
    pub fn new(logger: L, connection_string: &str) -> Self {
        DatabaseService {
            logger,
            connection_string: connection_string.to_string(),
            connected: RwLock::new(false),
        }
    }
}

#[async_trait]
impl<L: Logger> Service for DatabaseService<L> {
    fn name(&self) -> &str {
        "DatabaseService"
    }
    
    async fn start(&self) -> Result<(), ServiceError> {
        self.logger.log(&format!("连接数据库：{}", self.connection_string)).await;
        // 模拟连接延迟
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        *self.connected.write().await = true;
        self.logger.log("数据库连接成功").await;
        Ok(())
    }
    
    async fn stop(&self) -> Result<(), ServiceError> {
        self.logger.log("断开数据库连接").await;
        *self.connected.write().await = false;
        Ok(())
    }
    
    async fn health_check(&self) -> bool {
        *self.connected.read().await
    }
    
    fn state(&self) -> ServiceState {
        if *self.connected.read().await {
            ServiceState::Running
        } else {
            ServiceState::Stopped
        }
    }
}

/// 服务管理器 - 管理多个服务的生命周期
pub struct ServiceManager {
    services: Arc<RwLock<Vec<Arc<dyn Service>>>>,
    state: Arc<RwLock<ServiceState>>,
}

impl ServiceManager {
    pub fn new() -> Self {
        ServiceManager {
            services: Arc::new(RwLock::new(vec![])),
            state: Arc::new(RwLock::new(ServiceState::Stopped)),
        }
    }
    
    /// 添加服务
    pub async fn add_service(&self, service: Arc<dyn Service>) {
        self.logger().log(&format!("注册服务：{}", service.name())).await;
        self.services.write().await.push(service);
    }
    
    /// 启动所有服务
    pub async fn start_all(&self) -> Result<(), ServiceError> {
        *self.state.write().await = ServiceState::Starting;
        self.logger().log("启动所有服务").await;
        
        let services = self.services.read().await;
        for service in services.iter() {
            self.logger().log(&format!("启动服务：{}", service.name())).await;
            service.start().await?;
        }
        
        *self.state.write().await = ServiceState::Running;
        self.logger().log("所有服务已启动").await;
        Ok(())
    }
    
    /// 停止所有服务
    pub async fn stop_all(&self) -> Result<(), ServiceError> {
        *self.state.write().await = ServiceState::Stopping;
        self.logger().log("停止所有服务").await;
        
        let services = self.services.read().await;
        for service in services.iter() {
            self.logger().log(&format!("停止服务：{}", service.name())).await;
            service.stop().await?;
        }
        
        *self.state.write().await = ServiceState::Stopped;
        self.logger().log("所有服务已停止").await;
        Ok(())
    }
    
    /// 健康检查
    pub async fn health_check(&self) -> bool {
        let services = self.services.read().await;
        for service in services.iter() {
            if !service.health_check().await {
                return false;
            }
        }
        true
    }
    
    /// 获取服务数量
    pub async fn service_count(&self) -> usize {
        self.services.read().await.len()
    }
    
    /// 获取内部 logger（用于演示）
    fn logger(&self) -> ConsoleLogger {
        ConsoleLogger::new("ServiceManager")
    }
}

#[tokio::main]
async fn main() -> Result<(), ServiceError> {
    println!("╔════════════════════════════════════════╗");
    println!("║   服务框架示例                         ║");
    println!("╚════════════════════════════════════════╝\n");

    // 创建服务管理器
    let manager = ServiceManager::new();
    
    // 创建日志器
    let logger = ConsoleLogger::new("App");
    
    // 创建服务（依赖注入日志器）
    let user_service = Arc::new(UserService::new(ConsoleLogger::new("UserService")));
    let db_service = Arc::new(DatabaseService::new(
        ConsoleLogger::new("Database"),
        "postgresql://localhost/mydb"
    ));
    
    // 注册服务
    manager.add_service(user_service).await;
    manager.add_service(db_service).await;
    
    println!("已注册 {} 个服务\n", manager.service_count().await);
    
    // 启动所有服务
    manager.start_all().await?;
    
    // 模拟业务操作
    println!("\n--- 业务操作 ---");
    user_service.add_user("Alice").await?;
    user_service.add_user("Bob").await?;
    println!("当前用户数：{}", user_service.get_user_count().await);
    
    // 健康检查
    println!("\n--- 健康检查 ---");
    let healthy = manager.health_check().await;
    println!("系统健康状态：{}", if healthy { "✓ 健康" } else { "✗ 异常" });
    
    // 停止所有服务
    println!("\n--- 关闭系统 ---");
    manager.stop_all().await?;
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║   示例完成 ✓                           ║");
    println!("╚════════════════════════════════════════╝");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockLogger;
    
    #[async_trait]
    impl Logger for MockLogger {
        async fn log(&self, _message: &str) {}
        async fn flush(&self) {}
    }

    #[tokio::test]
    async fn test_user_service() {
        let logger = MockLogger;
        let service = UserService::new(logger);
        
        service.add_user("Test").await.unwrap();
        assert_eq!(service.get_user_count().await, 1);
    }

    #[tokio::test]
    async fn test_service_manager() {
        let manager = ServiceManager::new();
        let service = Arc::new(UserService::new(MockLogger));
        
        manager.add_service(service).await;
        assert_eq!(manager.service_count().await, 1);
        
        manager.start_all().await.unwrap();
        assert!(manager.health_check().await);
        
        manager.stop_all().await.unwrap();
    }
}
