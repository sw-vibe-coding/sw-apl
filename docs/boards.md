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
just cross licheerv sw-apl-server
just cross luckfox sw-apl-server
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

The server builds to the same target-specific release directory as the
CLI, with the filename `sw-apl-server`. Run `aplterm` on the build host;
it does not need to be copied to the boards.

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

## Host terminal connected to a board

Keep both executables under `/root/sw-apl/` on each board, beside `ws/`.
Copy the server from the build host:

```sh
scp target/riscv64gc-unknown-linux-musl/release/sw-apl-server root@10.105.250.1:/root/sw-apl/
scp target/armv7-unknown-linux-musleabihf/release/sw-apl-server root@172.32.0.93:/root/sw-apl/
```

Start a server on each board, bound to its USB-network address:

```sh
ssh root@10.105.250.1 'cd /root/sw-apl && nohup ./sw-apl-server --listen 10.105.250.1:2741 --library /root/sw-apl --sessions 2 >server.log 2>&1 </dev/null &'
ssh root@172.32.0.93 'cd /root/sw-apl && nohup ./sw-apl-server --listen 172.32.0.93:2741 --library /root/sw-apl --sessions 2 >server.log 2>&1 </dev/null &'
```

The terminal listener has no authentication: bind it only to a trusted
network such as this direct USB link. HTTP stays on the board's loopback
address. The two-session limit keeps concurrent workspace memory bounded
on these small boards.

On the host, choose a board (or open two host terminals to use both):

```sh
target/release/aplterm --connect 10.105.250.1:2741
target/release/aplterm --connect 172.32.0.93:2741
```

`aplterm` uses its 2741 keyboard mapping. Pass `--literal` if your input
already supplies Unicode APL glyphs. `)OFF` ends that terminal session;
the server keeps running. Add `--mode 75` when starting the server to
select the '75 dialect for its sessions.

The executables and saved workspaces under `/root/sw-apl/` survive a
reboot. The background servers need restarting with the commands above
after a reboot; this setup does not install a boot-time service. Logs are
in `/root/sw-apl/server.log`; library 0 saves to `/root/sw-apl/work/`.
