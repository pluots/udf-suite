pub mod generate;
pub mod namespaces;
pub mod valid;

/// Validate arg count; return a formatted message if not
pub fn validate_arg_count(count: usize, expected: usize, fn_name: &str) -> Result<(), String> {
    if count != expected {
        let pluralized = if expected == 1 {
            "argument"
        } else {
            "arguments"
        };

        Err(format!(
            "{} takes {} {} but got {}",
            fn_name, expected, pluralized, count
        ))
    } else {
        Ok(())
    }
}
