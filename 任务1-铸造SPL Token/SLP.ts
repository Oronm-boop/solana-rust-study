import {
    Keypair,
    Connection,
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
} from "@solana/web3.js";

import {
    createAssociatedTokenAccountInstruction,
    createInitializeMint2Instruction,
    createMintToCheckedInstruction, // 使用 Checked 指令是最佳实践，它可以防止精度错误
    MINT_SIZE,
    getMinimumBalanceForRentExemptMint,
    TOKEN_PROGRAM_ID,
    getAssociatedTokenAddressSync,
    ASSOCIATED_TOKEN_PROGRAM_ID
} from "@solana/spl-token";

import bs58 from "bs58";

// 从钱包文件中导入我们的密钥对
const feePayer = Keypair.fromSecretKey(
    // ⚠️ 不安全的密钥。请勿在本次挑战之外使用。
    bs58.decode(process.env.SECRET!)
);

// 创建到 RPC 端点的连接
const connection = new Connection(
    process.env.RPC_ENDPOINT!,
    "confirmed"
);

// TypeScript 代码的入口点
async function main() {
    try {
        // 为铸币账户生成一个新的密钥对
        const mint = Keypair.generate();

        // 获取铸币账户免租金所需的最小余额
        const mintRent = await getMinimumBalanceForRentExemptMint(connection);

        // --- 从这里开始 ---

        // 1. 创建铸币账户 (Mint Account)
        // 我们使用 SystemProgram 在链上为账户分配空间
        const createAccountIx = SystemProgram.createAccount({
            fromPubkey: feePayer.publicKey, // 支付租金的账户
            newAccountPubkey: mint.publicKey, // 新铸币账户的公钥
            lamports: mintRent, // 免租金所需的 lamports
            space: MINT_SIZE, // 铸币账户所需的空间大小
            programId: TOKEN_PROGRAM_ID, // 该账户的所有者程序 (Token Program)
        });


        // 2. 初始化铸币账户
        // 设置精度为 6，并将铸币权限 (Mint Authority) 和冻结权限 (Freeze Authority) 设置为付费人 (你)。
        const decimals = 6;
        const initializeMintIx = createInitializeMint2Instruction(
            mint.publicKey,
            decimals,
            feePayer.publicKey, // 铸币权限
            feePayer.publicKey, // 冻结权限
            TOKEN_PROGRAM_ID
        );


        // 3. 创建关联代币账户 (ATA)
        // 首先，我们需要推导出 ATA 的地址
        const associatedTokenAccount = getAssociatedTokenAddressSync(
            mint.publicKey,
            feePayer.publicKey,
            false,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );

        // 创建初始化 ATA 的指令
        const createAssociatedTokenAccountIx = createAssociatedTokenAccountInstruction(
            feePayer.publicKey, // 支付账户创建费用的账户
            associatedTokenAccount, // 要创建的 ATA 地址
            feePayer.publicKey, // 账户所有者
            mint.publicKey, // 对应的铸币 (Mint)
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );


        // 4. 铸造 21,000,000 个代币到关联代币账户
        // 我们必须考虑到精度：21,000,000 * 10^6
        const tokenAmount = 21_000_000 * Math.pow(10, decimals);

        // 使用 createMintToCheckedInstruction 进行铸造 (比普通 mintTo 更安全，因为它会检查精度)
        const mintToCheckedIx = createMintToCheckedInstruction(
            mint.publicKey,
            associatedTokenAccount,
            feePayer.publicKey, // 铸币权限
            tokenAmount,
            decimals // 精度检查
        );


        // 获取最新的区块哈希
        const recentBlockhash = await connection.getLatestBlockhash();

        // 构建交易
        const transaction = new Transaction({
            feePayer: feePayer.publicKey,
            blockhash: recentBlockhash.blockhash,
            lastValidBlockHeight: recentBlockhash.lastValidBlockHeight
        }).add(
            createAccountIx,
            initializeMintIx,
            createAssociatedTokenAccountIx,
            mintToCheckedIx
        );

        // 发送并确认交易
        // 签名者列表说明:
        // 1. feePayer: 签名以支付交易费用和账户租金。
        // 2. mint: 必须签名，因为我们正在创建一个全新的账户地址 (Mint Account)，需要证明所有权。
        const transactionSignature = await sendAndConfirmTransaction(
            connection,
            transaction,
            [feePayer, mint]
        );

        console.log("铸币地址 (Mint Address):", mint.publicKey.toBase58());
        console.log("交易签名 (Transaction Signature):", transactionSignature);


    } catch (error) {
        console.error(`哎呀，出错了: ${error}`);
    }
}

main();