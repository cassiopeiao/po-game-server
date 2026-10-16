use libdeno::{LibdenoOptions, SourceLang, run_source_with_output_async};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

const MAIN_SCRIPT: &[u8] = include_bytes!("../dist/main.js");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = LibdenoOptions {
        permissions: vec![],
        allow_all_permissions: true,
        prompt: false,
        args: vec![],
        cwd: None,
        max_heap_bytes: None,
        execution_deadline: None,
        capture_stdout: false,
        capture_stderr: false,
        max_capture_bytes: None,
        features: Some(vec!["kv".to_string()]),
    };
    run_source_with_output_async(MAIN_SCRIPT, SourceLang::JavaScript, ".", &options).await?;
    Ok(())
}
