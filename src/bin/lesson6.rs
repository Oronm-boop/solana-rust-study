use borsh::{BorshDeserialize, BorshSerialize};

// =================================================================
// 模拟 Solana Pubkey
// =================================================================
// 在真实环境中，Pubkey 来自 solana-program 库。
// 这里我们在本地定义它，并 deriving Borsh traits，
// 这样就能完美演示序列化过程，且没有任何依赖问题。

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct Pubkey([u8; 32]);

impl Pubkey {
    pub fn new_unique() -> Self {
        // 返回一个填充了 1 的公钥
        Pubkey([1; 32])
    }
}

// =================================================================
// 正式课程内容
// =================================================================

// 1. Traits (特质)
trait Printable {
    fn print_info(&self);
}

// 2. 序列化结构体
#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct GameAccountState {
    is_active: bool,     // 1 byte
    level: u32,          // 4 bytes
    player: Pubkey,      // 32 bytes
    item_count: u64,     // 8 bytes
}

impl Printable for GameAccountState {
    fn print_info(&self) {
        println!("\n[Trait] 玩家等级: {}, 物品数量: {}", self.level, self.item_count);
    }
}

fn main() {
    println!("=== Solana Rust 核心: Trait 与 序列化 (Traits & Serialization) ===");

    // 创建一个实例
    let state = GameAccountState {
        is_active: true,
        level: 10,
        player: Pubkey::new_unique(),
        item_count: 999,
    };
    
    state.print_info();

    // 3. 序列化演示 (Struct -> Bytes)
    let encoded_data: Vec<u8> = state.try_to_vec().unwrap();
    
    println!("\n[Serialize] 序列化后的二进制数据 (长度: {}):", encoded_data.len());
    println!("{:?}", encoded_data);
    
    // 验证长度: 1 + 4 + 32 + 8 = 45 bytes
    assert_eq!(encoded_data.len(), 45);


    // 4. 反序列化演示 (Bytes -> Struct)
    let decoded_state = GameAccountState::try_from_slice(&encoded_data).unwrap();
    
    println!("\n[Deserialize] 反序列化回结构体:");
    println!("{:#?}", decoded_state);
    
    assert_eq!(state.level, decoded_state.level);
    assert_eq!(state.player, decoded_state.player);
    
    println!("\n恭喜！你搞懂了 Solana 上数据存取的奥秘 (Mock 版运行成功)。");
}
