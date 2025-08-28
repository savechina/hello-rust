use std::any::{Any, TypeId};
use std::boxed;
use std::collections::HashMap;
use std::sync::Arc;

// Define the Logger trait with Any for downcasting
trait Logger: Any + Send + Sync {
    fn log(&self, message: &str);
}

struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("Log: {}", message);
    }
}

// Service Locator struct
struct ServiceContainer {
    services: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl ServiceContainer {
    fn new() -> Self {
        ServiceContainer {
            services: HashMap::new(),
        }
    }

    fn register_logger(&mut self, logger: impl Logger) {
        self.services.insert(
            TypeId::of::<dyn Logger>(),
            Arc::new(Box::new(logger) as Box<dyn Logger>),
        );
    }

    fn get_logger(&self) -> Option<Arc<Box<dyn Logger>>> {
        self.services
            .get(&TypeId::of::<dyn Logger>())
            .and_then(|service| service.clone().downcast::<Box<dyn Logger>>().ok())
            .map(|boxed| boxed)
    }
}

// Service that uses the Service Locator
struct UserService {
    locator: Arc<ServiceContainer>,
}

impl UserService {
    fn new(locator: Arc<ServiceContainer>) -> Self {
        UserService { locator }
    }

    fn perform_action(&self) {
        // 使用 trait 对象
        if let Some(logger) = self.locator.get_logger() {
            logger.log("Action with trait object");
        }
    }
}

fn service_container_main() {
    let mut locator = ServiceContainer::new();
    locator.register_logger(ConsoleLogger);
    let locator = Arc::new(locator);

    let user_service = UserService::new(locator);
    user_service.perform_action();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locator_main() {
        service_container_main();
    }
}
