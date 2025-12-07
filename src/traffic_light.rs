/*
 * @file traffic_light.rs
 * @brief Traffic light state management
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

//! FILE: traffic_light.rs
//!
//! DESCRIPTION:
//! Traffic Light State Management for RP2350.
//!
//! BRIEF:
//! Provides traffic light controller for Red, Yellow, Green simulation.
//! Manages state transitions with configurable timing.
//!
//! AUTHOR: Kevin Thomas
//! CREATION DATE: December 7, 2025
//! UPDATE DATE: December 7, 2025

use crate::config::{GREEN_DURATION_MS, RED_DURATION_MS, YELLOW_DURATION_MS};

/// Traffic light state enumeration.
///
/// # Details
/// Represents the current state of the traffic light.
/// Follows standard traffic light sequence.
///
/// # Variants
/// * `Red` - Stop signal (red LED on)
/// * `Yellow` - Caution signal (yellow LED on)
/// * `Green` - Go signal (green LED on)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TrafficLightState {
    Red,
    Yellow,
    Green,
}

/// Traffic light controller with state tracking.
///
/// # Details
/// Maintains traffic light state and timing configuration.
/// Provides methods for advancing through light sequence.
///
/// # Fields
/// * `current_state` - Current traffic light state
/// * `red_duration` - Duration for red light in milliseconds
/// * `yellow_duration` - Duration for yellow light in milliseconds
/// * `green_duration` - Duration for green light in milliseconds
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub struct TrafficLightController {
    current_state: TrafficLightState,
    red_duration: u64,
    yellow_duration: u64,
    green_duration: u64,
}

impl Default for TrafficLightController {
    /// Returns default TrafficLightController instance.
    ///
    /// # Details
    /// Delegates to new() for initialization.
    ///
    /// # Returns
    /// * `Self` - New TrafficLightController with default values
    #[allow(dead_code)]
    fn default() -> Self {
        Self::new()
    }
}

impl TrafficLightController {
    /// Creates new traffic light controller with default settings.
    ///
    /// # Details
    /// Initializes controller starting at Red state.
    ///
    /// # Returns
    /// * `Self` - New TrafficLightController instance
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            current_state: TrafficLightState::Red,
            red_duration: RED_DURATION_MS,
            yellow_duration: YELLOW_DURATION_MS,
            green_duration: GREEN_DURATION_MS,
        }
    }

    /// Advances to next state in sequence and returns new state.
    ///
    /// # Details
    /// Transitions: Red -> Green -> Yellow -> Red.
    /// Implements standard traffic light behavior.
    ///
    /// # Returns
    /// * `TrafficLightState` - New state after advancement
    #[allow(dead_code)]
    pub fn advance(&mut self) -> TrafficLightState {
        self.current_state = match self.current_state {
            TrafficLightState::Red => TrafficLightState::Green,
            TrafficLightState::Green => TrafficLightState::Yellow,
            TrafficLightState::Yellow => TrafficLightState::Red,
        };
        self.current_state
    }

    /// Returns current traffic light state.
    ///
    /// # Details
    /// State of the traffic light.
    ///
    /// # Returns
    /// * `TrafficLightState` - Current state
    #[allow(dead_code)]
    pub fn current_state(&self) -> TrafficLightState {
        self.current_state
    }

    /// Returns duration for current state in milliseconds.
    ///
    /// # Details
    /// Returns timing based on current state.
    ///
    /// # Returns
    /// * `u64` - Duration in milliseconds
    #[allow(dead_code)]
    pub fn current_duration(&self) -> u64 {
        match self.current_state {
            TrafficLightState::Red => self.red_duration,
            TrafficLightState::Yellow => self.yellow_duration,
            TrafficLightState::Green => self.green_duration,
        }
    }

    /// Returns red light duration.
    ///
    /// # Details
    /// Duration for red state in milliseconds.
    ///
    /// # Returns
    /// * `u64` - Red duration in milliseconds
    #[allow(dead_code)]
    pub fn red_duration(&self) -> u64 {
        self.red_duration
    }

    /// Returns yellow light duration.
    ///
    /// # Details
    /// Duration for yellow state in milliseconds.
    ///
    /// # Returns
    /// * `u64` - Yellow duration in milliseconds
    #[allow(dead_code)]
    pub fn yellow_duration(&self) -> u64 {
        self.yellow_duration
    }

    /// Returns green light duration.
    ///
    /// # Details
    /// Duration for green state in milliseconds.
    ///
    /// # Returns
    /// * `u64` - Green duration in milliseconds
    #[allow(dead_code)]
    pub fn green_duration(&self) -> u64 {
        self.green_duration
    }

    /// Returns true if red light should be on.
    ///
    /// # Details
    /// Checks if current state is Red.
    ///
    /// # Returns
    /// * `bool` - true if red, false otherwise
    #[allow(dead_code)]
    pub fn is_red(&self) -> bool {
        self.current_state == TrafficLightState::Red
    }

    /// Returns true if yellow light should be on.
    ///
    /// # Details
    /// Checks if current state is Yellow.
    ///
    /// # Returns
    /// * `bool` - true if yellow, false otherwise
    #[allow(dead_code)]
    pub fn is_yellow(&self) -> bool {
        self.current_state == TrafficLightState::Yellow
    }

    /// Returns true if green light should be on.
    ///
    /// # Details
    /// Checks if current state is Green.
    ///
    /// # Returns
    /// * `bool` - true if green, false otherwise
    #[allow(dead_code)]
    pub fn is_green(&self) -> bool {
        self.current_state == TrafficLightState::Green
    }
}

/// Converts TrafficLightState to boolean for GPIO control.
///
/// # Details
/// Maps specified state to true if current, false otherwise.
///
/// # Arguments
/// * `current` - Current traffic light state
/// * `target` - Target state to check
///
/// # Returns
/// * `bool` - true if current matches target
#[allow(dead_code)]
pub fn state_to_level(current: TrafficLightState, target: TrafficLightState) -> bool {
    current == target
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== TrafficLightState Enum Tests ====================

    #[test]
    fn test_state_red_exists() {
        let _state = TrafficLightState::Red;
    }

    #[test]
    fn test_state_yellow_exists() {
        let _state = TrafficLightState::Yellow;
    }

    #[test]
    fn test_state_green_exists() {
        let _state = TrafficLightState::Green;
    }

    #[test]
    fn test_state_equality_red() {
        assert_eq!(TrafficLightState::Red, TrafficLightState::Red);
    }

    #[test]
    fn test_state_equality_yellow() {
        assert_eq!(TrafficLightState::Yellow, TrafficLightState::Yellow);
    }

    #[test]
    fn test_state_equality_green() {
        assert_eq!(TrafficLightState::Green, TrafficLightState::Green);
    }

    #[test]
    fn test_state_inequality_red_yellow() {
        assert_ne!(TrafficLightState::Red, TrafficLightState::Yellow);
    }

    #[test]
    fn test_state_inequality_red_green() {
        assert_ne!(TrafficLightState::Red, TrafficLightState::Green);
    }

    #[test]
    fn test_state_inequality_yellow_green() {
        assert_ne!(TrafficLightState::Yellow, TrafficLightState::Green);
    }

    #[test]
    fn test_state_copy() {
        let state = TrafficLightState::Red;
        let copy = state;
        assert_eq!(state, copy);
    }

    #[test]
    fn test_state_clone() {
        let state = TrafficLightState::Green;
        #[allow(clippy::clone_on_copy)]
        let cloned = state.clone();
        assert_eq!(state, cloned);
    }

    #[test]
    fn test_state_debug_red() {
        let debug_str = format!("{:?}", TrafficLightState::Red);
        assert_eq!(debug_str, "Red");
    }

    #[test]
    fn test_state_debug_yellow() {
        let debug_str = format!("{:?}", TrafficLightState::Yellow);
        assert_eq!(debug_str, "Yellow");
    }

    #[test]
    fn test_state_debug_green() {
        let debug_str = format!("{:?}", TrafficLightState::Green);
        assert_eq!(debug_str, "Green");
    }

    #[test]
    fn test_state_size() {
        assert_eq!(core::mem::size_of::<TrafficLightState>(), 1);
    }

    // ==================== state_to_level Function Tests ====================

    #[test]
    fn test_state_to_level_red_match() {
        assert!(state_to_level(
            TrafficLightState::Red,
            TrafficLightState::Red
        ));
    }

    #[test]
    fn test_state_to_level_yellow_match() {
        assert!(state_to_level(
            TrafficLightState::Yellow,
            TrafficLightState::Yellow
        ));
    }

    #[test]
    fn test_state_to_level_green_match() {
        assert!(state_to_level(
            TrafficLightState::Green,
            TrafficLightState::Green
        ));
    }

    #[test]
    fn test_state_to_level_red_no_match() {
        assert!(!state_to_level(
            TrafficLightState::Red,
            TrafficLightState::Green
        ));
    }

    #[test]
    fn test_state_to_level_yellow_no_match() {
        assert!(!state_to_level(
            TrafficLightState::Yellow,
            TrafficLightState::Red
        ));
    }

    #[test]
    fn test_state_to_level_green_no_match() {
        assert!(!state_to_level(
            TrafficLightState::Green,
            TrafficLightState::Yellow
        ));
    }

    // ==================== TrafficLightController::new() Tests ====================

    #[test]
    fn test_new_controller() {
        let ctrl = TrafficLightController::new();
        assert_eq!(ctrl.red_duration(), RED_DURATION_MS);
    }

    #[test]
    fn test_new_controller_starts_at_red() {
        let ctrl = TrafficLightController::new();
        assert_eq!(ctrl.current_state(), TrafficLightState::Red);
    }

    #[test]
    fn test_new_controller_yellow_duration() {
        let ctrl = TrafficLightController::new();
        assert_eq!(ctrl.yellow_duration(), YELLOW_DURATION_MS);
    }

    #[test]
    fn test_new_controller_green_duration() {
        let ctrl = TrafficLightController::new();
        assert_eq!(ctrl.green_duration(), GREEN_DURATION_MS);
    }

    #[test]
    fn test_new_returns_consistent_value() {
        let ctrl1 = TrafficLightController::new();
        let ctrl2 = TrafficLightController::new();
        assert_eq!(ctrl1, ctrl2);
    }

    // ==================== TrafficLightController::default() Tests ====================

    #[test]
    fn test_default_equals_new() {
        let default = TrafficLightController::default();
        let new = TrafficLightController::new();
        assert_eq!(default, new);
    }

    #[test]
    fn test_default_starts_at_red() {
        let default = TrafficLightController::default();
        assert_eq!(default.current_state(), TrafficLightState::Red);
    }

    // ==================== TrafficLightController::advance() Tests ====================

    #[test]
    fn test_advance_from_red() {
        let mut ctrl = TrafficLightController::new();
        assert_eq!(ctrl.advance(), TrafficLightState::Green);
    }

    #[test]
    fn test_advance_from_green() {
        let mut ctrl = TrafficLightController::new();
        ctrl.advance();
        assert_eq!(ctrl.advance(), TrafficLightState::Yellow);
    }

    #[test]
    fn test_advance_from_yellow() {
        let mut ctrl = TrafficLightController::new();
        ctrl.advance();
        ctrl.advance();
        assert_eq!(ctrl.advance(), TrafficLightState::Red);
    }

    #[test]
    fn test_advance_returns_new_state() {
        let mut ctrl = TrafficLightController::new();
        let new_state = ctrl.advance();
        assert_eq!(new_state, ctrl.current_state());
    }

    #[test]
    fn test_advance_full_cycle() {
        let mut ctrl = TrafficLightController::new();
        ctrl.advance();
        ctrl.advance();
        ctrl.advance();
        assert_eq!(ctrl.current_state(), TrafficLightState::Red);
    }

    #[test]
    fn test_advance_multiple_cycles() {
        let mut ctrl = TrafficLightController::new();
        for _ in 0..9 {
            ctrl.advance();
        }
        assert_eq!(ctrl.current_state(), TrafficLightState::Red);
    }

    // ==================== TrafficLightController::current_duration() Tests ====================

    #[test]
    fn test_current_duration_red() {
        let ctrl = TrafficLightController::new();
        assert_eq!(ctrl.current_duration(), RED_DURATION_MS);
    }

    #[test]
    fn test_current_duration_green() {
        let mut ctrl = TrafficLightController::new();
        ctrl.advance();
        assert_eq!(ctrl.current_duration(), GREEN_DURATION_MS);
    }

    #[test]
    fn test_current_duration_yellow() {
        let mut ctrl = TrafficLightController::new();
        ctrl.advance();
        ctrl.advance();
        assert_eq!(ctrl.current_duration(), YELLOW_DURATION_MS);
    }

    // ==================== TrafficLightController::is_red() Tests ====================

    #[test]
    fn test_is_red_initial() {
        let ctrl = TrafficLightController::new();
        assert!(ctrl.is_red());
    }

    #[test]
    fn test_is_red_after_advance() {
        let mut ctrl = TrafficLightController::new();
        ctrl.advance();
        assert!(!ctrl.is_red());
    }

    // ==================== TrafficLightController::is_yellow() Tests ====================

    #[test]
    fn test_is_yellow_initial() {
        let ctrl = TrafficLightController::new();
        assert!(!ctrl.is_yellow());
    }

    #[test]
    fn test_is_yellow_after_two_advances() {
        let mut ctrl = TrafficLightController::new();
        ctrl.advance();
        ctrl.advance();
        assert!(ctrl.is_yellow());
    }

    // ==================== TrafficLightController::is_green() Tests ====================

    #[test]
    fn test_is_green_initial() {
        let ctrl = TrafficLightController::new();
        assert!(!ctrl.is_green());
    }

    #[test]
    fn test_is_green_after_advance() {
        let mut ctrl = TrafficLightController::new();
        ctrl.advance();
        assert!(ctrl.is_green());
    }

    // ==================== State Exclusivity Tests ====================

    #[test]
    fn test_only_one_state_active_red() {
        let ctrl = TrafficLightController::new();
        assert!(ctrl.is_red() && !ctrl.is_yellow() && !ctrl.is_green());
    }

    #[test]
    fn test_only_one_state_active_green() {
        let mut ctrl = TrafficLightController::new();
        ctrl.advance();
        assert!(!ctrl.is_red() && !ctrl.is_yellow() && ctrl.is_green());
    }

    #[test]
    fn test_only_one_state_active_yellow() {
        let mut ctrl = TrafficLightController::new();
        ctrl.advance();
        ctrl.advance();
        assert!(!ctrl.is_red() && ctrl.is_yellow() && !ctrl.is_green());
    }

    // ==================== Trait Implementation Tests ====================

    #[test]
    fn test_controller_clone() {
        let ctrl1 = TrafficLightController::new();
        let ctrl2 = ctrl1;
        assert_eq!(ctrl1.red_duration(), ctrl2.red_duration());
    }

    #[test]
    fn test_controller_copy() {
        let ctrl1 = TrafficLightController::new();
        let ctrl2 = ctrl1;
        assert_eq!(ctrl1, ctrl2);
    }

    #[test]
    fn test_controller_partial_eq() {
        let ctrl1 = TrafficLightController::new();
        let ctrl2 = TrafficLightController::new();
        assert_eq!(ctrl1, ctrl2);
    }

    #[test]
    fn test_controller_debug() {
        let ctrl = TrafficLightController::new();
        let debug_str = format!("{:?}", ctrl);
        assert!(debug_str.contains("TrafficLightController"));
    }

    #[test]
    fn test_controller_debug_contains_state() {
        let ctrl = TrafficLightController::new();
        let debug_str = format!("{:?}", ctrl);
        assert!(debug_str.contains("Red"));
    }

    // ==================== State Transition Tests ====================

    #[test]
    fn test_sequence_cycle_red_to_green() {
        let mut ctrl = TrafficLightController::new();
        assert_eq!(ctrl.current_state(), TrafficLightState::Red);
        ctrl.advance();
        assert_eq!(ctrl.current_state(), TrafficLightState::Green);
    }

    #[test]
    fn test_sequence_cycle_green_to_yellow() {
        let mut ctrl = TrafficLightController::new();
        ctrl.advance();
        ctrl.advance();
        assert_eq!(ctrl.current_state(), TrafficLightState::Yellow);
    }

    #[test]
    fn test_sequence_cycle_yellow_to_red() {
        let mut ctrl = TrafficLightController::new();
        ctrl.advance();
        ctrl.advance();
        ctrl.advance();
        assert_eq!(ctrl.current_state(), TrafficLightState::Red);
    }

    #[test]
    fn test_independent_controllers() {
        let mut ctrl1 = TrafficLightController::new();
        let ctrl2 = TrafficLightController::new();
        ctrl1.advance();
        assert_eq!(ctrl2.current_state(), TrafficLightState::Red);
        assert_eq!(ctrl1.current_state(), TrafficLightState::Green);
    }

    // ==================== Edge Case Tests ====================

    #[test]
    fn test_many_controllers() {
        let controllers: Vec<TrafficLightController> =
            (0..100).map(|_| TrafficLightController::new()).collect();
        for ctrl in controllers {
            assert_eq!(ctrl.red_duration(), RED_DURATION_MS);
        }
    }

    #[test]
    fn test_controller_in_option() {
        let maybe_ctrl: Option<TrafficLightController> = Some(TrafficLightController::new());
        assert!(maybe_ctrl.is_some());
        assert_eq!(maybe_ctrl.unwrap().red_duration(), RED_DURATION_MS);
    }

    #[test]
    fn test_controller_in_result() {
        let result: Result<TrafficLightController, ()> = Ok(TrafficLightController::new());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().red_duration(), RED_DURATION_MS);
    }

    #[test]
    fn test_advance_100_times() {
        let mut ctrl = TrafficLightController::new();
        for _ in 0..99 {
            ctrl.advance();
        }
        assert_eq!(ctrl.current_state(), TrafficLightState::Red);
    }

    #[test]
    fn test_controller_size() {
        assert!(core::mem::size_of::<TrafficLightController>() <= 32);
    }

    #[test]
    fn test_controller_alignment() {
        assert!(core::mem::align_of::<TrafficLightController>() <= 8);
    }
}
