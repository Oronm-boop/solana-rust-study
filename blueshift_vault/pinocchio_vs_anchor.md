# Pinocchio vs Anchor：从 Anchor 开发者视角看 Pinocchio 代码

本文档旨在帮助习惯了 Anchor 框架的开发者理解 Pinocchio 代码（以 `blueshift_vault` 为例）。

**核心区别**：Anchor 帮你自动处理了所有底层繁琐的工作（"魔法"），而在 Pinocchio 中，为了极致的性能和零依赖，所有事情都必须**手动完成**。

---

## 1. 标准库与 Panic 处理

### Anchor
默认开启 `std`，帮你处理内存分配和 Panic 捕获。你基本不需要关心程序挂了会怎么样，框架会返回错误码。

### Pinocchio
**代码对应**：
```rust
#![no_std] 
// ...
nostd_panic_handler!(); 
```
*   **`#![no_std]`**：必须禁用标准库，因为我们没有任何运行时支持。
*   **`nostd_panic_handler!()`**：必须手动定义“程序崩溃时该怎么办”。它告诉机器：如果代码出错了（Panic），请打印错误位置并立即停止运行 (`abort`)。

---

## 2. 程序的入口点与 ID

### Anchor
使用宏 `declare_id!("...");` 和 `#[program]` 属性宏自动处理路由。

### Pinocchio
**代码对应**：
```rust
entrypoint!(process_instruction);
pub const ID: Pubkey = [...];

fn process_instruction(...) { ... }
```
*   **`entrypoint!`**：手动声明入口。
*   **`process_instruction`**：Solana Runtime 会把所有原始数据（Program ID, Accounts 数组, Instruction Data 字节数组）塞给这个函数。你必须亲自处理这些原始数据。

---

## 3. 指令路由 (Discriminator)

### Anchor
自动根据函数名生成 Discriminator，自动匹配并路由到对应的函数（如 `fn deposit(...)`）。

### Pinocchio
**代码对应**：
```rust
match instruction_data.split_first() {
    Some((Deposit::DISCRIMINATOR, data)) => Deposit::try_from((data, accounts))?.process(),
    Some((Withdraw::DISCRIMINATOR, _)) => Withdraw::try_from(accounts)?.process(),
    _ => Err(ProgramError::InvalidInstructionData),
}
```
*   **手动路由**：你必须自己切分 `instruction_data` 的第一个字节（通常定为 Discriminator），然后写 `match` 语句决定调用哪个逻辑。

---

## 4. 账户验证 (硬核部分)

这是区别最大的地方。Anchor 的核心魔法 `#[derive(Accounts)]` 在这里变成了纯手动的 `TryFrom` 实现。

### Anchor 写法
```rust
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, signer)]
    pub owner: Signer<'info>,
    #[account(mut, seeds = [b"vault", owner.key.as_ref()], bump)]
    pub vault: Account<'info, Vault>, // 自动检查 PDA、Owner、数据结构等
    pub system_program: Program<'info, System>,
}
```

### Pinocchio 写法
**代码对应**：`DepositAccounts::try_from`
1.  **解析数组**：你拿到的是一个扁平的 `&[AccountInfo]` 数组，必须自己按顺序解构：
    ```rust
    let [owner, vault, _] = accounts else { ... }; // 必须清楚客户端传账户的顺序！
    ```
2.  **手动检查 Signer**：
    ```rust
    if !owner.is_signer() { ... } // 对应 Anchor 的 signer 约束
    ```
3.  **手动检查 Owner**：
    ```rust
    if vault.owner() != &pinocchio_system::ID { ... } // 对应 Anchor 的 owner 检查
    ```
4.  **手动计算并检查 PDA**：
    ```rust
    let (vault_key, _) = find_program_address(&[b"vault", owner.key()], &crate::ID);
    if vault.key().ne(&vault_key) { ... } // 必须手动对比地址
    ```
    > **注意**：Anchor 帮你自动计算 PDA 并验证传入是否正确。在这里，你必须自己算一遍，然后跟传入的账号对比。如果不对比，黑客就可以传入一个假账号来骗过程序。

---

## 5. 参数反序列化

### Anchor
使用 Borsh 自动序列化/反序列化，参数直接出现在函数签名里 `fn deposit(ctx, amount: u64)`。

### Pinocchio
**代码对应**：
```rust
fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
    // ...
    let amount = u64::from_le_bytes(data.try_into().unwrap());
    // ...
}
```
*   **字节操作**：为了省 Compute Unit，通常直接读字节（如 `from_le_bytes`）。不做复杂的序列化，零拷贝。

---

## 6. 业务逻辑与 CPI

### Anchor
封装了 `CpiContext`，调用其他程序像调用本地函数一样简单：
```rust
anchor_lang::system_program::transfer(cpi_ctx, amount)
```

### Pinocchio
**代码对应**：
```rust
Transfer {
    from: self.accounts.owner,
    to: self.accounts.vault,
    lamports: self.instruction_data.amount,
}
.invoke()?;
```
*   **Invoke**：你得构建类似底层的指令结构体，然后调用 `.invoke()`。
*   **PDA 签名**：如果是 PDA 签名，得用 `.invoke_signed(&signers)`，并且必须手动构建 `seeds` 数组。

---

## 总结

| 概念 | Anchor | Pinocchio |
| :--- | :--- | :--- |
| **开销** | 大（包含大量检查代码，二进制体积大） | **极小**（只包含你写的代码，二进制体积极小） |
| **账户验证** | `#[derive(Accounts)]` 声明式 | **手动 `if` 判断** 过程式 |
| **数据解析** | Borsh 自动 | **字节操作** 手动 (Zero Copy) |
| **安全性** | 框架帮你兜底，不容易出错 | **全靠你自己**（漏写一个 check 就可能有漏洞） |

Pinocchio 虽然写起来繁琐，但它赋予了你对每一个字节、每一个指令的完全控制权。
