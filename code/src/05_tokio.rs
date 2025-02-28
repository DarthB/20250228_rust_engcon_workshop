#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _array: Vec<String> = vec![
        "hello".into(),
        "asyn".into(),
        "rust".into(),
        "with.".into(),
        "function-like".into(),
        "and".into(),
        "attribute".into(),
        "macros".into(),
    ];

    // todo is another macro
    todo!()
}
