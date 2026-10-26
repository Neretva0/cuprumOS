# cuprumOS

**cuprumOS** is a minimal, experimental 64-bit operating system kernel written in Rust for the x86_64 architecture. I started this project to explore operating system development, bare-metal Rust, and low-level x86 hardware architecture from the ground up.

The project is actively being developed. The foundational layers—from bare-metal boot, exception handling, and hardware interrupts to virtual memory paging and multiple custom heap allocator designs—are fully implemented and functional.

---

## Current Progress

The kernel boots into 64-bit long mode, supports interactive input and timer interrupts, manages virtual memory via 4-level paging, and provides full dynamic heap allocation:

- **Freestanding Bare-Metal Core:** Custom `x86_64` bare-metal target (`#![no_std]`, `#![no_main]`) with bootloader integration and entry point.
- **VGA Text Buffer Driver:** Safe wrapper around the `0xb8000` text memory with scrolling, color support, and formatted console output via custom `print!` and `println!` macros.
- **Serial Output:** 16550 UART driver for sending logs and debugging output directly to the host console.
- **Automated Testing Harness:** Custom test runner integrated with QEMU's `isa-debug-exit` device for running automated in-kernel unit and integration tests.
- **Interrupt Descriptor Table (IDT):** Exception handling infrastructure configured to catch CPU faults (such as breakpoint and page fault exceptions).
- **GDT & Interrupt Stack Table (IST):** Global Descriptor Table (GDT) and Task State Segment (TSS) setup with dedicated interrupt stacks.
- **Double Fault Handling:** Safe handling for double faults using a separate, known-good stack in the IST to catch critical issues (like kernel stack overflows) and prevent triple faults.
- **Hardware Interrupts (8259 PIC):** Dual 8259 PIC configuration handling 8254 PIT periodic timer ticks and PS/2 keyboard scancode decoding for interactive text input.
- **Paging & Virtual Memory Management:** Complete 4-level x86_64 page table navigation, physical memory offset mapping, and a `BootInfoFrameAllocator` for physical page frame allocation.
- **Kernel Heap Allocation & Custom Allocator Designs:** Full support for `alloc` types (`Box`, `Vec`, `String`, `Rc`, `Arc`). Includes three custom allocator implementations built from scratch:
  - **Bump Allocator:** Ultra-fast linear allocator with an active allocation counter.
  - **Linked List Allocator:** Free-list allocator storing node metadata inside freed memory blocks, with layout adjustments to avoid fragmentation.
  - **Fixed-Size Block Allocator:** High-performance allocator with power-of-two size classes (8 B to 2048 B) and fallback to a linked list allocator for large allocations, configured as the active `#[global_allocator]`.

---

## Future Roadmap

This is an ongoing project, and I plan to build out additional operating system subsystems incrementally:

- [x] Hardware interrupts via the 8259 PIC (timer ticks, PS/2 keyboard input)
- [x] Memory management (paging, virtual memory mapping, frame allocator)
- [x] Kernel heap allocation and custom allocator designs (`Box`, `Vec`, `Rc` support)
- [ ] Cooperative multitasking (async/await, custom task executors)
- [ ] Preemptive multitasking and process/thread scheduling
- [ ] Basic user space execution (ring 3, system calls)

---

## Building and Running

### Prerequisites

- Nightly Rust toolchain (configured via `rust-toolchain`)
- Cargo `bootimage` runner:
  ```bash
  cargo install bootimage
  ```
- [QEMU](https://www.qemu.org/) (`qemu-system-x86_64`)

### Run in QEMU

Build the bootable kernel image and launch QEMU:

```bash
cargo run
```

### Run Tests

Run the test suite directly inside QEMU:

```bash
cargo test
```

#### Acknowledgments

Thanks to Philipp Oppermann for creating and maintaining the
[Writing an OS in Rust](https://os.phil-opp.com/) tutorial series.
