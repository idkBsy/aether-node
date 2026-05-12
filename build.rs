use std::env;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    
    // Link the custom native library "synapse"
    println!("cargo:rustc-link-search=native={}/include", manifest_dir);
    println!("cargo:rustc-link-lib=static=synapse");
    
    // Check standard ROCm installation directories for the linker
    let rocm_paths = [
        "/opt/rocm/lib", 
        "/opt/rocm/lib64", 
        "/usr/local/rocm/lib", 
        "/usr/lib",
        "/usr/local/lib/ollama/rocm" // Added path based on user environment
    ];
    for path in &rocm_paths {
        if Path::new(path).exists() {
            println!("cargo:rustc-link-search=native={}", path);
            // Instruct the runtime linker to also look in this directory
            println!("cargo:rustc-link-arg=-Wl,-rpath,{}", path);
        }
    }
    
    // Link pcap and amdhip64
    println!("cargo:rustc-link-lib=dylib=pcap");
    // Depending on the environment, we might make hip optional, but for AETHER-NODE it's required.
    println!("cargo:rustc-link-lib=dylib=amdhip64");

    // We can also re-run the build script if any C/ASM files change
    println!("cargo:rerun-if-changed=src/synapse/capture.c");
    println!("cargo:rerun-if-changed=src/synapse/rdtsc_raw.s");
    println!("cargo:rerun-if-changed=src/nucleus/skew_est.hip");
    println!("cargo:rerun-if-changed=Makefile");
}
