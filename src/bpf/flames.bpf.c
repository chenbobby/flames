#include <linux/bpf.h>

#include <bpf/bpf_helpers.h>

#include "common.h"

char LICENSE[] SEC("license") = "Dual BSD/GPL";

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 256 * 1024);
} ring_buffer SEC(".maps");

SEC("tp/sched/sched_process_exec")
int handle_exec(struct trace_event_raw_sched_process_exec* ctx) {
    int* data = bpf_ringbuf_reserve(&ring_buffer, sizeof(int*), 0);
    if (data == 0) {
        return 0;
    }

    *data = 69;
    bpf_ringbuf_submit(data, 0);
    return 0;
}