To build this binary, we need to compile for a bare metal target such as `thumbv7em-none-eabihf`
```bash
# An example of such a bare metal environment is the thumbv7em-none-eabihf target triple, which describes an embedded ARM system.
rustup target add thumbv7em-none-eabihf
cargo build --target thumbv7em-none-eabihf
```

To build this binary, we need to compile for a bare metal target such as `thumbv7em-none-eabihf`

```bash
# Linux
cargo rustc -- -C link-arg=-nostartfiles
# Windows
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
# macOS
cargo rustc -- -C link-args="-e __start -static -nostartfiles"
```

```bash
# 提交代码到主分支
git add .
git commit -m "first commit info"
git push origin main
# 给代码打标签
git tag -a post-01 -m "First blog code"
git push origin post-01
```