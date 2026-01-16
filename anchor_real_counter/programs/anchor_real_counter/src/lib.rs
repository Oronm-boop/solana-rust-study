use anchor_lang::prelude::*;

// 1. 声明程序 ID (Program ID)
// -------------------------------------------------------------
// 这是程序在 Solana 链上的唯一标识地址。
// 当你运行 `anchor build` 时，Anchor 会自动生成一个新的密钥对并在 target/deploy 下保存。
// 如果这里的值和生成的公钥不匹配，部署时会报错。
declare_id!("CmUmkqkPs3bTyWciXxWHws3LGDSBQRFZcHc9wU3M8Uz8");


// 2. 程序模块 (Program Module)
// -------------------------------------------------------------
// #[program] 是 Anchor 的核心宏，它定义了整个模块是一个 Solana 智能合约。
// 这里的每个 public 函数都会变成一个可以被外部调用的 "指令" (Instruction/RPC method)。
#[program]
pub mod anchor_real_counter {
    // 引入父模块的所有内容，通常是为了使用下面定义的 Account 结构体
    use super::*;

    // 指令 A: 初始化 (Initialize)
    // ctx: Context<Initialize> -> 包含了执行这个指令所需的所有账户上下文信息
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // 获取 counter 账户的可变引用
        // 注意：这里能获取到 &mut，是因为在 Initialize 结构体里我们标注了 #[account(init, ...)]
        // Anchor 已经帮我们在底层完成了创建账户的复杂过程。
        let counter = &mut ctx.accounts.counter;
        
        // 设置初始状态
        counter.count = 0;
        
        // 记录谁是这个计数器的管理员 (Authority)
        // ctx.accounts.user.key() 获取签名者的公钥
        // 以后通过 checked constraint 来确保只有他能修改数据
        counter.authority = ctx.accounts.user.key();
        
        // 记录日志 (Compute Budget 允许的情况下，方便调试)
        msg!("Counter Initialized!");
        Ok(())
    }

    // 指令 B: 增加 (Increment)
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        // 直接获取数据引用并修改
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!("Counter incremented. Current: {}", counter.count);
        Ok(())
    }

    // 指令 C: 减少 (Decrement)
    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        
        // 检查逻辑 (require! 宏)
        // 如果条件 (count > 0) 不满足，则抛出 CounterError::CountBelowZero 错误，并回滚整个交易。
        // 这比 Rust 原生的 panic! 更节省计算资源，且前端能收到具体的错误码。
        require!(counter.count > 0, CounterError::CountBelowZero);
        
        counter.count -= 1;
        msg!("Counter decremented. Current: {}", counter.count);
        Ok(())
    }
}


// 3. 账户状态结构定义 (Data Structures)
// -------------------------------------------------------------

// #[account] 宏
// 标记这个结构体是一个 Solana 账户的数据布局。
// Anchor 会自动为它序列化/反序列化 (Borsh)，
// 并添加一个 8 字节的 "Discriminator" (鉴别器头)，用于在链上安全地区分不同的账户类型。
#[account]
pub struct Counter {
    pub authority: Pubkey, // 管理员公钥 (32 bytes)
    pub count: u64,        // 计数数值 (8 bytes)
}


// 4. 指令上下文验证 (Context Validation)
// -------------------------------------------------------------
// 这里是 Anchor 最强大的地方：分离 "验证逻辑" 和 "业务逻辑"。
// 在执行指令函数之前，Anchor 会先检查这里的 Account 约束条件。

// Context: Initialize
#[derive(Accounts)]
pub struct Initialize<'info> {
    // #[account(init, ...)]
    // 表示我们要创建并初始化这个新账户。
    #[account(
        init, 
        // payer = user: 谁付钱开通这个账户的空间(Rent)？这里指定是 user。
        payer = user, 
        // space = ...: 需要分配多少字节的空间？
        // 8 (Discriminator) + 32 (Pubkey) + 8 (u64)。
        // 建议稍微多留一点余地，或者精确计算。
        space = 8 + 32 + 8 
    )]
    pub counter: Account<'info, Counter>,
    
    // #[account(mut)]: 标记为可变，因为要从它的余额里扣除 SOL 来付租金。
    #[account(mut)]
    pub user: Signer<'info>, // Signer 类型表示这个账户必须对交易签名，否则交易失败。
    
    // 凡是 create_account 必须调用 System Program，所以必须把它传进来。
    pub system_program: Program<'info, System>,
}

// Context: Increment
#[derive(Accounts)]
pub struct Increment<'info> {
    // #[account(mut)]: 我们要修改 count 的值，所以必须是可变的。
    // has_one = authority: 【安全检查】
    // 这是一个非常便捷的约束。它会自动检查: counter.authority == authority.key()
    // 意思就是：只有这个计数器的管理员，才能调用这个指令！
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    
    pub authority: Signer<'info>,
}

// Context: Decrement
#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    
    pub authority: Signer<'info>,
}


// 5. 错误定义 (Errors)
// -------------------------------------------------------------
#[error_code]
pub enum CounterError {
    #[msg("Counter cannot go below zero")] // 这里的 msg 是报错时给用户看的
    CountBelowZero,
}
