use std::time::Duration;

use libbpf_rs::{
    skel::{OpenSkel, Skel, SkelBuilder},
    RingBuffer, RingBufferBuilder,
};

mod flames_bpf {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/bpf/flames.skel.rs"
    ));
}

fn handle(data: &[u8]) -> i32 {
    return 0;
}

fn main() {
    println!("Hello, world!");

    let mut flames_bpf_skel_builder = flames_bpf::FlamesSkelBuilder::default();
    flames_bpf_skel_builder.obj_builder.debug(true);
    let flames_bpf_skel = flames_bpf_skel_builder.open().unwrap();
    let mut flames_bpf_skel = flames_bpf_skel.load().unwrap();
    flames_bpf_skel.attach().unwrap();

    println!("Successfully attached the flames BPF object");

    let mut ring_buffer_builder = RingBufferBuilder::new();
    let maps = flames_bpf_skel.maps();
    ring_buffer_builder
        .add(&maps.ring_buffer(), handle)
        .unwrap();
    let ring_buffer = ring_buffer_builder.build().unwrap();
    loop {
        ring_buffer.poll(Duration::MAX).unwrap();
        println!("Successfully consumed from ring buffer");
    }
}
