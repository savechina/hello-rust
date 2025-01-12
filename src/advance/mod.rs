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
mod sqlx_sqmple;

/// include_dir sample
mod includedir_sample;

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

    futures_sample::futures_block_sample();
    futures_sample::futures_block_handle_sample();
    futures_sample::futures_await_main();
    futures_sample::futures_async_block_main();

    // tokio_sample::tokio_server_main();
    // tokio_sample::tokio_client_main();
}
