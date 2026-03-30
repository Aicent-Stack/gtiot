//! # RFC-005: GTIOT (Global Trusted IoT)
//! 
//! The embodied edge and physical reflex system for #AicentStack.
//! 
//! ## Core Specifications:
//! - Embodied execution primitives (Action-Collapse)
//! - Shadow state synchronization (Digital Twin)
//! - 1.2 kHz Perception-action loop (High-frequency Reflex)
//! 
//! Copyright 2026 Aicent.com Organization.
//! Licensed under the Apache-2.0 License.
//! Specification: RFC-005 Draft.

pub mod sensory_motor_loop;

pub use sensory_motor_loop::*;

// TODO: Synchronize Action-Collapse logic with RFC-002 (RTTP) Pulse Frames.
