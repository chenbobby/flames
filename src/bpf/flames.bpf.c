#include <linux/bpf.h>

#include <bpf/bpf_helpers.h>

#include "common.h"

char LICENSE[] SEC("license") = "Dual BSD/GPL";

typedef struct {
    __u32 sample_id;
} flames_sample;

// TODO: Send constant value to userspace program via some map.
extern int LINUX_KERNEL_VERSION __kconfig;

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 256 * 1024);
} ring_buffer SEC(".maps");

SEC("tp/sched/sched_process_exec")
int handle_exec(struct trace_event_raw_sched_process_exec* ctx) {
    flames_sample* sample =
        bpf_ringbuf_reserve(&ring_buffer, sizeof(flames_sample), 0);
    if (sample == 0) {
        return 0;
    }

    // Extract data.
    sample->sample_id = bpf_get_prandom_u32();

    bpf_ringbuf_submit(sample,
                       0 // Send adaptive notification for data availability.
    );

    return 0;
}