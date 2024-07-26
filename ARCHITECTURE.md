# Architecture

flames uses the BPF map type `ringbuf`.
`ringbuf` is introduced in Linux 5.8 and has better performance than the BPF map type `perfbuf`.

* [See BPF ring buffer design documentation](https://docs.kernel.org/6.6/bpf/ringbuf.html)
* [See blog post on `ringbuf` vs. `perfbuf`](https://nakryiko.com/posts/bpf-ringbuf/)
* [See `ringbuf` vs. `perfbuf` benchmarks](https://patchwork.ozlabs.org/project/netdev/patch/20200529075424.3139988-5-andriin@fb.com/)
