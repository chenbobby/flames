// TODO: Setup build.rs to include headers for the build environment's
// architecture.
#include "x86_64/vmlinux.h"

#include <bpf/bpf_core_read.h>
#include <bpf/bpf_helpers.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

// This struct needs to be deconstructed in Rust.
typedef struct {
    __u32 sample_id;
    __s32 process_id;
    char program_name[32];
    char comm[32];
} FlamesSample;

// TODO: Send constant value to userspace program via some map.
extern int LINUX_KERNEL_VERSION __kconfig;

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 256 * 1024);
} ring_buffer SEC(".maps");

// Programs
//
// For a list of all program types, see
// https://docs.kernel.org/bpf/libbpf/program_types.html
//
// TODO: Find a list of tracepoints.

SEC("tp/sched/sched_process_exec")
int handle_exec(struct trace_event_raw_sched_process_exec* ctx) {
    FlamesSample* sample =
        bpf_ringbuf_reserve(&ring_buffer, sizeof(FlamesSample), 0);
    if (sample == 0) {
        bpf_printk("Failed to reserve space in ring buffer. There may be no "
                   "space left in the ring buffer.\n");
        return 0;
    }

    // Extract data.
    sample->sample_id = bpf_get_prandom_u32();
    sample->process_id = (__s32)(bpf_get_current_pid_tgid() >> 32);
    bpf_get_current_comm(&sample->program_name, sizeof(sample->program_name));

    struct task_struct* task = (struct task_struct*)bpf_get_current_task();

    const char* comm;
    comm = BPF_CORE_READ(task, comm);

    bpf_ringbuf_submit(sample,
                       0 // Send adaptive notification for data availability.
    );

    return 0;
}
