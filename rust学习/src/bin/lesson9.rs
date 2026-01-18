// =================================================================
// 模拟 Solana 核心类型 (Mocking Core Types)
// =================================================================
#[derive(Debug)]
struct Pubkey([u8; 32]);

#[derive(Debug)]
struct AccountInfo<'a> {
    key: &'a Pubkey,
    is_signer: bool,
    is_writable: bool,
}

#[derive(Debug)]
struct Instruction {
    program_id: Pubkey,
    accounts: Vec<AccountMeta>,
    data: Vec<u8>,
}

#[derive(Debug)]
struct AccountMeta {
    pubkey: Pubkey,
    is_signer: bool,
    is_writable: bool,
}

// 模拟 solana_program::program::invoke
// 能够在程序中调用另一个程序
fn invoke(instruction: &Instruction, _account_infos: &[AccountInfo]) {
    println!("\n[CPI Call] 正在调用程序: {:?}", instruction.program_id);
    println!("  - 发送数据: {:?}", instruction.data);
    println!("  - 涉及账户数: {}", instruction.accounts.len());
    println!("  >> 调用成功 (Simulated)");
}

// 模拟 invoke_signed (用于 PDA 签名)
fn invoke_signed(
    instruction: &Instruction, 
    _account_infos: &[AccountInfo], 
    signers_seeds: &[&[&[u8]]]
) {
    println!("\n[CPI Call Signed] 正在以 PDA 身份调用程序: {:?}", instruction.program_id);
    println!("  - 签名种子: {:?}", signers_seeds);
    println!("  >> 签名验证通过，调用成功 (Simulated)");
}

// =================================================================
// 课程内容: CPI
// =================================================================

fn main() {
    println!("=== Solana Rust 高级: CPI (跨程序调用) ===");

    // 假设我们要写一个程序，帮你把 SOL 转给你的朋友
    // 我们的程序自己没法变出钱，所以必须 "调用 (Invoke)" System Program 的 transfer 指令。

    let system_program_id = Pubkey([0; 32]); // 模拟 System Program ID
    let from_pubkey = Pubkey([1; 32]);
    let to_pubkey = Pubkey([2; 32]);

    // 1. 构建指令 (Construct Instruction)
    // ------------------------------------------------------------------
    // 一个标准的 Solana 指令包含三部分：
    // 1. 调用的程序 ID (这里是 System Program)
    // 2. 涉及的账户列表 (AccountMeta)
    // 3. 指令数据 (Data) - 告诉程序具体干什么 (比如 Transfer, amount=100)
    
    let instruction = Instruction {
        program_id: system_program_id,
        accounts: vec![
            AccountMeta { pubkey: Pubkey([1; 32]), is_signer: true, is_writable: true }, // From
            AccountMeta { pubkey: Pubkey([2; 32]), is_signer: false, is_writable: true }, // To
        ],
        // 假设 [2, 0, 0, ...] 代表 Transfer 指令 + 金额
        data: vec![2, 100, 0, 0, 0, 0, 0, 0], 
    };

    // 2. 准备 AccountInfo
    // ------------------------------------------------------------------
    // CPI 要求我们将 AccountInfo 也传过去，以便被调用的程序可以读写这些账户。
    
    let from_acc = AccountInfo { key: &from_pubkey, is_signer: true, is_writable: true };
    let to_acc = AccountInfo { key: &to_pubkey, is_signer: false, is_writable: true };
    
    let account_infos = [from_acc, to_acc];

    // 3. 执行 invoke (普通调用)
    // ------------------------------------------------------------------
    // 场景: 用户签了名调用你的程序，你的程序拿着用户的签名去调用 System Program。
    // 这叫 "权限传递"。
    invoke(&instruction, &account_infos);


    // 4. 执行 invoke_signed (PDA 签名)
    // ------------------------------------------------------------------
    // 场景: 这是一个 "金库" 账户 (PDA)，它没有私钥。
    // 只有你的程序能通过 "种子 (Seeds)" 证明自己是它的主人，从而控制它转账。
    
    let seeds = b"my_vault"; // 本质上就是一个字节数组切片
    let bump = 254;  // 需要转成切片，可转成包含一个元素的字节切片
    let signer_seeds: &[&[u8]] = &[seeds, &[bump]];    // &[&[u8]], &[u8]是切片类型
    
    invoke_signed(
        &instruction, 
        &account_infos, 
        &[signer_seeds] // 传入种子，运行时会自动生成签名
    );
    
    println!("\n恭喜！你理解了 Solana 乐高积木的核心机制：用别人的程序干活。");
}
