#[test]
fn test_argument() {
    let args = super::arg::Argument::new();

    assert_eq!(args.version_info.is_empty(), true);
}
