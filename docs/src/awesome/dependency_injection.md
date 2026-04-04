# 依赖注入 (Dependency Injection)

## 开篇故事

想象你在组装一台电脑。如果你把 CPU、内存、硬盘全部焊死在主板上（**硬编码依赖**），那么升级任何一个部件都需要更换整块主板。但如果你使用插槽和接口（**依赖注入**），你可以随时更换任何部件，而无需改动主板。

在软件中，依赖注入就是这种"插槽"机制——服务不自己创建依赖，而是通过外部"注入"依赖。这让代码更易测试、更易维护、更易扩展。

---

## 本章适合谁

如果你在构建中大型 Rust 应用，需要管理服务之间的依赖关系、实现可测试的代码架构、或者理解 Rust 中的 DI 模式，本章适合你。

---

## 你会学到什么

完成本章后，你可以：

1. 理解 Rust 中依赖注入的三种主要模式
2. 使用具体类型注入（Concrete Injection）
3. 使用 Arc 和 Box 实现动态依赖注入
4. 构建 Service Container（服务容器）
5. 使用工厂模式实现延迟初始化
6. 解析具体服务和 Trait 对象

---

## 前置要求

- [特征](../basic/trait.md) - trait 基础
- [泛型](../basic/generic.md) - 泛型基础
- [Arc/Mutex](../advance/async/tokio.md) - 并发基础

---


### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add tokio --features full
cargo add anyhow
```

## 第一个例子

最简单的具体类型注入：

```rust,ignore
// 定义 Repository Trait
trait UserRepository {
    fn get_user(&self, id: u32) -> String;
}

// 具体实现
struct InMemoryUserRepository;

impl UserRepository for InMemoryUserRepository {
    fn get_user(&self, id: u32) -> String {
        format!("User {}", id)
    }
}

// Service 依赖注入
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

// 使用
let repo = InMemoryUserRepository;
let service = UserService::new(repo);
println!("{}", service.greet_user(42));
```

完整示例：[crates/awesome/src/services/concrete_injection_sample.rs](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/services/concrete_injection_sample.rs)

---

## 原理解析

### Rust 依赖注入的三种模式

```
Rust 依赖注入
├── 模式 1: 具体类型注入 (Concrete Injection)
│   ├── 使用泛型参数
│   ├── 编译时确定类型
│   └── 零运行时开销
├── 模式 2: Trait 对象注入 (Dynamic Injection)
│   ├── 使用 Arc<dyn Trait> 或 Box<dyn Trait>
│   ├── 运行时确定类型
│   └── 轻微动态分发开销
└── 模式 3: 服务容器 (Service Container)
    ├── 基于 TypeId 的注册/解析
    ├── 支持工厂模式
    └── 类似 Spring/IoC 容器
```

### 模式 1: 具体类型注入

**原理**：使用泛型参数，在编译时确定具体类型。

```rust,ignore
// 编译时确定类型，零运行时开销
struct UserService<R: UserRepository> {
    repo: R,  // 具体类型，非 trait 对象
}
```

**优点**：
- 零运行时开销（单态化）
- 编译时类型安全
- 内联优化

**缺点**：
- 每种类型组合生成独立代码
- 无法在运行时切换实现

**适用场景**：性能关键路径、简单依赖关系

### 模式 2: Arc 动态注入

**原理**：使用 `Arc<dyn Trait>` 包装 trait 对象，支持运行时切换。

```rust,ignore
struct BusinessService {
    logger: Arc<dyn LoggerService>,      // 动态分发
    database: Arc<dyn DatabaseService>,  // 动态分发
}
```

**优点**：
- 运行时可切换实现
- 共享所有权（Arc）
- 线程安全（Send + Sync）

**缺点**：
- 虚表查找开销
- 引用计数开销

**适用场景**：需要运行时灵活性、多实现切换

### 模式 3: Box 动态注入

**原理**：与 Arc 类似，但使用 `Box` 独占所有权。

```rust,ignore
struct ServiceContainer {
    services: Mutex<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
}
```

**Arc vs Box 选择**：
- **Arc**: 多服务共享同一实例
- **Box**: 每个服务独占实例

### 模式 4: 服务容器 (Service Container)

**原理**：基于 `TypeId` 的注册/解析系统，类似 Spring/IoC 容器。

```rust,ignore
struct ServiceContainer {
    services: Mutex<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
    factories: Mutex<HashMap<TypeId, Box<dyn Fn(&ServiceContainer) -> Arc<dyn Any + Send + Sync>>>>,
}

impl ServiceContainer {
    // 注册具体服务
    fn register<T: Any + Send + Sync + 'static>(&self, service: T) {
        let type_id = TypeId::of::<T>();
        self.services.lock().unwrap()
            .insert(type_id, Arc::new(service));
    }

    // 注册工厂（延迟初始化）
    fn register_factory<T, F>(&self, factory: F)
    where
        T: Service + 'static,
        F: Fn(&ServiceContainer) -> Arc<T> + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<T>();
        let wrapped_factory = Box::new(move |container| factory(container));
        self.factories.lock().unwrap().insert(type_id, wrapped_factory);
    }

    // 解析服务
    fn resolve<T: Service + 'static>(&self) -> Option<Arc<T>> {
        let type_id = TypeId::of::<T>();
        // 先查缓存，再查工厂
        ...
    }
}
```

**完整使用示例**：

```rust,ignore
let container = Arc::new(ServiceContainer::new());

// 1. 注册具体服务
container.register(ConsoleLogger);
container.register(InMemoryDatabase);

// 2. 注册 trait 对象
container.register::<Arc<dyn LoggerService>>(
    Arc::new(ConsoleLogger) as Arc<dyn LoggerService>
);

// 3. 注册工厂（自动解析依赖）
container.register_factory::<BusinessService, _>(|container| {
    let logger = container.resolve::<ConsoleLogger>()
        .expect("Logger not found");
    let database = container.resolve::<InMemoryDatabase>()
        .expect("Database not found");
    Arc::new(BusinessService::new(logger, database))
});

// 4. 解析并使用
let business = container.resolve::<BusinessService>()
    .expect("BusinessService not found");
business.perform_task("Process data");
```

完整示例：
- [Arc 版本](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/services/dynmaic_injection_arc_sample.rs)
- [Box 版本](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/services/dynmaic_injection_box_sample.rs)
- [服务容器](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/services/service_container_sample.rs)

### 工厂模式与延迟初始化

```rust,ignore
// 工厂在首次 resolve 时执行，之后缓存结果
container.register_factory::<BusinessService, _>(|container| {
    // 自动解析依赖
    let logger = container.resolve::<ConsoleLogger>().unwrap();
    let database = container.resolve::<InMemoryDatabase>().unwrap();
    Arc::new(BusinessService::new(logger, database))
});
```

**优势**：
- 按需创建（非启动时全部创建）
- 自动解析依赖链
- 缓存结果（单次初始化）

---

## Rust DI 生态对比

| 方案 | 类型 | 特点 | 适用场景 |
|------|------|------|---------|
| **手写容器** | 运行时 | 灵活，无额外依赖 | 中大型项目 |
| **[shaku]** | 运行时 | 类型安全，编译时检查 | 需要严格类型 |
| **[inject]** | 编译时 | 宏驱动，零运行时开销 | 追求性能 |
| **[poem/inject]** | 运行时 | Web 框架集成 | Web 应用 |

**推荐**：对于大多数项目，手写 Service Container 已足够。Rust 社区更倾向于**显式依赖传递**而非重型 DI 框架。

---

## 常见错误

### 错误 1: 在异步上下文中使用 std::sync::Mutex

```rust,ignore
// ❌ 错误：阻塞异步运行时
struct ServiceContainer {
    services: std::sync::Mutex<HashMap<TypeId, Arc<dyn Any>>>,
}

// ✅ 正确：使用 tokio::sync::Mutex
struct ServiceContainer {
    services: tokio::sync::Mutex<HashMap<TypeId, Arc<dyn Any>>>,
}
```

### 错误 2: 循环依赖

```rust,ignore
// A 依赖 B，B 依赖 A → 死锁
// 解决：引入事件总线或消息队列解耦
```

### 错误 3: Trait 对象类型不匹配

```rust,ignore
// ❌ 错误：TypeId 不匹配
container.register::<Arc<dyn LoggerService>>(Arc::new(ConsoleLogger));
// 注册和解析必须使用相同的 TypeId

// ✅ 正确：确保注册和解析类型一致
container.register_trait::<dyn LoggerService>(Arc::new(ConsoleLogger) as Arc<dyn LoggerService>);
let logger = container.resolve_trait::<dyn LoggerService>();
```

---

## 知识检查

**问题 1**: Rust 中依赖注入的三种主要模式是什么？

**问题 2**: Arc 和 Box 在 DI 中有什么区别？

**问题 3**: 工厂模式的优势是什么？

<details>
<summary>点击查看答案与解析</summary>

1. 具体类型注入（泛型）、Trait 对象注入（Arc/Box）、服务容器（TypeId）
2. Arc 支持共享所有权（多服务共享），Box 独占所有权（单服务使用）
3. 延迟初始化、自动解析依赖链、缓存结果

**关键理解**: Rust 的 DI 更注重显式和类型安全，而非魔法。
</details>

---

## 延伸阅读

学习完依赖注入后，你可能还想了解：

- [服务生命周期管理](services.md) - ApplicationFramework 模式
- [插件系统](plugin.md) - 编译时插件注册
- [shaku crate](https://docs.rs/shaku) - 类型安全的 DI 框架
- [inject crate](https://docs.rs/inject) - 编译时 DI

**选择建议**:
- 简单项目 → 具体类型注入（泛型）
- 中型项目 → Arc 动态注入
- 大型项目 → 服务容器 + 工厂模式

---

## 小结

核心要点：具体注入零开销、Arc 支持共享、Service Container 最灵活、工厂模式延迟初始化

完整示例：[crates/awesome/src/services/](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/services/)
