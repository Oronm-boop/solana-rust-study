# 🚀 Solana 全栈开发挑战 (Solana Full Stack Challenges)

本仓库记录了 Solana 开发的学习路径和实战任务挑战，涵盖从基础的 SPL Token 操作到使用 Anchor 和 Pinocchio 框架开发复杂的链上程序。

## 📋 任务列表 (Challenge Tasks)

| 任务 | 名称 | 说明 | 对应目录 |
| :--- | :--- | :--- | :--- |
| **Task 1** | 铸造 SPL Token | 使用 web3.js 铸造一个 SPL Token | `任务1-铸造SPL Token` |
| **Task 2** | Anchor 金库 | 使用 Anchor 创建用户金库 | `任务2-Anchor金库` |
| **Task 3** | Anchor 托管 | 使用 Anchor 创建托管应用 | `任务3-Anchor 托管` |
| **Task 4** | Pinocchio 金库 | 使用 Pinocchio 创建用户金库并提交 | `任务4-Pinocchio金库` |
| **Task 5** | Pinocchio 托管 | 使用 Pinocchio 创建用户托管并提交 | *(待创建)* |
| **Task 6** | Pinocchio AMM (可选) | 使用 Pinocchio 开发一个 AMM Swap | *(待创建)* |
| **毕业设计** | Solana 应用开发 | 使用 create-solana-dapp 结合课上所学做一个 Solana 小应用并提交 Solana 黑客松，主题自选 | *(待创建)* |

## 📚 详细任务说明

### Task 1: 铸造 SPL Token
利用 `@solana/web3.js` 和 `@solana/spl-token` 库，编写 TS 脚本在 Devnet 上完成代币的 Mint Authority 设置、Token Account 创建及铸造流程。

### Task 2: Anchor 金库 (Vault)
编写 Anchor 程序，学习 PDA (Program Derived Address) 的使用。
- **目标**：实现用户可以将 SOL 或 Token 存入程序控制的金库，并能安全取出。

### Task 3: Anchor 托管 (Escrow)
实现去中心化交易中最基础的原子交换（Atomic Swap）。
- **流程**：Maker 初始化订单 -> 转移代币到金库 -> Taker 接受订单 -> 交换代币 -> 完成交易。
- 重点在于理解 Token Program 的 CPI (Cross-Program Invocation) 调用。

### Task 4 & 5: Pinocchio 开发
Pinocchio 是一种更接近原生的开发方式，有助于理解 Anchor 底层做了什么。
- **Task 4**: 重写金库逻辑。
- **Task 5**: 重写托管逻辑。

### 毕业设计
**要求**：
- 使用 `create-solana-dapp` 脚手架。
- 包含完整的前端 UI 和后端合约。
- 提交到 Solana Hackathon 或作为结课作业。

---
*Happy Coding! 🦀*
