# 消息队列与 MQTT

## 开篇故事

想象你正在经营一家快递公司的调度中心。每天有成千上万的包裹需要配送，如果每个快递员都要直接联系仓库查询取货，电话线会被打爆，仓库也忙不过来。这时候你引入了"消息中心"——快递员只需要把取货请求发送到消息中心，仓库按顺序处理，处理完再通过消息中心通知快递员取货。

这就是**消息队列（Message Queue）**的核心思想：**解耦生产者与消费者，实现异步通信**。而 **MQTT（Message Queuing Telemetry Transport）** 是物联网时代最流行的消息协议之一，它轻量、高效，特别适合网络带宽有限的场景。

在 Rust 生态中，**rumqttc** 是一个全功能的 MQTT 客户端库，支持 MQTT 3.1/5 协议规范。本章将带你掌握如何使用 rumqttc 构建可靠的 MQTT 消息应用。

---

## 本章适合谁

如果你已经掌握了 Rust 异步编程基础，现在想要：

- 理解消息队列的基本概念和 MQTT 协议
- 学习如何在 Rust 中使用 MQTT 进行消息收发
- 掌握同步和异步两种 MQTT 客户端模式
- 构建物联网、实时通信或事件驱动的应用

本章适合你。MQTT 是物联网（IoT）、移动应用和实时消息系统的首选协议，掌握它将为你打开新的应用领域。

---

## 你会学到什么

完成本章后，你可以：

1. 解释 MQTT 的核心概念（Broker、Topic、QoS、发布/订阅模式）
2. 使用 rumqttc 创建同步 MQTT 客户端
3. 使用 rumqttc 创建异步 MQTT 客户端（配合 Tokio）
4. 实现消息的发布和订阅功能
5. 配置 QoS 级别保证消息可靠性
6. 处理 MQTT 连接、断线和重连逻辑

---

## 前置要求

学习本章前，你需要理解：

- [异步编程](../advance/async.md) - `async/await` 语法和 Tokio 运行时
- [错误处理](../basic/enums.md) - `Result` 类型的使用
- [线程](../basic/threads.md) - 了解并发基本概念
- 一个可用的 MQTT Broker（如 Mosquitto、EMQX 或测试用的公共 Broker）

**安装 MQTT Broker（可选，用于本地测试）**：

```bash
# macOS
brew install mosquitto
brew services start mosquitto

# 或使用 Docker
docker run -d -p 1883:1883 eclipse-mosquitto
```

---

## 第一个例子

让我们从一个简单的 MQTT 同步客户端开始：

```rust
use rumqttc::{Client, MqttOptions, QoS};
use std::time::Duration;
use std::thread;

fn main() {
    // 配置 MQTT 连接选项
    let mut mqttoptions = MqttOptions::new("rumqtt-sync", "127.0.0.1", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    // 创建同步客户端和连接
    let (mut client, mut connection) = Client::new(mqttoptions, 20);

    // 订阅主题
    client.subscribe("hello/rumqtt", QoS::AtMostOnce).unwrap();
    println!("Subscribed to topic 'hello/rumqtt'");

    // 在后台线程发布消息
    thread::spawn(move || {
        for i in 0..10 {
            client
                .publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize])
                .unwrap();
            println!("Published message {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    // 主线程接收消息
    for (i, notification) in connection.iter().enumerate() {
        println!("Notification {} = {:?}", i, notification);
    }
}
```

完整示例：[crates/awesome/src/mq/rumqtt_sample.rs](../../../crates/awesome/src/mq/rumqtt_sample.rs)

**发生了什么？**

1. `MqttOptions` - 配置客户端 ID、Broker 地址和端口
2. `Client::new()` - 创建同步客户端，返回 `(client, connection)` 元组
3. `subscribe()` - 订阅指定主题，接收该主题的消息
4. `publish()` - 向指定主题发布消息
5. `connection.iter()` - 阻塞迭代，接收 MQTT 事件（连接确认、消息到达等）

---

## 原理解析

### MQTT 架构概览

```
┌─────────────────────────────────────────────────────────────────┐
│                        MQTT 系统架构                              │
│                                                                 │
│   ┌──────────┐    发布消息    ┌──────────┐    推送消息    ┌──────────┐
│   │          │ ─────────────►│          │ ─────────────►│          │
│   │ Publisher│               │  Broker  │               │ Subscriber│
│   │  (发布者) │               │  (代理)   │               │  (订阅者) │
│   └──────────┘               └────┬─────┘               └──────────┘
│        │                          │                             ▲
│        │                          │    ┌──────────┐             │
│        │                          └───►│          │─────────────┘
│        │                               │ Subscriber│
│        │                               └──────────┘
│        │
│   发送到主题: sensors/temperature
│   QoS: 消息可靠性级别
│
└─────────────────────────────────────────────────────────────────┘
```

**图 1-1**: MQTT 发布/订阅模型

### 核心概念

**1. Broker（代理）**

Broker 是 MQTT 系统的核心，负责接收所有消息并转发给对应的订阅者：

```rust
// 连接到本地 Broker
let mqttoptions = MqttOptions::new("client-id", "127.0.0.1", 1883);

// 或连接到公共测试 Broker
let mqttoptions = MqttOptions::new("client-id", "test.mosquitto.org", 1883);
```

**2. Topic（主题）**

主题是消息的分类标签，使用 `/` 分隔的层级结构：

```
sensors/temperature/living_room
sensors/humidity/bedroom
devices/lamp/001/status
home/living_room/lights
```

```rust
// 订阅单个主题
client.subscribe("sensors/temperature", QoS::AtMostOnce)?;

// 通配符订阅
// + 匹配单一层级
client.subscribe("sensors/+/temperature", QoS::AtMostOnce)?;

// # 匹配多层剩余部分
client.subscribe("home/living_room/#", QoS::AtMostOnce)?;
```

**3. QoS（服务质量级别）**

**表 1-1**: MQTT QoS 级别对比

| QoS 级别 | 名称 | 描述 | 使用场景 |
|---------|------|------|---------|
| 0 | AtMostOnce（至多一次） | 消息可能丢失，不重试 | 高频 telemetry，可容忍丢失 |
| 1 | AtLeastOnce（至少一次） | 消息确保送达，可能重复 | 关键命令，需确保收到 |
| 2 | ExactlyOnce（恰好一次） | 消息确保送达且不重复 | 支付、关键状态更新 |

```rust
use rumqttc::QoS;

// QoS 0 - 最多一次（最快，可能丢失）
client.publish("sensors/temp", QoS::AtMostOnce, false, payload)?;

// QoS 1 - 至少一次（确保送达，可能重复）
client.publish("commands/lock", QoS::AtLeastOnce, false, payload)?;

// QoS 2 - 恰好一次（最安全，性能开销大）
client.publish("payments/process", QoS::ExactlyOnce, false, payload)?;
```

### rumqttc 同步 vs 异步模式

**同步模式**：

```rust
use rumqttc::{Client, MqttOptions};

fn sync_example() {
    let mqttoptions = MqttOptions::new("sync-client", "127.0.0.1", 1883);
    let (mut client, mut connection) = Client::new(mqttoptions, 10);
    
    // 阻塞迭代接收消息
    for notification in connection.iter() {
        match notification {
            Ok(msg) => println!("Received: {:?}", msg),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
```

**异步模式**（推荐用于生产环境）：

```rust
use rumqttc::{AsyncClient, MqttOptions, Event, Packet};

#[tokio::main]
async fn async_example() {
    let mqttoptions = MqttOptions::new("async-client", "127.0.0.1", 1883);
    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    
    // 异步等待事件
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(p))) => {
                println!("Received on {}: {:?}", p.topic, p.payload);
            }
            Ok(Event::Incoming(Packet::ConnAck(_))) => {
                println!("Connected to broker");
            }
            Err(e) => {
                eprintln!("Connection error: {:?}", e);
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
            _ => {}
        }
    }
}
```

### 异步客户端架构

```rust
use rumqttc::{AsyncClient, MqttOptions, QoS};
use tokio::{task, time};

#[tokio::main]
async fn mqtt_async_sample() {
    let mut mqttoptions = MqttOptions::new("rumqtt-async", "127.0.0.1", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    // 创建异步客户端
    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 15);
    
    // 订阅主题
    client.subscribe("hello/rumqtt", QoS::AtMostOnce).await.unwrap();

    // 在独立任务中发布消息
    task::spawn(async move {
        for i in 0..10 {
            client.publish(
                "hello/rumqtt", 
                QoS::AtLeastOnce, 
                false, 
                vec![i; i as usize]
            ).await.ok();
            time::sleep(Duration::from_secs(3)).await;
        }
    });

    // 主循环处理事件
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(p))) => {
                println!("Topic: {}, Payload: {:?}", p.topic, p.payload);
            }
            Ok(Event::Incoming(Packet::ConnAck(_))) => {
                println!("Connected!");
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                time::sleep(Duration::from_secs(1)).await;
            }
            _ => {}
        }
    }
}
```

**图 1-2**: 异步 MQTT 客户端架构

```
┌──────────────────────────────────────────────────────────────┐
│                    Tokio Runtime                              │
│                                                               │
│  ┌──────────────┐        ┌──────────────┐                   │
│  │  Publisher   │        │  EventLoop   │                   │
│  │   Task       │        │   Task       │                   │
│  │              │        │              │                   │
│  │  client.     │───────►│ eventloop.   │◄───────┐          │
│  │  publish()   │  send  │ poll().await │        │          │
│  │              │        │              │        │          │
│  └──────────────┘        └──────┬───────┘        │          │
│                                 │                │          │
│                                 │  TCP/TLS       │          │
│                                 ▼                │          │
│                        ┌──────────────┐          │          │
│                        │ MQTT Broker  │──────────┘          │
│                        │  127.0.0.1   │  publish msg        │
│                        └──────────────┘                     │
│                                                               │
└──────────────────────────────────────────────────────────────┘
```

---

## 常见错误

### 错误 1: 忘记处理连接事件

```rust
// ❌ 错误：直接开始发布，未等待连接建立
#[tokio::main]
async fn bad_example() {
    let (client, mut eventloop) = AsyncClient::new(options, 10);
    
    // 可能还没连上就发送，导致消息丢失
    client.publish("test", QoS::AtLeastOnce, false, "hello").await.ok();
}

// ✅ 正确：等待连接确认后再发布
#[tokio::main]
async fn good_example() {
    let (client, mut eventloop) = AsyncClient::new(options, 10);
    
    // 等待连接确认
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::ConnAck(_))) => break,
            Err(e) => panic!("Connection failed: {:?}", e),
            _ => {}
        }
    }
    
    // 现在安全了
    client.publish("test", QoS::AtLeastOnce, false, "hello").await.ok();
}
```

---

### 错误 2: QoS 级别选择不当

```rust
// ❌ 错误：对 telemetry 数据使用 QoS 2（不必要的开销）
for reading in sensors {
    // QoS 2 需要 4 次握手，每秒 1000 次读取会导致 Broker 过载
    client.publish("sensors/data", QoS::ExactlyOnce, false, reading).await?;
}

// ✅ 正确：根据场景选择 QoS
// 高频 telemetry - QoS 0（最多一次）
client.publish("sensors/temperature", QoS::AtMostOnce, false, temp).await?;

// 重要命令 - QoS 1（至少一次）
client.publish("commands/door_lock", QoS::AtLeastOnce, false, "lock").await?;

// 关键交易 - QoS 2（恰好一次）
client.publish("payments/transaction", QoS::ExactlyOnce, false, tx_data).await?;
```

---

### 错误 3: 阻塞事件循环

```rust
// ❌ 错误：在事件循环中执行阻塞操作
loop {
    match eventloop.poll().await {
        Ok(Event::Incoming(Packet::Publish(p))) => {
            // 阻塞操作！会卡住整个 MQTT 事件处理
            std::thread::sleep(Duration::from_secs(10));
            process_message(p);
        }
        _ => {}
    }
}

// ✅ 正确：将阻塞操作放到单独任务
loop {
    match eventloop.poll().await {
        Ok(Event::Incoming(Packet::Publish(p))) => {
            //  spawned 到独立任务，不阻塞事件循环
            tokio::spawn(async move {
                process_message(p).await;
            });
        }
        _ => {}
    }
}
```

---

### 错误 4: 订阅后未等待确认

```rust
// ❌ 错误：订阅后立即发布，可能订阅还未生效
client.subscribe("commands/#", QoS::AtLeastOnce).await?;
client.publish("commands/test", QoS::AtLeastOnce, false, "test").await?;
// 可能收不到自己发布的消息！

// ✅ 正确：处理订阅确认
client.subscribe("commands/#", QoS::AtLeastOnce).await?;

// 等待 SubAck
loop {
    if let Ok(Event::Incoming(Packet::SubAck(_))) = eventloop.poll().await {
        break;
    }
}

// 现在安全发布
client.publish("commands/test", QoS::AtLeastOnce, false, "test").await?;
```

---

## 动手练习

### 练习 1: 修复消息丢失问题

下面的代码有什么问题？如何修复？

```rust
#[tokio::main]
async fn main() {
    let options = MqttOptions::new("client1", "127.0.0.1", 1883);
    let (client, mut eventloop) = AsyncClient::new(options, 10);
    
    // 订阅主题
    client.subscribe("test/topic", QoS::AtLeastOnce).await.unwrap();
    
    // 立即发布消息
    client.publish("test/topic", QoS::AtLeastOnce, false, "hello").await.unwrap();
    
    // 接收消息
    loop {
        if let Ok(notification) = eventloop.poll().await {
            println!("{:?}", notification);
        }
    }
}
```

<details>
<summary>点击查看答案与解析</summary>

**问题**：
1. 订阅和发布都立即执行，但订阅可能还未生效
2. 没有等待连接确认（ConnAck）
3. 发布的消息可能在订阅生效前到达 Broker，导致自己收不到

**修复方案**：
```rust
#[tokio::main]
async fn main() {
    let options = MqttOptions::new("client1", "127.0.0.1", 1883);
    let (client, mut eventloop) = AsyncClient::new(options, 10);
    
    // 等待连接确认
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::ConnAck(_))) => {
                println!("Connected!");
                break;
            }
            Err(e) => panic!("Connection failed: {:?}", e),
            _ => {}
        }
    }
    
    // 订阅主题
    client.subscribe("test/topic", QoS::AtLeastOnce).await.unwrap();
    
    // 等待订阅确认
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::SubAck(_))) => {
                println!("Subscribed!");
                break;
            }
            _ => {}
        }
    }
    
    // 现在安全发布
    client.publish("test/topic", QoS::AtLeastOnce, false, "hello").await.unwrap();
    
    // 继续接收消息
    loop {
        if let Ok(notification) = eventloop.poll().await {
            println!("{:?}", notification);
        }
    }
}
```

</details>

---

### 练习 2: 实现温度监控传感器

完成代码，实现一个简单的温度传感器模拟器：

```rust
use rumqttc::{AsyncClient, MqttOptions, QoS};
use tokio::time::{sleep, Duration};
use rand::random;

#[tokio::main]
async fn main() {
    let options = MqttOptions::new("temp-sensor-01", "127.0.0.1", 1883);
    let (client, mut eventloop) = AsyncClient::new(options, 10);
    
    // TODO: 等待连接确认
    
    // TODO: 每 5 秒发布一次温度读数到 sensors/temperature/living_room
    // 温度范围 20-30 度，格式为 JSON: {"temperature": 25.5, "unit": "C"}
    
    // 在后台运行事件循环
    tokio::spawn(async move {
        loop {
            let _ = eventloop.poll().await;
        }
    });
    
    // TODO: 保持主程序运行
}
```

<details>
<summary>点击查看答案</summary>

```rust
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use tokio::time::{sleep, Duration};
use rand::random;

#[tokio::main]
async fn main() {
    let options = MqttOptions::new("temp-sensor-01", "127.0.0.1", 1883);
    let (client, mut eventloop) = AsyncClient::new(options, 10);
    
    // 等待连接确认
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::ConnAck(_))) => {
                println!("Connected to broker");
                break;
            }
            Err(e) => {
                eprintln!("Connection error: {:?}", e);
                sleep(Duration::from_secs(5)).await;
            }
            _ => {}
        }
    }
    
    // 克隆客户端用于温度发布任务
    let client_clone = client.clone();
    
    // 温度发布任务
    tokio::spawn(async move {
        loop {
            let temp = 20.0 + random::<f32>() * 10.0;
            let payload = format!(
                "{{\"temperature\": {:.1}, \"unit\": \"C\"}}",
                temp
            );
            
            if let Err(e) = client_clone.publish(
                "sensors/temperature/living_room",
                QoS::AtMostOnce,  // telemetry 数据，允许丢失
                false,
                payload
            ).await {
                eprintln!("Publish error: {:?}", e);
            } else {
                println!("Published temperature: {:.1}°C", temp);
            }
            
            sleep(Duration::from_secs(5)).await;
        }
    });
    
    // 事件循环任务（处理 ping 等内部消息）
    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(Event::Incoming(Packet::Publish(p))) => {
                    println!("Received: {:?}", p);
                }
                Err(e) => {
                    eprintln!("Event loop error: {:?}", e);
                    sleep(Duration::from_secs(5)).await;
                }
                _ => {}
            }
        }
    });
    
    // 保持主程序运行
    sleep(Duration::from_secs(3600)).await;
}
```

</details>

---

### 练习 3: 理解 QoS 行为

假设你正在设计一个智能家居系统，请为以下场景选择合适的 QoS 级别：

1. 每 30 秒上报一次的室温数据
2. 门锁控制命令（开锁/关锁）
3. 火灾报警信号
4. 灯泡亮度调节（连续滑动调节）

<details>
<summary>点击查看建议</summary>

**答案**：

1. **室温数据** → `QoS::AtMostOnce`
   - 高频上报，偶尔丢失一帧无影响
   - 下一帧很快就到

2. **门锁控制** → `QoS::AtLeastOnce`
   - 必须确保命令送达
   - 即使重复执行一次锁门也无害

3. **火灾报警** → `QoS::ExactlyOnce`
   - 必须送达且不能重复（避免重复报警）
   - 关键安全事件

4. **灯泡亮度** → `QoS::AtMostOnce`
   - 连续调节，最新值最重要
   - 中间值丢失无影响，只需最后一条到达

</details>

---

## 实际应用

### 应用场景 1: 物联网传感器数据采集

```rust
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use serde_json::json;
use tokio::time::{sleep, Duration};

struct SensorNode {
    client: AsyncClient,
    sensor_id: String,
}

impl SensorNode {
    async fn new(broker: &str, sensor_id: String) -> Result<Self, Box<dyn std::error::Error>> {
        let options = MqttOptions::new(&sensor_id, broker, 1883);
        let (client, mut eventloop) = AsyncClient::new(options, 10);
        
        // 等待连接
        loop {
            match eventloop.poll().await {
                Ok(Event::Incoming(Packet::ConnAck(_))) => break,
                Err(e) => return Err(e.into()),
                _ => {}
            }
        }
        
        // 后台事件循环
        tokio::spawn(async move {
            loop {
                let _ = eventloop.poll().await;
            }
        });
        
        Ok(Self { client, sensor_id })
    }
    
    async fn publish_reading(&self, temperature: f32, humidity: f32) -> Result<(), rumqttc::ClientError> {
        let payload = json!({
            "sensor_id": self.sensor_id,
            "temperature": temperature,
            "humidity": humidity,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });
        
        self.client.publish(
            &format!("sensors/{}/readings", self.sensor_id),
            QoS::AtMostOnce,
            false,
            payload.to_string()
        ).await
    }
}
```

---

### 应用场景 2: 命令下发与状态反馈

```rust
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

type DeviceStates = Arc<RwLock<HashMap<String, String>>>;

async fn device_controller(broker: &str) {
    let options = MqttOptions::new("controller", broker, 1883);
    let (client, mut eventloop) = AsyncClient::new(options, 10);
    let states: DeviceStates = Arc::new(RwLock::new(HashMap::new()));
    
    // 订阅命令响应和状态更新
    client.subscribe("devices/+/status", QoS::AtLeastOnce).await.unwrap();
    client.subscribe("devices/+/response", QoS::AtLeastOnce).await.unwrap();
    
    let states_clone = states.clone();
    
    // 事件处理任务
    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(Event::Incoming(Packet::Publish(p))) => {
                    let topic = p.topic;
                    let payload = String::from_utf8_lossy(&p.payload);
                    
                    // 解析主题: devices/{id}/status
                    let parts: Vec<&str> = topic.split('/').collect();
                    if parts.len() >= 3 {
                        let device_id = parts[1].to_string();
                        let mut states = states_clone.write().await;
                        states.insert(device_id, payload.to_string());
                        println!("Device {} status updated: {}", parts[1], payload);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    sleep(Duration::from_secs(5)).await;
                }
                _ => {}
            }
        }
    });
    
    // 主循环：下发命令
    loop {
        // 向设备 001 发送重启命令
        client.publish(
            "devices/001/commands",
            QoS::AtLeastOnce,
            false,
            r#"{"command": "reboot"}"#
        ).await.ok();
        
        sleep(Duration::from_secs(60)).await;
    }
}
```

---

### 应用场景 3: 带遗嘱消息的可靠设备管理

```rust
use rumqttc::{AsyncClient, LastWill, MqttOptions, QoS};

async fn robust_device_client(device_id: &str) {
    let mut options = MqttOptions::new(device_id, "127.0.0.1", 1883);
    
    // 设置遗嘱消息：当设备异常断开时，Broker 自动发布此消息
    let will = LastWill::new(
        &format!("devices/{}/status", device_id),
        r#"{"status": "offline", "reason": "unexpected_disconnect"}"#,
        QoS::AtLeastOnce,
        false
    );
    options.set_last_will(will);
    
    // 设置自动重连
    options.set_keep_alive(Duration::from_secs(5));
    
    let (client, mut eventloop) = AsyncClient::new(options, 10);
    
    // 连接成功后发布在线状态
    client.publish(
        &format!("devices/{}/status", device_id),
        QoS::AtLeastOnce,
        true,  // retain = true，保留消息
        r#"{"status": "online"}"#
    ).await.unwrap();
    
    // 事件处理循环...
}
```

---

## 故障排查 (FAQ)

### Q: 连接失败，报错 "Connection refused"？

**A**: 检查以下几点：

```bash
# 1. 确认 Broker 是否运行
netstat -an | grep 1883

# 2. 检查防火墙
sudo ufw allow 1883/tcp

# 3. 确认地址和端口正确
let options = MqttOptions::new("client", "127.0.0.1", 1883);  // 本地
let options = MqttOptions::new("client", "broker.hivemq.com", 1883);  // 公共
```

---

### Q: 消息发布后订阅者收不到？

**A**: 常见原因：

1. **订阅在发布之后**：MQTT 不保证订阅前的消息送达
2. **主题不匹配**：检查主题名称是否完全一致（区分大小写）
3. **QoS 不匹配**：发布 QoS 和订阅 QoS 取较小值
4. **未等待 SubAck**：订阅后立即发布，订阅可能未生效

```rust
// 调试技巧：打印所有事件
loop {
    match eventloop.poll().await {
        Ok(event) => println!("Event: {:?}", event),
        Err(e) => println!("Error: {:?}", e),
    }
}
```

---

### Q: 如何处理断线重连？

**A**: rumqttc 支持自动重连，但建议自定义重连逻辑：

```rust
async fn connect_with_retry(broker: &str, client_id: &str) -> (AsyncClient, EventLoop) {
    let mut retry_count = 0;
    let max_retries = 10;
    
    loop {
        let options = MqttOptions::new(client_id, broker, 1883);
        let (client, eventloop) = AsyncClient::new(options, 10);
        
        // 尝试连接（通过 poll 触发）
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // 检查是否连接成功
        if client.publish("test", QoS::AtMostOnce, false, "").await.is_ok() {
            println!("Connected successfully");
            return (client, eventloop);
        }
        
        retry_count += 1;
        if retry_count > max_retries {
            panic!("Failed to connect after {} attempts", max_retries);
        }
        
        let delay = Duration::from_secs(2u64.pow(retry_count.min(6)));
        println!("Connection failed, retrying in {:?}...", delay);
        tokio::time::sleep(delay).await;
    }
}
```

---

### Q: 订阅通配符主题不生效？

**A**: 检查 Broker 配置：

```rust
// ✅ 正确的通配符用法
client.subscribe("sensors/+/temperature", QoS::AtLeastOnce).await?;
// 匹配: sensors/living_room/temperature, sensors/bedroom/temperature

client.subscribe("home/#", QoS::AtLeastOnce).await?;
// 匹配: home/living_room/light, home/garage/door/status

// ❌ 错误：+ 和 # 不能在同一层级混用
client.subscribe("sensors+/#/data", QoS::AtLeastOnce).await?;  // 无效！
```

---

## 知识扩展 (选学)

### TLS/SSL 加密连接

```rust
use rumqttc::{MqttOptions, Transport, TlsConfiguration};

fn setup_tls() -> MqttOptions {
    let mut options = MqttOptions::new("secure-client", "mqtt.example.com", 8883);
    
    let tls_config = TlsConfiguration::Simple {
        ca: vec![],  // CA 证书
        alpn: None,
        client_auth: None,
    };
    
    options.set_transport(Transport::tls_with_config(tls_config.into()));
    options
}
```

---

### 与 Web 框架集成

```rust
use axum::{extract::State, routing::post, Json, Router};
use rumqttc::AsyncClient;
use std::sync::Arc;
use tokio::sync::Mutex;

type MqttClient = Arc<Mutex<AsyncClient>>;

async fn publish_handler(
    State(client): State<MqttClient>,
    Json(payload): Json<serde_json::Value>,
) -> Result<String, String> {
    let client = client.lock().await;
    
    client.publish(
        "api/commands",
        QoS::AtLeastOnce,
        false,
        payload.to_string()
    ).await.map_err(|e| e.to_string())?;
    
    Ok("Published".to_string())
}

#[tokio::main]
async fn main() {
    let options = MqttOptions::new("api-bridge", "127.0.0.1", 1883);
    let (client, _) = AsyncClient::new(options, 10);
    let client = Arc::new(Mutex::new(client));
    
    let app = Router::new()
        .route("/publish", post(publish_handler))
        .with_state(client);
    
    axum::serve(listener, app).await.unwrap();
}
```

---

### MQTT 5 新特性

```rust
use rumqttc::{AsyncClient, MqttOptions, QoS, mqttbytes::v5::Packet};

// MQTT 5 支持用户属性、消息过期、内容类型等
async fn mqtt5_example() {
    let options = MqttOptions::new("client", "127.0.0.1", 1883);
    let (client, mut eventloop) = AsyncClient::new(options, 10);
    
    // MQTT 5 特性：消息属性
    let props = rumqttc::mqttbytes::v5::PublishProperties {
        message_expiry_interval: Some(60),  // 消息 60 秒后过期
        content_type: Some("application/json".to_string()),
        user_properties: vec![
            ("version".to_string(), "1.0".to_string()),
        ],
        ..Default::default()
    };
    
    // 注意：rumqttc 目前主要支持 MQTT 3.1.1
    // 完整 MQTT 5 支持请参考 rumqttc 最新文档
}
```

---

## 小结

**核心要点**：

1. **MQTT 是发布/订阅模式**：解耦生产者和消费者
2. **主题使用层级结构**：使用 `/` 分隔，`+` 匹配单层，`#` 匹配多层
3. **QoS 决定可靠性**：0（最多一次）、1（至少一次）、2（恰好一次）
4. **同步 vs 异步**：同步适合简单脚本，异步适合生产环境
5. **连接管理重要**：处理 ConnAck、SubAck、断线重连

**关键术语**：

- **Broker**：MQTT 代理，消息中转中心
- **Topic**：主题，消息的分类标签
- **QoS**：服务质量级别，消息可靠性保证
- **Publish**：发布消息到主题
- **Subscribe**：订阅主题接收消息
- **LastWill**：遗嘱消息，异常断开时自动发布
- **Retain**：保留消息，新订阅者立即收到

**下一步**：

- 学习 [微服务架构](./services.md) - 将 MQTT 集成到服务架构
- 探索 [异步编程](../advance/async.md) - 深入理解 Tokio 运行时
- 实践 [物联网项目](../../projects) - 构建完整的 IoT 应用

---

## 术语表

| English | 中文 | 说明 |
|---------|------|------|
| Broker | 代理 | MQTT 消息中转服务器 |
| Topic | 主题 | 消息的分类标识符 |
| QoS | 服务质量 | 消息可靠性级别 |
| Publish | 发布 | 发送消息到主题 |
| Subscribe | 订阅 | 注册接收某主题的消息 |
| Last Will | 遗嘱 | 异常断开时自动发送的消息 |
| Retain | 保留 | 让 Broker 保留最后一条消息 |
| Payload | 负载 | 消息的实际内容 |
| Keep Alive | 保活 | 维持连接的定时心跳 |

---

## 继续学习

- 下一步：[微服务架构](./services.md)
- 进阶：[数据库高级应用](./database.md)
- 回顾：[异步编程](../advance/async.md)

---

## 知识检查点

### 检查点 1 （基础概念）

MQTT 中的 QoS 1（AtLeastOnce）表示什么？

A) 消息最多送达一次，可能丢失  
B) 消息至少送达一次，可能重复  
C) 消息恰好送达一次，不会重复  
D) 消息保证在 1 秒内送达

<details>
<summary>答案与解析</summary>

**答案**: B) 消息至少送达一次，可能重复

**解析**:
- QoS 0（AtMostOnce）：最多一次，可能丢失
- QoS 1（AtLeastOnce）：至少一次，确保送达但可能重复
- QoS 2（ExactlyOnce）：恰好一次，确保送达且不重复

QoS 1 通过 PUBLISH → PUBACK 的两次握手实现可靠性。

</details>

---

### 检查点 2 （实践应用）

以下代码存在什么问题？

```rust
#[tokio::main]
async fn main() {
    let options = MqttOptions::new("client", "127.0.0.1", 1883);
    let (client, mut eventloop) = AsyncClient::new(options, 10);
    
    // 立即订阅并发布
    client.subscribe("test", QoS::AtLeastOnce).await.unwrap();
    client.publish("test", QoS::AtLeastOnce, false, "hello").await.unwrap();
    
    // 接收消息
    loop {
        if let Ok(Event::Incoming(Packet::Publish(p))) = eventloop.poll().await {
            println!("Received: {:?}", p.payload);
        }
    }
}
```

A) 编译错误  
B) 订阅失败  
C) 可能收不到自己发布的消息  
D) 会导致内存泄漏

<details>
<summary>答案与解析</summary>

**答案**: C) 可能收不到自己发布的消息

**解析**:
代码没有等待连接确认（ConnAck）就直接订阅和发布。如果网络有延迟，订阅可能还未生效，发布的消息会"错过"。

**正确做法**：
```rust
// 等待连接确认
loop {
    match eventloop.poll().await {
        Ok(Event::Incoming(Packet::ConnAck(_))) => break,
        _ => {}
    }
}

// 再执行订阅和发布
```

</details>

---

### 检查点 3 （综合理解）

在一个智能家居系统中，以下场景应该如何选择 QoS？

1. 每 5 秒上报的温度传感器数据
2. 门锁控制命令（开锁/关锁）
3. 紧急火灾报警

<details>
<summary>答案与解析</summary>

**推荐选择**：

1. **温度传感器** → QoS 0（AtMostOnce）
   - 高频上报，偶尔丢失一帧无影响
   - 追求最低延迟和开销

2. **门锁控制** → QoS 1（AtLeastOnce）
   - 必须确保命令送达
   - 重复执行一次锁门操作无害

3. **火灾报警** → QoS 2（ExactlyOnce）
   - 必须送达且不能重复
   - 关键安全事件，值得性能开销

**选择原则**：根据业务重要性、频率和幂等性选择 QoS 级别。

</details>

---

## 参考资料

1. [MQTT 官方协议规范](https://mqtt.org/mqtt-specification/)
2. [rumqttc 文档](https://docs.rs/rumqttc/)
3. [MQTT Essentials](https://www.hivemq.com/mqtt-essentials/)
4. [Eclipse Mosquitto](https://mosquitto.org/)

---

> **完整示例**: [crates/awesome/src/mq/rumqtt_sample.rs](../../../crates/awesome/src/mq/rumqtt_sample.rs)

> **记住**：MQTT 的设计哲学是简单、轻量、可靠。在 IoT 场景下，合理选择 QoS 级别、设计良好的主题层次结构，是构建高效消息系统的关键。

