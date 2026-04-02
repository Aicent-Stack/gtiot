// Aicent Stack | GTIOT (Global Trusted IoT)
// Domain: http://gtiot.com
// Purpose: 1.2 kHz high-frequency reflex & Action-Collapse (AAL) logic.
// Specification: RFC-005 (Body) with RFC-003/004 Integrated Verification.
// License: Apache-2.0 via Aicent.com Organization.
//! # RFC-005: GTIOT Sensory-Motor Reflex Arc

use embassy_time::{Duration, Instant, Timer};
use rttp::PulseFrameHeader;
use crate::aal::ActionAbstractionLayer;
use crate::shadow::ShadowState;
use crate::fusion::SensorFusion;

/// [RFC-005] Sensory-Motor Task Execution
/// This task manages the 1.2 kHz proprioceptive loop, collapsing digital intent 
/// from the Aicent Brain into deterministic physical actuation.
#[embassy_executor::task]
pub async fn sensory_motor_loop() {
    // Initialization of the "Embodied Primitives"
    let mut sensor_fusion = SensorFusion::new();      // Kalman-filter + NPU-accelerated fusion
    let mut shadow_state = ShadowState::default();    // Local predictive digital twin
    let aal = ActionAbstractionLayer::new();          // Action Abstraction Layer engine
    
    log_gtiot("Body Homeostasis Initialized. RFC-005 active.");

    let mut last_pulse_time = Instant::now();

    loop {
        // --- PHASE 1: SENSORY INGRESS (Perception) ---
        // High-fidelity fusion of multi-modal sensors (IMU, Vibration, Vision)
        let raw_data = crate::hw::read_sensors();
        let fused_fingerprint = sensor_fusion.fuse(&raw_data); // 512-byte semantic fingerprint

        // --- PHASE 2: NEURAL EMISSION (Nerves) ---
        // Package the sensory pulse with RPKI watermark (RFC-003) and ZCMK bid (RFC-004)
        let header = PulseFrameHeader::new_for_gtiot(0x882, fused_fingerprint.semantic_hash());
        let pulse = header.serialize(&fused_fingerprint.as_bytes());
        
        // Asynchronous non-blocking broadcast to the RTTP spine
        if let Err(_) = rttp::publish(pulse).await {
            log_gtiot("RTTP Nerve Jitter detected. Entering autonomous dead-reckoning.");
        }

        // --- PHASE 3: MOTOR REFLEX (Action-Collapse) ---
        // Listen for incoming commands from the Aicent Brain (RFC-001)
        if let Some(cmd_frame) = rttp::next_command().await {
            // Safety Audit: Immediate zero-copy header mapping
            let cmd_header = unsafe { &*(cmd_frame.as_ptr() as *const PulseFrameHeader) };
            let payload = &cmd_frame[64..];

            // 🛡️ [RFC-003] Parallel Immunity Scan
            // Verify identity and tensor watermark at the hardware gate
            if !rpki::parallel_immune_scan(cmd_header, payload).is_safe() {
                crate::hw::hardware_kill_switch(); // Physical isolation (Immune Reflex)
                log_gtiot("Pathogen detected! Hardware locked for protection.");
                continue;
            }

            // 🩸 [RFC-004] Circulatory Settlement
            // Ensure the command is "Paid" via ZCMK micro-settlement
            if zcmk::circulatory_pump(cmd_header, payload).is_none() {
                continue; // Unfunded intent: ignore to prevent resource exhaustion
            }

            // 🦾 [RFC-005] ACTION-COLLAPSE
            // Collapsing high-level cognitive intent into low-level physical actuation
            let intent = crate::parser::parse_brain_intent(payload);
            let action_primitive = aal.collapse(intent, &shadow_state);

            // Execute on the physical substrate (PWM, CAN, or Robotic Bus)
            crate::hw::execute_actuators(&action_primitive);
            last_pulse_time = Instant::now();

            // --- PHASE 4: PROPRIOCEPTIVE SYNC (Homeostasis) ---
            // Update local shadow twin and predict next 5 trajectories
            shadow_state.update(&action_primitive, &fused_fingerprint);
            shadow_state.predict_trajectories();

            // Sync the shadow-state delta back to the Brain via RTTP
            let shadow_delta = shadow_state.delta_since_last();
            rttp::publish_shadow_delta(shadow_delta).await;
        }

        // --- PHASE 5: FAIL-SAFE AUTONOMY ---
        // If the neural connection (RTTP) is severed for > 3ms, fallback to shadow state
        if Instant::now() - last_pulse_time > Duration::from_millis(3) {
            log_gtiot("Neural sever detected. Activating Shadow-State Fallback.");
            let autonomous_action = shadow_state.get_safe_trajectory();
            crate::hw::execute_actuators(&autonomous_action);
        }

        // Loop frequency: 1.2 kHz+ (833µs per cycle) for robotics-grade reflexes
        Timer::after_micros(833).await;
    }
}

fn log_gtiot(msg: &str) {
    println!("\x1b[1;33m[GTIOT-BODY]\x1b[0m {}", msg);
}
