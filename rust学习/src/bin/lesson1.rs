fn main() {
    println!("=== Solana Rust 基础: 变量与可变性 ===");

    // 1. 变量与可变性 (Variables & Mutability)
    // ------------------------------------------------------------------
    // 场景: 用户由于转账，余额发生变化
    let user_wallet_balance_initial = 1000; 
    // let balance = 1000; // 默认是不可变的 (immutable)
    // balance = 900; // ❌ 报错: cannot assign twice to immutable variable

    let mut user_wallet_balance = 1000; // 加上 mut 关键字
    println!("初始余额: {} lamports", user_wallet_balance);

    // 模拟转账 100 lamports
    user_wallet_balance = user_wallet_balance - 100;
    println!("转账后余额: {} lamports", user_wallet_balance);


    // 2. 基础数据类型 (Basic Types in Solana context)
    // ------------------------------------------------------------------
    
    // [u64] 
    // Solana 中金额 (Lamports) 都是 u64 类型 (64位无符号整数)
    let amount: u64 = 5000000000; // 5 SOL (1 SOL = 10^9 Lamports)
    println!("转账金额: {} lamports", amount);

    // [u8]
    // 通常用于 bump seeds (0-255) 或者字节数据
    let bump: u8 = 254;
    println!("PDA Bump: {}", bump);

    // [Boolean]
    let is_initialized: bool = true;
    if is_initialized {
        println!("账户已初始化");
    }

    // [Pubkey 模拟] -> [u8; 32]
    // 在真正的 Solana 程序中会使用 Pubkey 类型，这里我们用 [u8; 32] 数组模拟底层结构
    let mock_pubkey: [u8; 32] = [0; 32]; // 创建一个全为 0 的 32 字节数组
    println!("模拟公钥 (长度): {} bytes", mock_pubkey.len());

    // [String vs &str]
    // String 是堆上分配的，可增长的
    // &str 是字符串切片，通常是硬编码的或者借用的
    let Program_name: &str = "MySolanaProgram"; // &str
    let mut log_message = String::from("处理指令: "); // String
    
    log_message.push_str("Initialize"); // 修改 String
    
    // msg! 宏通常接受 &str
    println!("[Log] {} - Program: {}", log_message, Program_name);

    println!("\n恭喜！你已经掌握了 Solana 最基础的变量概念。");
}
