use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::value::Value;

// Simple thread-local LCG to avoid pulling in external RNG crates.
fn next_random_bits() -> u64 {
    static STATE: AtomicU64 = AtomicU64::new(0);

    let mut state = STATE.load(Ordering::Relaxed);
    if state == 0 {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|dur| dur.as_nanos() as u64)
            .unwrap_or(1);
        state = seed | 1; // keep it odd and non-zero to avoid a stuck LCG.
    }

    // https://en.wikipedia.org/wiki/Linear_congruential_generator
    state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
    STATE.store(state, Ordering::Relaxed);
    state
}

/// Mirrors `Math.random()` returning a floating point in the range [0, 1).
pub fn random() -> Value {
    Value::Number(random_number())
}

pub fn random_number() -> f64 {
    // Use the upper 53 bits for an f64 mantissa to stay within [0, 1).
    let bits = next_random_bits() >> 11;
    (bits as f64) / ((1u64 << 53) as f64)
}

pub fn sqrt<V>(value: V) -> Value
where
    V: Into<Value>,
{
    let num = value.into().to_number();
    Value::Number(num.sqrt())
}

pub fn sqrt_number(value: f64) -> f64 {
    value.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_returns_number_value_in_unit_interval() {
        match random() {
            Value::Number(value) => {
                assert!(value >= 0.0);
                assert!(value < 1.0);
            }
            other => panic!("expected number value from Math.random, got {other:?}"),
        }
    }

    #[test]
    fn random_number_stays_within_bounds() {
        let value = random_number();
        assert!(value >= 0.0);
        assert!(value < 1.0);
    }

    #[test]
    fn sqrt_operates_on_value_and_number() {
        let v = sqrt(Value::Number(9.0));
        assert_eq!(v, Value::Number(3.0));

        let n = sqrt_number(16.0);
        assert_eq!(n, 4.0);
    }
}
