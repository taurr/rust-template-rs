//! Create a suite of code that shows different usecases while
//! it happens to test the code,
{% if tokio %}use {{crate_name}}::find_lines;
use tokio::test;

#[test]
async fn find_lines_of_text_containing_specific_str() {
    tracing_subscriber::fmt::init();
    assert!(
        find_lines("1\n2\n3\n10", "1")
        .await
        .count() == 2,
        "TODO: please implement me"
    );
}
{% else %}use {{crate_name}}::find_lines;

#[test]
fn find_lines_of_text_containing_specific_str() {
    tracing_subscriber::fmt::init();
    assert!(
        find_lines("1\n2\n3\n10", "1").count() == 2,
        "TODO: please implement me"
    );
}
{% endif %}