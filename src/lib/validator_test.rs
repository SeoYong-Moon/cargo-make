use super::*;

#[test]
fn validate_task_name_empty() {
    assert!(!validate_task_name(""));
}

#[test]
fn validate_task_name_simple_valid() {
    assert!(validate_task_name("build"));
    assert!(validate_task_name("test"));
    assert!(validate_task_name("deploy"));
    assert!(validate_task_name("A"));
    assert!(validate_task_name("a"));
}

#[test]
fn validate_task_name_with_hyphen() {
    assert!(validate_task_name("my-task"));
    assert!(validate_task_name("build-release"));
    assert!(validate_task_name("pre-build"));
}

#[test]
fn validate_task_name_with_underscore() {
    assert!(validate_task_name("my_task"));
    assert!(validate_task_name("build_release"));
    assert!(validate_task_name("pre_build"));
}

#[test]
fn validate_task_name_with_numbers() {
    assert!(validate_task_name("build123"));
    assert!(validate_task_name("task1"));
    assert!(validate_task_name("123task"));
    assert!(validate_task_name("build-123"));
    assert!(validate_task_name("task_456"));
}

#[test]
fn validate_task_name_mixed_valid() {
    assert!(validate_task_name("my-task_123"));
    assert!(validate_task_name("Build-Task_1"));
    assert!(validate_task_name("camelCaseTask"));
}

#[test]
fn validate_task_name_namespaced() {
    assert!(validate_task_name("namespace::task"));
    assert!(validate_task_name("my-namespace::my-task"));
    assert!(validate_task_name("ns1::ns2::task"));
    assert!(validate_task_name("project::build::release"));
}

#[test]
fn validate_task_name_with_spaces() {
    assert!(!validate_task_name("task with spaces"));
    assert!(!validate_task_name("task "));
    assert!(!validate_task_name(" task"));
    assert!(!validate_task_name("task\t"));
    assert!(!validate_task_name("\ntask"));
}

#[test]
fn validate_task_name_leading_trailing_separators() {
    assert!(!validate_task_name("-task"));
    assert!(!validate_task_name("task-"));
    assert!(!validate_task_name("_task"));
    assert!(!validate_task_name("task_"));
    assert!(!validate_task_name("::task"));
    assert!(!validate_task_name("task::"));
}

#[test]
fn validate_task_name_invalid_namespace_separators() {
    assert!(!validate_task_name("task::::name"));
    assert!(!validate_task_name("task:::name"));
    assert!(!validate_task_name("::"));
    assert!(!validate_task_name("::::"));
}

#[test]
fn validate_task_name_special_characters() {
    assert!(!validate_task_name("task@name"));
    assert!(!validate_task_name("task#name"));
    assert!(!validate_task_name("task$name"));
    assert!(!validate_task_name("task%name"));
    assert!(!validate_task_name("task&name"));
    assert!(!validate_task_name("task*name"));
    assert!(!validate_task_name("task!name"));
    assert!(!validate_task_name("task.name"));
    assert!(!validate_task_name("task/name"));
    assert!(!validate_task_name("task\\name"));
}

#[test]
fn validate_task_name_max_length() {
    // Create a name exactly at max length
    let max_length_name = "a".repeat(256);
    assert!(validate_task_name(&max_length_name));

    // Create a name over max length
    let over_max_name = "a".repeat(257);
    assert!(!validate_task_name(&over_max_name));
}

#[test]
fn validate_task_name_real_world_examples() {
    // Common real-world task names
    assert!(validate_task_name("format"));
    assert!(validate_task_name("clean"));
    assert!(validate_task_name("build"));
    assert!(validate_task_name("test"));
    assert!(validate_task_name("my-flow"));
    assert!(validate_task_name("pre-build"));
    assert!(validate_task_name("post-build"));
    assert!(validate_task_name("cargo-build"));
    assert!(validate_task_name("install_crate"));
    assert!(validate_task_name("check-format"));
    assert!(validate_task_name("run_tests"));
}

#[test]
fn validate_task_name_edge_cases() {
    // Edge cases
    assert!(validate_task_name("a"));
    assert!(validate_task_name("1"));
    assert!(validate_task_name("a1"));
    assert!(validate_task_name("a-b"));
    assert!(validate_task_name("a_b"));
    
    // Invalid edge cases
    assert!(!validate_task_name("-"));
    assert!(!validate_task_name("_"));
    assert!(!validate_task_name("a-"));
    assert!(!validate_task_name("a_"));
    assert!(!validate_task_name("-a"));
    assert!(!validate_task_name("_a"));
}

#[test]
fn validate_task_name_namespace_edge_cases() {
    // Valid namespace combinations
    assert!(validate_task_name("a::b"));
    assert!(validate_task_name("a1::b2"));
    
    // Invalid namespace combinations
    assert!(!validate_task_name("a::"));
    assert!(!validate_task_name("::b"));
    assert!(!validate_task_name("a:b"));
    assert!(!validate_task_name("a:::b"));
    assert!(!validate_task_name("a::::b"));
}

#[test]
fn validate_task_name_unicode() {
    // Unicode characters should be invalid
    assert!(!validate_task_name("task-名前"));
    assert!(!validate_task_name("задача"));
    assert!(!validate_task_name("tâche"));
    assert!(!validate_task_name("task-🚀"));
}

#[test]
fn validate_task_name_whitespace_variations() {
    assert!(!validate_task_name(" "));
    assert!(!validate_task_name("  "));
    assert!(!validate_task_name("\t"));
    assert!(!validate_task_name("\n"));
    assert!(!validate_task_name("task\nname"));
    assert!(!validate_task_name("task\tname"));
}