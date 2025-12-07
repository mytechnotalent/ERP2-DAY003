/*
 * @file led.rs
 * @brief LED control utilities
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

//! FILE: led.rs
//!
//! DESCRIPTION:
//! LED Control Utilities for RP2350.
//!
//! BRIEF:
//! Provides LED state management and GPIO control helpers.
//! Contains testable logic for LED state operations.
//!
//! AUTHOR: Kevin Thomas
//! CREATION DATE: December 7, 2025
//! UPDATE DATE: December 7, 2025

#[cfg(feature = "embassy-rp")]
use embassy_rp::gpio::Output;

/// LED state enumeration.
///
/// # Details
/// Represents the current state of an individual LED.
/// Used for state tracking and GPIO control.
///
/// # Variants
/// * `On` - LED is currently on (high)
/// * `Off` - LED is currently off (low)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum LedState {
    On,
    Off,
}

/// Converts boolean to LedState.
///
/// # Details
/// Maps true to On, false to Off.
///
/// # Arguments
/// * `state` - Boolean state to convert
///
/// # Returns
/// * `LedState` - On if true, Off if false
#[allow(dead_code)]
pub fn bool_to_led_state(state: bool) -> LedState {
    if state { LedState::On } else { LedState::Off }
}

/// Converts LedState to boolean for GPIO control.
///
/// # Details
/// Maps On state to true (high), Off state to false (low).
///
/// # Arguments
/// * `state` - LED state to convert
///
/// # Returns
/// * `bool` - true for On, false for Off
#[allow(dead_code)]
pub fn led_state_to_bool(state: LedState) -> bool {
    matches!(state, LedState::On)
}

/// Determines LED output level from boolean state.
///
/// # Details
/// Helper function to determine GPIO level from state.
/// Returns true for high (on), false for low (off).
///
/// # Arguments
/// * `state` - Boolean state (true = on, false = off)
///
/// # Returns
/// * `bool` - GPIO level (true = high, false = low)
#[allow(dead_code)]
pub fn get_led_level(state: bool) -> bool {
    state
}

/// Inverts LED state.
///
/// # Details
/// Toggles LED state from On to Off or Off to On.
///
/// # Arguments
/// * `state` - Current LED state
///
/// # Returns
/// * `LedState` - Inverted state
#[allow(dead_code)]
pub fn invert_led_state(state: LedState) -> LedState {
    match state {
        LedState::On => LedState::Off,
        LedState::Off => LedState::On,
    }
}

/// Inverts boolean LED state.
///
/// # Details
/// Toggles boolean state from true to false or false to true.
///
/// # Arguments
/// * `state` - Current boolean state
///
/// # Returns
/// * `bool` - Inverted state
#[allow(dead_code)]
pub fn invert_bool_state(state: bool) -> bool {
    !state
}

/// Sets LED GPIO output based on boolean state.
///
/// # Details
/// Helper function to set LED high or low based on state.
///
/// # Arguments
/// * `led` - Mutable reference to GPIO output pin.
/// * `state` - Boolean state (true = on, false = off).
#[cfg(feature = "embassy-rp")]
#[allow(dead_code)]
pub fn set_led(led: &mut Output<'_>, state: bool) {
    if get_led_level(state) {
        led.set_high();
    } else {
        led.set_low();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== LedState Enum Tests ====================

    #[test]
    fn test_led_state_on_exists() {
        let _state = LedState::On;
    }

    #[test]
    fn test_led_state_off_exists() {
        let _state = LedState::Off;
    }

    #[test]
    fn test_led_state_equality_on() {
        assert_eq!(LedState::On, LedState::On);
    }

    #[test]
    fn test_led_state_equality_off() {
        assert_eq!(LedState::Off, LedState::Off);
    }

    #[test]
    fn test_led_state_inequality() {
        assert_ne!(LedState::On, LedState::Off);
    }

    #[test]
    fn test_led_state_inequality_reverse() {
        assert_ne!(LedState::Off, LedState::On);
    }

    #[test]
    fn test_led_state_copy() {
        let state = LedState::On;
        let copy = state;
        assert_eq!(state, copy);
    }

    #[test]
    fn test_led_state_clone() {
        let state = LedState::Off;
        #[allow(clippy::clone_on_copy)]
        let cloned = state.clone();
        assert_eq!(state, cloned);
    }

    #[test]
    fn test_led_state_debug_on() {
        let debug_str = format!("{:?}", LedState::On);
        assert_eq!(debug_str, "On");
    }

    #[test]
    fn test_led_state_debug_off() {
        let debug_str = format!("{:?}", LedState::Off);
        assert_eq!(debug_str, "Off");
    }

    #[test]
    fn test_led_state_size() {
        assert_eq!(core::mem::size_of::<LedState>(), 1);
    }

    #[test]
    fn test_led_state_alignment() {
        assert_eq!(core::mem::align_of::<LedState>(), 1);
    }

    // ==================== bool_to_led_state Function Tests ====================

    #[test]
    fn test_bool_to_led_state_true() {
        assert_eq!(bool_to_led_state(true), LedState::On);
    }

    #[test]
    fn test_bool_to_led_state_false() {
        assert_eq!(bool_to_led_state(false), LedState::Off);
    }

    #[test]
    fn test_bool_to_led_state_consistent_true() {
        for _ in 0..10 {
            assert_eq!(bool_to_led_state(true), LedState::On);
        }
    }

    #[test]
    fn test_bool_to_led_state_consistent_false() {
        for _ in 0..10 {
            assert_eq!(bool_to_led_state(false), LedState::Off);
        }
    }

    // ==================== led_state_to_bool Function Tests ====================

    #[test]
    fn test_led_state_to_bool_on() {
        assert!(led_state_to_bool(LedState::On));
    }

    #[test]
    fn test_led_state_to_bool_off() {
        assert!(!led_state_to_bool(LedState::Off));
    }

    #[test]
    fn test_led_state_to_bool_on_returns_true() {
        assert_eq!(led_state_to_bool(LedState::On), true);
    }

    #[test]
    fn test_led_state_to_bool_off_returns_false() {
        assert_eq!(led_state_to_bool(LedState::Off), false);
    }

    #[test]
    fn test_led_state_to_bool_consistent() {
        for _ in 0..10 {
            assert!(led_state_to_bool(LedState::On));
            assert!(!led_state_to_bool(LedState::Off));
        }
    }

    // ==================== get_led_level Function Tests ====================

    #[test]
    fn test_get_led_level_true() {
        assert!(get_led_level(true));
    }

    #[test]
    fn test_get_led_level_false() {
        assert!(!get_led_level(false));
    }

    #[test]
    fn test_get_led_level_returns_input_true() {
        assert_eq!(get_led_level(true), true);
    }

    #[test]
    fn test_get_led_level_returns_input_false() {
        assert_eq!(get_led_level(false), false);
    }

    #[test]
    fn test_get_led_level_identity() {
        let states = [true, false, true, true, false];
        for state in states {
            assert_eq!(get_led_level(state), state);
        }
    }

    // ==================== invert_led_state Function Tests ====================

    #[test]
    fn test_invert_led_state_on_to_off() {
        assert_eq!(invert_led_state(LedState::On), LedState::Off);
    }

    #[test]
    fn test_invert_led_state_off_to_on() {
        assert_eq!(invert_led_state(LedState::Off), LedState::On);
    }

    #[test]
    fn test_invert_led_state_double_invert_on() {
        let state = LedState::On;
        assert_eq!(invert_led_state(invert_led_state(state)), state);
    }

    #[test]
    fn test_invert_led_state_double_invert_off() {
        let state = LedState::Off;
        assert_eq!(invert_led_state(invert_led_state(state)), state);
    }

    #[test]
    fn test_invert_led_state_not_equal_original() {
        assert_ne!(invert_led_state(LedState::On), LedState::On);
        assert_ne!(invert_led_state(LedState::Off), LedState::Off);
    }

    // ==================== invert_bool_state Function Tests ====================

    #[test]
    fn test_invert_bool_state_true_to_false() {
        assert_eq!(invert_bool_state(true), false);
    }

    #[test]
    fn test_invert_bool_state_false_to_true() {
        assert_eq!(invert_bool_state(false), true);
    }

    #[test]
    fn test_invert_bool_state_double_invert_true() {
        assert_eq!(invert_bool_state(invert_bool_state(true)), true);
    }

    #[test]
    fn test_invert_bool_state_double_invert_false() {
        assert_eq!(invert_bool_state(invert_bool_state(false)), false);
    }

    #[test]
    fn test_invert_bool_state_not_equal_original() {
        assert_ne!(invert_bool_state(true), true);
        assert_ne!(invert_bool_state(false), false);
    }

    // ==================== Roundtrip Conversion Tests ====================

    #[test]
    fn test_roundtrip_bool_to_led_to_bool_true() {
        assert_eq!(led_state_to_bool(bool_to_led_state(true)), true);
    }

    #[test]
    fn test_roundtrip_bool_to_led_to_bool_false() {
        assert_eq!(led_state_to_bool(bool_to_led_state(false)), false);
    }

    #[test]
    fn test_roundtrip_led_to_bool_to_led_on() {
        assert_eq!(
            bool_to_led_state(led_state_to_bool(LedState::On)),
            LedState::On
        );
    }

    #[test]
    fn test_roundtrip_led_to_bool_to_led_off() {
        assert_eq!(
            bool_to_led_state(led_state_to_bool(LedState::Off)),
            LedState::Off
        );
    }

    // ==================== Invert Consistency Tests ====================

    #[test]
    fn test_invert_bool_matches_led_invert_on() {
        let bool_state = true;
        let led_state = bool_to_led_state(bool_state);
        let inverted_led = invert_led_state(led_state);
        assert_eq!(
            led_state_to_bool(inverted_led),
            invert_bool_state(bool_state)
        );
    }

    #[test]
    fn test_invert_bool_matches_led_invert_off() {
        let bool_state = false;
        let led_state = bool_to_led_state(bool_state);
        let inverted_led = invert_led_state(led_state);
        assert_eq!(
            led_state_to_bool(inverted_led),
            invert_bool_state(bool_state)
        );
    }

    // ==================== Edge Case Tests ====================

    #[test]
    fn test_led_state_in_option_some() {
        let maybe_state: Option<LedState> = Some(LedState::On);
        assert!(maybe_state.is_some());
    }

    #[test]
    fn test_led_state_in_option_none() {
        let maybe_state: Option<LedState> = None;
        assert!(maybe_state.is_none());
    }

    #[test]
    fn test_led_state_in_result_ok() {
        let result: Result<LedState, ()> = Ok(LedState::On);
        assert!(result.is_ok());
    }

    #[test]
    fn test_led_state_in_result_err() {
        let result: Result<LedState, ()> = Err(());
        assert!(result.is_err());
    }

    #[test]
    fn test_led_state_in_vec() {
        let states = vec![LedState::On, LedState::Off, LedState::On];
        assert_eq!(states.len(), 3);
    }

    #[test]
    fn test_led_state_array() {
        let states: [LedState; 4] = [LedState::On, LedState::Off, LedState::On, LedState::Off];
        assert_eq!(states[0], LedState::On);
        assert_eq!(states[1], LedState::Off);
    }

    // ==================== Trait Implementation Tests ====================

    #[test]
    fn test_led_state_partial_eq() {
        assert!(LedState::On == LedState::On);
        assert!(LedState::Off == LedState::Off);
    }

    #[test]
    fn test_led_state_eq_reflexive() {
        let state = LedState::On;
        assert_eq!(state, state);
    }

    #[test]
    fn test_led_state_eq_symmetric() {
        let state1 = LedState::On;
        let state2 = LedState::On;
        assert_eq!(state1, state2);
        assert_eq!(state2, state1);
    }
}
