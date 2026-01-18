/** 挑战：铸造（Mint）一个 SPL 代币
 *
 * 在这个挑战中，你将创建一个 SPL 代币！
 *
 * 目标：
 * 使用 Web3.js 和 SPL Token 库，在单笔交易中铸造一个 SPL 代币。
 *
 * 任务目标：
 * 1. 创建一个 SPL 铸币账户 (Mint Account)。
 * 2. 初始化铸币账户，设置精度为 6，并将你的公钥 (feePayer) 设置为铸币权限和冻结权限所有者。
 * 3. 为你的公钥 (feePayer) 创建一个关联代币账户 (ATA) 以接收铸造的代币。
 * 4. 铸造 21,000,000 个代币到你的关联代币账户中。
 * 5. 签名并发送交易。
 */

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
    createMintToCheckedInstruction, // 使用 Checked 指令更安全，会检查精度
    MINT_SIZE,
    getMinimumBalanceForRentExemptMint,
    TOKEN_PROGRAM_ID,
    getAssociatedTokenAddressSync,
    ASSOCIATED_TOKEN_PROGRAM_ID
} from "@solana/spl-token";

import bs58 from "bs58";

// 从钱包文件或环境变量导入我们的密钥对
const feePayer = Keypair.fromSecretKey(
    // ⚠️ 极不安全。除了这个练习挑战外，千万不要在代码中直接使用私钥
    bs58.decode(process.env.SECRET!) // 添加 ! 断言它一定存在
);

// 创建一个连接到 RPC 端点的连接对象
const connection = new Connection(
    process.env.RPC_ENDPOINT!,
    "confirmed" // 确认级别
);

// TypeScript 代码入口点（我们将调用此函数）
async function main() {
    try {
        // 生成一个新的密钥对，作为“铸币账户”的地址
        // 注意：这只是生成了一个地址和私钥，此时链上还不存在这个账户
        const mint = Keypair.generate();

        // 获取铸币账户要想“免租金”（Rent Exempt）所需的最小 Lamports 余额
        // 在 Solana 上存储数据需要支付租金，存够一定金额可免租金
        const mintRent = await getMinimumBalanceForRentExemptMint(connection);

        // --- 开始构建指令 ---

        // 1. 创建铸币账户指令 (系统程序负责在链上开辟空间)
        const createAccountIx = SystemProgram.createAccount({
            fromPubkey: feePayer.publicKey,  // 谁出钱付租金
            newAccountPubkey: mint.publicKey,// 新账户的地址
            lamports: mintRent,              // 存入多少钱（刚好够免租）
            space: MINT_SIZE,                // 需要多大的存储空间（SPL Mint 固定大小）
            programId: TOKEN_PROGRAM_ID,     // 这个账户的所有者将是 Token Program
        });

        // 2. 初始化铸币账户指令
        // 设置精度为 6，并将铸币权限和冻结权限都设为 feePayer (也就是你自己)。
        const decimals = 6;
        const initializeMintIx = createInitializeMint2Instruction(
            mint.publicKey,           // 要初始化的铸币账户地址
            decimals,                 // 精度 (Decimals)
            feePayer.publicKey,       // 铸币权限 (Mint Authority) - 谁有权增发代币
            feePayer.publicKey,       // 冻结权限 (Freeze Authority) - 谁有权冻结账户
            TOKEN_PROGRAM_ID          // 代币程序 ID
        );

        // 3. 创建关联代币账户 (ATA)
        // 首先，计算出 ATA 的地址（这是一个确定性的算法，不需要链上交互）
        const associatedTokenAccount = getAssociatedTokenAddressSync(
            mint.publicKey,           // 代币类型 (Mint)
            feePayer.publicKey,       // 账户持有者 (Owner)
            false,                    // 是否允许持有者在曲线外 (通常为 false)
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );

        // 创建“初始化关联代币账户”的指令
        // 注意：如果该账户已存在，这步通常会报错，但在新 Mint 的场景下肯定不存在
        const createAssociatedTokenAccountIx = createAssociatedTokenAccountInstruction(
            feePayer.publicKey,       // 谁付钱创建这个账户 (Payer)
            associatedTokenAccount,   // 要创建的 ATA 地址
            feePayer.publicKey,       // 账户归谁所有 (Owner)
            mint.publicKey,           // 存的是哪种币 (Mint)
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );

        // 4. 铸造 21,000,000 个代币到关联代币账户
        // 记得处理精度问题: 21,000,000 * 10^6
        // 使用 BigInt 防止数字过大溢出
        const mintAmount = BigInt(21_000_000) * BigInt(10 ** decimals);

        const mintToCheckedIx = createMintToCheckedInstruction(
            mint.publicKey,           // 铸币账户 (Mint)
            associatedTokenAccount,   // 接收代币的目标地址 (ATA)
            feePayer.publicKey,       // 谁有权签名进行铸造 (Authority)
            mintAmount,               // 数量
            decimals                  // 精度 (作为安全检查，防止精度搞错)
        );

        // 获取最新的区块哈希，用于构建交易
        const recentBlockhash = await connection.getLatestBlockhash();

        // 创建交易对象并添加所有指令
        const transaction = new Transaction({
            feePayer: feePayer.publicKey,
            blockhash: recentBlockhash.blockhash,
            lastValidBlockHeight: recentBlockhash.lastValidBlockHeight
        }).add(
            createAccountIx,               // 1. 开辟空间
            initializeMintIx,              // 2. 格式化为 Mint
            createAssociatedTokenAccountIx,// 3. 创建接收账户
            mintToCheckedIx                // 4. 发币
        );

        // 5. 签名并发送交易
        const transactionSignature = await sendAndConfirmTransaction(
            connection,
            transaction,
            [feePayer, mint]  // ⚠️ 关键点：这里需要 feePayer (付Gas费) 和 mint (作为新账户的签名者)
        );

        console.log("铸币账户地址:", mint.publicKey.toBase58());
        console.log("交易签名:", transactionSignature);
        console.log(`浏览器查看链接: https://explorer.solana.com/tx/${transactionSignature}?cluster=devnet`); // 假设是 devnet
    } catch (error) {
        console.error(`哎呀，出错了: ${error}`);
    }
}

// 执行主函数
main();