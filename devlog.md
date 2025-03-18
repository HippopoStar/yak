
### In repository root directory
[The rustup book — Overrides #the-toolchain-file](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file)  
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
[The Cargo Book — Cargo Commands — Build Commands — cargo build #compilation-options](https://doc.rust-lang.org/cargo/commands/cargo-build.html#compilation-options)  
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

