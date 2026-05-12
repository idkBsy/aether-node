// SUMMARY: FFI boundary definitions mapping C ingestion structures to Rust.
#ifndef SYNAPSE_BRIDGE_H
#define SYNAPSE_BRIDGE_H

#include <stdint.h>

typedef struct {
    uint64_t cycle_count;
    uint32_t caplen;
    uint32_t len;
    const uint8_t *data;
} RawSignal;

typedef void (*RawSignalCallback)(const RawSignal *signal);

// External hardware cycle count
uint64_t get_rdtsc_raw(void);

// Initialize capture interface
int init_capture(const char *device, RawSignalCallback callback);

// Start blocking packet ingestion loop
void start_capture_loop(void);

// Stop capture gracefully
void stop_capture(void);

#endif // SYNAPSE_BRIDGE_H
