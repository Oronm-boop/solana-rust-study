// 1. 结构体 (Structs) - 定义账户状态 (Account State)
// ------------------------------------------------------------------
// 在 Solana 中，所有存贮在链上的数据都是以 Struct 形式定义的。

#[derive(Debug)] // 允许我们用 {:?} 打印结构体
struct UserProfile {
    is_initialized: bool,
    lamports: u64,
    pubkey: [u8; 32], // 模拟 Pubkey
}

// 2. 枚举 (Enums) - 定义指令 (Instructions)
// ------------------------------------------------------------------
// Solana 程序通过匹配不同的 Instruction 枚举变体来执行不同的逻辑。

#[derive(Debug)]
enum MyInstruction {
    Initialize, // 无参数
    Transfer { amount: u64 }, // 带参数 (类似 transfer 指令)
    CloseAccount, 
}

fn main() {
    println!("=== Solana Rust 基础: 结构体与枚举 (Structs & Enums) ===");


    // 初始化一个账户
    let mut user = UserProfile {
        is_initialized: true,
        lamports: 1000,
        pubkey: [1; 32],
    };

    println!("[Account] 用户状态: {:?}", user);

    let instruction = MyInstruction::Transfer { amount: 500 };
    println!("[Instruction] 接收到指令: {:?}", instruction);


    // 3. Option<T> - 处理可能为空的数据
    // ------------------------------------------------------------------
    // 场景: 用户可能设置了头像 URL，也可能没设置。
    
    let avatar_url: Option<String> = None; // 没设置
    // let avatar_url = Some(String::from("https://...")); // 设置了

    match avatar_url {
        Some(url) => println!("用户头像: {}", url),
        None => println!("用户未设置头像"),
    }


    // 4. Result<T, E> - 错误处理
    // ------------------------------------------------------------------
    // 这是 Solana 中最重要的部分。几乎所有从链上交互的函数都返回 Result。
    
    println!("\n--- 模拟转账操作 ---");
    match process_transfer(&mut user, 2000) {
        Ok(_) => println!("✅ 转账成功!"),
        Err(e) => println!("❌ 转账失败: {}", e),
    }
    
    match process_transfer(&mut user, 500) {
        Ok(_) => println!("✅ 转账成功!"),
        Err(e) => println!("❌ 转账失败: {}", e),
    }
    
    println!("最终余额: {}", user.lamports);
}

// 模拟处理转账的函数
// 返回 Result<(), String>: 成功返回 Ok(()), 失败返回 Err(错误信息)
fn process_transfer(user: &mut UserProfile, amount: u64) -> Result<(), String> {
    // 为什么需要加上&mut接收？
    // 因为我们不希望user对象传入函数后，所有权被转移，导致函数外无法再访问user变量
    if !user.is_initialized {
        return Err(String::from("账户未初始化"));
    }
    
    if user.lamports < amount {
        return Err(String::from("余额不足 (Insufficient Funds)"));
    }

    user.lamports -= amount;
    Ok(()) // 成功，返回空元组
}
