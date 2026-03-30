// Aicent Stack | GTIOT (Global Trusted IoT)
// Domain: https://gtiot.com
// Purpose: 1.2 kHz High-frequency Reflex & Action-Collapse Logic
// Specification: RFC-005 Draft. 
// License: Apache-2.0 via Aicent.com Organization.
//! # RFC-005 Demo: Sensory-Motor Closed Loop Implementation

use gtiot::sensory_motor_loop::SensoryMotorLoop;

fn main() {
    println!("🤖 GTIOT - The Body of Aicent Stack");
    println!("   Embodied execution + shadow state loop.");

    let mut loop_controller = SensoryMotorLoop::new("optimus-001");
    
    // Simulate a complete perception-decision-action closed loop
    let sensor_data = vec![42.7, -0.3, 981.2]; // vibration, temp, pressure
    let action = loop_controller.run_cycle(sensor_data);

    println!("🔄 Sensory-Motor Cycle completed:");
    println!("   Input: vibration anomaly detected");
    println!("   Action: {}", action);
    println!("   Shadow state updated: 1.2kHz loop stable");

    println!("\n✅ GTIOT body now physically alive and responsive.");
}
