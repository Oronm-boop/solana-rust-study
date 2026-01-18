
#[derive(Debug)]
enum SolanaInstruction {
    InitializeAccount,
    TransferLamborts { to: String, amount: u64 },
    UpdateData(String), // 元组变体
    CloseAccount,
}

fn main() {
    println!("=== Solana Rust 基础: 模式匹配 (Pattern Matching) ===");

    // 1. match 表达式 - 指令分发 (Instruction Dispatch)
    // ------------------------------------------------------------------
    // 在原生 Solana 开发中，entrypoint 会收到序列化后的指令数据，
    // 解析成枚举后，使用 match 来分发逻辑。

    let instruction = SolanaInstruction::TransferLamborts { 
        to: String::from("Pubkey123..."), 
        amount: 5000 
    };

    println!("[收到指令] 开始处理...");

    // process_instruction 模拟 Solana 程序的主逻辑
    process_instruction(instruction);
    process_instruction(SolanaInstruction::InitializeAccount);
    process_instruction(SolanaInstruction::UpdateData(String::from("New State")));


    // 2. if let - 简化匹配
    // ------------------------------------------------------------------
    // 场景: 我们只关心某个特定的 Option 是否有值，或者枚举是否是某个特定变体。
    
    let config_option: Option<u64> = Some(100);

    // 繁琐写法
    match config_option {
        Some(val) => println!("\n[Config] 配置值为: {}", val),
        None => (), // 必须处理 None，即使用空代码块
    }

    // 简洁写法 (if let)
    if let Some(val) = config_option {
        println!("[Config] (if let) 配置值为: {}", val);
    }
    
    // 3. 匹配守卫 (Match Guards)
    // ------------------------------------------------------------------
    // 在匹配的同时增加额外的条件判断
    
    let amount = 0;
    match amount {
        0 => println!("\n[Check] 金额为 0，跳过操作"),
        val if val < 100 => println!("[Check] 金额太小: {}", val),
        _ => println!("[Check] 金额有效"),
    }
}

fn process_instruction(ix: SolanaInstruction) {
    match ix {
        SolanaInstruction::InitializeAccount => {
            println!(">> 执行: 初始化账户逻辑");
        },
        // 解构 (Destructuring): 直接把内部数据提取出来赋值给 to 和 amount
        SolanaInstruction::TransferLamborts { to, amount } => {
            println!(">> 执行: 转账 {} lamports 给账户 {}", amount, to);
        },
        SolanaInstruction::UpdateData(data) => {
            println!(">> 执行: 更新数据为 '{}'", data);
        },
        // 通配符: 处理剩余所有情况
        _ => {
            println!(">> 执行: 其他指令或关闭账户");
        }
    }
}
