//! Rust Basic Samples Module
//!
//! Foundation Rust concepts: ownership, types, generics, closures, threads, etc.

pub mod cfg_if_sample;
pub mod closure_sample;
pub mod datatype_sample;
pub mod expression_sample;
pub mod generic_sample;
pub mod logger_sample;
pub mod module_sample;
pub mod ownership_sample;
pub mod pointer_sample;
pub mod rectangle;
pub mod threads_sample;
pub mod tracing_sample;
pub mod traits_sample;
pub mod visiable_sample;

use crate::cli::registry::{SampleType, Topic};

// Register all basic topics with inventory
inventory::submit! {
    Topic::new(
        "expression",
        "变量绑定、可变性、基础表达式",
        "basic",
        expression_sample::variable_bind,
        &[],
        Some("basic/expression.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "ownership",
        "所有权规则、移动语义、借用与引用",
        "basic",
        ownership_sample::ownership_sample,
        &[],
        Some("basic/ownership.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "datatype",
        "整数、浮点数、布尔值、字符、集合、日期/时间",
        "basic",
        datatype_sample::string_sample,
        &[],
        Some("basic/datatype.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "generic",
        "泛型函数、单态化、trait 约束",
        "basic",
        generic_sample::add_generic_sample,
        &[],
        Some("basic/generic.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "threads",
        "线程创建、通道、Mutex、Arc",
        "basic",
        threads_sample::create_thread_sample,
        &[],
        Some("basic/threads.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "module",
        "模块组织、可见性、use 语句",
        "basic",
        module_sample::function,
        &[],
        Some("basic/module.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "logger",
        "日志记录和追踪",
        "basic",
        logger_sample::logger_print,
        &[],
        Some("basic/logger.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "tracing",
        "使用 tracing 进行结构化日志",
        "basic",
        tracing_sample::tracing_demo,
        &[],
        Some("basic/tracing.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "closure",
        "闭包语法、环境捕获、Fn/FnMut/FnOnce",
        "basic",
        closure_sample::closure_sample,
        &[],
        Some("basic/closure.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "traits",
        "Trait 定义、实现、多态",
        "basic",
        traits_sample::traits_simple_sample,
        &[],
        Some("basic/trait.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "pointer",
        "裸指针和原始指针操作",
        "basic",
        pointer_sample::raw_pointer_sample,
        &[],
        Some("basic/pointer.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "rectangle",
        "结构体生命周期示例",
        "basic",
        rectangle::rectangle_example,
        &[],
        Some("basic/struct.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "cfg-if",
        "条件编译和平台检测",
        "basic",
        cfg_if_sample::cfg_if_sample,
        &[],
        Some("basic/cfg-if.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "visiable",
        "可见性和访问控制",
        "basic",
        visiable_sample::visiable_sample,
        &[],
        Some("basic/visiable.md"),
        SampleType::Function,
    )
}
