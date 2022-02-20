use {{crate_name}}::find_lines;
use tokio::test;

type Result = anyhow::Result<()>;

#[test]
async fn test() -> Result {
    tracing_subscriber::fmt::init();

    assert!(
        find_lines("1\n2", "").count() == 2,
        "TODO: please implement me"
    );
    Ok(())
}
