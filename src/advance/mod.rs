//!
//! Rust Advance Example
//! Rust 高阶样例代码
//!
//!
//!

///JSON
mod json_sample;

///临时文件
mod tempfile_sample;

mod memmap_sample;

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

    memmap_sample::memmap_file_sample();
    memmap_sample::sys_page_size_sample();
}
