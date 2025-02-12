use super::system::*;
use mockall::predicate::*;
use rstest::*;
use test_log::test;

#[test]
fn test_cpu_usage() {
    let sys_info = SystemInfo::new();
    let usage = sys_info.get_cpu_usage();
    assert!(usage >= 0.0 && usage <= 100.0);
}

#[test]
fn test_memory_info() {
    let sys_info = SystemInfo::new();
    let memory = sys_info.get_memory_info();
    assert!(memory.total > 0);
    assert!(memory.used <= memory.total);
}

#[rstest]
#[case::valid_disk_path("/")]
fn test_disk_space(#[case] path: &str) {
    let sys_info = SystemInfo::new();
    let disk_info = sys_info.get_disk_info(path);
    assert!(disk_info.is_ok());
    if let Ok(info) = disk_info {
        assert!(info.total > 0);
        assert!(info.used <= info.total);
    }
}

#[test]
fn test_system_load() {
    let sys_info = SystemInfo::new();
    let load = sys_info.get_system_load();
    assert!(load.one >= 0.0);
    assert!(load.five >= 0.0);
    assert!(load.fifteen >= 0.0);
}