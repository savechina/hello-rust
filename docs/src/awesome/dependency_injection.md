# 服务依赖注入(Dependency Injection)


## 简单的静态依赖注入

```rust
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, RwLock},
};


pub trait AnyService: Any + Send + Sync + 'static {}
impl<T: Any + Send + Sync + 'static> AnyService for T {}


// 定义一个 Repository Trait
trait UserRepository: Any + Send + Sync {
    fn get_user(&self, id: u32) -> String;
}

// 实现一个具体的 Repository
struct InMemoryUserRepository;

impl UserRepository for InMemoryUserRepository {
    fn get_user(&self, id: u32) -> String {
        format!("User {} from InMemoryRepo", id)
    }
}

// 定义一个 Service，它依赖 UserRepository
struct UserService<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UserService<R> {
    fn new(repo: R) -> Self {
        Self { repo }
    }

    fn greet_user(&self, id: u32) -> String {
        let user = self.repo.get_user(id);
        format!("Hello, {}!", user)
    }
}

 // di 通常与异步运行时配合，但这里可以不用 async
 fn dependency_injection_manul_sample() -> anyhow::Result<()> {
    // --- 创建 DI 容器 ---
    let mut services = HashMap::<TypeId, Arc<dyn Any + Send + Sync>>::new();

    services.insert(
        TypeId::of::<InMemoryUserRepository>(),
        Arc::new(InMemoryUserRepository),
    );

    let repo = services
        .get(&TypeId::of::<InMemoryUserRepository>())
        .unwrap();

    let repo = (repo.clone()).downcast::<InMemoryUserRepository>().unwrap();

    services.insert(
        TypeId::of::<UserService<InMemoryUserRepository>>(),
        Arc::new(UserService::new(InMemoryUserRepository)),
    );

    //  --- 从容器中解析出 UserService ---

    let user_serivce = services
        .get(&TypeId::of::<UserService<InMemoryUserRepository>>())
        .unwrap();

    let user_service = user_serivce
        .clone()
        .downcast::<UserService<InMemoryUserRepository>>()
        .unwrap();

    // // 调用业务方法
    let greeting = user_service.greet_user(42);
    println!("{}", greeting);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_injection_sample() {
        dependency_injection_manul_sample();
    }
}


```

执行输出结果：
```text
running 1 test
test services::dependency_injection_concrete_sample::tests::test_dependency_injection_sample ... ok

successes:

---- services::dependency_injection_concrete_sample::tests::test_dependency_injection_sample stdout ----
Hello, User 42 from InMemoryRepo!


successes:
    services::dependency_injection_concrete_sample::tests::test_dependency_injection_sample

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.00s

```

## 服务容器的动态依赖注入

```rust
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Base trait for all services
trait Service: Send + Sync + 'static {
    fn name(&self) -> &'static str;
}

// Example service interfaces
trait LoggerService: Send + Sync + 'static {
    fn log(&self, message: &str);
}

trait DatabaseService: Send + Sync + 'static {
    fn query(&self, query: &str) -> String;
}

// Concrete service implementations
struct ConsoleLogger;
impl Service for ConsoleLogger {
    fn name(&self) -> &'static str {
        "ConsoleLogger"
    }
}
impl LoggerService for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("Log: {}", message);
    }
}

struct InMemoryDatabase;
impl Service for InMemoryDatabase {
    fn name(&self) -> &'static str {
        "InMemoryDatabase"
    }
}
impl DatabaseService for InMemoryDatabase {
    fn query(&self, query: &str) -> String {
        format!("Query result for: {}", query)
    }
}

// Service with dependencies
struct BusinessService {
    logger: Arc<dyn LoggerService>,
    database: Arc<dyn DatabaseService>,
}
impl Service for BusinessService {
    fn name(&self) -> &'static str {
        "BusinessService"
    }
}
impl BusinessService {
    fn new(logger: Arc<dyn LoggerService>, database: Arc<dyn DatabaseService>) -> Self {
        BusinessService { logger, database }
    }

    fn perform_task(&self, task: &str) {
        self.logger.log(&format!("Performing task: {}", task));
        let result = self.database.query(task);
        self.logger.log(&format!("Task result: {}", result));
    }
}

// Service Container
struct ServiceContainer {
    services: Mutex<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
    factories: Mutex<HashMap<TypeId, Box<dyn Fn(&ServiceContainer) -> Arc<dyn Any + Send + Sync>>>>,
}

impl ServiceContainer {
    fn new() -> Self {
        ServiceContainer {
            services: Mutex::new(HashMap::new()),
            factories: Mutex::new(HashMap::new()),
        }
    }

    // Register a concrete service
    fn register<T: Service + 'static>(&self, service: T) {
        let type_id = TypeId::of::<T>();
        let service: Arc<dyn Any + Send + Sync> = Arc::new(service);
        self.services.lock().unwrap().insert(type_id, service);
    }

    // Register a trait object
    fn register_trait<T: ?Sized + Any + Send + Sync + 'static>(
        &self,
        service: Arc<dyn Any + Send + Sync>,
        type_id: TypeId,
    ) {
        // Ensure the service is a trait object
        // Insert the service into the container
        let service: Arc<dyn Any + Send + Sync> = service;
        // let type_id = TypeId::of::<T>();

        println!("Registering trait object: {:?}", type_id);

        // Insert the service into the services map
        self.services.lock().unwrap().insert(type_id, service);
    }

    // Register a factory for lazy initialization
    fn register_factory<
        T: Service + 'static,
        F: Fn(&ServiceContainer) -> Arc<T> + Send + Sync + 'static,
    >(
        &self,
        factory: F,
    ) {
        let type_id = TypeId::of::<T>();
        let wrapped_factory: Box<dyn Fn(&ServiceContainer) -> Arc<dyn Any + Send + Sync>> =
            Box::new(move |container| {
                let service = factory(container);
                service as Arc<dyn Any + Send + Sync>
            });
        self.factories
            .lock()
            .unwrap()
            .insert(type_id, wrapped_factory);
    }

    // Resolve a concrete service
    fn resolve<T: Service + 'static>(&self) -> Option<Arc<T>> {
        let type_id = TypeId::of::<T>();
        if let Some(service) = self.services.lock().unwrap().get(&type_id) {
            return service.clone().downcast::<T>().ok();
        }
        if let Some(factory) = self.factories.lock().unwrap().get(&type_id) {
            let service = factory(self);
            self.services
                .lock()
                .unwrap()
                .insert(type_id, service.clone());
            return service.downcast::<T>().ok();
        }
        None
    }

    // Resolve a trait object (accepts Arc<dyn Trait> types); trait objects themselves (dyn Trait) are unsized,
    // so call this with Arc<dyn Trait> as the type parameter (e.g. resolve_trait::<Arc<dyn LoggerService>>()).
    fn resolve_trait<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        let type_id = TypeId::of::<T>();

        println!("resolve trait trait object: {:?}", type_id);
        if let Some(service) = self.services.lock().unwrap().get(&type_id) {
            let downcast = service.clone().downcast::<T>().ok();

            println!("resolve trait trait object: {:?}", downcast.is_some());

            return downcast;
        }

        None
    }
}

fn container_injection_main() {
    let container = Arc::new(ServiceContainer::new());

    // Register concrete services
    container.register(ConsoleLogger);
    container.register(InMemoryDatabase);

    // Register trait objects
    // using a box or reference package to dyn trait objects
    container.register_trait::<Arc<dyn LoggerService>>(
        Arc::new(Arc::new(ConsoleLogger) as Arc<dyn LoggerService>),
        TypeId::of::<Arc<dyn LoggerService>>(),
    );

    container.register_trait::<Arc<dyn DatabaseService>>(
        Arc::new(Arc::new(InMemoryDatabase) as Arc<dyn DatabaseService>),
        TypeId::of::<Arc<dyn DatabaseService>>(),
    );

    // Register a factory for BusinessService (resolve concrete implementations and coerce to trait objects)
    container.register_factory::<BusinessService, _>(|container| {
        let logger_concrete = container
            .resolve::<ConsoleLogger>()
            .expect("LoggerService not found");
        let logger: Arc<dyn LoggerService> = logger_concrete;

        let database_concrete = container
            .resolve::<InMemoryDatabase>()
            .expect("DatabaseService not found");
        let database: Arc<dyn DatabaseService> = database_concrete;

        Arc::new(BusinessService::new(logger, database))
    });

    // Resolve and use BusinessService
    let business_service = container
        .resolve::<BusinessService>()
        .expect("BusinessService not found");
    business_service.perform_task("Process data");

    // Resolve and use a trait object (request Arc<dyn LoggerService> as the type parameter)
    let logger = container
        .resolve_trait::<Arc<dyn LoggerService>>()
        .expect("LoggerService not found");
    logger.log("Direct logger access");

    // Resolve and use a trait object (request Arc<dyn DatabaseService> as the type parameter)
    let database_service = container
        .resolve_trait::<Arc<dyn DatabaseService>>()
        .expect("DatabaseService not found");
    let result = database_service.query("Direct database access");

    println!("query: {}", result);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_injection_main() {
        container_injection_main();
    }
}


```

执行输出结果：
```text

running 1 test
test services::dependency_injection_dynmaic_sample::tests::test_injection_main ... ok

successes:

---- services::dependency_injection_dynmaic_sample::tests::test_injection_main stdout ----
Registering trait object: TypeId(0x56cbfa3cabf188feb9d956576e38424a)
Registering trait object: TypeId(0x8ffe278beee70f55493c3bb2f23af8b3)
Log: Performing task: Process data
Log: Task result: Query result for: Process data
resolve trait trait object: TypeId(0x56cbfa3cabf188feb9d956576e38424a)
resolve trait trait object: true
Log: Direct logger access
resolve trait trait object: TypeId(0x8ffe278beee70f55493c3bb2f23af8b3)
resolve trait trait object: true
query: Query result for: Direct database access


successes:
    services::dependency_injection_dynmaic_sample::tests::test_injection_main

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.00s

```