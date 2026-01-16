// 1. 错误处理 (Error Handling)
// ------------------------------------------------------------------
// 在 Solana/Anchor 开发中，我们通常定义一个 Error 枚举。
// 使用 require! 宏来进行前置检查 (Pre-checks).

#[derive(Debug)]
enum MyError {
    Unauthorized,
    InsufficientFunds,
    AmountTooSmall,
}

// 模拟 Anchor 的 require! 宏
// 如果条件不满足，直接返回错误
macro_rules! require {
    ($invariant:expr, $error:expr) => {
        if !($invariant) {
            return Err($error);
        }
    };
}

// 2. 泛型 (Generics)
// ------------------------------------------------------------------
// Anchor 框架的核心魔法：Context<T>
// 它允许我们写一套逻辑，然后应用到不同的账户结构上。

struct Context<T> {
    pub accounts: T,
}

// 定义两个不同的账户结构
#[derive(Debug)]
struct TransferAccounts {
    pub from: String,
    pub to: String,
    pub amount: u64,
}

#[derive(Debug)]
struct InitializeAccounts {
    pub new_account: String,
    pub payer: String,
}


fn main() {
    println!("=== Solana Rust 核心: 错误处理与泛型 (Error Handling & Generics) ===");

    // 演示错误处理
    println!("\n[1. Error Handling Check]");
    match transfer_logic(100, 500) {
        Ok(_) => println!("✅ 转账成功"),
        Err(e) => println!("❌ 转账失败: {:?}", e),
    }

    match transfer_logic(1000, 500) {
        Ok(_) => println!("✅ 转账成功"),
        Err(e) => println!("❌ 转账失败: {:?}", e),
    }


    // 演示泛型 Context
    println!("\n[2. Generics Context]");
    
    // 场景 A: 转账上下文
    let transfer_ctx = Context {
        accounts: TransferAccounts {
            from: "Alice".to_string(),
            to: "Bob".to_string(),
            amount: 100,
        }
    };
    process_transfer(transfer_ctx);

    // 场景 B: 初始化上下文
    let init_ctx = Context {
        accounts: InitializeAccounts {
            new_account: "NewWallet".to_string(),
            payer: "Alice".to_string(),
        }
    };
    process_initialize(init_ctx);

    println!("\n恭喜！你已经理解了 Anchor 框架最核心的 Context<T> 设计思想。");
}


// 模拟业务逻辑函数
fn transfer_logic(amount: u64, balance: u64) -> Result<(), MyError> {
    // 检查 1: 金额不能为 0
    require!(amount > 0, MyError::AmountTooSmall);
    
    // 检查 2: 余额必须充足
    require!(balance >= amount, MyError::InsufficientFunds);

    println!(">> 正在转账 {} ...", amount);
    Ok(())
}

// 模拟 Anchor 的处理函数
// 注意看参数：ctx: Context<TransferAccounts>
fn process_transfer(ctx: Context<TransferAccounts>) {
    let accounts = ctx.accounts;
    println!(">> [Handler] 处理转账: 从 {} 到 {}", accounts.from, accounts.to);
}

// 注意看参数：ctx: Context<InitializeAccounts>
// 虽然都是 Context，但因为泛型 T 不同，内部包含的数据也不同。
fn process_initialize(ctx: Context<InitializeAccounts>) {
    let accounts = ctx.accounts;
    println!(">> [Handler] 处理初始化: 创建者 {}", accounts.payer);
}
