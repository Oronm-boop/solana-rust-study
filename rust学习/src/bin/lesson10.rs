// =================================================================
// 模拟 Solana Pubkey 及 PDA 逻辑
// =================================================================
#[derive(Debug, PartialEq, Clone, Copy)]
struct Pubkey([u8; 32]);

impl Pubkey {
    // 模拟 find_program_address
    pub fn find_program_address(seeds: &[&[u8]], _program_id: &Pubkey) -> (Pubkey, u8) {
        println!("\n[System] 正在寻找合法的 PDA (耗费计算资源)...");
        let bump = 254; 
        
        let mut mock_addr = [0u8; 32];
        if seeds.len() > 0 {
            mock_addr[0] = seeds[0].get(0).copied().unwrap_or(0); 
        }
        mock_addr[31] = bump; 

        (Pubkey(mock_addr), bump)
    }
    
    // 模拟 create_program_address
    pub fn create_program_address(_seeds: &[&[u8]], _program_id: &Pubkey) -> Result<Pubkey, String> {
        println!("[Check] 正在验证 PDA 是否合法 (低开销)...");
        Ok(Pubkey([0; 32])) 
    }
}

// =================================================================
// 模拟 Anchor 框架结构
// =================================================================

// 1. 账户结构 (Account State) - 必须包含 bump 字段
#[derive(Debug)]
struct UserDepositAccount {
    pub owner: Pubkey,
    pub amount: u64,
    pub bump: u8, // <--- 关键：Anchor 最佳实践，存下来！
}

// 2. 上下文 (Context) - 模拟 Anchor 自动计算 bump
struct InitializeContext {
    // Anchor 会把所有验证过的 PDA 的 bump 放在这个 map 里
    pub bumps: std::collections::HashMap<String, u8>, 
}


// =================================================================
// 课程内容: PDA & Anchor Best Practices
// =================================================================

fn main() {
    println!("=== Solana Rust 最终章: PDA 与 Anchor 最佳实践 ===");

    let program_id = Pubkey([255; 32]);
    let user_pubkey = Pubkey([1; 32]);
    
    // 模拟 Anchor 幕后工作:
    // 在你的指令执行前，Anchor 已经调用 find_program_address 帮你算好了。
    let (_, _canonical_bump) = Pubkey::find_program_address(&[], &program_id);
    
    let mut bumps = std::collections::HashMap::new();
    bumps.insert("user_account".to_string(), 254); // 假设算出来是 254
    
    let ctx = InitializeContext { bumps };

    // ------------------------------------------------------------------
    // Q: 听说 Bump 是前端算的？
    // A: 是的，但不全对。流程是这样的：
    // 
    // 1. 【前端 (JS)】: 为了发交易，必须先 `find_program_address` 算出 PDA 地址，
    //     否则前端都不知道要把哪个 Account 塞进交易里发给链上。
    // 
    // 2. 【链上 (Anchor)】: 收到交易后，Anchor 会根据你的 `#[account(seeds=...)]` 宏，
    //     再次计算并验证传入的 PDA 账户是否合法。
    //     验证通过后，Anchor 会把算出来的正确 bump 塞进 `ctx.bumps`。
    // 
    // 3. 【存储】: 我们相信 Anchor 的验证结果，所以直接存 `ctx.bumps` 里的值。
    // ------------------------------------------------------------------

    // [拓展] 前端 JS/TS 如何计算 PDA? 
    // ----------------------------------------------------
    // const { PublicKey } = require("@solana/web3.js");
    // 
    // const programId = new PublicKey("Fg6PaF...");
    // const userPubkey = new PublicKey("User...");
    // 
    // // 注意: findProgramAddressSync 是同步方法，findProgramAddress 是异步
    // const [pda, bump] = PublicKey.findProgramAddressSync(
    //   [
    //     Buffer.from("user_deposit"), // 种子1: 字符串转 Buffer
    //     userPubkey.toBuffer(),       // 种子2: 公钥转 Buffer
    //   ],
    //   programId
    // );
    // 
    // console.log("PDA:", pda.toString());
    // console.log("Bump:", bump);




    // ------------------------------------------------------------------
    // Step 1: 初始化账户 (Initialization)
    // ------------------------------------------------------------------
    println!("\nStep 1: 初始化账户 (Anchor Style)");
    
    // 在 Anchor 中，我们不需要自己调 find_program_address。
    // 直接从 ctx.bumps 里拿！
    let bump = *ctx.bumps.get("user_account").unwrap();
    println!(">> [Anchor] 从 Context 中获取系统预算的 Bump: {}", bump);
    
    let mut user_account = UserDepositAccount {
        owner: user_pubkey,
        amount: 1000,
        bump: bump, // <--- 存入账户数据
    };
    println!(">> [Action] 账户已初始化，Bump 永久存储链上。");
    println!(">> 账户状态: {:#?}", user_account);


    // ------------------------------------------------------------------
    // Step 2: 取款 (Withdraw) - 使用存储的 Bump
    // ------------------------------------------------------------------
    println!("\nStep 2: 取款 (Withdraw)");
    
    withdraw(&program_id, &user_pubkey, &user_account);
    
    
    println!("\n恭喜！你不仅搞懂了 PDA 原理，还掌握了 Anchor 的工程化写法。");
}

fn withdraw(program_id: &Pubkey, user_pubkey: &Pubkey, account: &UserDepositAccount) {
    let seed_prefix = b"user_deposit";

    // 关键点：直接从 Account 数据里读 bump，不要重新算！
    let saved_bump = account.bump;
    println!(">> [Program] 读取存储的 Bump: {}", saved_bump);

    // 构造签名种子
    let signer_seeds = &[
        seed_prefix.as_slice(),
        &user_pubkey.0,
        &[saved_bump], // 直接使用
    ];

    // 模拟签名调用
    match Pubkey::create_program_address(signer_seeds, program_id) {
        Ok(_) => println!("✅ 签名验证成功！PDA 资金已转移。"),
        Err(_) => println!("❌ 签名失败"),
    }
}
