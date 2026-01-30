// 模拟数据库连接（重量级资源）
struct DatabaseConnection {
    connection_id: u32,
    host: String,
    is_open: bool,
}

impl DatabaseConnection {
    fn new(id: u32, host: &str) -> Self {
        println!("🔌 [连接 {}] 正在连接到数据库: {}", id, host);
        // 模拟建立连接的耗时操作
        std::thread::sleep(std::time::Duration::from_millis(100));
        println!("✅ [连接 {}] 连接成功！", id);

        DatabaseConnection {
            connection_id: id,
            host: host.to_string(),
            is_open: true,
        }
    }

    // 执行查询
    fn query(&self, sql: &str) -> Vec<String> {
        if !self.is_open {
            panic!("连接已关闭！");
        }
        println!("📊 [连接 {}] 执行查询: {}", self.connection_id, sql);
        vec!["结果1".to_string(), "结果2".to_string()]
    }

    // 关闭连接
    fn close(&mut self) {
        if self.is_open {
            println!("🔒 [连接 {}] 关闭数据库连接", self.connection_id);
            self.is_open = false;
            // 释放网络资源、清理缓冲区等
        }
    }
}

// 为 DatabaseConnection 实现 Drop
impl Drop for DatabaseConnection {
    fn drop(&mut self) {
        println!(
            "🧹 [连接 {}{}] Drop trait 被调用，自动清理资源！",
            self.connection_id, self.host
        );
        self.close(); // 确保连接被正确关闭
    }
}

// 用户服务
fn get_user_info(conn: &DatabaseConnection, user_id: u32) -> String {
    // ✨ Deref 的作用：Box<DatabaseConnection> 可以像 &DatabaseConnection 一样使用
    let results = conn.query(&format!("SELECT * FROM users WHERE id = {}", user_id));
    format!("用户信息: {:?}", results)
}

fn get_user_orders(conn: &DatabaseConnection, user_id: u32) -> Vec<String> {
    // ✨ 同样受益于 Deref
    conn.query(&format!("SELECT * FROM orders WHERE user_id = {}", user_id))
}

fn main() {
    println!("=== 数据库连接管理示例 ===\n");

    {
        println!("📦 场景1: 使用 Box 管理数据库连接\n");

        // 创建数据库连接（在堆上，因为是重量级资源）
        let db_conn = Box::new(DatabaseConnection::new(1, "localhost:5432"));

        println!("\n开始业务操作...\n");

        // ✨ Deref 特性：可以直接当作引用使用
        // Box<DatabaseConnection> 自动解引用为 &DatabaseConnection
        let user_info = get_user_info(&db_conn, 101);
        println!("→ {}", user_info);

        let orders = get_user_orders(&db_conn, 101);
        println!("→ 订单数量: {}", orders.len());

        // 直接调用方法（通过 Deref）
        db_conn.query("SELECT COUNT(*) FROM products");

        println!("\n业务操作完成！\n");

        // ✨ Drop 特性：离开作用域时自动调用
        println!("即将离开作用域...");
    } // ← db_conn 在这里自动调用 Drop，清理资源

    println!("\n✅ 作用域结束，资源已自动释放！\n");

    // ========================================

    {
        println!("📦 场景2: 多个连接的管理\n");

        let connections: Vec<Box<DatabaseConnection>> = vec![
            Box::new(DatabaseConnection::new(2, "db1.example.com")),
            Box::new(DatabaseConnection::new(3, "db2.example.com")),
            Box::new(DatabaseConnection::new(4, "db3.example.com")),
        ];

        println!("\n执行批量查询...\n");

        for conn in &connections {
            // ✨ Deref: 直接对 Box 调用方法
            conn.query("SELECT version()");
        }

        println!("\n批量操作完成！");
        println!("即将离开作用域...\n");
    } // ← 所有连接自动调用 Drop，按相反顺序清理

    println!("✅ 所有连接已自动清理！\n");

    // ========================================

    {
        println!("📦 场景3: 提前释放（显式 drop）\n");

        let db_conn = Box::new(DatabaseConnection::new(5, "cache.redis.com"));

        db_conn.query("GET user:101");

        println!("\n手动释放资源...");
        drop(db_conn); // ✨ 显式调用 Drop

        println!("继续执行其他代码...");
        println!("资源已经被清理，内存已释放！\n");
    }

    println!("=== 示例结束 ===");
}
