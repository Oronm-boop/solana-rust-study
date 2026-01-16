import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorRealCounter } from "../target/types/anchor_real_counter"; // 引入IDL,anchor_real_counter/target/idl/anchor_real_counter.json
import { assert } from "chai";

describe("anchor_real_counter", () => {
  // 1. 配置 Provider
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorRealCounter as Program<AnchorRealCounter>;

  // 生成一个测试用的 Keypair 作为计数器账户
  const counterAccount = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // 2. 调用 Initialize
    await program.methods
      .initialize()
      .accounts({
        counter: counterAccount.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([counterAccount]) // 必须签名，因为我们是 init 一个新账户
      .rpc();

    // 验证状态
    const account = await program.account.counter.fetch(counterAccount.publicKey);

    console.log("On-chain Count:", account.count.toString());
    assert.ok(account.count.eq(new anchor.BN(0)));
    assert.ok(account.authority.equals(provider.wallet.publicKey));
  });

  it("Increments the counter", async () => {
    await program.methods
      .increment()
      .accounts({
        counter: counterAccount.publicKey,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    const account = await program.account.counter.fetch(counterAccount.publicKey);
    console.log("After Increment:", account.count.toString());
    assert.ok(account.count.eq(new anchor.BN(1)));
  });

  it("Decrements the counter", async () => {
    await program.methods
      .decrement()
      .accounts({
        counter: counterAccount.publicKey,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    const account = await program.account.counter.fetch(counterAccount.publicKey);
    console.log("After Decrement:", account.count.toString());
    assert.ok(account.count.eq(new anchor.BN(0)));
  });

  it("Cannot decrement below zero", async () => {
    try {
      await program.methods
        .decrement()
        .accounts({
          counter: counterAccount.publicKey,
          authority: provider.wallet.publicKey,
        })
        .rpc();
      assert.fail("Should have failed!");
    } catch (error) {
      assert.include(error.message, "Counter cannot go below zero");
      console.log("✅ Expected error caught: Counter cannot go below zero");
    }
  });
});
