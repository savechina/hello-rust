//!
//! Rust Advance Example
//! Rust 高阶样例代码
//!
//!
//!

///JSON
mod json_sample;

/// memap
mod memmap_sample;

/// tempfile
mod tempfile_sample;

/// fetures async
mod futures_sample;

/// tokio async
mod tokio_sample;

/// bytes
mod bytes_sample;

/// Ollama
mod ollama_sample;

///Database Sqlx
mod sqlx_sample;

/// include_dir sample
mod includedir_sample;

/// dotenv
mod dotenv_sample;

/// diesel
mod diesel_sample;

/// rkyv
mod rkyv_sample;

/// csv
mod csv_sample;

///getset
pub(crate) mod getset_sample;

///hyper
mod hyper_sample;

///mio
mod mio_sample;

/// axum
mod axum_sample;

///object_store
mod objectstore_sample;

///sysinfo sample
mod sysinfo_sample;

///rayon_sample
mod rayon_sample;

/// cyclerc_sample
mod cyclerc_sample;

/// typealias sample
mod typealias_sample;

/// process sample
mod process_sample;

/// mock sample
mod mock_sample;

/// rspec sample
mod rspec_sample;

/// macros sample
mod macros_sample;

/// cow sample
mod cow_sample;

use crate::cli::registry::{SampleType, Topic};

pub(crate) fn advance_sample() {
    //JSON 序列化及反序列化解析
    json_sample::typed_sample().unwrap();
    json_sample::untyped_sample().unwrap();
    json_sample::json_process_sample();
    json_sample::json_transcode_sample();

    //临时目录
    tempfile_sample::tempfile_sample();
    tempfile_sample::temp_namedfile_sample();
    tempfile_sample::tempdir_addfile();

    //memap
    memmap_sample::memmap_file_sample();
    memmap_sample::sys_page_size_sample();

    //futures
    futures_sample::futures_block_sample();
    futures_sample::futures_block_handle_sample();
    futures_sample::futures_await_main();
    futures_sample::futures_async_block_main();

    // tokio_sample::tokio_server_main();
    // tokio_sample::tokio_client_main();

    //bytes
    bytes_sample::bytes_create();

    //ollama
    // ollama_sample::ollama_sample();

    //sqlx
    sqlx_sample::sqlx_sqlite_example().unwrap();
}

// Register all advance topics with inventory
inventory::submit! {
    Topic::new(
        "json",
        "JSON 序列化及反序列化",
        "advance",
        json_sample::json_process_sample,
        &[],
        Some("advance/json.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "tempfile",
        "临时文件和目录创建",
        "advance",
        tempfile_sample::tempfile_sample,
        &[],
        Some("advance/tempfile.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "memmap",
        "内存映射文件操作",
        "advance",
        memmap_sample::memmap_file_sample,
        &[],
        Some("advance/memmap.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "futures",
        "异步 Future 基础",
        "advance",
        futures_sample::futures_block_sample,
        &[],
        Some("advance/futures.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "bytes",
        "字节缓冲区、base64、位操作",
        "advance",
        bytes_sample::bytes_create,
        &[],
        Some("advance/bytes.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "include-dir",
        "编译时文件嵌入",
        "advance",
        includedir_sample::include_dir_sample,
        &[],
        Some("advance/include-dir.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "dotenv",
        "环境变量加载",
        "advance",
        || dotenv_sample::dotenv_sample().unwrap(),
        &[],
        Some("advance/dotenv.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "rkyv",
        "零拷贝序列化",
        "advance",
        rkyv_sample::rkyv_basic_serialize_sample,
        &[],
        Some("advance/rkyv.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "csv",
        "CSV 解析",
        "advance",
        || csv_sample::csv_sample().unwrap(),
        &[],
        Some("advance/csv.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "getset",
        "派生宏生成 getter/setter",
        "advance",
        getset_sample::getset_sample,
        &[],
        Some("advance/getset.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "sysinfo",
        "系统信息收集",
        "advance",
        sysinfo_sample::sysinfo_sample,
        &[],
        Some("advance/sysinfo.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "rayon",
        "数据并行化",
        "advance",
        rayon_sample::rayon_sample,
        &[],
        Some("advance/rayon.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "cycle-rc",
        "引用循环处理",
        "advance",
        cyclerc_sample::cycle_weak_sample,
        &[],
        Some("advance/cycle-rc.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "type-alias",
        "类型别名模式",
        "advance",
        typealias_sample::typealias_sample,
        &[],
        Some("advance/type-alias.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "process",
        "进程管理",
        "advance",
        || process_sample::process_getpid_sample().unwrap(),
        &[],
        Some("advance/process.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "macros",
        "声明式和过程宏",
        "advance",
        macros_sample::declare_macros_hello_sample,
        &[],
        Some("advance/macros.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "cow",
        "写时克隆模式",
        "advance",
        cow_sample::cow_reader_sample,
        &[],
        Some("advance/cow.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "sqlx",
        "异步数据库查询 (SQLite)",
        "advance",
        || sqlx_sample::sqlx_sqlite_example().unwrap(),
        &["SQLite"],
        Some("advance/sqlx.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "diesel",
        "Diesel ORM (SQLite)",
        "advance",
        diesel_sample::diesel_sample,
        &["SQLite"],
        Some("advance/diesel.md"),
        SampleType::Function,
    )
}

inventory::submit! {
    Topic::new(
        "ollama",
        "Ollama LLM 集成",
        "advance",
        || ollama_sample::ollama_chat_sample().unwrap(),
        &["Ollama"],
        Some("advance/ollama.md"),
        SampleType::Function,
    )
}
