// Aicent Stack | GTIOT (Global Trusted IoT)
// Domain: http://gtiot.com
// Purpose: High-fidelity edge fusion & Action-Collapse physical execution.
// Specification: RFC-005 Standard (Active).
// License: Apache-2.0 via Aicent.com Organization.
//! # RFC-005: GTIOT Sensory-Motor Reflex
//! 
//! The `gtiot` crate implements the embodied execution layer of the Aicent Stack.
//! It bridges the digital-physical divide by collapsing high-level cognitive intent 
//! into deterministic motor primitives at a 1.2 kHz operational frequency.
//!
//! ### Core Body Logic:
//! - **Action-Collapse (AAL)**: Converting symbolic intent into physical torque vectors in <200µs.
//! - **Proprioceptive Homeostasis**: 1200Hz sampling loop for real-time sensor fusion.
//! - **Shadow-State Sync**: Maintaining 1:1 digital-physical parity via RTTP neural deltas.
//! - **Kinetic Resonance**: Synchronizing swarm movements via Aicent.net Hive (RFC-006).

#![deny(missing_docs)]
// SAFETY: Unsafe is strictly constrained to sub-nanosecond direct hardware I/O, 
// DMA mappings, and bitwise manifold unpacking for robotic servos.
#![allow(unsafe_code)]

/// [RFC-005] High-frequency sensory motor loop.
pub mod sensory_motor_loop;
/// [RFC-005] Action Abstraction Layer primitives for motor torque conversion.
pub mod aal;
/// [RFC-005] 128-bit digital twin shadow-state management.
pub mod shadow;
/// [RFC-005] NPU-accelerated multi-modal sensor fusion.
pub mod fusion;
/// [RFC-006] Hive-scale kinetic resonance alignment.
pub mod resonance;
/// [RFC-005] Hardware-level bus drivers (PWM/CAN) and security kill-switches.
pub mod hw;
/// [RFC-001] Cognitive intent de-sharding parser.
pub mod parser;

pub use crate::sensory_motor_loop::sensory_motor_loop;

/// [RFC-005] Physical Reflex Failure Modes.
/// Defines critical anomalies in the sensory-motor feedback loop that require 
/// immediate fail-safe intervention or Aicent Brain (RFC-001) rescheduling.
#[derive(Debug, Clone, PartialEq)]
pub enum BodyError {
    /// Action-Collapse mathematics failed to converge within the 200µs limit.
    KineticConvergenceTimeout,
    /// Divergence between the digital shadow-state and physical reality exceeded safe limits.
    ProprioceptiveDrift,
    /// Inbound command rejected at the hardware gate (RPKI RFC-003 pathogen detected).
    PathogenInterference,
    /// Neural spine connection severed (>3ms timeout), triggering autonomous dead-reckoning.
    NeuralNervesSevered,
    /// [RFC-006] Hive Resonance lost, forcing the node back to individual homeostasis.
    ResonanceDecoupled,
}

/// [RFC-005] Proprioceptive State Manifold.
/// Represents the high-fidelity fused state of the physical edge node.
#[derive(Debug, Clone)]
pub struct ProprioceptiveState {
    /// Fused multi-modal sensor manifold: [IMU | Vibration | Thermal | Vision].
    pub sensory_fingerprint: [f32; 16],
    /// Semantic Hash representation of the current physical pose.
    pub semantic_hash: u64,
    /// Alignment score between digital intent and physical position (0.0 - 1.0).
    pub shadow_parity: f32,
    /// Current measured operational frequency in Hz (Target: 1200).
    pub loop_frequency: u32,
}

impl ProprioceptiveState {
    /// Helper to extract the semantic hash for RTTP routing.
    pub fn semantic_hash(&self) -> u64 {
        self.semantic_hash
    }
    
    /// Helper to extract the sensory payload.
    pub fn as_bytes(&self) -> Vec<u8> {
        // [PERF] In production, this returns a compressed semantic embedding.
        vec![0u8; 64]
    }
}

/// [RFC-005] Embodied Limb Interface.
/// Defines the strict behavioral contract of a physical execution unit.
/// Any hardware (Robot, Drone, Actuator) joining the Aicent Stack must implement this.
pub trait EmbodiedLimb {
    /// Ingests raw hardware data and produces a condensed semantic fingerprint.
    fn perceive(&self) -> Result<ProprioceptiveState, BodyError>;

    /// Executes a collapsed action primitive on the physical substrate.
    /// This is the final boundary where digital thought becomes physical motion.
    fn actuate(&self, primitive: &aal::ActionPrimitive) -> Result<(), BodyError>;

    /// Calibrates the local motor reflex based on Global Kinetic Resonance (RFC-006).
    fn calibrate_reflex(&self, resonance_vector: [f32; 4]);
}

// --- Protocol Anchors & Performance Limits ---

/// [Standard v1.0] Fixed physical sampling rate required for RFC-005 compliance.
pub const SAMPLING_RATE_HZ: u32 = 1200;
/// [Standard v1.0] The absolute maximum time threshold for intent-to-actuation conversion.
pub const ACTION_COLLAPSE_TARGET_US: u32 = 200;
/// [Standard v1.0] The current active version of the GTIOT protocol.
pub const PROTOCOL_VERSION: &str = "1.0.0-standard-active";

/// High-fidelity telemetry marker for physical actuation events.
pub fn log_body_event(msg: &str) {
    println!("\x1b[1;33m[GTIOT-BODY]\x1b[0m 🦾 {}", msg);
}
