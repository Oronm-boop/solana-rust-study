# Anchor Escrow 测试指南

本文档将指导你如何在 Solana Devnet 上测试 Anchor Escrow 程序。

## 1. 准备工作

### 1.1 环境配置
1.  确保你的浏览器钱包 (Phantom / Solflare) 已安装并切换到 **Devnet** 网络。
2.  确保你的钱包中有足够的 **SOL** (Devnet) 用于支付手续费。
    - 你可以在 [faucet.solana.com](https://faucet.solana.com/) 领取测试 SOL。

### 1.2 准备测试代币 (Mint A & Mint B)
你需要两种 SPL Token 来模拟交易：
- **Mint A**: Maker (你) 想要出售的代币。
- **Mint B**: Maker (你) 想要换取的代币。

如果你已经安装了 Solana CLI，可以使用以下命令创建：

**创建 Mint A**
```bash
spl-token create-token
spl-token create-account <Mint-A-Address>
spl-token mint <Mint-A-Address> 1000
```

**创建 Mint B**
```bash
spl-token create-token
spl-token create-account <Mint-B-Address>
spl-token mint <Mint-B-Address> 1000
```

请记下这两个 Token 的 **Mint Address (地址)**。

---

## 2. 测试流程

启动前端项目：
```bash
cd app
npm run dev
```

### 场景一：正常交易 (Make -> Take)

#### 第一步：创建订单 (Make)
1.  **连接钱包**: 点击右上角的 "Select Wallet" 并连接。
2.  在 "创建订单 (Make)" 卡片中填写信息：
    - **种子 (Seed)**: 输入一个随机数字 (例如 `101`)。注意：相同的种子不能重复使用。
    - **存入代币地址 (Mint A)**: 粘贴你刚才创建的 Token A 的地址。
    - **存入数量 (Amount A)**: 输入 `10` (表示你想卖出 10 个 Token A)。
    - **接收代币地址 (Mint B)**: 粘贴 Token B 的地址。
    - **接收数量 (Amount B)**: 输入 `20` (表示你想换取 20 个 Token B)。
3.  点击 **"创建担保订单"**。
4.  在钱包中批准交易。
5.  **验证**: 打开 Solana Explorer (Devnet)，查看你的钱包交易记录。你会发现 Token A 已经从你的账户转出去了。

#### 第二步：完成订单 (Take)
*为了方便测试，你可以使用同一个钱包“左手倒右手”（Self-Trade），也可以切换另一个钱包账号。*

1.  在 "完成订单 (Take)" 卡片中：
    - **Escrow 账户地址**: 这里需要填入上一步生成的 Escrow PDA 地址。
    - *小技巧*：目前的简易 UI 没有自动显示生成的 Escrow 地址，你需要去 Explorer查看刚才那笔 Make 交易，找到与之交互的程序账户中，那个 **数据大小为 Escrow 结构体大小** 的账户（通常是除了 Token Program 和 System Program 外的那个新创建账户）。或者在控制台 Console 查看我们打印的日志（目前还未打印具体的 PDA，稍后我会优化代码让它弹出来）。
2.  点击 **"完成订单 (Take)"**。
3.  批准交易。
4.  **验证**: 检查钱包余额。你应该收到了 20 个 Token B，同时 Escrow 账户被关闭，租金回收。

---

### 场景二：退款 (Make -> Refund)

#### 第一步：创建订单 (Make)
重复上述 Make 步骤，使用一个新的 Seed (例如 `102`)。

#### 第二步：执行退款 (Refund)
1.  在 "退款 (Refund)" 卡片中：
    - **Escrow 账户地址**: 填入这一笔订单的 Escrow PDA 地址。
2.  点击 **"执行退款 (Refund)"**。
3.  批准交易。
4.  **验证**: 检查钱包余额。你存入的 Token A 应该全额退回到了你的账户。

---

## 3. 常见问题
- **Error: Account Not Initialized**: 检查你填写的 Mint 地址是否正确，以及你的钱包是否真的拥有这些 Token。
- **Error: Constraint Seeds**: 说明 Escrow 地址算错了，或者你不是该订单的 Maker。
