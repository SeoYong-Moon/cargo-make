//! # validator
//!
//! Validation functions for cargo-make constructs
//!

#[cfg(test)]
#[path = "validator_test.rs"]
mod validator_test;

/// Maximum allowed length for a task name
const MAX_TASK_NAME_LENGTH: usize = 256;

/// Validates a task name according to cargo-make naming rules.
///
/// A valid task name must:
/// - Not be empty
/// - Not exceed 256 characters
/// - Contain only alphanumeric characters, hyphens (-), underscores (_), and namespace separators (::)
/// - Not have leading or trailing whitespace
/// - Not start or end with a hyphen or underscore
/// - Not contain consecutive namespace separators
///
/// # Arguments
///
/// * `name` - The task name to validate
///
/// # Returns
///
/// Returns `true` if the task name is valid, `false` otherwise.
///
/// # Examples
///
/// ```
/// use cli::validator::validate_task_name;
///
/// assert!(validate_task_name("build"));
/// assert!(validate_task_name("my-task"));
/// assert!(validate_task_name("my_task"));
/// assert!(validate_task_name("namespace::task"));
/// assert!(validate_task_name("build-123"));
///
/// assert!(!validate_task_name(""));
/// assert!(!validate_task_name(" task"));
/// assert!(!validate_task_name("task "));
/// assert!(!validate_task_name("task with spaces"));
/// assert!(!validate_task_name("task::"));
/// assert!(!validate_task_name("::task"));
/// assert!(!validate_task_name("task::::name"));
/// assert!(!validate_task_name("-task"));
/// assert!(!validate_task_name("task-"));
/// ```
pub fn validate_task_name(name: &str) -> bool {
    // Check if empty
    if name.is_empty() {
        return false;
    }

    // Check length
    if name.len() > MAX_TASK_NAME_LENGTH {
        return false;
    }

    // Check for leading or trailing whitespace
    if name != name.trim() {
        return false;
    }

    // Check for leading or trailing hyphen/underscore
    if name.starts_with('-') || name.starts_with('_') {
        return false;
    }
    if name.ends_with('-') || name.ends_with('_') {
        return false;
    }

    // Check for consecutive namespace separators
    if name.contains("::::") || name.contains(":::") {
        return false;
    }

    // Check for leading or trailing namespace separator
    if name.starts_with("::") || name.ends_with("::") {
        return false;
    }

    // Split by namespace separator and validate each part
    let parts: Vec<&str> = name.split("::").collect();
    
    for part in parts {
        // Each part must not be empty (already handled by :: checks above, but double-check)
        if part.is_empty() {
            return false;
        }

        // Check that each part contains only valid characters
        for ch in part.chars() {
            if !ch.is_alphanumeric() && ch != '-' && ch != '_' {
                return false;
            }
        }

        // Check that each part doesn't start or end with hyphen or underscore
        if part.starts_with('-') || part.starts_with('_') {
            return false;
        }
        if part.ends_with('-') || part.ends_with('_') {
            return false;
        }
    }

    true
}