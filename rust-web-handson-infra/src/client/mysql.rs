use std::{env, sync::Arc};

use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

#[derive(Clone)]
pub struct Rds(pub(crate) Arc<Pool<MySql>>);

impl Rds {
    pub async fn new() -> Rds {
        let pool = MySqlPoolOptions::new()
            .max_connections(
                *&env::var("DATABASE_CONNECTIONS")
                    .unwrap_or_else(|_| panic!("DATABASE_CONNECTIONS must be set!"))
                    .parse::<u32>()
                    .unwrap_or_else(|_| panic!("DATABASE_CONNECTIONS must be integer!")),
            )
            .connect(
                &env::var("DATABASE_URL").unwrap_or_else(|_| panic!("DATABASE_URL must be set!")),
            )
            .await
            .unwrap_or_else(|_| {
                panic!("Cannot connect to the database. Please check your configuration.")
            });
        Rds(Arc::new(pool))
    }
}
