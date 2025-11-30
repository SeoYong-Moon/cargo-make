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