# 项目实战：CLI 待办事项管理器

**难度**: 🟡 中级  
**代码量**: ~170 行  
**涉及知识点**: clap 参数解析、serde 序列化、anyhow 错误处理、文件系统操作

---

## 项目目标

构建一个支持增删改查的 CLI 待办事项工具，数据持久化到 JSON 文件。

---

## 技术栈

| Crate              | 用途                 |
| ------------------ | -------------------- |
| `clap` (derive)      | CLI 参数解析         |
| `serde` + `serde_json` | JSON 数据持久化      |
| `anyhow`             | 错误处理（带上下文） |
| `chrono`             | 时间戳               |
| `dirs`               | 获取用户主目录       |

---

## 项目结构

```
examples/todo/
├── Cargo.toml
├── .gitignore
├── src/
│   └── main.rs        # 主程序
└── tests/
    ├── common.rs      # 测试工具类
    └── todo_test.rs   # 集成测试 (8 个测试)
```

---

## 核心设计

### 1. CLI 参数解析 (clap)

使用 `clap` 的 derive API 定义命令结构：

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo
    Add { description: String },
    /// List all todos
    List,
    /// Mark a todo as done
    Done { id: usize },
    /// Delete a todo
    Delete { id: usize },
}
```

**关键知识点**:
- `#[derive(Parser)]`: 自动生成参数解析代码
- `#[command(subcommand)]`: 定义子命令
- `/// 注释`: 自动生成 `--help` 文本

### 2. 数据模型 (serde)

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: usize,
    description: String,
    done: bool,
    created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoStore {
    todos: Vec<Todo>,
    next_id: usize,
}
```

**关键知识点**:
- `#[derive(Serialize, Deserialize)]`: 自动生成序列化代码
- `next_id`: 自增 ID，避免删除后 ID 冲突

### 3. 数据持久化

```rust
impl TodoStore {
    fn load(path: &PathBuf) -> Result<Self> {
        if path.exists() {
            let data = fs::read_to_string(path)
                .with_context(|| format!("Failed to read {}", path.display()))?;
            let store: TodoStore = serde_json::from_str(&data)
                .with_context(|| "Failed to parse todo data")?;
            Ok(store)
        } else {
            Ok(TodoStore::new())
        }
    }

    fn save(&self, path: &PathBuf) -> Result<()> {
        let data = serde_json::to_string_pretty(self)
            .with_context(|| "Failed to serialize todo data")?;
        fs::write(path, data)
            .with_context(|| format!("Failed to write {}", path.display()))?;
        Ok(())
    }
}
```

**关键知识点**:
- `with_context()`: 添加错误上下文信息
- `to_string_pretty()`: 格式化 JSON 输出

### 4. 错误处理 (anyhow)

```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let data_path = get_data_path();
    let mut store = TodoStore::load(&data_path)?;

    match cli.command {
        Commands::Done { id } => {
            store.mark_done(id)
                .with_context(|| format!("Failed to mark todo {} as done", id))?;
            // ...
        }
        // ...
    }

    Ok(())
}
```

---

## 使用示例

```bash
# 添加待办
cargo run -- add "Learn Rust basics"
cargo run -- add "Build a CLI app"
cargo run -- add "Read Rust Book"

# 列出所有待办
cargo run -- list

# 输出:
# ID    Done  Description                    Created At
# ---------------------------------------------------------------------------
# 1     ⬜     Learn Rust basics              2026-04-05 15:00:12
# 2     ⬜     Build a CLI app                2026-04-05 15:00:13
# 3     ⬜     Read Rust Book                 2026-04-05 15:00:13

# 标记完成
cargo run -- done 1

# 删除待办
cargo run -- delete 2

# 查看帮助
cargo run -- --help
```

---

## 测试

项目包含 8 个集成测试，覆盖所有核心功能：

```bash
cd examples/todo
cargo test
```

**测试覆盖**:
- ✅ 添加待办
- ✅ 列出待办
- ✅ 标记完成
- ✅ 删除待办
- ✅ 错误处理（不存在的 ID）
- ✅ 空列表处理
- ✅ 数据持久化

---

## CLI 集成测试

本项目使用 **进程级集成测试** 模式，通过 `std::process::Command` 启动真实的 CLI 进程进行测试，确保端到端功能正确。

### 测试架构

```
tests/
├── common.rs      # 测试工具类 (TodoTest, TodoOutput)
└── todo_test.rs   # 8 个集成测试用例
```

### 核心组件

#### 1. TodoTest 测试工具类 (`common.rs`)

```rust
pub struct TodoTest {
    pub temp_dir: TempDir,       // 临时目录隔离测试
    pub cwd: PathBuf,
    pub env: HashMap<String, String>,
}

impl TodoTest {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let home_path = temp_dir.path().to_str().unwrap().to_string();
        
        let mut test = Self {
            temp_dir,
            cwd: temp_dir.path().into(),
            env: HashMap::new(),
        };
        
        // 重定向 HOME 到临时目录，隔离测试数据
        test.env.insert("HOME".into(), home_path);
        test
    }

    pub fn todo(&self, args: &[&str]) -> TodoOutput {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_todo"));
        cmd.current_dir(&self.cwd);
        cmd.env_clear().envs(&self.env);  // 清理环境变量
        cmd.args(args);
        
        let output = cmd.output().expect("Failed to execute");
        TodoOutput::new(output)
    }
}
```

**关键设计**:
- **`TempDir`**: 每个测试使用独立临时目录，测试结束后自动清理
- **`env_clear()`**: 清除环境变量，避免测试间相互影响
- **`HOME` 重定向**: 将数据存储路径指向临时目录

#### 2. TodoOutput 断言工具 (`common.rs`)

```rust
pub struct TodoOutput {
    pub output: std::process::Output,
}

impl TodoOutput {
    #[track_caller]
    pub fn assert_success(&self) -> &Self {
        assert!(self.output.status.success(),
            "Expected success, got:\n# STDERR\n{}\n# STDOUT\n{}",
            String::from_utf8_lossy(&self.output.stderr),
            String::from_utf8_lossy(&self.output.stdout)
        );
        self
    }

    #[track_caller]
    pub fn assert_stdout_contains(&self, expected: &str) -> &Self {
        let stdout = self.stdout();
        assert!(stdout.contains(expected),
            "Expected stdout to contain '{}', got:\n{}",
            expected, stdout
        );
        self
    }
}
```

**关键设计**:
- **`#[track_caller]`**: 断言失败时显示测试代码行号，而非断言工具行号
- **链式调用**: 支持 `.assert_success().assert_stdout_contains("...")` 链式断言

### 测试用例示例

```rust
#[test]
fn test_mark_todo_done() {
    let test = TodoTest::new();
    
    // 添加待办
    test.todo(&["add", "Learn Rust"]).assert_success();
    
    // 标记完成
    test.todo(&["done", "1"])
        .assert_success()
        .assert_stdout_contains("Marked todo #1 as done");
    
    // 验证状态
    test.todo(&["list"])
        .assert_stdout_contains("✅");
}

#[test]
fn test_multiple_todos_persistence() {
    let test = TodoTest::new();
    
    // 添加多个待办
    test.todo(&["add", "Task 1"]).assert_success();
    test.todo(&["add", "Task 2"]).assert_success();
    test.todo(&["done", "1"]).assert_success();
    
    // 验证数据持久化
    test.todo(&["list"])
        .assert_stdout_contains("Task 1")
        .assert_stdout_contains("Task 2")
        .assert_stdout_contains("✅");
}
```

### 运行测试

```bash
cd examples/todo
cargo test
```

**测试输出**:
```
running 8 tests
test test_add_todo ... ok
test test_list_empty ... ok
test test_delete_nonexistent_todo ... ok
test test_done_nonexistent_todo ... ok
test test_delete_todo ... ok
test test_mark_todo_done ... ok
test test_list_todos ... ok
test test_multiple_todos_persistence ... ok

test result: ok. 8 passed; 0 failed
```

### 测试覆盖矩阵

| 测试用例                        | 覆盖功能         | 断言点     |
| ------------------------------- | ---------------- | ---------- |
| `test_add_todo`                   | 添加待办         | 成功消息   |
| `test_list_todos`                 | 列出待办         | 内容显示   |
| `test_mark_todo_done`             | 标记完成         | 状态变更   |
| `test_delete_todo`                | 删除待办         | 删除确认   |
| `test_done_nonexistent_todo`      | 错误处理         | 失败状态   |
| `test_delete_nonexistent_todo`    | 错误处理         | 失败状态   |
| `test_list_empty`                 | 边界条件         | 空列表提示 |
| `test_multiple_todos_persistence` | 数据持久化       | 状态保持   |

---

## 相关章节

- [CLI 开发](../../advance/system/cli.md) - CLI 开发最佳实践
- [JSON 序列化](../../advance/data/json.md) - JSON 序列化/反序列化
- [错误处理](../../advance/error-handling/error-handling.md) - 错误处理最佳实践
- [测试](../../advance/testing/test.md) - 集成测试

---

## 扩展挑战

- [ ] 添加优先级和截止日期
- [ ] 支持分类标签
- [ ] 添加单元测试覆盖率达到 80%+
- [ ] 支持编辑待办描述
- [ ] 添加过滤功能（按状态、标签）
