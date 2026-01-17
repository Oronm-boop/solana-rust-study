import { LiteSVM } from "litesvm";
import { PublicKey, Transaction, Keypair, SystemProgram, ComputeBudgetProgram } from "@solana/web3.js";
import { assert } from "chai";
import { TOKEN_PROGRAM_ID, AccountLayout, MintLayout, ACCOUNT_SIZE, MINT_SIZE } from "@solana/spl-token";

describe("LiteSVM 教程", () => {
    let svm: LiteSVM;
    // 示例程序 ID (在真实测试中应替换为你的程序 ID)
    const programId = new PublicKey("22222222222222222222222222222222222222222222");

    beforeEach(() => {
        // 在每个测试前创建一个新的 LiteSVM 实例，确保状态隔离
        svm = new LiteSVM();
    });

    it("应该创建一个基础的系统账户 (System Account)", () => {
        const payer = Keypair.generate();
        // 模拟空投：直接修改账户余额
        // 在 LiteSVM 中，我们可以随意给予任何账户任意数量的 SOL
        svm.airdrop(payer.publicKey, 100_000_000_000n);

        // 验证余额是否更新
        const balance = svm.getBalance(payer.publicKey);
        assert.equal(balance, 100_000_000_000n);
    });

    it("应该设置并获取自定义账户数据", () => {
        const account = Keypair.generate();
        const customData = Buffer.from([1, 2, 3, 4]); // 模拟一些二进制数据

        // 手动设置账户状态
        // 这是一个非常强大的功能，允许我们直接构造特定的账户状态进行测试
        svm.setAccount(account.publicKey, {
            lamports: 1_000_000,
            data: customData, // 写入自定义数据
            owner: programId, // 将所有者设置为我们的程序 ID
            executable: false,
        });

        // 验证账户是否已正确保存到 LiteSVM 的“账本”中
        const fetchedAccount = svm.getAccount(account.publicKey);
        assert.isNotNull(fetchedAccount);
        assert.deepEqual(fetchedAccount!.data, customData);
    });

    it("应该处理 SPL Token Mint 账户设置 (模拟)", () => {
        // 创建 Mint 和 Owner 的密钥对
        const mint = Keypair.generate();
        const owner = Keypair.generate();

        // 1. 构造 Mint 账户的数据结构
        // 使用 @solana/spl-token 的 Layout 工具进行二进制编码
        const mintData = Buffer.alloc(MINT_SIZE);
        MintLayout.encode(
            {
                mintAuthorityOption: 1, // 有 Mint 权限
                mintAuthority: owner.publicKey,
                supply: BigInt(0), // 初始供应量为 0
                decimals: 6, // 6 位小数
                isInitialized: true,
                freezeAuthorityOption: 0, // 无冻结权限
                freezeAuthority: PublicKey.default,
            },
            mintData
        );

        // 2. 将 Mint 账户写入 LiteSVM
        // 注意 owner 必须是 TOKEN_PROGRAM_ID
        svm.setAccount(mint.publicKey, {
            lamports: 1_000_000_000,
            data: mintData,
            owner: TOKEN_PROGRAM_ID,
            executable: false,
        });

        // 验证 Mint 账户数据长度是否正确
        const fetchedMint = svm.getAccount(mint.publicKey);
        assert.isNotNull(fetchedMint);
        assert.equal(fetchedMint!.data.length, MINT_SIZE);
    });

    it("应该执行一个简单的转账交易", () => {
        const sender = Keypair.generate();
        const receiver = Keypair.generate();

        // 1. 设置发送方账户，给它一些初始资金
        svm.setAccount(sender.publicKey, {
            lamports: 1_000_000_000,
            data: Buffer.alloc(0),
            owner: SystemProgram.programId,
            executable: false
        });

        // 2. 构造转账交易
        const tx = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: sender.publicKey,
                toPubkey: receiver.publicKey,
                lamports: 500_000_000, // 转账 5 SOL
            })
        );

        // 3. 设置最近的区块哈希并签名
        tx.recentBlockhash = svm.latestBlockhash();
        tx.sign(sender);

        // 4. 发送交易到 LiteSVM 执行
        const res = svm.sendTransaction(tx);

        // 5. 验证结果
        // 检查接收方余额是否增加
        const receiverBalance = svm.getBalance(receiver.publicKey);
        assert.equal(receiverBalance, 500_000_000n);
    });

    it("应该可以操作 Sysvars (例如修改时钟 Clock)", () => {
        // 获取当前时钟状态
        const initialClock = svm.getClock();
        // 修改时间戳，模拟时间流逝（例如用于测试锁仓过期）
        initialClock.unixTimestamp = 1234567890n;
        svm.setClock(initialClock);

        // 验证时钟是否被修改
        const newClock = svm.getClock();
        assert.equal(newClock.unixTimestamp, 1234567890n);
    });

    it("应该加载并执行真实的 Anchor 程序 (.so)", () => {
        // 1. 定义程序 ID (必须与 Anchor代码中的 declare_id 一致)
        const anchorProgramId = new PublicKey("F6RoirWhKAwEsChZy3mSm9aYdAxCiZ25W7tS8FYmGD5G");
        // 2. 加载编译好的 .so 文件
        // 路径相对于运行测试的目录
        // 注意：如果你还没有编译 anchor-example，这一步会失败。请先运行 compilation。
        // 为了避免在没有文件时报错导致整个测试套件 crash，我们可以用 try-catch 或者 fs check，
        // 但 LiteSVM 会抛出错误，mocha 会捕获测试失败。
        try {
            svm.addProgramFromFile(anchorProgramId, "../anchor-example/target/deploy/anchor_example.so");
        } catch (e) {
            console.warn("跳过测试：无法加载 .so 文件。请先编译 anchor-example 项目。");
            return;
        }

        // 3. 准备账户
        const user = Keypair.generate();
        const myAccount = Keypair.generate();

        // 给用户一些 SOL 支付 rent 和 gas
        svm.airdrop(user.publicKey, 10_000_000_000n);

        // 4. 构造 Instruction Data
        // Anchor 的 instruction data 前 8 个字节是 discriminator
        // sig("global:initialize") 的前 8 个字节
        // initialize(data: u64) -> data = 42
        // Discriminator for "initialize": [175, 175, 109, 31, 13, 152, 155, 237]
        const discriminator = Buffer.from([175, 175, 109, 31, 13, 152, 155, 237]);
        const dataParam = Buffer.alloc(8);
        dataParam.writeBigUInt64LE(42n); // data = 42

        const instructionData = Buffer.concat([discriminator, dataParam]);

        // 5. 构造交易
        const tx = new Transaction().add({
            keys: [
                { pubkey: myAccount.publicKey, isSigner: true, isWritable: true },
                { pubkey: user.publicKey, isSigner: true, isWritable: true },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
            ],
            programId: anchorProgramId,
            data: instructionData,
        });

        tx.recentBlockhash = svm.latestBlockhash();
        tx.sign(user, myAccount);

        // 6. 执行交易
        const res = svm.sendTransaction(tx);

        // 7. 验证状态
        const fetchedAccount = svm.getAccount(myAccount.publicKey);
        assert.isNotNull(fetchedAccount);
        // Anchor 账户数据前 8 个字节是 Discriminator，后面是 data (u64)
        const accountData = Buffer.from(fetchedAccount!.data);
        const storedValue = accountData.readBigUInt64LE(8);
        assert.equal(storedValue, 42n);
    });
});
