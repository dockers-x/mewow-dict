use std::env;
use std::path::PathBuf;

pub const MDX_FILES: &[&str] = &[
    "./resources/mdx/en/牛津高阶英汉双解词典（第9版）.mdx",
     "./resources/mdx/en/韦氏高阶英汉双解词典v3.mdx",
];

pub fn static_path() -> anyhow::Result<PathBuf> {
    let mut path: PathBuf = env!("CARGO_MANIFEST_DIR").into();
    path.push("resources/static");
    Ok(path)
}
