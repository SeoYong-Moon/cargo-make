//! # validator
//!
//! Validation functions for cargo-make constructs
//!

#[cfg(test)]
#[path = "validator_test.rs"]
mod validator_test;

use std::fmt;

/// Maximum allowed length for a task name
const MAX_TASK_NAME_LENGTH: usize = 256;

/// Represents validation errors for task names
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskNameValidationError {
    /// Task name is empty
    Empty,
    /// Task name exceeds maximum length
    TooLong { length: usize, max: usize },
    /// Task name contains leading or trailing whitespace
    InvalidWhitespace,
    /// Task name starts with an invalid character
    InvalidLeadingCharacter { character: char },
    /// Task name ends with an invalid character
    InvalidTrailingCharacter { character: char },
    /// Task name contains consecutive namespace separators
    ConsecutiveNamespaceSeparators,
    /// Task name starts with namespace separator
    LeadingNamespaceSeparator,
    /// Task name ends with namespace separator
    TrailingNamespaceSeparator,
    /// Task name contains an invalid character
    InvalidCharacter { character: char, position: usize },
    /// A namespace part has invalid leading character
    InvalidNamespacePartLeading { part: String, character: char },
    /// A namespace part has invalid trailing character
    InvalidNamespacePartTrailing { part: String, character: char },
}

impl fmt::Display for TaskNameValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskNameValidationError::Empty => {
                write!(f, "Task name cannot be empty")
            }
            TaskNameValidationError::TooLong { length, max } => {
                write!(
                    f,
                    "Task name is too long: {} characters (maximum: {})",
                    length, max
                )
            }
            TaskNameValidationError::InvalidWhitespace => {
                write!(f, "Task name cannot have leading or trailing whitespace")
            }
            TaskNameValidationError::InvalidLeadingCharacter { character } => {
                write!(
                    f,
                    "Task name cannot start with '{}' (hyphens and underscores not allowed at the start)",
                    character
                )
            }
            TaskNameValidationError::InvalidTrailingCharacter { character } => {
                write!(
                    f,
                    "Task name cannot end with '{}' (hyphens and underscores not allowed at the end)",
                    character
                )
            }
            TaskNameValidationError::ConsecutiveNamespaceSeparators => {
                write!(f, "Task name cannot contain consecutive namespace separators (:::)")
            }
            TaskNameValidationError::LeadingNamespaceSeparator => {
                write!(f, "Task name cannot start with namespace separator (::)")
            }
            TaskNameValidationError::TrailingNamespaceSeparator => {
                write!(f, "Task name cannot end with namespace separator (::)")
            }
            TaskNameValidationError::InvalidCharacter { character, position } => {
                write!(
                    f,
                    "Invalid character '{}' at position {} (only ASCII alphanumeric, hyphens, underscores, and '::' are allowed)",
                    character, position
                )
            }
            TaskNameValidationError::InvalidNamespacePartLeading { part, character } => {
                write!(
                    f,
                    "Namespace part '{}' cannot start with '{}' (hyphens and underscores not allowed at the start)",
                    part, character
                )
            }
            TaskNameValidationError::InvalidNamespacePartTrailing { part, character } => {
                write!(
                    f,
                    "Namespace part '{}' cannot end with '{}' (hyphens and underscores not allowed at the end)",
                    part, character
                )
            }
        }
    }
}

impl std::error::Error for TaskNameValidationError {}

/// Validates a task name according to cargo-make naming rules and returns detailed error information.
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
/// Returns `Ok(())` if the task name is valid, or `Err(TaskNameValidationError)` with details about why it's invalid.
///
/// # Examples
///
/// ```
/// use cli::validator::validate_task_name_with_error;
///
/// // Valid names
/// assert!(validate_task_name_with_error("build").is_ok());
/// assert!(validate_task_name_with_error("my-task").is_ok());
/// assert!(validate_task_name_with_error("namespace::task").is_ok());
///
/// // Invalid names return specific errors
/// assert!(validate_task_name_with_error("").is_err());
/// assert!(validate_task_name_with_error(" task").is_err());
/// assert!(validate_task_name_with_error("task-").is_err());
/// ```
pub fn validate_task_name_with_error(name: &str) -> Result<(), TaskNameValidationError> {
    // Check if empty
    if name.is_empty() {
        return Err(TaskNameValidationError::Empty);
    }

    // Check length
    if name.len() > MAX_TASK_NAME_LENGTH {
        return Err(TaskNameValidationError::TooLong {
            length: name.len(),
            max: MAX_TASK_NAME_LENGTH,
        });
    }

    // Check for leading or trailing whitespace
    if name != name.trim() {
        return Err(TaskNameValidationError::InvalidWhitespace);
    }

    // Check for leading or trailing hyphen/underscore
    if let Some(first_char) = name.chars().next() {
        if first_char == '-' || first_char == '_' {
            return Err(TaskNameValidationError::InvalidLeadingCharacter {
                character: first_char,
            });
        }
    }
    if let Some(last_char) = name.chars().last() {
        if last_char == '-' || last_char == '_' {
            return Err(TaskNameValidationError::InvalidTrailingCharacter {
                character: last_char,
            });
        }
    }

    // Check for consecutive namespace separators
    if name.contains("::::") || name.contains(":::") {
        return Err(TaskNameValidationError::ConsecutiveNamespaceSeparators);
    }

    // Check for leading or trailing namespace separator
    if name.starts_with("::") {
        return Err(TaskNameValidationError::LeadingNamespaceSeparator);
    }
    if name.ends_with("::") {
        return Err(TaskNameValidationError::TrailingNamespaceSeparator);
    }

    // Split by namespace separator and validate each part
    let parts: Vec<&str> = name.split("::").collect();

    for part in parts {
        // Each part must not be empty (already handled by :: checks above, but double-check)
        if part.is_empty() {
            return Err(TaskNameValidationError::ConsecutiveNamespaceSeparators);
        }

        // Check that each part contains only valid characters (ASCII alphanumeric, hyphen, underscore)
        for (idx, ch) in part.chars().enumerate() {
            if !ch.is_ascii_alphanumeric() && ch != '-' && ch != '_' {
                // Calculate actual position in full string
                let position = name.find(part).unwrap_or(0) + idx;
                return Err(TaskNameValidationError::InvalidCharacter {
                    character: ch,
                    position,
                });
            }
        }

        // Check that each part doesn't start or end with hyphen or underscore
        if let Some(first_char) = part.chars().next() {
            if first_char == '-' || first_char == '_' {
                return Err(TaskNameValidationError::InvalidNamespacePartLeading {
                    part: part.to_string(),
                    character: first_char,
                });
            }
        }
        if let Some(last_char) = part.chars().last() {
            if last_char == '-' || last_char == '_' {
                return Err(TaskNameValidationError::InvalidNamespacePartTrailing {
                    part: part.to_string(),
                    character: last_char,
                });
            }
        }
    }

    Ok(())
}

pub fn validate_task_name(name: &str) -> bool {
    validate_task_name_with_error(name).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for validate_task_name_with_error function

    #[test]
    fn test_error_empty() {
        let result = validate_task_name_with_error("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TaskNameValidationError::Empty);
    }

    #[test]
    fn test_error_too_long() {
        let long_name = "a".repeat(257);
        let result = validate_task_name_with_error(&long_name);
        assert!(result.is_err());
        match result.unwrap_err() {
            TaskNameValidationError::TooLong { length, max } => {
                assert_eq!(length, 257);
                assert_eq!(max, 256);
            }
            _ => panic!("Expected TooLong error"),
        }
    }

    #[test]
    fn test_error_max_length_valid() {
        let max_name = "a".repeat(256);
        let result = validate_task_name_with_error(&max_name);
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_invalid_whitespace_leading() {
        let result = validate_task_name_with_error(" task");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TaskNameValidationError::InvalidWhitespace
        );
    }

    #[test]
    fn test_error_invalid_whitespace_trailing() {
        let result = validate_task_name_with_error("task ");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TaskNameValidationError::InvalidWhitespace
        );
    }

    #[test]
    fn test_error_invalid_leading_hyphen() {
        let result = validate_task_name_with_error("-task");
        assert!(result.is_err());
        match result.unwrap_err() {
            TaskNameValidationError::InvalidLeadingCharacter { character } => {
                assert_eq!(character, '-');
            }
            _ => panic!("Expected InvalidLeadingCharacter error"),
        }
    }

    #[test]
    fn test_error_invalid_leading_underscore() {
        let result = validate_task_name_with_error("_task");
        assert!(result.is_err());
        match result.unwrap_err() {
            TaskNameValidationError::InvalidLeadingCharacter { character } => {
                assert_eq!(character, '_');
            }
            _ => panic!("Expected InvalidLeadingCharacter error"),
        }
    }

    #[test]
    fn test_error_invalid_trailing_hyphen() {
        let result = validate_task_name_with_error("task-");
        assert!(result.is_err());
        match result.unwrap_err() {
            TaskNameValidationError::InvalidTrailingCharacter { character } => {
                assert_eq!(character, '-');
            }
            _ => panic!("Expected InvalidTrailingCharacter error"),
        }
    }

    #[test]
    fn test_error_invalid_trailing_underscore() {
        let result = validate_task_name_with_error("task_");
        assert!(result.is_err());
        match result.unwrap_err() {
            TaskNameValidationError::InvalidTrailingCharacter { character } => {
                assert_eq!(character, '_');
            }
            _ => panic!("Expected InvalidTrailingCharacter error"),
        }
    }

    #[test]
    fn test_error_consecutive_namespace_separators_triple() {
        let result = validate_task_name_with_error("task:::name");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TaskNameValidationError::ConsecutiveNamespaceSeparators
        );
    }

    #[test]
    fn test_error_consecutive_namespace_separators_quadruple() {
        let result = validate_task_name_with_error("task::::name");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TaskNameValidationError::ConsecutiveNamespaceSeparators
        );
    }

    #[test]
    fn test_error_leading_namespace_separator() {
        let result = validate_task_name_with_error("::task");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TaskNameValidationError::LeadingNamespaceSeparator
        );
    }

    #[test]
    fn test_error_trailing_namespace_separator() {
        let result = validate_task_name_with_error("task::");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TaskNameValidationError::TrailingNamespaceSeparator
        );
    }

    #[test]
    fn test_error_invalid_character() {
        let result = validate_task_name_with_error("task@name");
        assert!(result.is_err());
        match result.unwrap_err() {
            TaskNameValidationError::InvalidCharacter { character, position } => {
                assert_eq!(character, '@');
                assert_eq!(position, 4);
            }
            _ => panic!("Expected InvalidCharacter error"),
        }
    }

    #[test]
    fn test_error_invalid_character_space() {
        let result = validate_task_name_with_error("task name");
        assert!(result.is_err());
        match result.unwrap_err() {
            TaskNameValidationError::InvalidCharacter { character, position } => {
                assert_eq!(character, ' ');
                assert_eq!(position, 4);
            }
            _ => panic!("Expected InvalidCharacter error"),
        }
    }

    #[test]
    fn test_error_namespace_part_leading_hyphen() {
        let result = validate_task_name_with_error("namespace::-build");
        assert!(result.is_err());
        match result.unwrap_err() {
            TaskNameValidationError::InvalidNamespacePartLeading { part, character } => {
                assert_eq!(part, "-build");
                assert_eq!(character, '-');
            }
            _ => panic!("Expected InvalidNamespacePartLeading error"),
        }
    }

    #[test]
    fn test_error_whole_name_trailing_hyphen() {
        // When the whole name ends with hyphen, it's caught as InvalidTrailingCharacter
        let result = validate_task_name_with_error("namespace::build-");
        assert!(result.is_err());
        match result.unwrap_err() {
            TaskNameValidationError::InvalidTrailingCharacter { character } => {
                assert_eq!(character, '-');
            }
            _ => panic!("Expected InvalidTrailingCharacter error for whole name"),
        }
    }

    #[test]
    fn test_error_namespace_part_leading_underscore() {
        let result = validate_task_name_with_error("namespace::_build");
        assert!(result.is_err());
        match result.unwrap_err() {
            TaskNameValidationError::InvalidNamespacePartLeading { part, character } => {
                assert_eq!(part, "_build");
                assert_eq!(character, '_');
            }
            _ => panic!("Expected InvalidNamespacePartLeading error"),
        }
    }

    #[test]
    fn test_valid_simple_names() {
        assert!(validate_task_name_with_error("build").is_ok());
        assert!(validate_task_name_with_error("test").is_ok());
        assert!(validate_task_name_with_error("deploy").is_ok());
        assert!(validate_task_name_with_error("a").is_ok());
        assert!(validate_task_name_with_error("A").is_ok());
        assert!(validate_task_name_with_error("z").is_ok());
        assert!(validate_task_name_with_error("Z").is_ok());
    }

    #[test]
    fn test_valid_with_numbers() {
        assert!(validate_task_name_with_error("task1").is_ok());
        assert!(validate_task_name_with_error("123task").is_ok());
        assert!(validate_task_name_with_error("task123").is_ok());
        assert!(validate_task_name_with_error("1").is_ok());
    }

    #[test]
    fn test_valid_with_hyphens() {
        assert!(validate_task_name_with_error("my-task").is_ok());
        assert!(validate_task_name_with_error("pre-build").is_ok());
        assert!(validate_task_name_with_error("post-deploy").is_ok());
        assert!(validate_task_name_with_error("a-b-c").is_ok());
    }

    #[test]
    fn test_valid_with_underscores() {
        assert!(validate_task_name_with_error("my_task").is_ok());
        assert!(validate_task_name_with_error("pre_build").is_ok());
        assert!(validate_task_name_with_error("post_deploy").is_ok());
        assert!(validate_task_name_with_error("a_b_c").is_ok());
    }

    #[test]
    fn test_valid_mixed_format() {
        assert!(validate_task_name_with_error("my-task_123").is_ok());
        assert!(validate_task_name_with_error("Build-Task_1").is_ok());
        assert!(validate_task_name_with_error("test-my_task-123").is_ok());
    }

    #[test]
    fn test_valid_namespaced() {
        assert!(validate_task_name_with_error("namespace::task").is_ok());
        assert!(validate_task_name_with_error("my-namespace::my-task").is_ok());
        assert!(validate_task_name_with_error("ns1::ns2::task").is_ok());
        assert!(validate_task_name_with_error("project::build::release").is_ok());
    }

    #[test]
    fn test_error_display_empty() {
        let error = TaskNameValidationError::Empty;
        assert_eq!(error.to_string(), "Task name cannot be empty");
    }

    #[test]
    fn test_error_display_too_long() {
        let error = TaskNameValidationError::TooLong {
            length: 300,
            max: 256,
        };
        assert_eq!(
            error.to_string(),
            "Task name is too long: 300 characters (maximum: 256)"
        );
    }

    #[test]
    fn test_error_display_invalid_whitespace() {
        let error = TaskNameValidationError::InvalidWhitespace;
        assert_eq!(
            error.to_string(),
            "Task name cannot have leading or trailing whitespace"
        );
    }

    #[test]
    fn test_error_display_invalid_leading_character() {
        let error = TaskNameValidationError::InvalidLeadingCharacter { character: '-' };
        assert!(error.to_string().contains("cannot start with '-'"));
    }

    #[test]
    fn test_error_display_invalid_trailing_character() {
        let error = TaskNameValidationError::InvalidTrailingCharacter { character: '_' };
        assert!(error.to_string().contains("cannot end with '_'"));
    }

    #[test]
    fn test_error_display_invalid_character() {
        let error = TaskNameValidationError::InvalidCharacter {
            character: '@',
            position: 5,
        };
        let msg = error.to_string();
        assert!(msg.contains("Invalid character '@'"));
        assert!(msg.contains("position 5"));
    }

    #[test]
    fn test_error_clone() {
        let error = TaskNameValidationError::Empty;
        let cloned = error.clone();
        assert_eq!(error, cloned);
    }

    #[test]
    fn test_error_equality() {
        let error1 = TaskNameValidationError::Empty;
        let error2 = TaskNameValidationError::Empty;
        let error3 = TaskNameValidationError::InvalidWhitespace;

        assert_eq!(error1, error2);
        assert_ne!(error1, error3);
    }

    #[test]
    fn test_error_debug() {
        let error = TaskNameValidationError::Empty;
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("Empty"));
    }

    // Tests for validate_task_name boolean function

    #[test]
    fn test_bool_valid_names() {
        assert!(validate_task_name("build"));
        assert!(validate_task_name("my-task"));
        assert!(validate_task_name("my_task"));
        assert!(validate_task_name("namespace::task"));
        assert!(validate_task_name("build-123"));
    }

    #[test]
    fn test_bool_invalid_names() {
        assert!(!validate_task_name(""));
        assert!(!validate_task_name(" task"));
        assert!(!validate_task_name("task "));
        assert!(!validate_task_name("task with spaces"));
        assert!(!validate_task_name("task::"));
        assert!(!validate_task_name("::task"));
        assert!(!validate_task_name("task::::name"));
        assert!(!validate_task_name("-task"));
        assert!(!validate_task_name("task-"));
    }

    #[test]
    fn test_bool_real_world_examples() {
        // Common cargo-make task names
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
    fn test_unicode_rejection() {
        // Unicode should be rejected
        assert!(validate_task_name_with_error("task-名前").is_err());
        assert!(validate_task_name_with_error("задача").is_err());
        assert!(validate_task_name_with_error("tâche").is_err());
        assert!(validate_task_name_with_error("task-🚀").is_err());
    }

    #[test]
    fn test_special_characters() {
        // Various special characters should be rejected
        assert!(validate_task_name_with_error("task@name").is_err());
        assert!(validate_task_name_with_error("task#name").is_err());
        assert!(validate_task_name_with_error("task$name").is_err());
        assert!(validate_task_name_with_error("task%name").is_err());
        assert!(validate_task_name_with_error("task&name").is_err());
        assert!(validate_task_name_with_error("task*name").is_err());
        assert!(validate_task_name_with_error("task!name").is_err());
        assert!(validate_task_name_with_error("task.name").is_err());
        assert!(validate_task_name_with_error("task/name").is_err());
        assert!(validate_task_name_with_error("task\\name").is_err());
    }

    #[test]
    fn test_edge_cases() {
        // Single character valid
        assert!(validate_task_name("a"));
        assert!(validate_task_name("1"));
        
        // Single invalid characters
        assert!(!validate_task_name("-"));
        assert!(!validate_task_name("_"));
        
        // Minimum valid combinations
        assert!(validate_task_name("a1"));
        assert!(validate_task_name("a-b"));
        assert!(validate_task_name("a_b"));
        assert!(validate_task_name("a::b"));
    }

    #[test]
    fn test_namespace_validation() {
        // Valid namespace combinations
        assert!(validate_task_name("a::b"));
        assert!(validate_task_name("a1::b2"));
        assert!(validate_task_name("abc::def::ghi"));
        
        // Invalid namespace combinations
        assert!(!validate_task_name("a::"));
        assert!(!validate_task_name("::b"));
        assert!(!validate_task_name("a:b"));
        assert!(!validate_task_name("a:::b"));
        assert!(!validate_task_name("a::::b"));
    }

    #[test]
    fn test_whitespace_variations() {
        assert!(!validate_task_name(" "));
        assert!(!validate_task_name("  "));
        assert!(!validate_task_name("\t"));
        assert!(!validate_task_name("\n"));
        assert!(!validate_task_name("task\nname"));
        assert!(!validate_task_name("task\tname"));
    }

    #[test]
    fn test_error_as_std_error() {
        let error: Box<dyn std::error::Error> = Box::new(TaskNameValidationError::Empty);
        assert_eq!(error.to_string(), "Task name cannot be empty");
    }

    #[test]
    fn test_result_propagation() {
        fn validate_wrapper(name: &str) -> Result<(), TaskNameValidationError> {
            validate_task_name_with_error(name)?;
            Ok(())
        }

        assert!(validate_wrapper("valid-name").is_ok());
        assert!(validate_wrapper("").is_err());
    }
}