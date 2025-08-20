use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub trait AnyService: Any + Send + Sync + 'static {}
impl<T: Any + Send + Sync + 'static> AnyService for T {}

// pub(crate) type ServiceInstance = Arc<RwLock<dyn AnyService + Send + Sync + 'static>>;
// pub(crate) type ScopedMap = HashMap<ServiceKey, ServiceInstance>;
// pub(crate) type FactoryMap = HashMap<ServiceKey, ServiceFactory>;
// pub(crate) type ServiceKey = (String, String);
// pub(crate) type RegisteredInstances = OnceCell<ArcSwap<FactoryMap>>;
// pub(crate) type ServiceFactory = Arc<
//     dyn Fn(Arc<NaiveDate>) -> Pin<Box<dyn Future<Output = Result<ServiceInstance, DiError>> + Send>>
//         + Send
//         + Sync
//         + 'static,
// >;

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
