use mysql;

static DATABASE_URL: &'static str = env!("DATABASE_URL");

pub fn connect() -> mysql::Pool {
    mysql::Pool::new(DATABASE_URL).unwrap()
}