#[path = "common/mod.rs"]
mod test_utils;

#[path = "e2e/simple_math.rs"]
mod simple_math;

#[cfg(test)]
#[allow(unused)]
#[path = "e2e/console_log.rs"]
mod console_log;
