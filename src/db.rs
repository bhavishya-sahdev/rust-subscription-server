use std::{env, sync::Arc};

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenv::dotenv;

pub type DbPool = Arc<Pool<ConnectionManager<PgConnection>>>;

pub fn init_db() -> DbPool {
    dotenv().ok();
    let database_url = &env::var("DATABASE_URL").expect("DATABASE_URL not found");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create pool");

    // Wrap the pool in an Arc to share across threads
    Arc::new(pool)
}
