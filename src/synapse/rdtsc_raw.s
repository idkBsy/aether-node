# SUMMARY: High-precision hardware cycle counter using serialized RDTSC.
.intel_syntax noprefix
.global get_rdtsc_raw
.text
get_rdtsc_raw:
    # Serialize instruction pipeline to ensure strict timing
    lfence
    # Read Time-Stamp Counter into EDX:EAX
    rdtsc
    # Shift RDX left by 32 bits and OR with RAX to form 64-bit result
    shl rdx, 32
    or rax, rdx
    ret
