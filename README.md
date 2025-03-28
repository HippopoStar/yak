# yak
Yet Another Kernel  

## Resources
[O'Reilly Media — Programming Rust, 2nd Edition](https://oreil.ly/programming-rust-2e)  
[Philipp Oppermann — Writing an OS in Rust (First Edition)](https://os.phil-opp.com/edition-1/)  
[OSDev Wiki](https://wiki.osdev.org/Main_Page)  

## Similar projects
[Redox](https://gitlab.redox-os.org/redox-os/kernel)  
[Maestro](https://github.com/maestro-os/maestro)  
[vcombey — Turbofish](https://github.com/sclolus/Turbofish)  
[endcerro — KFS_N](https://github.com/endcerro/KFS_N)  

## Related 42 projects
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
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
[The rustup book — Concepts — Channels (#working-with-nightly-rust)](https://rust-lang.github.io/rustup/concepts/channels.html#working-with-nightly-rust)  
```
rustup toolchain install nightly
```
[The Cargo Book — Cargo Reference — Unstable Features (#build-std)](https://doc.rust-lang.org/cargo/reference/unstable.html#build-std)  
```
rustup component add rust-src --toolchain nightly
```

## Miscellaneous
[ProgrammerHumor.io — Rust](https://programmerhumor.io/backend-memes/rust-3/)  
[Kali Linux Blog — 2024-10-22 — The end of the i386 kernel and images](https://www.kali.org/blog/end-of-i386-kernel-and-images/)  
[InfoWorld — 2024-02-27 — White House urges developers to dump C and C++](https://www.infoworld.com/article/3713203/white-house-urges-developers-to-dump-c-and-c.html)  
[Analytics Insight — 2022-12-15 — Updated Linux Kernel 6.1 Makes Rust the Greatest Programming Language](https://www.analyticsinsight.net/latest-news/updated-linux-kernel-6-1-makes-rust-the-greatest-programming-language)  
[Linux Weekly News — 2007-09-06 — Re: [RFC] Convert builin-mailinfo.c to use The Better String Library](https://lwn.net/Articles/249460/)  
[YouTube — The Linux Foundation — 2024-09-16 — Keynote: Linus Torvalds in Conversation with Dirk Hohndel (16:42)](https://youtu.be/OM_8UOPFpqE?t=1002)  

