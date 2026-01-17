#![no_std]

use core::mem::size_of;
use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    instruction::{Seed, Signer},
    nostd_panic_handler,
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
    ProgramResult,
};
use pinocchio_system::instructions::Transfer;

// 声明入口点
entrypoint!(process_instruction);

// 声明 Panic 处理器（用于 no_std 环境）
nostd_panic_handler!();

// 你的程序 ID (教程中提供的)
pub const ID: Pubkey = [
    0x0f, 0x1e, 0x6b, 0x14, 0x21, 0xc0, 0x4a, 0x07,
    0x04, 0x31, 0x26, 0x5c, 0x19, 0xc5, 0xbb, 0xee,
    0x19, 0x92, 0xba, 0xe8, 0xaf, 0xd1, 0xcd, 0x07,
    0x8e, 0xf8, 0xaf, 0x70, 0x47, 0xdc, 0x11, 0xf7,
];

// 主处理函数
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        // 0 = Deposit
        Some((Deposit::DISCRIMINATOR, data)) => Deposit::try_from((data, accounts))?.process(),
        // 1 = Withdraw
        Some((Withdraw::DISCRIMINATOR, _)) => Withdraw::try_from(accounts)?.process(),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

// ==========================================
//              DEPOSIT 逻辑
// ==========================================

pub struct DepositAccounts<'a> {
    pub owner: &'a AccountInfo,
    pub vault: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for DepositAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        // 解析账户列表，预期为 [owner, vault, system_program]
        // 注意：教程代码在析构时使用了 [owner, vault, _]，这意味着需要传入 SystemProgram 作为第三个账户
        let [owner, vault, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // 账户检查
        if !owner.is_signer() {
            return Err(ProgramError::InvalidAccountOwner);
        }
        // 检查 Vault 是否归 System Program 所有（未初始化状态）
        if vault.owner() != &pinocchio_system::ID {
            return Err(ProgramError::InvalidAccountOwner);
        }
        // 确保 Vault 是空的
        if vault.lamports().ne(&0) {
            return Err(ProgramError::InvalidAccountData);
        }
        // 验证 PDA 地址
        let (vault_key, _) = find_program_address(&[b"vault", owner.key()], &crate::ID);
        if vault.key().ne(&vault_key) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        Ok(Self { owner, vault })
    }
}

pub struct DepositInstructionData {
    pub amount: u64,
}

impl<'a> TryFrom<&'a [u8]> for DepositInstructionData {
    type Error = ProgramError;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != size_of::<u64>() {
            return Err(ProgramError::InvalidInstructionData);
        }
        let amount = u64::from_le_bytes(data.try_into().unwrap());

        if amount.eq(&0) {
            return Err(ProgramError::InvalidInstructionData);
        }
        Ok(Self { amount })
    }
}

pub struct Deposit<'a> {
    pub accounts: DepositAccounts<'a>,
    pub instruction_data: DepositInstructionData,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Deposit<'a> {
    type Error = ProgramError;
    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = DepositAccounts::try_from(accounts)?;
        let instruction_data = DepositInstructionData::try_from(data)?;
        Ok(Self {
            accounts,
            instruction_data,
        })
    }
}

impl<'a> Deposit<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0;

    pub fn process(&mut self) -> ProgramResult {
        // 调用 System Program 进行转账：Owner -> Vault
        Transfer {
            from: self.accounts.owner,
            to: self.accounts.vault,
            lamports: self.instruction_data.amount,
        }
            .invoke()?;
        Ok(())
    }
}

// ==========================================
//              WITHDRAW 逻辑
// ==========================================

pub struct WithdrawAccounts<'a> {
    pub owner: &'a AccountInfo,
    pub vault: &'a AccountInfo,
    pub bumps: [u8; 1],
}

impl<'a> TryFrom<&'a [AccountInfo]> for WithdrawAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        // 预期账户：[owner, vault, system_program]
        let [owner, vault, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // 基础账户检查
        if !owner.is_signer() {
            return Err(ProgramError::InvalidAccountOwner);
        }
        // 即使有钱，所有者也应该是 System Program (因为这是纯 SOL 转账，没有数据空间所有权变更)
        // 注意：如果你在存款时没有分配数据空间，Vault 仍然归 System Program 所有
        if vault.owner() != &pinocchio_system::ID {
            return Err(ProgramError::InvalidAccountOwner);
        }
        // 确保有钱可取
        if vault.lamports().eq(&0) {
            return Err(ProgramError::InvalidAccountData);
        }

        // 验证 PDA 并获取 bump
        let (vault_key, bump) = find_program_address(&[b"vault", owner.key()], &crate::ID);
        if &vault_key != vault.key() {
            return Err(ProgramError::InvalidAccountOwner);
        }

        Ok(Self { owner, vault, bumps: [bump] })
    }
}

pub struct Withdraw<'a> {
    pub accounts: WithdrawAccounts<'a>,
}

impl<'a> TryFrom<&'a [AccountInfo]> for Withdraw<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let accounts = WithdrawAccounts::try_from(accounts)?;
        Ok(Self { accounts })
    }
}

impl<'a> Withdraw<'a> {
    pub const DISCRIMINATOR: &'a u8 = &1;

    pub fn process(&mut self) -> ProgramResult {
        // 创建 PDA 签名种子
        let seeds = [
            Seed::from(b"vault"),
            Seed::from(self.accounts.owner.key().as_ref()),
            Seed::from(&self.accounts.bumps),
        ];
        let signers = [Signer::from(&seeds)];

        // 调用 System Program 进行转账：Vault -> Owner (带 PDA 签名)
        Transfer {
            from: self.accounts.vault,
            to: self.accounts.owner,
            lamports: self.accounts.vault.lamports(),
        }
            .invoke_signed(&signers)?;

        Ok(())
    }
}