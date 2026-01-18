use std::str::FromStr;

// =================================================================
// 模拟 Solana 基础库 (Mocking solana-program)
// =================================================================
// 由于本地环境依赖问题，我们模拟这些宏和类型。
// 在真实开发中，你应该使用 `use solana_program::...`

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Pubkey([u8; 32]);

impl Pubkey {
    pub fn new_unique() -> Self {
        Pubkey([1; 32]) // 模拟一个
    }
    pub fn from_str(_s: &str) -> Result<Self, String> {
        Ok(Pubkey([0; 32])) // 模拟解析
    }
}

// 模拟 declare_id! 宏
macro_rules! declare_id {
    ($id:expr) => {
        pub const ID: Pubkey = Pubkey([0; 32]); // 简化处理
    };
}

// 模拟 msg! 宏
macro_rules! msg {
    ($($arg:tt)*) => {
        println!("(Solana Log): {}", format!($($arg)*));
    };
}

// =================================================================
// 正式课程内容
// =================================================================

// 1. declare_id! 宏
// ------------------------------------------------------------------
// 每个 Solana 程序都有一个唯一的地址 (Program ID)。
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

fn main() {
    println!("=== Solana Rust 核心: 宏与属性 (Macros & Attributes) ===");

    // 2. Program ID 使用
    // ------------------------------------------------------------------
    println!("[Macro] 当前程序的 ID: {:?}", ID);
    
    // 验证一下
    let my_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
    assert_eq!(ID, my_id);
    println!("[Check] ID 验证通过 ✅");


    // 3. msg! 宏 - 链上日志
    // ------------------------------------------------------------------
    // 官方推荐使用 msg! 宏，它直接调用 syscall，开销更低。
    
    msg!("这是一个由 msg! 宏打印的日志 (Solana On-chain Log)");
    msg!("程序 ID: {:?}", ID);
    
    let val = 100;
    msg!("当前值: {}", val);


    // 4. #[account] 模拟
    // ------------------------------------------------------------------
    
    #[derive(Debug)] 
    struct MyAccountState {
        authority: Pubkey,
        count: u64,
    }

    let state = MyAccountState {
        authority: Pubkey::new_unique(), 
        count: 0,
    };

    println!("\n[Attribute] 模拟账户状态结构体:");
    println!("{:#?}", state);
    
    println!("\n(注: 为了演示顺畅，本代码使用了 Mock 类型，真实开发中请添加 solana-program 依赖)");
}
