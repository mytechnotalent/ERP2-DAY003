/*
 * @file config.rs
 * @brief Application configuration constants
 * @author Kevin Thomas
 * @date 2025
 *
 * MIT License
 *
 * Copyright (c) 2025 Kevin Thomas
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! FILE: config.rs
//!
//! DESCRIPTION:
//! RP2350 Traffic Light Simulation Configuration Constants.
//!
//! BRIEF:
//! Defines configuration constants for traffic light timing.
//! Contains delay intervals and GPIO pin configuration.
//!
//! AUTHOR: Kevin Thomas
//! CREATION DATE: December 7, 2025
//! UPDATE DATE: December 7, 2025

/// Red light duration in milliseconds.
///
/// # Details
/// Duration the red light stays on before transitioning.
///
/// # Value
/// 3000 milliseconds (3 seconds)
#[allow(dead_code)]
pub const RED_DURATION_MS: u64 = 3000;

/// Yellow light duration in milliseconds.
///
/// # Details
/// Duration the yellow light stays on before transitioning.
///
/// # Value
/// 1000 milliseconds (1 second)
#[allow(dead_code)]
pub const YELLOW_DURATION_MS: u64 = 1000;

/// Green light duration in milliseconds.
///
/// # Details
/// Duration the green light stays on before transitioning.
///
/// # Value
/// 3000 milliseconds (3 seconds)
#[allow(dead_code)]
pub const GREEN_DURATION_MS: u64 = 3000;

/// Minimum allowed light duration in milliseconds.
///
/// # Details
/// Prevents excessively fast transitions which may cause issues.
///
/// # Value
/// 100 milliseconds
#[allow(dead_code)]
pub const MIN_DURATION_MS: u64 = 100;

/// Maximum allowed light duration in milliseconds.
///
/// # Details
/// Prevents excessively slow transitions for practical use.
///
/// # Value
/// 10000 milliseconds (10 seconds)
#[allow(dead_code)]
pub const MAX_DURATION_MS: u64 = 10000;

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== RED_DURATION_MS Tests ====================

    #[test]
    fn test_red_duration_value() {
        assert_eq!(RED_DURATION_MS, 3000);
    }

    #[test]
    fn test_red_duration_is_u64() {
        let _: u64 = RED_DURATION_MS;
    }

    #[test]
    fn test_red_duration_non_zero() {
        assert!(RED_DURATION_MS > 0);
    }

    #[test]
    fn test_red_duration_reasonable() {
        assert!(RED_DURATION_MS >= 1000);
    }

    // ==================== YELLOW_DURATION_MS Tests ====================

    #[test]
    fn test_yellow_duration_value() {
        assert_eq!(YELLOW_DURATION_MS, 1000);
    }

    #[test]
    fn test_yellow_duration_is_u64() {
        let _: u64 = YELLOW_DURATION_MS;
    }

    #[test]
    fn test_yellow_duration_non_zero() {
        assert!(YELLOW_DURATION_MS > 0);
    }

    #[test]
    fn test_yellow_duration_reasonable() {
        assert!(YELLOW_DURATION_MS >= 500);
    }

    // ==================== GREEN_DURATION_MS Tests ====================

    #[test]
    fn test_green_duration_value() {
        assert_eq!(GREEN_DURATION_MS, 3000);
    }

    #[test]
    fn test_green_duration_is_u64() {
        let _: u64 = GREEN_DURATION_MS;
    }

    #[test]
    fn test_green_duration_non_zero() {
        assert!(GREEN_DURATION_MS > 0);
    }

    #[test]
    fn test_green_duration_reasonable() {
        assert!(GREEN_DURATION_MS >= 1000);
    }

    // ==================== MIN_DURATION_MS Tests ====================

    #[test]
    fn test_min_duration_value() {
        assert_eq!(MIN_DURATION_MS, 100);
    }

    #[test]
    fn test_min_duration_is_u64() {
        let _: u64 = MIN_DURATION_MS;
    }

    #[test]
    fn test_min_duration_non_zero() {
        assert!(MIN_DURATION_MS > 0);
    }

    #[test]
    fn test_min_duration_less_than_red() {
        assert!(MIN_DURATION_MS < RED_DURATION_MS);
    }

    // ==================== MAX_DURATION_MS Tests ====================

    #[test]
    fn test_max_duration_value() {
        assert_eq!(MAX_DURATION_MS, 10000);
    }

    #[test]
    fn test_max_duration_is_u64() {
        let _: u64 = MAX_DURATION_MS;
    }

    #[test]
    fn test_max_duration_greater_than_red() {
        assert!(MAX_DURATION_MS > RED_DURATION_MS);
    }

    #[test]
    fn test_max_duration_is_10_seconds() {
        assert_eq!(MAX_DURATION_MS, 10 * 1000);
    }

    // ==================== Range Relationship Tests ====================

    #[test]
    fn test_duration_range_valid() {
        assert!(MIN_DURATION_MS < MAX_DURATION_MS);
    }

    #[test]
    fn test_red_within_range() {
        assert!(RED_DURATION_MS >= MIN_DURATION_MS);
        assert!(RED_DURATION_MS <= MAX_DURATION_MS);
    }

    #[test]
    fn test_yellow_within_range() {
        assert!(YELLOW_DURATION_MS >= MIN_DURATION_MS);
        assert!(YELLOW_DURATION_MS <= MAX_DURATION_MS);
    }

    #[test]
    fn test_green_within_range() {
        assert!(GREEN_DURATION_MS >= MIN_DURATION_MS);
        assert!(GREEN_DURATION_MS <= MAX_DURATION_MS);
    }

    // ==================== Arithmetic Safety Tests ====================

    #[test]
    fn test_no_overflow_on_total() {
        let total = RED_DURATION_MS + YELLOW_DURATION_MS + GREEN_DURATION_MS;
        assert_eq!(total, 7000);
    }

    #[test]
    fn test_no_overflow_max_doubled() {
        let doubled = MAX_DURATION_MS.checked_mul(2);
        assert!(doubled.is_some());
    }

    #[test]
    fn test_values_fit_in_u32() {
        assert!(RED_DURATION_MS <= u32::MAX as u64);
        assert!(YELLOW_DURATION_MS <= u32::MAX as u64);
        assert!(GREEN_DURATION_MS <= u32::MAX as u64);
    }

    // ==================== Constant Immutability Tests ====================

    #[test]
    fn test_constants_are_const() {
        const _A: u64 = RED_DURATION_MS;
        const _B: u64 = YELLOW_DURATION_MS;
        const _C: u64 = GREEN_DURATION_MS;
    }

    #[test]
    fn test_constants_usable_in_const_context() {
        const TOTAL: u64 = RED_DURATION_MS + YELLOW_DURATION_MS + GREEN_DURATION_MS;
        assert_eq!(TOTAL, 7000);
    }

    #[test]
    fn test_yellow_shortest_duration() {
        assert!(YELLOW_DURATION_MS <= RED_DURATION_MS);
        assert!(YELLOW_DURATION_MS <= GREEN_DURATION_MS);
    }

    #[test]
    fn test_red_equals_green() {
        assert_eq!(RED_DURATION_MS, GREEN_DURATION_MS);
    }
}
