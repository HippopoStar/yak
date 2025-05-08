
### In repository root directory
[The rustup book — Overrides (#the-toolchain-file)](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file)  
```
cat <<_EOF > rust-toolchain.toml
[toolchain]
channel = "nightly"
_EOF
rustup show
```

[The Cargo Book — Cargo Commands — Package Commands — cargo new](https://doc.rust-lang.org/cargo/commands/cargo-new.html)  
```
cargo new --lib --name yak rust
```

### In rust directory
[The Cargo Book — Cargo Reference — Configuration](https://doc.rust-lang.org/cargo/reference/config.html)  
[The Cargo Book — Cargo Commands — Build Commands — cargo build (#compilation-options)](https://doc.rust-lang.org/cargo/commands/cargo-build.html#compilation-options)  
[The rustc book — Targets — Custom Targets](https://doc.rust-lang.org/rustc/targets/custom.html)  
```
mkdir .cargo
cat <<_EOF > .cargo/config.toml
[build]
# cargo build --target <triple>
target = "arch/x86/x86-unknown-none.json"

[unstable]
# cargo +nightly build -Z build-std=core
build-std = ["core"]
_EOF
```

---

## Documentation & References

### The Rust Core Library
[The Embedded Rust Book — no\_std (#summary)](https://docs.rust-embedded.org/book/intro/no-std.html#summary)  
```
rustup doc --core
```

#### Volatile
[crates.io — volatile](https://crates.io/crates/volatile)  
[Rust API documentation — core — pointer (primitive type) (#method.write_volatile)](https://doc.rust-lang.org/stable/core/primitive.pointer.html#method.write_volatile)  

#### Atomic
[crates.io — spin](https://crates.io/crates/spin)  
[Rust API documentation — core::hint::spin_loop (function)](https://doc.rust-lang.org/stable/core/hint/fn.spin_loop.html)  
[Rust API documentation — core::sync::atomic (module)](https://doc.rust-lang.org/stable/core/sync/atomic/index.html)  
[Rust API documentation — spin::mutex::Mutex (struct)](https://docs.rs/spin/0.9.8/spin/mutex/struct.Mutex.html)  

### Programmable Interrupt Controller 8259
[crates.io — pic8259](https://crates.io/crates/pic8259) (2nd Edition — 2.3 Interrupts - Hardware Interrupts)  
[GitHub — Rust OSDev — pic8259 0.10](https://github.com/rust-osdev/pic8259/tree/v0.10.4)  
[crates.io — x86_64](https://crates.io/crates/x86_64) (2nd Edition — 2.1 Interrupts - CPU Exceptions)  
[GitHub — Rust OSDev — x86_64 0.14](https://github.com/rust-osdev/x86_64/tree/v0.14.13)  
[crates.io — bootloader](https://crates.io/crates/bootloader) (2nd Edition — 1.2 Bare Bones - A Minimal Rust Kernel)  
[GitHub — Rust OSDev — bootloader 0.9](https://github.com/rust-osdev/bootloader/tree/v0.9.30)  
[crates.io — multiboot2](https://crates.io/crates/multiboot2) (1st Edition — 2.1 Memory Management - Allocating Frames)  
[GitHub — Rust OSDev — multiboot2 0.1](https://github.com/rust-osdev/multiboot2/tree/v0.1.0)  

### target-triplet
[The rustc book — Platform Support — x86_64-unknown-none](https://doc.rust-lang.org/rustc/platform-support/x86_64-unknown-none.html)  
[GitHub — maestro-os — maestro/kernel/arch/x86/x86.json](https://github.com/maestro-os/maestro/blob/master/kernel/arch/x86/x86.json)  
[GitLab — redox-os — bootloader-coreboot/targets/x86-unknown-none.json](https://gitlab.redox-os.org/redox-os/bootloader-coreboot/-/blob/master/targets/x86-unknown-none.json)  

[LLVM — Documentation — Reference — Intermediate Representation (#data-layout)](https://llvm.org/docs/LangRef.html#data-layout)  

### x86 assembly
[Wikipedia — i386 (#Architecture)](https://en.wikipedia.org/wiki/I386#Architecture)  
[Wikipedia — x86 assembly language (#"Hello_world!"_program_for_Linux_in_NASM_style_assembly)](https://en.wikipedia.org/wiki/X86_assembly_language#"Hello_world!"_program_for_Linux_in_NASM_style_assembly)  
[Wikipedia — x86 memory models](https://en.wikipedia.org/wiki/X86_memory_models)  

#### The Linux Kernel Archive
pub/scm/linux — kernel/git/torvalds/linux.git  
[sys_write — scripts/syscall.tbl](https://web.git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/scripts/syscall.tbl#n83)  
[sys_write — arch/x86/entry/syscalls/syscall_32.tbl](https://web.git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/arch/x86/entry/syscalls/syscall_32.tbl)  
[sys_write — include/linux/syscalls.h](https://web.git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/include/linux/syscalls.h#n476)  
[sys_write — tools/include/nolibc/sys.h](https://web.git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/tools/include/nolibc/sys.h#n1246)  
[__NR_write — tools/include/uapi/asm-generic/unistd.h](https://web.git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/tools/include/uapi/asm-generic/unistd.h#n174)  
[__NR_write — include/uapi/asm-generic/unistd.h](https://web.git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/include/uapi/asm-generic/unistd.h#n174)  
[my_syscall3 — tools/include/nolibc/arch-i386.h](https://web.git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/tools/include/nolibc/arch-i386.h#n78)  

