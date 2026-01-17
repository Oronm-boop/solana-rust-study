use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// 0. 定义 ProgramResult (Pinocchio 0.6 可能移除了在这个路径下的定义)
pub type ProgramResult = Result<(), ProgramError>;

// 1. 定义入口 (Entrypoint)
entrypoint!(process_instruction);

// 2. 核心处理逻辑
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Entrypoint: Pinocchio Counter");

    // [Step 1] 获取账户
    let account_iter = &mut accounts.iter();
    
    // 获取第一个账户 (Counter)
    let counter_account = next_account(account_iter)?;

    // [Step 2] 安全检查
    if counter_account.owner() != program_id {
        msg!("Error: Incorrect program id");
        return Err(ProgramError::IncorrectProgramId);   
    }
    
    // [Step 3] 零拷贝读写
    // 注意：具体 borrow 方法可能因版本不同而略有差异，这里假设 try_borrow_mut_data 可用
    // 如果报错，可能需要使用 unsafe 直接操作 data 字段（AccountInfo 内部通常是裸指针）
    // 为了最简单的演示，我们尝试直接访问（依赖 crate 具体实现，如果报错再改）
    
    match counter_account.try_borrow_mut_data() {
        Ok(mut data) => {
            if data.len() < 8 {
                return Err(ProgramError::AccountDataTooSmall);
            }
            let mut count = u64::from_le_bytes(data[0..8].try_into().unwrap());
            msg!("Current Count: {}", count);
            count += 1;
            data[0..8].copy_from_slice(&count.to_le_bytes());
            msg!("New Count: {}", count);
        },
        Err(_) => return Err(ProgramError::AccountBorrowFailed),
    }
    
    Ok(())
}

// 辅助函数：获取下一个账户
fn next_account<'a, I>(iter: &mut I) -> Result<I::Item, ProgramError>
where
    I: Iterator<Item = &'a AccountInfo>,
{
    iter.next().ok_or(ProgramError::NotEnoughAccountKeys)
}

