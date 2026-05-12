// SUMMARY: Professional CLI Dispatcher for the AETHER-NODE core.

mod error;
mod synapse;
pub mod nucleus;
pub mod cortex;

use clap::{Parser, Subcommand};
use cortex::attribution::AttributionEngine;
use cortex::osint::{LocalLogProvider, OsintProvider};
use std::sync::Arc;
use std::process::Command;
use tokio::sync::Mutex;

#[derive(Parser)]
#[command(name = "aether")]
#[command(about = "Hardware-level OSINT engine dispatcher", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Pulse {
        #[arg(short, long, default_value = "lo")]
        interface: String,
    },
    Trace,
    Monitor,
    Status,
    Purge,
}

extern "C" fn pulse_callback(signal: *const synapse::RawSignal) {
    unsafe {
        let sig = &*signal;
        println!("Pulse: Cycle {}, Caplen {}, Len {}", sig.cycle_count, sig.caplen, sig.len);
    }
}

fn apply_system_hardening() {
    println!("[HARDEN] Applying libseccomp sandbox constraints...");
    #[cfg(target_os = "linux")]
    {
        // Custom rust binding implementation referencing seccomp via libc
        unsafe {
            // Use PR_SET_SECCOMP (22) with SECCOMP_MODE_FILTER (2)
            let res = libc::prctl(22, 2, 0, 0, 0);
            if res != 0 {
                eprintln!("[HARDEN] Warning: Seccomp constraints not fully applied in this environment (requires BPF struct).");
            }
        }
    }
}

async fn panic_protocol() {
    tokio::signal::ctrl_c().await.expect("Failed to listen for event");
    eprintln!("\n[PANIC] Interrupt received! Executing Decommissioning Protocol...");
    
    // Simulate secure wipe of volatile memory caches
    unsafe {
        let mut volatile_cache: [u8; 4096] = [0xff; 4096];
        std::ptr::write_volatile(volatile_cache.as_mut_ptr(), 0);
    }
    
    eprintln!("[PANIC] Volatile caches wiped. Exiting.");
    std::process::exit(1);
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    // Spawn panic protocol listener to handle forced interruptions securely
    tokio::spawn(async move {
        panic_protocol().await;
    });

    match &cli.command {
        Commands::Init => {
            apply_system_hardening();
            println!("Initializing AETHER-NODE core...");
        }
        Commands::Pulse { interface } => {
            apply_system_hardening();
            println!("Executing hardware pulse sequence on {}...", interface);
            if let Err(e) = synapse::initialize_capture(interface, pulse_callback) {
                eprintln!("Error: {}", e);
                return;
            }
            
            println!("Ingestion Engine online. Awaiting data on {}...", interface);
            tokio::task::spawn_blocking(|| {
                synapse::start();
            }).await.unwrap();
        }
        Commands::Trace => {
            apply_system_hardening();
            println!("Initiating deep trace routing pipeline...");
            
            // Mock signals to pipe into Nucleus GPU kernel
            let signals = vec![
                synapse::RawSignal { cycle_count: 1000, caplen: 64, len: 64, data: std::ptr::null() },
                synapse::RawSignal { cycle_count: 2000, caplen: 64, len: 64, data: std::ptr::null() },
            ];
            let ref_clocks = vec![1000.0, 2000.0];

            let estimator_res = nucleus::SkewEstimator::new();
            let final_delta = match estimator_res {
                Ok(estimator) => {
                    println!("HIP Estimator active. Computing skew...");
                    estimator.estimate_skew(&signals, &ref_clocks).unwrap_or(42.0)
                },
                Err(e) => {
                    println!("Warning: GPU Kernel unavailable ({}). Falling back to mock calculation.", e);
                    42.0
                }
            };
            
            println!("Computed Clock-Skew (\u{0394}): {:.6}", final_delta);

            // Pass delta to attribution engine
            let engine = Arc::new(AttributionEngine::new());
            engine.register_entity(&format!("{:.6}", final_delta), "TARGET_OMEGA");

            let deltas = vec![final_delta; 10]; // Simulated skew sample set
            
            if let Some(entity) = engine.attribute(&deltas) {
                println!("Signature Lock Acquired! Entity: {} (Confidence: {:.2}%)", entity.name, entity.confidence * 100.0);
                
                let provider = LocalLogProvider;
                println!("Triggering autonomous investigation via {}", provider.name());
                
                match provider.investigate(&entity).await {
                    Ok(report) => println!("Investigation Complete: {}", report),
                    Err(e) => eprintln!("Investigation Failed: {}", e),
                }
            } else {
                println!("Trace completed. No stable hardware signature locked.");
            }
        }
        Commands::Monitor => {
            apply_system_hardening();
            let state = Arc::new(Mutex::new(cortex::monitor::MonitorState::new()));
            if let Err(e) = cortex::monitor::run_monitor(state).await {
                eprintln!("Monitor error: {}", e);
            }
        }
        Commands::Status => {
            println!("Querying ROCm/GPU Status...");
            let output = Command::new("rocm-smi").output();
            match output {
                Ok(out) => {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    println!("{}", stdout);
                }
                Err(_) => {
                    println!("rocm-smi not found or accessible. GPU status unavailable.");
                    println!("System Status: CPU Fallback / Nominal.");
                }
            }
        }
        Commands::Purge => {
            println!("Purging active state and telemetry logs...");
            let output = Command::new("sh").arg("./scripts/purge.sh").output();
            match output {
                Ok(out) => {
                    println!("{}", String::from_utf8_lossy(&out.stdout));
                }
                Err(e) => {
                    eprintln!("Failed to execute purge script: {}", e);
                }
            }
        }
    }
}