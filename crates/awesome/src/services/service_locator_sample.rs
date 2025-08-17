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

// impl Any for ConsoleLogger {
//     fn type_id(&self) -> TypeId {
//         TypeId::of::<Self>()
//     }
// }

// Service Locator struct
struct ServiceLocator {
    services: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl ServiceLocator {
    fn new() -> Self {
        ServiceLocator {
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
    locator: Arc<ServiceLocator>,
}

impl UserService {
    fn new(locator: Arc<ServiceLocator>) -> Self {
        UserService { locator }
    }

    fn perform_action(&self) {
        // 使用 trait 对象
        if let Some(logger) = self.locator.get_logger() {
            logger.log("Action with trait object");
        }
    }
}

fn service_locator_main() {
    let mut locator = ServiceLocator::new();
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
        service_locator_main();
    }
}
