pub struct Logger;

impl Logger {
    const RESET: &'static str = "\x1b[0m";

    const SUCCESS: &'static str = "\x1b[30;102m"; // черный на ярко-зеленом
    const INFO: &'static str = "\x1b[30;104m"; // черный на ярко-синем
    const WARN: &'static str = "\x1b[30;103m"; // черный на ярко-желтом
    const ERROR: &'static str = "\x1b[97;101m"; // белый на ярко-красном
    const NOT_SUPPORTED: &'static str = "\x1b[97;101m"; // белый на ярко-красном

    const TARGET: &'static str = "\x1b[90m"; // серый

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
}
