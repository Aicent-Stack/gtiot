// Aicent Stack | GTIOT (Global Trusted IoT)
// Domain: http://gtiot.com
// Purpose: 1.2 kHz high-frequency reflex & Action-Collapse (AAL) logic.
// Specification: RFC-005 Standard (Active).
// License: Apache-2.0 via Aicent.com Organization.
//! # RFC-005: GTIOT Sensory-Motor Reflex Arc

use embassy_time::{Duration, Instant, Timer};
use rttp::PulseFrameHeader;
use crate::aal::ActionAbstractionLayer;
use crate::shadow::ShadowState;
use crate::fusion::SensorFusion;
use crate::resonance::KineticResonance; // [RFC-006] Swarm alignment

/// [RFC-005] Sensory-Motor Task Execution
/// This task manages the 1.2 kHz proprioceptive loop, collapsing digital intent 
/// from the Aicent Brain into deterministic physical actuation across the grid.
#[embassy_executor::task]
pub async fn sensory_motor_loop() {
    // --- Initialization of Embodied Primitives ---
    let mut sensor_fusion = SensorFusion::new();      // NPU-accelerated multi-modal fusion
    let mut shadow_state = ShadowState::default();    // Local high-fidelity digital twin
    let aal = ActionAbstractionLayer::new();          // Action Abstraction Layer engine
    let mut hive_sync = KineticResonance::new();     // [RFC-006] Hive alignment unit
    
    log_gtiot("Body Homeostasis Initialized. RFC-005 Standard Active.");

    let mut last_pulse_time = Instant::now();

    loop {
        // --- PHASE 1: SENSORY INGRESS (Perception) ---
        // [RFC-005] High-fidelity sampling (1200Hz).
        // Captures IMU, Vibration, and Thermodynamics into a 512-byte fingerprint.
        let raw_sensors = crate::hw::read_sensors();
        let fused_fingerprint = sensor_fusion.fuse(&raw_sensors);

        // --- PHASE 2: NEURAL EMISSION (Pulse Generation) ---
        // Packaging the sensory state with RPKI signatures and ZCMK bids.
        let mut header = PulseFrameHeader::new_for_gtiot(0x882, fused_fingerprint.semantic_hash());
        
        // Check if Hive Mode is enabled via Aicent.net (RFC-006)
        if hive_sync.is_active() {
            header.flags |= 0b1000; // Set Hive-Sync Multicast flag
        }

        let pulse = header.serialize(&fused_fingerprint.as_bytes());
        
        // Non-blocking asynchronous broadcast to the RTTP backbone
        if let Err(_) = rttp::publish(pulse).await {
            log_gtiot("RTTP Nerve Jitter detected. Entering predictive dead-reckoning.");
        }

        // --- PHASE 3: MOTOR REFLEX (Action-Collapse) ---
        // Listening for high-priority commands from the Aicent Brain (RFC-001)
        if let Some(cmd_frame) = rttp::next_command().await {
            let cmd_header = unsafe { &*(cmd_frame.as_ptr() as *const PulseFrameHeader) };
            let payload = &cmd_frame[64..];

            // 🛡️ [RFC-003] Parallel Immunity Scan
            // Hardware-level verification of intent and tensor watermark.
            if !rpki::parallel_immune_scan(cmd_header, payload).is_safe() {
                crate::hw::hardware_kill_switch(); // Atomic physical isolation
                log_gtiot("Pathogen detected in command stream! Actuators locked.");
                continue;
            }

            // 🩸 [RFC-004] Circulatory Settlement
            // Finalizing nanosecond resource clearing before physical execution.
            if zcmk::circulatory_pump(cmd_header, payload).is_none() {
                continue; // Unfunded intent rejected to maintain economic homeostasis.
            }

            // 🦾 [RFC-005] ACTION-COLLAPSE
            // Converting digital intent into low-level motor primitives in <200µs.
            let brain_intent = crate::parser::parse_intent(payload);
            let mut action_primitive = aal.collapse(brain_intent, &shadow_state);

            // 🟣 [RFC-006] Hive Kinetic Alignment
            // Adjusting local trajectory to maintain resonance with the collective swarm.
            if cmd_header.flags & 0b1000 != 0 {
                action_primitive = hive_sync.align_with_swarm(action_primitive);
            }

            // Direct physical drive (PWM/CAN/EtherCAT)
            crate::hw::execute_actuators(&action_primitive);
            last_pulse_time = Instant::now();

            // --- PHASE 4: PROPRIOCEPTIVE SYNC ---
            // Maintaining 1:1 parity between digital model and physical state.
            shadow_state.update(&action_primitive, &fused_fingerprint);
            shadow_state.predict_next_trajectories(); // Lookahead trajectory caching

            // Delta sync back to Brain for global reasoning alignment
            let shadow_delta = shadow_state.delta_since_last();
            rttp::publish_shadow_delta(shadow_delta).await;
        }

        // --- PHASE 5: FAIL-SAFE AUTONOMY ---
        // Autonomous trajectory fallback if RTTP heartbeat is lost (>3ms).
        if Instant::now() - last_pulse_time > Duration::from_millis(3) {
            let safe_action = shadow_state.get_safe_trajectory();
            crate::hw::execute_actuators(&safe_action);
        }

        // Maintaining the 1.2 kHz cycle (833µs interval)
        Timer::after_micros(833).await;
    }
}

fn log_gtiot(msg: &str) {
    println!("\x1b[1;33m[GTIOT-BODY]\x1b[0m {}", msg);
}
