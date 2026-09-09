//! Python-compatible rounding. Python 3's `round()` is round-half-to-even
//! ("banker's rounding"); Rust's `f64::round()` is round-half-away-from-zero.
//! The two disagree exactly on ties (e.g. `round(62.5)`: Python gives `62`,
//! Rust's `.round()` gives `63`) — which shows up for real in this dataset
//! (`ArcticPine`'s week 2026-W13 load, 25h/40h capacity = exactly 62.5%).
//! Every place `pussla_engine.py`/`validate_planning_data.py` calls
//! Python's `round()` must go through here instead of `f64::round()`.

/// Round-half-to-even, matching Python 3's `round(x)` for non-negative `x`
/// (the only case this domain produces).
pub fn python_round(x: f64) -> f64 {
    let floor = x.floor();
    let frac = x - floor;
    if (frac - 0.5).abs() < 1e-9 {
        let floor_i = floor as i64;
        if floor_i % 2 == 0 {
            floor
        } else {
            floor + 1.0
        }
    } else {
        x.round()
    }
}

/// Matches Python's `round(x, 1)`.
pub fn round1(x: f64) -> f64 {
    python_round(x * 10.0) / 10.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounds_half_to_even() {
        assert_eq!(python_round(62.5), 62.0);
        assert_eq!(python_round(63.5), 64.0);
        assert_eq!(python_round(0.5), 0.0);
        assert_eq!(python_round(1.5), 2.0);
    }

    #[test]
    fn rounds_non_ties_normally() {
        assert_eq!(python_round(62.4), 62.0);
        assert_eq!(python_round(62.6), 63.0);
    }

    #[test]
    fn round1_matches_python_round_x_1() {
        // Verified against `python3 -c "print(round(147.45,1)); print(round(36.75,1))"`.
        assert_eq!(round1(147.45), 147.4);
        assert_eq!(round1(36.75), 36.8);
    }
}
