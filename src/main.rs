fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use seq_macro::seq;
    use sqlx::MySqlPool;

    // Generate 100 empty SQLx tests using a MySqlPool
    // 100 tests is typically enough to generate a few failures, increase test count if need be
    seq!(N in 0..=99 {

        #[sqlx::test]
        fn test~N(_db_pool: MySqlPool) {}
    });
}
