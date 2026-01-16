# Solana Rust 学习全攻略 🚀

这份 README 记录了我们从零开始掌握 Solana 开发所需的 Rust 核心语法的完整路径。按照顺序学习，你将构建起坚实的智能合约开发基础。

> 💡 **适用人群**：
> 本项目特别适合 **具备一定 Rust 基础且了解 Solana 基本概念** 的开发者。
> 我们剔除了与链上开发无关的细枝末节，通过 Rust 在 Solana 智能合约中的实际应用场景（如 Context, Account, PDA），帮助你快速提炼和掌握 Solana 开发真正需要的那部分 Rust 核心语法，实现从"语言使用者"到"合约开发者"的快速跃迁。


## 📅 课程大纲 (Learning Path)

### 第一阶段：Rust 基础 (The Essentials)
> 目标：掌握写智能合约必须懂的 Rust "土话"。

*   **Lesson 1: 变量与数据类型**
    *   **核心**: `let` vs `let mut`, `u64` (钱), `[u8; 32]` (地址), `String` vs `&str`.
    *   📂 代码: [`src/bin/lesson1.rs`](src/bin/lesson1.rs)
*   **Lesson 2: 所有权与借用 (Ownership)**
    *   **核心**: Rust 的灵魂。理解为什么账户要传 `&mut`.
    *   📂 代码: [`src/bin/lesson2.rs`](src/bin/lesson2.rs)
*   **Lesson 3: 结构体与枚举 (Structs & Enums)**
    *   **核心**: 定义账户状态 (`State`) 和指令类型 (`Instruction`).
    *   📂 代码: [`src/bin/lesson3.rs`](src/bin/lesson3.rs)
*   **Lesson 4: 模式匹配 (Pattern Matching)**
    *   **核心**: `match` 和 `if let`，处理不同的指令分支。
    *   📂 代码: [`src/bin/lesson4.rs`](src/bin/lesson4.rs)

### 第二阶段：Solana 核心概念 (Smart Contract Core)
> 目标：理解 Solana 独有的 "方言"。

*   **Lesson 5: 宏与属性 (Macros & Attributes)**
    *   **核心**: `declare_id!`, `msg!`, 以及 `#[derive(...)]`.
    *   📂 代码: [`src/bin/lesson5.rs`](src/bin/lesson5.rs)
*   **Lesson 6: 特质与序列化 (Traits & Serialization)**
    *   **核心**: `BorshSerialize`/`Deserialize`，数据是怎么存进链上的。
    *   📂 代码: [`src/bin/lesson6.rs`](src/bin/lesson6.rs)
*   **Lesson 7: 错误处理与泛型 (Error Handling & Generics)**
    *   **核心**: `require!` 宏，以及 Anchor 的魔法 `Context<T>`.
    *   📂 代码: [`src/bin/lesson7.rs`](src/bin/lesson7.rs)

### 第三阶段：高级应用 (Advanced Patterns)
> 目标：掌握构建复杂 DApp 的能力。

*   **Lesson 8: 迭代器与生命周期 (Iterators & Lifetimes)**
    *   **核心**: 处理账户数组，理解 `<'info>` 的含义。
    *   📂 代码: [`src/bin/lesson8.rs`](src/bin/lesson8.rs)
*   **Lesson 9: CPI (跨程序调用)**
    *   **核心**: 乐高积木 —— 如何调用 System Program 转账。
    *   📂 代码: [`src/bin/lesson9.rs`](src/bin/lesson9.rs)
*   **Lesson 10: PDA (Program Derived Addresses)**
    *   **核心**: Solana 的终极奥义。Seeds, Bump, 以及 `invoke_signed`.
    *   📂 代码: [`src/bin/lesson10.rs`](src/bin/lesson10.rs)

### 💡 特别篇：前端交互 (Frontend Integration)
> 目标：理解 Rust 写的合约怎么被 JS/TS 调用。

*   **Lesson IDL: TypeScript 与 IDL 交互**
    *   **核心**: `anchor.workspace`, `program.methods.rpc()`, `program.account.fetch()`.
    *   📂 代码: [`src/bin/lesson_idl.ts`](src/bin/lesson_idl.ts)


---

## 🏗️ 实战项目 (Projects)

### 1. Anchor 计数器 (Standard Real-World Project)
这是一个使用 **Anchor 框架** 构建的完整标准项目，包含测试用例。
*   **功能**: 初始化 (Initialize), 增加 (Increment), 减少 (Decrement).
*   **位置**: [`anchor_real_counter/`](anchor_real_counter/)
*   **关键代码**: [`programs/anchor_real_counter/src/lib.rs`](anchor_real_counter/programs/anchor_real_counter/src/lib.rs)
*   **运行测试**:
    ```bash
    cd anchor_real_counter
    anchor test
    ```

---

## 🛠️ 常用命令速查

* **初始化 Anchor 项目**:
  ```bash
  anchor init <project_name>
  ```
* **构建与测试**:
  ```bash
  anchor build
  anchor test
  ```
