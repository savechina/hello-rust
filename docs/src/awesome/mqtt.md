# MQTT 消息协议

## 开篇故事

想象你在设计一个智能家居系统。温度传感器需要每分钟上报数据，门锁需要接收开关命令，摄像头需要在有人移动时发送警报。如果每个设备都直接连接服务器，服务器会被大量连接压垮。MQTT 协议就像是一个智能邮局——设备只需把消息发送到邮局（Broker），邮局负责将消息分发给需要的订阅者。

MQTT（Message Queuing Telemetry Transport）是轻量级的发布/订阅消息协议，特别适合物联网场景——从智能家居到工业传感器，MQTT 都能稳定可靠地传递消息。

---

## 本章适合谁

如果你已经理解 Rust 异步编程基础，现在想学习：
- MQTT 协议的发布/订阅模式
- 如何使用 rumqttc 客户端库连接 MQTT Broker
- 实现物联网设备间的消息通信

本章适合你。

---

## 你会学到什么

完成本章后，你可以：

1. 理解 MQTT 的核心概念（Broker、Topic、QoS）
2. 使用 rumqttc 创建同步和异步 MQTT 客户端
3. 实现消息的发布(Publish)和订阅(Subscribe)
4. 理解 QoS 等级及其对消息可靠性的影响
5. 处理连接保持和重连逻辑
6. 编写基于 MQTT 的物联网通信程序

---

## 前置要求

学习本章前，你需要理解：

- [异步编程](../advance/async/async.md) - async/await 基础
- [Tokio](../advance/async/tokio.md) - 使用 Tokio 异步运行时
- 安装 MQTT Broker（推荐 Mosquitto 或 EMQX）用于测试

**安装 MQTT Broker**：

```bash
# macOS
brew install mosquitto
brew services start mosquitto

# 或使用 Docker
docker run -d -p 1883:1883 eclipse-mosquitto
```

---

### 依赖安装

运行以下命令安装所需依赖：

```bash
cargo add tokio --features full
cargo add rumqttc
cargo add serde --features derive
cargo add serde_json
```

---

## 第一个例子

让我们看一个最简单的 MQTT 同步客户端示例：

```rust
use rumqttc::{Client, MqttOptions, QoS};
use std::time::Duration;

fn main() {
    // 配置 MQTT 连接选项
    let mut mqttoptions = MqttOptions::new("client-001", "127.0.0.1", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    // 创建客户端和连接
    let (mut client, mut connection) = Client::new(mqttoptions, 20);

    // 订阅主题
    client.subscribe("hello/rumqtt", QoS::AtMostOnce).unwrap();
    println!("已订阅主题 'hello/rumqtt'");

    // 在独立线程中发布消息
    std::thread::spawn(move || {
        for i in 0..10 {
            client.publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize]).unwrap();
            println!("发送消息 {}", i);
            std::thread::sleep(Duration::from_millis(500));
        }
    });

    // 接收消息
    for notification in connection.iter() {
        println!("收到通知: {:?}", notification);
    }
}
```

**发生了什么？**

1. `MqttOptions` 配置 MQTT Broker 地址和客户端 ID
2. `Client::new()` 创建同步客户端，返回 `(client, connection)` 元组
3. `subscribe()` 订阅指定主题，准备接收消息
4. `publish()` 向主题发送消息，所有订阅者都会收到
5. `connection.iter()` 阻塞等待并处理入站消息

完整示例：[crates/awesome/src/mq/rumqtt_sample.rs](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/mq/rumqtt_sample.rs)

---

## 原理解析

### MQTT 架构概览

```
┌─────────────────────────────────────────────────────────────────────┐
│                        MQTT 发布/订阅架构                             │
│                                                                      │
│   ┌──────────────┐         ┌──────────────┐         ┌──────────────┐│
│   │  Publisher   │         │              │         │  Subscriber  ││
│   │  (发布者)     │────────→│ MQTT Broker  │←────────│  (订阅者)     ││
│   │              │ publish │  (消息代理)   │ forward │              ││
│   └──────────────┘         └──────┬───────┘         └──────────────┘│
│                                   │                                  │
│   ┌──────────────┐         ┌──────┴───────┐         ┌──────────────┐│
│   │  Publisher   │         │              │         │  Subscriber  ││
│   │  (传感器)     │────────→│              │←────────│  (控制器)     ││
│   └──────────────┘         └──────────────┘         └──────────────┘│
│                                                                      │
│   主题层级:                                                            │
│   home/                                                               │
│   ├── bedroom/                                                        │
│   │   ├── temperature  ← 传感器发布到这里                               │
│   │   └── humidity                                                    │
│   └── livingroom/                                                     │
│       ├── light          ← 控制器订阅这里                               │
│       └── temperature                                                 │
│                                                                      │
│   通配符: home/+/temperature  匹配所有房间温度                          │
│          home/bedroom/#      匹配卧室所有子主题                         │
└─────────────────────────────────────────────────────────────────────┘
```

### 消息生命周期（QoS 1 流程）

```
Publisher                           Broker                          Subscriber
   │                                   │                               │
   │  1. CONNECT                       │                               │
   │ ────────────────────────────────→ │                               │
   │                                   │  2. CONNACK                   │
   │ ←──────────────────────────────── │                               │
   │                                   │                               │
   │  3. SUBSCRIBE (由订阅者发送)       │                               │
   │ ────────────────────────────────→ │                               │
   │                                   │  4. SUBACK                    │
   │ ←──────────────────────────────── │                               │
   │                                   │                               │
   │  5. PUBLISH (QoS 1)               │                               │
   │ ────────────────────────────────→ │                               │
   │                                   │  6. 匹配主题                   │
   │                                   │ ────────────────────────────→ │
   │                                   │                               │
   │                                   │  7. PUBLISH (转发)             │
   │                                   │ ←──────────────────────────── │
   │                                   │                               │
   │                                   │  8. PUBACK                    │
   │                                   │ ────────────────────────────→ │
   │  9. PUBACK                        │                               │
   │ ←──────────────────────────────── │                               │
   │                                   │                               │
```

### 核心概念

**1. MQTT Broker（消息代理）**

MQTT Broker 是消息的中转站，负责接收发布者的消息并转发给订阅者：

```rust
// 连接到本地 MQTT Broker
let mut mqttoptions = MqttOptions::new("client-id", "127.0.0.1", 1883);

// 或连接到公共测试 Broker
let mut mqttoptions = MqttOptions::new("client-id", "test.mosquitto.org", 1883);

mqttoptions.set_keep_alive(Duration::from_secs(5));
```

**2. Topic（主题）**

主题是消息的地址，使用层级结构：

```rust
// 简单主题
let topic = "sensors/temperature";

// 多层主题
let topic = "home/bedroom/temperature";

// 使用通配符订阅
client.subscribe("sensors/+/temperature", QoS::AtMostOnce).unwrap(); // + 匹配一级
client.subscribe("home/#", QoS::AtMostOnce).unwrap(); // # 匹配多级
```

**3. QoS（服务质量等级）**

| QoS 等级 | 名称 | 说明 | 应用场景 |
|---------|------|------|---------|
| 0 | AtMostOnce | 最多一次，不确认 | 高频数据，丢失可接受 |
| 1 | AtLeastOnce | 至少一次，需确认 | 关键数据，允许重复 |
| 2 | ExactlyOnce | 恰好一次，四次握手 | 关键命令，不可重复 |

```rust
use rumqttc::QoS;

// QoS 0: 发送即忘
client.publish("sensor/data", QoS::AtMostOnce, false, payload).unwrap();

// QoS 1: 确保送达
client.publish("device/command", QoS::AtLeastOnce, false, payload).unwrap();

// QoS 2: 确保仅送达一次（开销最大）
client.publish("payment/confirm", QoS::ExactlyOnce, false, payload).unwrap();
```

### 异步客户端架构

```
┌──────────────────────────────────────────────────────────────────┐
│                        Tokio Runtime                              │
│                                                                   │
│  ┌──────────────┐        ┌──────────────┐                        │
│  │  Publisher   │        │  EventLoop   │                        │
│  │   Task       │        │   Task       │                        │
│  │              │        │              │                        │
│  │  client.     │───────→│ eventloop.   │←───────┐               │
│  │  publish()   │  send  │ poll().await │        │               │
│  │              │        │              │        │               │
│  └──────────────┘        └──────┬───────┘        │               │
│                                 │                │               │
│                                 │  TCP/TLS       │               │
│                                 ▼                │               │
│                        ┌──────────────┐          │               │
│                        │ MQTT Broker  │──────────┘               │
│                        │  127.0.0.1   │  publish msg             │
│                        └──────────────┘                          │
└──────────────────────────────────────────────────────────────────┘
```

### 异步客户端完整示例

```rust
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("async-client", "127.0.0.1", 1883);
    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 15);

    // 异步订阅
    client.subscribe("hello/rumqtt", QoS::AtMostOnce).await.unwrap();

    // 异步发布
    tokio::spawn(async move {
        for i in 0..10 {
            client.publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i]).await.unwrap();
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    });

    // 异步处理事件
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(p))) => {
                println!("收到消息: {:?}", p.payload);
            }
            Err(e) => {
                eprintln!("错误: {:?}", e);
                break;
            }
            _ => {}
        }
    }
}
```

### MQTT 连接状态机

```
                  ┌─────────────┐
                  │  Disconnected│
                  └──────┬──────┘
                         │ CONNECT
                         ▼
                  ┌─────────────┐
            ┌─────│  Connecting  │─────┐
            │     └──────┬──────┘     │
            │            │ CONNACK    │
            │            ▼            │
            │     ┌─────────────┐     │
            │     │  Connected   │     │
            │     └──────┬──────┘     │
            │            │            │
            │     ┌──────┴──────┐     │
            │     │             │     │
            ▼     ▼             │     ▼
      ┌──────────┐      ┌──────────┐  ┌──────────┐
      │Publishing│      │Subscribing│  │Pinging   │
      └────┬─────┘      └────┬─────┘  └────┬─────┘
           │                 │              │
           └─────────────────┴──────────────┘
                         │
                         │ DISCONNECT / Error
                         ▼
                  ┌─────────────┐
                  │  Disconnected│
                  └─────────────┘
```

---

## 常见错误

### 错误 1: 忘记处理连接保持

```rust
// ❌ 错误：没有设置 keep_alive
let mqttoptions = MqttOptions::new("client", "broker", 1883);

// ✅ 正确：设置 keep_alive 保持连接
let mut mqttoptions = MqttOptions::new("client", "broker", 1883);
mqttoptions.set_keep_alive(Duration::from_secs(5));
```

**问题**：没有 keep_alive，Broker 会在空闲后断开连接。

### 错误 2: 同步客户端在异步上下文中使用

```rust
// ❌ 错误：在 async 函数中使用同步 Client
async fn bad_example() {
    let (client, _) = Client::new(mqttoptions, 10); // 阻塞！
    client.subscribe("topic", QoS::AtMostOnce).unwrap(); // 阻塞！
}

// ✅ 正确：使用 AsyncClient
async fn good_example() {
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("topic", QoS::AtMostOnce).await.unwrap();
}
```

### 错误 3: 没有处理连接断开和重连

```rust
// ❌ 错误：连接断开时直接退出
loop {
    let notification = connection.iter().next().unwrap(); // 断开时 panic
}

// ✅ 正确：优雅处理连接错误
loop {
    match eventloop.poll().await {
        Ok(event) => handle_event(event),
        Err(e) => {
            eprintln!("连接错误: {:?}, 尝试重连...", e);
            tokio::time::sleep(Duration::from_secs(5)).await;
            // 重连逻辑
        }
    }
}
```

### 错误 4: 主题名称包含非法字符

```rust
// ❌ 错误：主题包含空格和特殊字符
let topic = "my topic with spaces";

// ✅ 正确：使用合法字符
let topic = "my/topic/with/hierarchy";

// 合法字符：字母、数字、下划线、正斜杠
// 正斜杠用于层级分隔
```

---

## 动手练习

### 练习 1: 修复订阅示例

下面的代码有什么问题？

```rust
fn main() {
    let mqttoptions = MqttOptions::new("client", "127.0.0.1", 1883);
    let (client, mut connection) = Client::new(mqttoptions, 10);
    
    // 订阅主题
    client.subscribe("test/topic", QoS::AtMostOnce).unwrap();
    
    // 发送消息
    client.publish("test/topic", QoS::AtLeastOnce, false, vec![1, 2, 3]).unwrap();
    
    // 接收消息
    while let Some(notification) = connection.iter().next() {
        println!("{:?}", notification);
    }
}
```

<details>
<summary>点击查看答案与解析</summary>

**问题**：
1. 没有设置 `keep_alive`，连接可能被 Broker 断开
2. `connection.iter()` 返回 `Result`，需要正确处理
3. 发送和接收在同一线程，没有并发处理

**修复方案**：

```rust
use rumqttc::{Client, MqttOptions, QoS, Packet, Event};
use std::time::Duration;
use std::thread;

fn main() {
    let mut mqttoptions = MqttOptions::new("client", "127.0.0.1", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5)); // ✅ 保持连接
    
    let (mut client, mut connection) = Client::new(mqttoptions, 10);
    
    // 订阅主题
    client.subscribe("test/topic", QoS::AtMostOnce).unwrap();
    
    // 在独立线程发送消息 ✅
    thread::spawn(move || {
        for i in 0..5 {
            client.publish("test/topic", QoS::AtLeastOnce, false, vec![i]).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    // 接收消息
    for notification in connection.iter() {
        match notification {
            Ok(Event::Incoming(Packet::Publish(p))) => {
                println!("收到消息: {:?}", p.payload);
            }
            Err(e) => eprintln!("错误: {:?}", e), // ✅ 错误处理
            _ => {}
        }
    }
}
```

</details>

### 练习 2: 实现温度监控器

补全下面的代码，实现一个简单的温度监控器：

```rust
#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("temp-monitor", "127.0.0.1", 1883);
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    
    // TODO: 订阅温度主题 "home/+/temperature"
    // TODO: 当温度超过 30 度时打印警告
    
    loop {
        match eventloop.poll().await {
            // TODO: 处理收到的消息
            _ => {}
        }
    }
}
```

<details>
<summary>点击查看答案</summary>

```rust
use rumqttc::{AsyncClient, MqttOptions, QoS, Packet, Event};

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("temp-monitor", "127.0.0.1", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    
    // 订阅所有房间的温度
    client.subscribe("home/+/temperature", QoS::AtMostOnce).await.unwrap();
    println!("已订阅温度主题");
    
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(p))) => {
                // 解析温度值
                if let Ok(temp_str) = String::from_utf8(p.payload.to_vec()) {
                    if let Ok(temp) = temp_str.parse::<f32>() {
                        println!("[{}] 温度: {:.1}°C", p.topic, temp);
                        
                        // 超过 30 度报警
                        if temp > 30.0 {
                            println!("⚠️ 警告：{} 温度过高！", p.topic);
                        }
                    }
                }
            }
            Ok(Event::Incoming(Packet::ConnAck(_))) => {
                println!("已连接到 Broker");
            }
            Err(e) => {
                eprintln!("连接错误: {:?}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
            _ => {}
        }
    }
}
```

</details>

### 练习 3: 理解 QoS 等级

预测以下代码中消息的传输可靠性（假设网络不稳定）：

```rust
// 传感器 A：QoS 0
client.publish("sensor/a", QoS::AtMostOnce, false, vec![1]).await.unwrap();

// 传感器 B：QoS 1  
client.publish("sensor/b", QoS::AtLeastOnce, false, vec![2]).await.unwrap();

// 传感器 C：QoS 2
client.publish("sensor/c", QoS::ExactlyOnce, false, vec![3]).await.unwrap();
```

如果网络突然中断，哪些消息可能丢失？

<details>
<summary>点击查看解析</summary>

**结果**：

| 传感器 | QoS | 可能丢失？ | 原因 |
|-------|-----|-----------|------|
| A | 0 | ✅ 可能丢失 | 发送即忘，无确认 |
| B | 1 | ❌ 不会丢失 | 需要 ACK，会重试 |
| C | 2 | ❌ 不会丢失 | 四次握手确保送达 |

**关键点**：
- QoS 0：性能最好，但不可靠
- QoS 1：可靠性+性能平衡，可能重复
- QoS 2：最高可靠性，但开销最大

**适用场景**：
- QoS 0：高频传感器数据（每秒上报）
- QoS 1：设备控制命令（开关灯）
- QoS 2：支付、关键配置更新

</details>

---

## 实际应用

### 应用场景 1: 智能家居控制中心

```rust
use rumqttc::{AsyncClient, MqttOptions, QoS, Packet, Event};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("home-controller", "127.0.0.1", 1883);
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    
    // 存储设备状态
    let devices: Arc<RwLock<HashMap<String, String>>> = Arc::new(RwLock::new(HashMap::new()));
    
    // 订阅所有设备主题
    client.subscribe("home/+/status", QoS::AtMostOnce).await.unwrap();
    client.subscribe("home/+/command", QoS::AtLeastOnce).await.unwrap();
    
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(p))) => {
                let topic = p.topic;
                let payload = String::from_utf8_lossy(&p.payload);
                
                if topic.contains("/status") {
                    // 更新设备状态
                    let mut dev = devices.write().await;
                    dev.insert(topic.clone(), payload.to_string());
                    println!("设备 [{}] 状态: {}", topic, payload);
                }
            }
            Err(e) => {
                eprintln!("MQTT 错误: {:?}", e);
            }
            _ => {}
        }
    }
}
```

### 应用场景 2: 传感器数据采集

```rust
use rumqttc::{AsyncClient, MqttOptions, QoS};
use tokio::time::{interval, Duration};
use rand::Rng;

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("sensor-sim", "127.0.0.1", 1883);
    let (client, _) = AsyncClient::new(mqttoptions, 10);
    
    let mut ticker = interval(Duration::from_secs(5));
    let mut rng = rand::thread_rng();
    
    loop {
        ticker.tick().await;
        
        // 模拟传感器数据
        let temperature = 20.0 + rng.gen::<f32>() * 10.0;
        let humidity = 40.0 + rng.gen::<f32>() * 30.0;
        
        // 发布温度（QoS 0，高频数据）
        client.publish(
            "factory/sensor1/temperature",
            QoS::AtMostOnce,
            false,
            format!("{:.2}", temperature).into_bytes()
        ).await.unwrap();
        
        // 发布湿度（QoS 0）
        client.publish(
            "factory/sensor1/humidity",
            QoS::AtMostOnce,
            false,
            format!("{:.2}", humidity).into_bytes()
        ).await.unwrap();
        
        println!("已发布: T={:.1}°C, H={:.1}%", temperature, humidity);
    }
}
```

### 应用场景 3: 带遗嘱消息的客户端

```rust
use rumqttc::{AsyncClient, LastWill, MqttOptions, QoS};

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("device-001", "127.0.0.1", 1883);
    
    // 设置遗嘱消息：意外断开时自动发布
    let will = LastWill::new(
        "device/001/status",
        b"offline",
        QoS::AtLeastOnce,
        true // retained 消息
    );
    mqttoptions.set_last_will(will);
    
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    
    // 上线时发送状态
    client.publish(
        "device/001/status",
        QoS::AtLeastOnce,
        true, // retained
        b"online"
    ).await.unwrap();
    
    println!("设备已上线");
    
    // 保持连接
    loop {
        match eventloop.poll().await {
            Err(e) => {
                eprintln!("连接断开: {:?}", e);
                break;
            }
            _ => {}
        }
    }
}
```

---

## 故障排查 (FAQ)

### Q: 连接失败 "Connection refused" 怎么办？

**A**: 检查以下几点：

1. **Broker 是否运行**：
   ```bash
   # 测试 Broker 连通性
   mosquitto_pub -t test -m "hello" -h 127.0.0.1
   ```

2. **端口是否正确**：
   - MQTT 默认端口：1883
   - MQTT over TLS：8883
   - WebSocket：8083

3. **防火墙设置**：
   ```bash
   # 开放端口（Linux）
   sudo ufw allow 1883
   ```

### Q: 什么时候用同步客户端，什么时候用异步客户端？

**A**:

| 场景 | 推荐客户端 | 原因 |
|------|-----------|------|
| 简单脚本、命令行工具 | `Client` | 简单直接 |
| 需要与 Tokio 集成 | `AsyncClient` | 兼容性 |
| 高性能服务器 | `AsyncClient` | 非阻塞，高吞吐 |
| 嵌入式/资源受限 | `Client` | 更少的依赖 |

### Q: 如何处理消息积压？

**A**: 使用背压控制和合理配置：

```rust
// 增大接收缓冲区
let (client, eventloop) = AsyncClient::new(mqttoptions, 100); // 缓冲区 100

// 使用多消费者处理
for _ in 0..4 {
    let rx = rx.clone();
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            process_message(msg).await;
        }
    });
}
```

### Q: MQTT vs Kafka vs RabbitMQ 如何选择？

**A**:

| 特性 | MQTT | Kafka | RabbitMQ |
|------|------|-------|----------|
| 协议复杂度 | 简单 | 中等 | 复杂 |
| 适用场景 | IoT、移动端 | 大数据流 | 企业消息 |
| 消息持久化 | 可选 | 强制 | 可选 |
| QoS 支持 | 内置 | 需配置 | 需配置 |
| 资源占用 | 极低 | 高 | 中等 |
| 典型部署 | 边缘设备 | 数据中心 | 企业服务 |

---

## 知识扩展 (选学)

### MQTT 5.0 新特性

MQTT 5.0 相比 3.1.1 增加了许多功能：

```rust
// 消息过期时间
let properties = PublishProperties {
    message_expiry_interval: Some(60), // 60秒后过期
    ..Default::default()
};

// 用户属性（元数据）
let user_properties = vec![
    ("device-type".to_string(), "sensor".to_string()),
    ("version".to_string(), "1.0".to_string()),
];
```

### TLS 加密连接

```rust
use rumqttc::{MqttOptions, Transport};

let mut mqttoptions = MqttOptions::new("secure-client", "broker.example.com", 8883);

// 配置 TLS
mqttoptions.set_transport(Transport::tls_with_config(
    rumqttc::TlsConfiguration::Native
));
```

---

## 小结

**核心要点**：

1. **MQTT** 是轻量级的发布/订阅协议，适合 IoT
2. **Topic** 使用层级结构组织消息，`+` 和 `#` 是通配符
3. **QoS** 控制消息可靠性：0(最快) → 2(最可靠)
4. **同步客户端** 使用 `Client`，**异步客户端** 使用 `AsyncClient`
5. **遗嘱消息** 在意外断开时自动通知其他客户端

**关键术语**：

| English | 中文 | 说明 |
|---------|------|------|
| MQTT | 消息队列遥测传输 | 轻量级消息协议 |
| Broker | 消息代理 | 消息中转服务器 |
| Topic | 主题 | 消息的地址标识 |
| Publish | 发布 | 发送消息到主题 |
| Subscribe | 订阅 | 注册接收主题消息 |
| QoS | 服务质量 | 消息可靠性等级 |
| Payload | 消息载荷 | 实际传输的数据 |
| Retained | 保留消息 | 存储在 Broker 的最后一条消息 |
| LastWill | 遗嘱消息 | 意外断开时自动发送的消息 |

**下一步**：

- 学习 [消息队列总览](mq.md) - 了解 Rust 生态各种 MQ 方案
- 了解 [服务框架](services.md) - 基于 MQTT 的微服务架构
- 探索 Tokio - 异步运行时基础

---

## 术语表

| English | 中文 |
|---------|------|
| MQTT | 消息队列遥测传输 |
| Broker | 消息代理 |
| Topic | 主题 |
| Publish | 发布 |
| Subscribe | 订阅 |
| QoS | 服务质量 |
| Payload | 载荷/消息体 |
| Retained Message | 保留消息 |
| Last Will | 遗嘱消息 |
| Keep Alive | 保持连接 |
| Clean Session | 清除会话 |
| Persistent Session | 持久会话 |

完整示例：[crates/awesome/src/mq/rumqtt_sample.rs](https://github.com/savechina/hello-rust/blob/main/crates/awesome/src/mq/rumqtt_sample.rs)

---

## 继续学习

- 上一步：[消息队列总览](mq.md) - Rust 生态各种 MQ 方案对比
- 下一步：[服务框架](services.md) - 生产级服务架构
- 相关：Tokio 异步运行时 - rumqttc 的基础
- 实战：尝试连接公共 MQTT Broker 如 `test.mosquitto.org`

> 💡 **记住**：MQTT 的核心是"发布/订阅解耦"——生产者无需知道消费者是谁，只需关注消息主题。这种松耦合架构让系统更具弹性和可扩展性！
