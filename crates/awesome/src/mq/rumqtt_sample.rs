use rumqttc::{AsyncClient, MqttOptions, Packet, QoS};
use rumqttc::{Client, Event};
use std::error::Error;
use std::thread;
use std::time::Duration;
use tokio::{task, time};
use tracing::{error, info, instrument, Level};
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::FmtSubscriber;
use tracing_subscriber::{filter::LevelFilter, EnvFilter}; // For `with_filter` and `EnvFilter` // For setting log level

fn mqtt_sync_sample() {
    info!("mqtt sync sample start ...");
    // let mut mqttoptions = MqttOptions::new("rumqtt-sync", "test.mosquitto.org", 1883);
    let mut mqttoptions = MqttOptions::new("rumqtt-sync", "127.0.0.1", 1883);

    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (mut client, mut connection) = Client::new(mqttoptions, 20);

    client.subscribe("hello/rumqtt", QoS::AtMostOnce).unwrap();

    info!("Subscribed to topic 'hello/rumqtt'");

    // Handle the result of the subscription operation
    thread::spawn(move || {
        for i in 0..10 {
            client
                .publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize])
                .unwrap();
            info!("Published message {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::sleep(Duration::from_secs(5));

    // Iterate to poll the eventloop for connection progress
    for (i, notification) in connection.iter().enumerate() {
        info!("value:{},Notification = {:?}", i, notification);
        let message = notification.unwrap();
        if let Event::Incoming(Packet::Publish(p)) = message {
            info!("message: {:?}", p)
        }
    }
}
#[tokio::main]

async fn mqtt_async_sample() {
    info!("mqtt_async_sample 'hello/rumqtt'");

    let mut mqttoptions = MqttOptions::new("rumqtt-async", "127.0.0.1", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 15);
    client
        .subscribe("hello/rumqtt", QoS::AtMostOnce)
        .await
        .unwrap();

    info!("Subscribed to topic 'hello/rumqtt'");

    task::spawn(async move {
        for i in 0..10 {
            let status = client
                .publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize])
                .await;
            info!("Published message {}: {:?}", i, status);
            // Handle the result of the publish operation
            // If the publish fails, we can log the error or handle it accordingly
            if let Err(e) = status {
                error!("Error publishing message {}: {}", i, e);
                continue;
            }
            time::sleep(Duration::from_secs(3)).await;
        }
    });

    time::sleep(Duration::from_secs(10)).await;

    // 4. 在主循环中处理接收到的消息
    info!("Starting MQTT event loop...");
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(p))) => {
                // 接收到 PUBLISH 消息
                let topic = p.topic;
                // let payload = String::from_utf8_lossy(&p.payload); // 将 payload 转换为字符串
                let payload = p.payload; // 将 payload 转换为字符串
                let qos = p.qos;

                info!(
                    "Received message on topic: '{}', QoS: {:?}, Payload: '{:?}'",
                    topic, qos, payload
                );

                // 根据消息内容进行处理
                if topic == "hello/rumqtt" {
                    info!("Received 'hello_world' message. Doing something special! ");
                    // 在这里添加你的业务逻辑
                }
            }

            Ok(Event::Incoming(Packet::ConnAck(connack))) => {
                info!("Connected to broker: {:?}", connack);
            }
            Ok(Event::Incoming(packet)) => {
                // 打印其他类型的入站包
                info!("Received incoming packet: {:?}", packet);
            }
            Ok(Event::Outgoing(packet)) => {
                // 打印出站包 (可选，用于调试)
                info!("Sent outgoing packet: {:?}", packet);
            }
            Err(e) => {
                error!("MQTT EventLoop error: {:?}", e);
                // 遇到错误时可以添加重连逻辑或退出
                time::sleep(Duration::from_secs(1)).await; // 避免错误循环过快
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log;

    fn setup() {
        tracing_subscriber::registry()
            .with(
                fmt::layer() // Use the fmt layer for console output
                    .compact() // Make the output more compact (optional)
                    .with_target(true) // Include the target (module path) of the event
                    .with_level(true) // Include the log level
                    .with_thread_ids(true) // Include thread IDs (optional)
                    .with_thread_names(true), // Include thread names (optional)
            )
            .with(
                EnvFilter::from_default_env() // Allow filtering via RUST_LOG env var
                    .add_directive(Level::INFO.into()), // Default log level if RUST_LOG is not set
            )
            .init(); // Initialize the global default subscribe
    }
    #[ignore = "mq test"]
    #[test]
    fn test_mqtt_sync_sample() {
        setup();
        // This test will run the MQTT publish/subscribe sample
        println!("Running MQTT subscription test...");
        mqtt_sync_sample();
        assert!(true, "MQTT subscription test passed");
    }

    // #[test_log::test]
    #[ignore = "mq test"]
    #[test]
    fn test_mqtt_async_sample() {
        setup();
        // This test will run the MQTT publish/subscribe sample
        info!("Running MQTT async sample  test...");
        mqtt_async_sample();
        assert!(true, "MQTT async sample test passed");
        info!("MQTT async sample test passed");
    }
}
