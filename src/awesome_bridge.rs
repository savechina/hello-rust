//! Awesome crate sample topics registration
//!
//! This module registers sample topics from the `awesome` workspace crate
//! for the hello CLI system using inventory-based compile-time registration.

use crate::cli::registry::{SampleType, Topic};
use awesome::mq;
use awesome::services;

inventory::submit! {
    Topic::new(
        "inventory",
        "编译时插件注册",
        "awesome",
        services::inventory_sample::inventory_main,
        &[],
        Some("awesome/inventory.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "di-concrete",
        "依赖注入：具体类型",
        "awesome",
        || services::concrete_injection_sample::dependency_injection_concrete_sample().unwrap(),
        &[],
        Some("awesome/di.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "di-arc",
        "依赖注入：Arc trait 对象",
        "awesome",
        services::dynmaic_injection_arc_sample::container_injection_main,
        &[],
        Some("awesome/di.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "di-box",
        "依赖注入：Box trait 对象",
        "awesome",
        services::dynmaic_injection_box_sample::container_injection_main,
        &[],
        Some("awesome/di.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "service-locator",
        "服务定位器模式",
        "awesome",
        services::service_container_sample::service_container_main,
        &[],
        Some("awesome/di.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "consul",
        "Consul 服务发现",
        "awesome",
        || services::consul_sample::main_consul().unwrap(),
        &["Consul"],
        Some("awesome/consul.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "mqtt",
        "MQTT 消息队列",
        "awesome",
        mq::rumqtt_sample::mqtt_sync_sample,
        &["MQTT"],
        Some("awesome/mqtt.md"),
        SampleType::Function,
    )
}
