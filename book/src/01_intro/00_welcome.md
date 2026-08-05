# 欢迎

欢迎来到 **「100 道练习学 Rust」中文延伸版**！

本课程带你**一道练习接着一道练习**学习 Rust：从零基础到能独立写出自己的程序。

> 原课程由 [Mainmatter](https://mainmatter.com/rust-consulting/) 编写。
> 本仓库是在其开源内容之上的**全局简体中文延伸**（含每题 `教学.md`），仍遵循 CC BY-NC 4.0。

## 如何使用本仓库

1. 安装 [Rust](https://www.rust-lang.org/tools/install)（建议 stable）。
2. 克隆本仓库并进入根目录。
3. 打开某一练习目录，先读 **`教学.md`**（中文：目标 / 怎么改 / 怎么跑）。
4. 按提示修改 `src/lib.rs`。
5. 运行测试：

```bash
cargo test -p <crate名>
```

crate 名写在该练习的 `Cargo.toml` 以及 `教学.md` 里。

## 目录结构（简要）

```text
exercises/          # 全部练习（Cargo workspace 成员）
  01_intro/
    00_welcome/
      src/lib.rs    # 你要改的代码
      教学.md       # 中文运行教学（本仓库延伸）
book/src/           # 教程正文（mdBook）
helpers/            # 公共辅助 crate
```

## 工具建议

- **RustRover**，或
- **VS Code** + [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)

## 参考答案

上游官方参考实现见原仓库的 [`solutions` 分支](https://github.com/mainmatter/100-exercises-to-learn-rust/tree/solutions)。  
**请先自己做**；`教学.md` 默认只给思路与运行方式，不直接贴完整答案。

## 第一题

打开 `exercises/01_intro/00_welcome`，阅读 `教学.md`，然后修改 `greeting`，让测试通过。

```bash
cargo test -p welcome_00
```

祝学习顺利！
