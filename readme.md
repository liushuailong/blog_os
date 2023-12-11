To build this binary, we need to compile for a bare metal target such as `thumbv7em-none-eabihf`
```bash
# An example of such a bare metal environment is the thumbv7em-none-eabihf target triple, which describes an embedded ARM system.
rustup target add thumbv7em-none-eabihf
cargo build --target thumbv7em-none-eabihf
```

To build this binary, we need to compile for a bare metal target such as `thumbv7em-none-eabihf`

```bash
# 需要用到一些不稳定特性，需要使用nightly版本，使用下面命令将该项目下面的toolchain使用nightly版本，使用cargo编译时调用nightly版本的rust。
cd D:\southgene\project\software\blog_os
rustup override set nightly
# Linux
cargo rustc -- -C link-arg=-nostartfiles
# Windows
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
# macOS
cargo rustc -- -C link-args="-e __start -static -nostartfiles"
# for blog_os
# 添加rsut源码，才能重新编译核心库
rustup component add rust-src --toolchain nightly-x86_64-pc-windows-msvc
cargo build --target x86_64-blog_os.json
# To install the tool, go into your home directory (or any directory outside of your cargo project) and execute the following command in your terminal
cargo install bootimage # 不能在项目目录下
rustup component add llvm-tools-preview
# 在vscode中使用cargo run 时找不到qemu软件地址，而在系统的powershell中就可以，应该是vscode的中断没有将该环境变量引入，可能时由于我没有在系统变量中添加，只添加了用户环境变量。
cargo run
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