use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;

// An alias to the type for a pool of Diesel SQLite connections.
type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

// The URL to the database, set via the `DATABASE_URL` environment variable.
static DATABASE_URL: &'static str = env!("DATABASE_URL");

/// Initializes a database pool.
pub fn init_pool() -> SqlitePool {
    let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_URL);
    Pool::new(manager).expect("db pool")
}
