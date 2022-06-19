//! Creating a suite of code that shows different usecases while
//! it happens to test the code,
use {{crate_name}}::find_lines;
use tokio::test;

type Result = anyhow::Result<()>;

#[test]
async fn find_lines_of_text_containing_specific_str() -> Result {
    tracing_subscriber::fmt::init();
    assert!(
        find_lines("1\n2\n3\n10", "1").count() == 2,
        "TODO: please implement me"
    );
    Ok(())
}
