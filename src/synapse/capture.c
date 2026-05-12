// SUMMARY: High-speed ring-buffer packet ingestion utilizing libpcap with zero-copy intent.
#include "../include/synapse_bridge.h"
#include <pcap.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

static pcap_t *handle = NULL;
static RawSignalCallback rust_callback = NULL;

void packet_handler(u_char *user, const struct pcap_pkthdr *pkthdr, const u_char *packet) {
    (void)user;
    if (!rust_callback) return;

    RawSignal signal;
    signal.cycle_count = get_rdtsc_raw();
    signal.caplen = pkthdr->caplen;
    signal.len = pkthdr->len;
    // Minimal overhead: passing pointer directly for zero-copy FFI transition
    signal.data = packet; 

    rust_callback(&signal);
}

int init_capture(const char *device, RawSignalCallback callback) {
    char errbuf[PCAP_ERRBUF_SIZE];
    
    // Create handle directly (mmap packet capture is standard on Linux for modern libpcap)
    handle = pcap_create(device, errbuf);
    if (handle == NULL) {
        fprintf(stderr, "pcap_create failed: %s\n", errbuf);
        return -1;
    }

    // Configure snapshot length and promiscuous mode
    pcap_set_snaplen(handle, 65535);
    pcap_set_promisc(handle, 1);
    pcap_set_timeout(handle, 1);
    
    // Activate the interface
    int status = pcap_activate(handle);
    if (status != 0) {
        fprintf(stderr, "pcap_activate failed: %s\n", pcap_geterr(handle));
        return -1;
    }

    rust_callback = callback;
    return 0;
}

void start_capture_loop(void) {
    if (handle != NULL) {
        pcap_loop(handle, -1, packet_handler, NULL);
    }
}

void stop_capture(void) {
    if (handle != NULL) {
        pcap_breakloop(handle);
        pcap_close(handle);
        handle = NULL;
    }
}
