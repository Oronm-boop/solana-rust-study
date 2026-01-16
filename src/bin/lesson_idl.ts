import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
// 在真实项目中，IDL 的类型定义会自动生成到这里
import { AnchorRealCounter } from "../anchor_real_counter/target/types/anchor_real_counter";

/**
 * 🎓 Extra Lesson: TypeScript 客户端与 IDL 交互
 * =================================================================
 * 
 * 现在的 DApp 开发模式是：前端 (TS) <--> 胶水层 (Anchor Client) <--> 链上 (Rust Program)
 * 
 * IDL (Interface Description Language) 是这三者之间的桥梁。
 * 它就像 REST API 的 Swagger/OpenAPI 文档，告诉前端：
 * "我有哪些指令？需要传什么参数？需要哪些账户？"
 */

async function main() {
    // 1. 设置 Provider (环境配置)
    // -------------------------------------------------------------
    // Provider = Connection (节点连接) + Wallet (私钥签名)
    // Anchor 会自动从环境变量 (ANCHOR_PROVIDER_URL, ANCHOR_WALLET) 读取
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);


    // 2. 加载程序 (Load Program)
    // -------------------------------------------------------------
    // 以前我们需要手动写 Buffer 布局来解析数据，现在不用了。
    // Anchor 生成的 IDL JSON 文件里包含了所有结构体定义。
    // 
    // Anchor.workspace 会自动扫描 target/idl 目录加载程序。
    const program = anchor.workspace.AnchorRealCounter as Program<AnchorRealCounter>;


    // 3. 构造与发送交易 (RPC Calls)
    // -------------------------------------------------------------
    // 这是 Anchor 最爽的地方：链式调用 (Method Builder Pattern)。
    // 以前写 web3.js 要手动构造 TransactionInstruction, 这里只需要一行。

    const counterKeypair = anchor.web3.Keypair.generate();

    const txSignature = await program.methods
        .initialize() // <--- A. 指定调用哪个指令 (函数名)
        .accounts({   // <--- B. 传入账户上下文 (Context)
            // 这里的 key 必须和 Rust #[derive(Accounts)] 里定义的字段名一模一样！
            counter: counterKeypair.publicKey,
            user: provider.wallet.publicKey,
            // (systemProgram 在新版 Anchor 里可以省略，自动推导)
        })
        .signers([counterKeypair]) // <--- C. 补充签名者 (Signers)
        // .instruction()  <--- 如果你只想构建指令但不发交易，用这个
        // .transaction()  <--- 如果你想构建 Transaction 对象，用这个
        .rpc(); // <--- D. 发送交易并确认 (Send & Confirm)

    console.log("Transaction sent:", txSignature);


    // 4. 获取账户数据 (Fetch Account)
    // -------------------------------------------------------------
    // 如何读取链上数据？直接用 program.account.<AccountStructName>.fetch()

    // fetch: 获取单个账户
    const accountData = await program.account.counter.fetch(
        counterKeypair.publicKey
    );

    console.log("Counter Value:", accountData.count.toString()); // count 是 BN (BigNumber)
    console.log("Authority:", accountData.authority.toBase58());


    // 5. 过滤与批量获取 (Filters & Fetch All)
    // -------------------------------------------------------------
    // 如果我想找到 "所有属于当前用户创建的计数器" 怎么办？

    const myCounters = await program.account.counter.all([
        {
            memcmp: {
                offset: 8, // 跳过 8 字节的 Discriminator
                bytes: provider.wallet.publicKey.toBase58(), // 匹配 authority 字段
            },
        },
    ]);

    console.log(`Found ${myCounters.length} counters.`);
}

/**
 * 💡 总结：
 * 
 * Rust (后端) 写好了 "业务逻辑" 和 "数据结构"。
 * Anchor Build 编译出 IDL。
 * TypeScript (前端) 拿着 IDL，像调用本地函数一样调用链上指令。
 * 
 * 这就是 Solana 开发之所以高效的秘密。
 */
