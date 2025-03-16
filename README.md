# yak
Yet Another Kernel  

[O'Reilly Media — Programming Rust, 2nd Edition](https://oreil.ly/programming-rust-2e)  
[Philipp Oppermann — Writing an OS in Rust (First Edition)](https://os.phil-opp.com/edition-1/)  
[OSDev Wiki](https://wiki.osdev.org/Main_Page)  

[The Embedded Rust Book — no\_std (#summary)](https://docs.rust-embedded.org/book/intro/no-std.html#summary)  
[LLVM — Documentation — Reference — Intermediate Representation (#data-layout)](https://llvm.org/docs/LangRef.html#data-layout)  

[Redox](https://gitlab.redox-os.org/redox-os/kernel)  
[Maestro](https://github.com/maestro-os/maestro)  
[vcombey — Turbofish](https://github.com/sclolus/Turbofish)  
[endcerro — KFS_N](https://github.com/endcerro/KFS_N)  

[libasm](https://cdn.intra.42.fr/pdf/pdf/134392/en.subject.pdf) (Assembly yourself! — ~49h — [🎞](https://cdn.intra.42.fr/video/video/213/low_d_2015-02-02_COURS_42_-_Introduction_a_l_asm_1.mp4))  
[ft\_linux](https://cdn.intra.42.fr/pdf/pdf/129297/en.subject.pdf) (how\_to\_train\_your\_kernel — ~49h — [🎞](https://cdn.intra.42.fr/video/video/511/low_d_vidlol1.mp4))  
[little-penguin-1](https://cdn.intra.42.fr/pdf/pdf/62177/en.subject.pdf) (Linux Kernel Development — ~100h — [🎞](https://cdn.intra.42.fr/video/video/514/low_d__projet__little-pinguin-1.mp4))  
[kfs-1](https://cdn.intra.42.fr/pdf/pdf/66157/en.subject.pdf) (Grub, boot and screen — ~294h — [🎞](https://cdn.intra.42.fr/video/video/547/low_d__projet__KFS_1.mp4))  
[kfs-2](https://cdn.intra.42.fr/pdf/pdf/66158/en.subject.pdf) (GDT & Stack — ~294h — [🎞](https://cdn.intra.42.fr/video/video/832/low_d__projet_____KFS_2.mp4))  
[kfs-3](https://cdn.intra.42.fr/pdf/pdf/110689/en.subject.pdf) (Memory — ~294h — [🎞](https://cdn.intra.42.fr/video/video/833/low_d__projet___KFS_3.mp4))  
[kfs-4](https://cdn.intra.42.fr/pdf/pdf/66160/en.subject.pdf) (Interrupts — ~196h — [🎞](https://cdn.intra.42.fr/video/video/902/low_d__projet__KFS_4.mp4))  
[kfs-5](https://cdn.intra.42.fr/pdf/pdf/66161/en.subject.pdf) (Processes — ~392h — [🎞](https://cdn.intra.42.fr/video/video/913/low_d__projet__KFS_5.mp4))  
[kfs-6](https://cdn.intra.42.fr/pdf/pdf/66162/en.subject.pdf) (Filesystem — ~294h — [🎞](https://cdn.intra.42.fr/video/video/919/low_d__projet__KFS_6.mp4))  
[kfs-7](https://cdn.intra.42.fr/pdf/pdf/66163/en.subject.pdf) (Syscalls, Sockets and env — ~630h — [🎞](https://cdn.intra.42.fr/video/video/920/low_d__projet__KFS_7.mp4))  
[kfs-8](https://cdn.intra.42.fr/pdf/pdf/66164/en.subject.pdf) (Modules — ~196h — [🎞](https://cdn.intra.42.fr/video/video/922/low_d__projet__KFS_8.mp4))  
[kfs-9](https://cdn.intra.42.fr/pdf/pdf/66165/en.subject.pdf) (ELF — ~245h — [🎞](https://cdn.intra.42.fr/video/video/923/low_d__projet__KFS_9.mp4))  
[kfs-x](https://cdn.intra.42.fr/pdf/pdf/66166/en.subject.pdf) (The END — ~56h — [🎞](https://cdn.intra.42.fr/video/video/924/low_d__projet__KFS_10.mp4))  

[ProgrammerHumor.io — Rust](https://programmerhumor.io/backend-memes/rust-3/)  
[Kali Linux Blog — 2024-10-22 — The end of the i386 kernel and images](https://www.kali.org/blog/end-of-i386-kernel-and-images/)  
[InfoWorld — 2024-02-27 — White House urges developers to dump C and C++](https://www.infoworld.com/article/3713203/white-house-urges-developers-to-dump-c-and-c.html)  
[Analytics Insight — 2022-12-15 — Updated Linux Kernel 6.1 Makes Rust the Greatest Programming Language](https://www.analyticsinsight.net/latest-news/updated-linux-kernel-6-1-makes-rust-the-greatest-programming-language)  
[Linux Weekly News — 2007-09-06 — Re: [RFC] Convert builin-mailinfo.c to use The Better String Library](https://lwn.net/Articles/249460/)  
[YouTube — The Linux Foundation — 2024-09-16 — Keynote: Linus Torvalds in Conversation with Dirk Hohndel (16:42)](https://youtu.be/OM_8UOPFpqE?t=1002)  

## Prerequisites
```
sudo apt-get update && sudo apt-get install \
	build-essential \
	grub-common \
	grub-pc-bin \
	nasm \
	qemu-system-x86 \
	xorriso
```
[The Rust toolchain installer](https://rustup.rs)  
```
# https://rust-lang.github.io/rustup/concepts/channels.html#working-with-nightly-rust
rustup toolchain install nightly

# https://doc.rust-lang.org/cargo/reference/unstable.html#build-std
rustup component add rust-src --toolchain nightly
```

## target-triplet
[Rust Programming Language — Learn - rustc book — Platform Support — x86_64-unknown-none](https://doc.rust-lang.org/rustc/platform-support/x86_64-unknown-none.html)  
[GitHub — maestro-os — maestro/kernel/arch/x86/x86.json](https://github.com/maestro-os/maestro/blob/master/kernel/arch/x86/x86.json)  
[GitLab — redox-os — bootloader-coreboot/targets/x86-unknown-none.json](https://gitlab.redox-os.org/redox-os/bootloader-coreboot/-/blob/master/targets/x86-unknown-none.json)  

## x86 assembly
[Wikipedia — i386 #Architecture](https://en.wikipedia.org/wiki/I386#Architecture)  
[Wikipedia — x86 assembly language #"Hello_world!"_program_for_Linux_in_NASM_style_assembly](https://en.wikipedia.org/wiki/X86_assembly_language#"Hello_world!"_program_for_Linux_in_NASM_style_assembly)  

### The Linux Kernel Archive
**pub/scm/linux — kernel/git/torvalds/linux.git**  
[sys_write — scripts/syscall.tbl](https://web.git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/scripts/syscall.tbl#n83)  
[sys_write — arch/x86/entry/syscalls/syscall_32.tbl](https://web.git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/arch/x86/entry/syscalls/syscall_32.tbl)  
[sys_write — include/linux/syscalls.h](https://web.git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/include/linux/syscalls.h#n476)  
[sys_write — tools/include/nolibc/sys.h](https://web.git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/tools/include/nolibc/sys.h#n1246)  
[__NR_write — tools/include/uapi/asm-generic/unistd.h](https://web.git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/tools/include/uapi/asm-generic/unistd.h#n174)  
[__NR_write — include/uapi/asm-generic/unistd.h](https://web.git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/include/uapi/asm-generic/unistd.h#n174)  
[my_syscall3 — tools/include/nolibc/arch-i386.h](https://web.git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/tools/include/nolibc/arch-i386.h#n78)  

