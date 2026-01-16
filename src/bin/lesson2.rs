fn main() {
    println!("=== Solana Rust 基础: 所有权与借用 (Ownership & Borrowing) ===");

    // 1. 所有权规则 (Ownership)
    // ------------------------------------------------------------------
    // 核心规则: Rust 中每个值都有一个变量作为它的 owner。一次只能有一个 owner。
    
    // 模拟一个账户数据 (String 在堆上，没有 Copy trait)
    let account_data = String::from("Some Account Data");
    
    // 所有权转移 (Move)
    // 在 Solana 中，这类似于你把一个账户的所有权移交给了另一个函数去处理，
    // 原来的变量就不能用了。
    let another_variable = account_data; 
    
    // println!("{}", account_data); // ❌ 报错: value borrowed here after move
    println!("数据的所有权现在属于: another_variable: {}", another_variable);


    // 2. 借用 (Borrowing / References) - 只读引用 &
    // ------------------------------------------------------------------
    // 场景: 我们只是想读取账户余额进行检查，不需要修改它，也不想拿走所有权。
    
    let wallet_balance = 5000u64;
    
    // 传递引用 (&)
    check_balance(&wallet_balance); 
    
    // 因为只是借用，原变量还可以继续使用
    println!("Main 函数依然拥有 balance: {}", wallet_balance);


    // 3. 可变借用 (Mutable Borrowing) - 可写引用 &mut
    // ------------------------------------------------------------------
    // 场景: 我们需要修改账户余额。
    // 规则: 在同一作用域内，只能有一个可变引用 (&mut)，或者多个不可变引用 (&)。
    // 不能同时拥有可变和不可变引用。
    
    let mut my_lamports = 1000u64;
    
    println!("\n[操作前] 余额: {}", my_lamports);
    
    // 创建一个可变借用传给函数
    add_lamports(&mut my_lamports, 500);
    
    println!("[操作后] 余额: {}", my_lamports);
    
    
    // 4. Solana 常见错误演示
    // ------------------------------------------------------------------
    let mut account = String::from("Account Info");
    
    let r1 = &account; 
    let r2 = &account;
    // let r3 = &mut account; // ❌ 报错: cannot borrow `account` as mutable because it is also borrowed as immutable
    
    println!("Safe borrows: {}, {}", r1, r2);
}

// 模拟一个只接收引用的函数 (比如 Anchor 中的 view 函数)
fn check_balance(balance: &u64) {
    println!("[Check] 正在检查余额: {}", balance);
    // *balance = 0; // ❌ 报错: cannot assign to `*balance` which is behind a `&` reference
}

// 模拟一个需要修改数据的函数 (比如 Anchor 中的 mutation 函数)
fn add_lamports(balance: &mut u64, amount: u64) {
    *balance += amount; // 解引用并修改值
    println!("[Update] 增加 {} lamports", amount);
}
