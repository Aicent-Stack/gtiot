// Aicent Stack | GTIOT (Global Trusted IoT)
// Domain: http://gtiot.com
// Purpose: 1.2 kHz high-frequency reflex & Action-Collapse logic.
// Specification: RFC-005 Draft.
// License: Apache-2.0 via Aicent.com Organization.
//! # RFC-005: GTIOT Sensory-Motor Reflex

pub mod sensory_motor_loop;

pub use sensory_motor_loop::*;

// TODO: Synchronize Action-Collapse logic with RFC-002 (RTTP) Pulse Frames.
