// =================================================================
// 模拟 Solana AccountInfo
// =================================================================
// 为什么这里需要 <'a> ?
// 因为 AccountInfo 中的 `key` 字段是一个引用 (&str)。
// ！！！Rust 编译器必须确保：AccountInfo 这个结构体实例存活的时间，绝对不能超过它引用的那个字符串存活的时间。
// 否则，AccountInfo 就会指向一段被释放的内存（悬垂指针），这是 Rust 绝对禁止的。
#[derive(Debug, Clone)] 
struct AccountInfo<'a> {
    pub key: &'a str, 
    pub is_signer: bool,
    pub is_writable: bool,
    pub lamports: u64,
}

fn main() {
    println!("=== Solana Rust 高级: 迭代器与生命周期 (Iterators & Lifetimes) ===");

    // ------------------------------------------------------------------
    // Part 1: 迭代器 (Iterators) - 快速回顾
    // ------------------------------------------------------------------
    let account_1 = AccountInfo { key: "UserWallet", is_signer: true, is_writable: true, lamports: 1000 };
    let account_2 = AccountInfo { key: "SystemProgram", is_signer: false, is_writable: false, lamports: 0 };
    let accounts = vec![account_1, account_2];
    
    let mut account_iter = accounts.iter();
    println!("\n[Iterators] 这里的 user_acc 本质上也是一个引用: {:?}", account_iter.next().unwrap());


    // ------------------------------------------------------------------
    // Part 2: 生命周期深度解析 (Lifetimes Deep Dive)
    // ------------------------------------------------------------------
    println!("\n[Lifetimes] 深入理解生命周期...");

    // 场景 A: 悬垂引用的噩梦 (Rust 会在编译期阻止它)
    /*
    let r;
    {
        let s = String::from("hello");
        r = &s; // ❌ 报错: `s` does not live long enough
    } // s 在这里被销毁了
    println!("{}", r); // 如果这里允许访问 r，就会读到垃圾数据
    */
    println!(">> Rust 编译器保证了引用永远有效，不会指向被销毁的数据。");


    // 场景 B: 函数中的生命周期
    // ---------------------------------------------------------
    // 假设我们要写一个函数，返回两个账户中 AccountInfo.key 更长的那个 key。
    
    let acc1_key = String::from("ShortKey");
    let acc2_key = String::from("VeryLongKeyName");

    let result;
    {
        // 这里的 result 引用了 acc2_key (或 acc1_key)
        // 只要 acc1_key 和 acc2_key 还没有死，result 就可以活着
        result = longest_key(acc1_key.as_str(), acc2_key.as_str());
        println!(">> 最长的 Key 是: {}", result);
    } 

    // 场景 C: Solana 中的 Context 生命周期
    // ---------------------------------------------------------
    // 你经常看到 fn instruction<'info>(ctx: Context<'_, '_, '_, 'info, ...>) 
    // 这里的 'info 代表 "Solana 运行时传进来的所有账户数据的存活时间"。
    // 你的 Instruction 处理逻辑 (函数体) 肯定比这个时间短，所以是安全的。
    
    let runtime_data = String::from("Solana Runtime Data"); // 模拟运行时数据
    
    // 创建一个引用 runtime_data 的 Context
    let ctx = MyContext {
        account_data: &runtime_data, 
    };
    
    process_instruction(&ctx);
    
    println!("\n总结: 生命周期就是编译器在你看不到的地方，给每个引用打上了 '开始' 和 '结束' 的标签，并确保它们像俄罗斯方块一样完美契合。");
}

// ------------------------------------------------------------------
// 重点: 函数签名中的生命周期
// ------------------------------------------------------------------
// 如果不加 <'a>，编译器就不知道返回的 &str 到底引用的是 x 还是 y。
// 这里的标注含义是:
// "输入参数 x 和 y 至少都活这一样长 ('a)，而返回值的寿命也是 'a (即不会超过 x 和 y 中较短的那个)"
fn longest_key<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 模拟 Solana 的 Context 结构
struct MyContext<'info> {
    pub account_data: &'info str,
}

fn process_instruction<'info>(ctx: &MyContext<'info>) {
    println!(">> [Solana] 处理指令，读取到的数据: {}", ctx.account_data);
    // ctx.account_data 的生命周期是 'info，它保证在函数执行期间，
    // 底层的 runtime_data (String) 绝对不会被销毁。
}
