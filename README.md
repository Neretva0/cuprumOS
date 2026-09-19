# cuprumOS

**cuprumOS** is a minimal, experimental 64-bit operating system kernel written in Rust for the x86_64 architecture. I started this project to explore operating system development, bare-metal Rust, and low-level x86 hardware architecture from the ground up.

The project is actively being developed and is currently in its early stages. While it is far from a finished operating system, the initial foundational layers—from bare-metal boot to robust CPU exception handling—are implemented and functional.

---

## Current Progress

The kernel currently boots into 64-bit long mode, provides basic I/O interfaces, and safely catches CPU exceptions:

- **Freestanding Bare-Metal Core:** Custom `x86_64` bare-metal target (`#![no_std]`, `#![no_main]`) with a custom bootloader and entry point.
- **VGA Text Buffer Driver:** Safe wrapper around the `0xb8000` text memory with scrolling, color support, and formatted console output via custom `print!` and `println!` macros.
- **Serial Output:** 16550 UART driver for sending logs and debugging output directly to the host console.
- **Automated Testing Harness:** Custom test runner integrated with QEMU's `isa-debug-exit` device for running automated in-kernel unit and integration tests.
- **Interrupt Descriptor Table (IDT):** Exception handling infrastructure configured to catch CPU faults (such as breakpoint exceptions).
- **GDT & Interrupt Stack Table (IST):** Global Descriptor Table (GDT) and Task State Segment (TSS) setup with dedicated interrupt stacks.
- **Double Fault Handling:** Safe handling for double faults using a separate, known-good stack in the IST to catch critical issues (like kernel stack overflows) and prevent triple faults.

---

## Future Roadmap

This is an ongoing project, and I plan to build out additional operating system subsystems incrementally:

- [ ] Hardware interrupts via the 8259 PIC (timer ticks, PS/2 keyboard input)
- [ ] Memory management (paging, virtual memory mapping, frame allocator)
- [ ] Kernel heap allocation (`Box`, `Vec`, `Arc` support)
- [ ] Multitasking and process/thread scheduling
- [ ] Basic user space execution

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
