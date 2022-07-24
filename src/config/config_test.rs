#[test]
fn test_config() {
    let mut c = super::config::Config::new("".to_string(), "".to_string());
    assert!(c.config().is_err());

    let mut c = super::config::Config::new("config.txt".to_string(), "".to_string());
    assert!(c.config().is_err());

    let mut c = super::config::Config::new("/config.yml".to_string(), "".to_string());
    assert!(c.config().is_err());

    let n = std::path::Path::new(&std::env::current_dir().unwrap())
        .join("src")
        .join("config")
        .join("config.yml");
    let mut c =
        super::config::Config::new(n.into_os_string().into_string().unwrap(), "".to_string());
    assert!(c.config().is_ok());
}
