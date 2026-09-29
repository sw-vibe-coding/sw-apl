# Running sw-apl on Linux boards

The standalone CLI runs its REPL directly on the board through SSH.
It needs one terminal; no sw-apl-server or aplterm process is required.
Use a UTF-8 terminal with an APL-capable font for glyph input and display.

## Cross-build

Run from the repository root on the build host. Rustup uses the
repository's Rust toolchain version for both target installations.

```sh
rustup target add riscv64gc-unknown-linux-musl armv7-unknown-linux-musleabihf
just cross licheerv
just cross luckfox
```

| Board | Rust target | Executable |
|---|---|---|
| Sipeed LicheeRV Nano, RISC-V Linux | riscv64gc-unknown-linux-musl | target/riscv64gc-unknown-linux-musl/release/sw-apl |
| Luckfox Pico RV1103, ARM Cortex-A7 Linux | armv7-unknown-linux-musleabihf | target/armv7-unknown-linux-musleabihf/release/sw-apl |

These match the hardwarewrighter Rust hello-world demos. The configurations
in `.cargo/config.toml` use Rust's bundled rust-lld and static musl runtime.
No vendor compiler or board dynamic loader is needed. The RISC-V build uses
the generic GC instruction set, without vector extensions. The Luckfox
build targets its Linux ARM CPU, not its auxiliary RISC-V MCU.
Other board models or operating systems need their CPU and ABI checked.

## Copy and start a REPL

For the LicheeRV Nano, using the hello-world demo's USB address:

```sh
ssh root@10.105.250.1 'mkdir -p /root/sw-apl'
scp target/riscv64gc-unknown-linux-musl/release/sw-apl root@10.105.250.1:/root/sw-apl/
scp -r ws root@10.105.250.1:/root/sw-apl/
ssh -t root@10.105.250.1 'cd /root/sw-apl && ./sw-apl'
```

For the Luckfox Pico, using the hello-world demo's USB address:

```sh
ssh root@172.32.0.93 'mkdir -p /root/sw-apl'
scp target/armv7-unknown-linux-musleabihf/release/sw-apl root@172.32.0.93:/root/sw-apl/
scp -r ws root@172.32.0.93:/root/sw-apl/
ssh -t root@172.32.0.93 'cd /root/sw-apl && ./sw-apl'
```

Substitute the board's current address if different. Add `--mode 75`
after `./sw-apl` for the '75 dialect. The copied `ws/` provides library 1;
`)LOAD 1 LIFE` loads its LIFE workspace. Library 0 saves under `work/`
in the current directory. Large arrays are limited by the board's RAM.

Cross-compilation alone does not verify execution on the board. A quick
batch smoke test after copying is:

```sh
printf '1+2\n' | ssh root@172.32.0.93 'cd /root/sw-apl && ./sw-apl'
```

It should print `3`. Use the Nano's address to test that board as well.

Interactive smoke tests on both boards verified `1+2` gives `3`,
`+/⍳10` gives `55`, and `)OFF` exits cleanly over SSH. This checks
startup, terminal input, glyph handling and basic evaluation; it is
not a full on-board conformance or memory-capacity test.
