extern crate alloc;

use embassy_time::Timer;
use esp_hal::gpio::Output;
use log::info;

/// Direction for stepper motor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Backward,
}

/// DRV2285 Stepper Motor Driver
pub struct DRV2285<'a> {
    // GPIO pins
    step_pin: Output<'a>,
    dir_pin: Output<'a>,

    // State tracking
    position: i32,
    current_direction: Direction,

    // Configuration
    step_pulse_duration_ns: u32,
}

impl<'a> DRV2285<'a> {
    /// Create a new DRV2285 driver instance
    ///
    /// # Arguments
    /// * `step_pin` - GPIO pin for step pulse
    /// * `dir_pin` - GPIO pin for direction
    pub fn new(step_pin: Output<'a>, dir_pin: Output<'a>) -> Self {
        info!("Initializing DRV2285 stepper motor driver");
        Self {
            step_pin,
            dir_pin,
            position: 0,
            current_direction: Direction::Forward,
            step_pulse_duration_ns: 10, // 10 microseconds default pulse width
        }
    }

    /// Set the step pulse duration (in nnaos)
    pub fn set_step_pulse_duration(&mut self, duration_ns: u32) {
        self.step_pulse_duration_ns = duration_ns;
        info!("Step pulse duration set to: {} us", duration_ns);
    }

    /// Set motor direction
    pub fn set_direction(&mut self, direction: Direction) {
        if direction != self.current_direction {
            self.current_direction = direction;
            match direction {
                Direction::Forward => self.dir_pin.set_high(),
                Direction::Backward => self.dir_pin.set_low(),
            };
            info!("Motor direction set to: {:?}", direction);
        }
    }

    /// Send a single step pulse
    /// Returns true if pulse was sent successfully
    pub async fn step(&mut self) -> bool {
        // Pull step pin high
        self.step_pin.set_high();

        // Wait for pulse duration
        Timer::after_nanos(self.step_pulse_duration_ns as u64).await;

        // Pull step pin low
        self.step_pin.set_low();

        // Update position based on direction
        match self.current_direction {
            Direction::Forward => self.position += 1,
            Direction::Backward => self.position -= 1,
        }

        true
    }

    /// Send multiple steps with a given interval between steps
    ///
    /// # Arguments
    /// * `num_steps` - Number of steps to send
    /// * `interval_ms` - Time between step pulses in milliseconds
    pub async fn step_multiple(&mut self, num_steps: u32, interval_ms: u32) {
        info!("Sending {} steps with {} ms interval", num_steps, interval_ms);

        for i in 0..num_steps {
            self.step().await;
            if i < num_steps - 1 {
                Timer::after_millis(interval_ms as u64).await;
            }
        }

        info!(
            "Step sequence complete. Current position: {}",
            self.position
        );
    }

    /// Move to a specific position
    /// Automatically calculates steps needed and direction
    pub async fn move_to(&mut self, target_position: i32) {
        let steps_needed = (target_position - self.position).abs();
        let direction = if target_position > self.position {
            Direction::Forward
        } else {
            Direction::Backward
        };

        info!(
            "Moving from {} to {} ({} steps)",
            self.position, target_position, steps_needed
        );

        self.set_direction(direction);
        self.step_multiple(steps_needed as u32, 10).await;
    }

    /// Jog (move a few steps) in a direction with specified interval
    pub async fn jog(&mut self, direction: Direction, num_steps: u32, interval_ms: u32) {
        self.set_direction(direction);
        self.step_multiple(num_steps, interval_ms).await;
    }

    /// Get current position
    pub fn position(&self) -> i32 {
        self.position
    }

    /// Reset position to zero
    pub fn reset_position(&mut self) {
        self.position = 0;
        info!("Position reset to 0");
    }

    /// Get current direction
    pub fn direction(&self) -> Direction {
        self.current_direction
    }

    /// Check if motor is at home position
    pub fn is_home(&self) -> bool {
        self.position == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_enum() {
        assert_ne!(Direction::Forward, Direction::Backward);
    }
}
