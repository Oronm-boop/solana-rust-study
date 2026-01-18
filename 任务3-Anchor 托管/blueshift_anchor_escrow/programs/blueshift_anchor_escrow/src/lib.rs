use anchor_lang::prelude::*;

mod state;
mod errors;
mod instructions;

use instructions::*;

declare_id!("3XecnsANxY9SWjYfJA4vdr11RveqiC97cyRrtjrxaRSu");

#[program]
pub mod blueshift_anchor_escrow {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn make(
        ctx: Context<Make>, 
        seed: u64,    // 唯一种子：用于生成唯一的 Escrow 账户地址，防止同一用户创建重复订单
        receive: u64, // 期望接收数量：Maker 想要交换得到的 Token B 的数量
        amount: u64   // 存款数量：Maker 存入 Vault 的 Token A 的数量
    ) -> Result<()> {
        instructions::make::handler(ctx, seed, receive, amount)
    }

    #[instruction(discriminator = 1)]
    pub fn take(ctx: Context<Take>) -> Result<()> {
        // take 指令只需要 Context 上下文，因为它所需要的所有账户（Signer, Escrow, Vault, Token Accounts）
        // 都已经包含在 Context<Take> 里的结构体定义中了，不需要额外的数值参数。
        instructions::take::handler(ctx)
    }

    #[instruction(discriminator = 2)]
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        // refund 指令同样只需要 Context 上下文。
        // 它只需要 Maker 签名确认，以及对应的 Escrow 和 Vault 账户即可执行退款逻辑。
        instructions::refund::handler(ctx)
    }
}
