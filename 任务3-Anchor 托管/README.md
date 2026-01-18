# Anchor Escrow (担保交易) 项目教程

## 1. 项目简介

本项目是一个基于 Solana Anchor 框架实现的去中心化担保交易 (Escrow) 程序。
它的核心功能是允许两方 (Maker 和 Taker) 在不需要第三方信任的情况下进行代币互换。

### 核心机制
- **Maker (发起者)**: 初始化交易，将自己的代币 (Token A) 存入一个由程序控制的保险库 (Vault)。
- **Escrow Account**: 记录交易信息（如 Maker 是谁，想要交换的 Token B 数量等）的状态账户。
- **Taker (接受者)**: 支付 Maker 想要的代币 (Token B)，一旦支付成功，程序会自动将 Vault 中的 Token A 转给 Taker。
- **Refund (退款)**: 如果没有人接单，Maker 可以撤回自己的 Token A 并关闭交易。

---

## 2. 核心概念解释

### PDA (Program Derived Address)
程序派生地址。本项目中使用了 PDA 来控制 **Escrow State** 和 **Vault**。
- **Escrow PDA**: 存储交易状态数据。
    - Seeds: `[b"escrow", maker_pubkey, seed_u64]`
- **Vault PDA**: 存放 Token A 的代币账户。虽然它是一个代币账户，但它的 Authority (所有权) 是 Escrow PDA，这意味着只有程序能控制资金流向。

### ATA (Associated Token Account)
关联代币账户。用户的代币存储在 ATA 中。
- `maker_ata_a`: Maker 持有 Token A 的账户。
- `maker_ata_b`: Maker 用来接收 Token B 的账户。
- `taker_ata_b`: Taker 持有 Token B 的账户。
- `taker_ata_a`: Taker 用来接收 Token A 的账户。

---

## 3. 业务流程图 (ASCII Flowcharts)

### 3.1 Make (创建订单)

Maker 初始化 Escrow 状态并将 Token A 存入 Vault。

#### 核心账户解析：Vault vs Maker ATA
这是 Escrow 最关键的设计：
*   **Maker ATA (`maker_ata_a`)**:
    *   **持有者**: `Maker` (你)。
    *   **性质**: 你的私人钱包。
    *   **作用**: 资金的**来源**。你必须签名才能转出钱。
*   **Vault (`vault`)**:
    *   **持有者**: `Escrow` (PDA 程序)。
    *   **性质**: 程序的保险柜。
    *   **作用**: 资金的**托管地**。没有私钥，只有程序能控制它。

#### Make 指令账户全景关系图 (ASCII)

```txt
                                      [ 你的世界 ]                                           [ 程序的世界 (PDA) ]
                                    (User Space)                                           (Program Space)

       +---------------------+                                             
       |      Maker (你)      | <===========================================================+ 这是你的签名授权
       |     (Signer/Payer)  |                                                             | (Authority)
       +---------------------+                                                             |
          |           |                                                                    |
          |           | (1. 付房租创建账户)                                                   |
          |           +---------------------------------------------+                      |
          |                                                         |                      |
          v (2. 控制权)                                              v                      |
+---------------------+     (3. 这是什么币?)               +----------------------+        |
|     maker_ata_a     | ------------------------------> |       Mint A         |        |
| (你的 Token A 钱包)   |                                 |   (Token A 定义)     |        |
+---------------------+                                 +----------------------+        |
          |                                                         ^  ^                |
          |                                                         |  |                |
          | (4. 转账: 你的钱 -> 托管)                                 |  | (这个保险柜也是装 Token A)
          |                                                         |  |                |
          v                                                         |  |                |   +--------------------------+
+---------------------+                                 +----------------------+        |   |         Escrow           |
|        Vault        | <------------------------------ |  Token Program       |        |   |         (State)          |
| (托管 Token A 钱包)   |                                 |  System Program      |        |   | (数据: Price, Amount...) |
+---------------------+                                 |  Assoc Token Prog    |        |   +--------------------------+
          ^                                             +----------------------+        |                ^
          |                                                                             |                |
          +-----------------------------------------------------------------------------+----------------+
                    (5. 托管权: Vault 的主人是 Escrow，而不是你！)
```

**关键步骤**:
1. **Initialize Escrow**: 创建 Escrow 账户，保存 `seed`, `receive_amount`, `maker_key` 等信息。
2. **Deposit**: 将 `amount` 数量的 Token A 从 `Maker ATA A` 转入 `Vault`。

---

### 3.2 Take (完成交易)

Taker 支付 Token B，换取 Vault 中的 Token A。

```txt
Start: Take Instruction
       |
       v
+----------------+      1. Transfer Token B    +----------------+
|  Taker ATA B   | --------------------------> |  Maker ATA B   |
| (Source Token) |                             | (Dest Token)   |
+----------------+                             +----------------+
       |
       |
       v
   [Verification] <--- Check: Is Token B amount == Escrow.receive?
       |
       | (If Valid)
       |
       v
+----------------+      2. Transfer Token A    +----------------+
|     Vault      | --------------------------> |  Taker ATA A   |
| (Source Token) |    (Signed by Escrow PDA)   | (Dest Token)   |
+----------------+                             +----------------+
       |
       | 3. Close Account
       v
+----------------+
|     Maker      | <--- Rent Lamports returned to Maker
+----------------+
```

**关键步骤**:
1. **Transfer to Maker**: Taker 将指定数量的 Token B 转给 Maker。
2. **Withdraw from Vault**: 程序通过 PDA 签名，将 Vault 中的 Token A 转给 Taker。
3. **Close Vault**: 交易完成后，关闭 Vault 和 Escrow 账户，剩余的 SOL (Rent) 退还给 Maker。

---

### 3.3 Refund (退款/取消)

Maker 决定取消订单，取回 Token A。

```txt
Start: Refund Instruction
       |
       v
+----------------+      1. Transfer Token A    +----------------+
|     Vault      | --------------------------> |  Maker ATA A   |
| (Source Token) |    (Signed by Escrow PDA)   | (Return Funds) |
+----------------+                             +----------------+
       |
       | 2. Close Account
       v
+----------------+
|     Maker      | <--- Rent Lamports returned to Maker
+----------------+
```

**关键步骤**:
1. **Withdraw**: 程序通过 PDA 签名，将 Vault 中的 Token A 全部退还给 Maker。
2. **Close**: 关闭 Vault 和 Escrow 账户，回收存储空间租金。

---

## 4. 代码结构说明

```
programs/blueshift_anchor_escrow/src/
├── lib.rs            # 程序入口，定义指令路由 (make, take, refund)
├── state.rs          # 定义 Escrow 结构体 (数据存储格式)
├── errors.rs         # 自定义错误码
└── instructions/     # 具体业务逻辑实现
    ├── mod.rs        # 模块导出
    ├── make.rs       # Make 指令逻辑：初始化 + 存币
    ├── take.rs       # Take 指令逻辑：转币给 Maker + 提币 + 关闭账户
    └── refund.rs     # Refund 指令逻辑：退币 + 关闭账户
```

---

## 5. 环境操作指南 (WSL 用户必读)

**特别注意**: 在 Windows 系统下开发 Solana 智能合约，**所有**涉及编译、测试、部署的命令都必须在 WSL (Ubuntu) 子系统中执行。

### 正确的命令执行方式

每一次你打开终端或 Windows PowerShell，第一件事就是输入 `wsl` 进入 Linux 环境。

**步骤 1**: 进入 WSL
```powershell
wsl
```
*(注意：输入 wsl 后回车，等待终端变为 Linux 样式，例如 `username@machine:~$`)*

**步骤 2**: 在 WSL 中执行后续命令
例如，编译项目的正确操作流程：

```bash
# 1. 编译
anchor build

# 2. 测试
anchor test
```

**错误示例 (绝对禁止)**:
❌ `wsl anchor build` (不要把命令拼在一起)

**正确示例**:
✅ 第一行: `wsl`
✅ 第二行: `anchor build`

---

### 常用命令参考

**构建项目**:
```bash
anchor build
```

**运行测试**:
```bash
anchor test
```

**部署程序 (Devnet)**:
```bash
solana config set --url devnet
anchor deploy
```
