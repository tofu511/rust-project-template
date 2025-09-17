#[ignore]
#[test]
fn scaffold_compiles_and_links() {
    // Demonstrates structure; no real I/O.
    let _ctx = observability::context::Context::default();
    let _uid = domain::UserId(1);
    assert_eq!(domain::calc::add(2, 2), 4);
}

