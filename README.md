# 100 道练习学 Rust（简体中文延伸版）

基于 [mainmatter/100-exercises-to-learn-rust](https://github.com/mainmatter/100-exercises-to-learn-rust) 的**全局简体中文延伸**。

原课程：一道练习接一道练习，从零学会用 Rust 写程序。  
本仓库额外提供：

- 每个练习目录下的 **`教学.md`**（目标 / 改哪里 / **如何运行** / 期望结果 / 易错点）
- 第 1–2 章教程与题面注释的简体中文
- 中文 `README`、中文 `book` 目录与校验脚本

> **许可证**：原作 © Mainmatter GmbH，[CC BY-NC 4.0](https://creativecommons.org/licenses/by-nc/4.0/)。  
> 本延伸版保留署名与非商业条款，详见 [NOTICE.md](./NOTICE.md)。

---

## 环境要求

- [Rust 工具链](https://www.rust-lang.org/tools/install)（stable）
- 推荐 IDE：RustRover，或 VS Code + rust-analyzer

```bash
rustup update stable
```

## 快速开始

```bash
git clone https://github.com/xiaoqianran/100-exercises-to-learn-rust-zh.git
cd 100-exercises-to-learn-rust-zh

# 第一题
cargo test -p welcome_00
# 同时阅读
# exercises/01_intro/00_welcome/教学.md
```

### 标准学习闭环

1. 打开 `exercises/<章>/<题>/教学.md`
2. 修改 `src/lib.rs`（不要改测试，除非题目要求）
3. 运行：

```bash
cargo test -p <crate名>
```

4. 全部 `ok` 后进入下一题

`crate` 名称见该题 `Cargo.toml` 的 `name` 字段，以及 `教学.md` 标题下说明。

## 仓库结构

```text
exercises/                 # 98 个练习包（Cargo workspace）
  01_intro/
  02_basic_calculator/
  ...
  └── <题>/
        Cargo.toml
        src/lib.rs
        教学.md            # 本仓库延伸（简体中文）
book/src/                  # 教程（mdBook；第1–2章已中文化）
helpers/                   # 公共辅助 crate
scripts/check_guides.py    # 校验每个练习是否有教学.md
NOTICE.md                  # 署名与许可说明
```

## 章节一览

| 章 | 目录 | 主题 | 教学.md |
| --- | --- | --- | --- |
| 1 | `01_intro` | 入门与语法 | 完整 |
| 2 | `02_basic_calculator` | 整数/变量/分支/循环/溢出 | 完整 |
| 3 | `03_ticket_v1` | 结构体与所有权 | 骨架（可运行指引齐全） |
| 4 | `04_traits` | Trait | 骨架 |
| 5 | `05_ticket_v2` | 枚举与错误处理 | 骨架 |
| 6 | `06_ticket_management` | 集合与迭代器 | 骨架 |
| 7 | `07_threads` | 线程与并发 | 骨架 |
| 8 | `08_futures` | 异步 | 骨架 |

「完整」= 中文题解级 `教学.md` + 中文题面注释/教程。  
「骨架」= 统一中文模板 + 准确 `cargo test -p` 命令 + 与 book 的链接；讲解将按章补全。

## 校验教学文档

```bash
python3 scripts/check_guides.py
```

要求：每个 `exercises/*/*/Cargo.toml` 旁必须存在 `教学.md`，且包含运行命令段落。

## 参考答案

请优先独立完成。官方参考实现见上游：

https://github.com/mainmatter/100-exercises-to-learn-rust/tree/solutions

## 与上游的关系

| 项目 | 说明 |
| --- | --- |
| 练习逻辑 / 测试 | 与上游保持一致，便于对照 |
| 中文教学 | 本仓库新增 `教学.md` |
| 教程语言 | 第 1–2 章简体中文；其余章目录中文，正文逐步翻译 |
| 贡献 | 欢迎 PR 补全第 3–8 章详细教学与 book 翻译 |

## 致谢

- 原作者与维护者：[Mainmatter](https://mainmatter.com/) 及贡献者
- 原项目主页：https://rust-exercises.com
