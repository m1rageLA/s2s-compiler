use std::any::type_name;
use std::fmt::Debug;

pub struct Logger;

impl Logger {
    const RESET: &'static str = "\x1b[0m";

    const SUCCESS: &'static str = "\x1b[30;102m"; // черный на ярко-зеленом
    const INFO: &'static str = "\x1b[30;104m"; // черный на ярко-синем
    const WARN: &'static str = "\x1b[30;103m"; // черный на ярко-желтом
    const ERROR: &'static str = "\x1b[97;101m"; // белый на ярко-красном
    const NOT_SUPPORTED: &'static str = "\x1b[97;101m"; // белый на ярко-красном

    const TARGET: &'static str = "\x1b[90m"; // серый
    const LABEL: &'static str = "\x1b[93m"; // ярко-желтый
    const VALUE: &'static str = "\x1b[96m"; // ярко-голубой

    fn print(icon: &str, bg: &str, level: &str, target: &str, message: &str) {
        println!(
            "{bg} {icon} {level:^7} {reset}  {target_color}[{target}]\
{reset}  {message}",
            bg = bg,
            icon = icon,
            level = level,
            target = target,
            message = message,
            target_color = Self::TARGET,
            reset = Self::RESET,
        );
    }

    pub fn success(message: &str, target: &str) {
        Self::print("✓", Self::SUCCESS, "SUCCESS", target, message);
    }
    pub fn not_supported(message: &str, target: &str) {
        Self::print("✗", Self::NOT_SUPPORTED, "NOT SUPPORTED", target, message);
    }

    pub fn step(message: &str, target: &str) {
        Self::print("ℹ", Self::INFO, "STEP", target, message);
    }

    pub fn warn(message: &str, target: &str) {
        Self::print("⚠", Self::WARN, "WARN", target, message);
    }

    pub fn error(message: &str, target: &str) {
        Self::print("✖", Self::ERROR, "ERROR", target, message);
    }

    /// Reports an AST node for which a match arm has not been implemented yet.
    ///
    /// Prefer the [`unsupported!`] macro at call sites: besides the node details,
    /// it automatically supplies the module, file, and line of the match arm.
    #[track_caller]
    pub fn unsupported_node<T: Debug>(node: &T, module: &str, file: &str, line: u32) -> ! {
        let (node_type, node_variant, node_value) = node_details(node);

        panic!(
            "\n{error} ✗ NOT SUPPORTED {reset}  {target}[{module}]{reset}\n\
             {label}Node:{reset}     {value}{node_type}::{node_variant}{reset}\n\
             {label}Location:{reset} {file}:{line}\n\
             {label}Value:{reset}\n{value}{node_value}{reset}",
            error = Self::NOT_SUPPORTED,
            reset = Self::RESET,
            target = Self::TARGET,
            label = Self::LABEL,
            value = Self::VALUE,
        );
    }
}

fn node_details<T: Debug>(node: &T) -> (&'static str, String, String) {
    let node_type = type_name::<T>()
        .trim_start_matches('&')
        .rsplit("::")
        .next()
        .unwrap_or("Unknown");
    let compact = format!("{node:?}");
    let node_variant = compact
        .split(['(', '{', ' ', '\n'])
        .next()
        .unwrap_or("Unknown")
        .rsplit("::")
        .next()
        .unwrap_or("Unknown")
        .to_owned();

    (node_type, node_variant, format!("{node:#?}"))
}

/// Panic with diagnostics for an unsupported AST match value.
///
/// # Example
///
/// ```ignore
/// match node {
///     Node::Supported(value) => compile(value),
///     _ => logger::unsupported!(node),
/// }
/// ```
#[macro_export]
macro_rules! unsupported {
    ($node:expr $(,)?) => {
        $crate::Logger::unsupported_node(&$node, module_path!(), file!(), line!())
    };
}

#[cfg(test)]
mod tests {
    use super::node_details;

    #[derive(Debug)]
    #[allow(dead_code)]
    enum TestNode {
        Missing { value: u8 },
    }

    #[test]
    fn extracts_the_node_type_variant_and_value() {
        let node = TestNode::Missing { value: 7 };
        let (node_type, variant, value) = node_details(&node);

        assert_eq!(node_type, "TestNode");
        assert_eq!(variant, "Missing");
        assert!(value.contains("value: 7"));
    }

    #[test]
    #[should_panic(expected = "NOT SUPPORTED")]
    fn unsupported_macro_panics() {
        let node = TestNode::Missing { value: 7 };
        crate::unsupported!(node);
    }
}
