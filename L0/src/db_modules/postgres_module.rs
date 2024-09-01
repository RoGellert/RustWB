use log::error;
use tokio_postgres::{Client, Error, NoTls};

pub struct PostgresDB {
    client: Client,
}

pub async fn connect_to_postgres(
    host: &str,
    user: &str,
    password: &str,
    dbname: &str,
) -> Result<PostgresDB, Error> {
    let (client, connection) = tokio_postgres::connect(
        &format!(
            "host={} user={} password={} dbname={}",
            host, user, password, dbname
        ),
        NoTls,
    )
    .await?;

    let postgres_instance = PostgresDB { client };

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("connection error: {e}");
        }
    });

    Ok(postgres_instance)
}

impl PostgresDB {
    pub async fn create_table(&self, table_name: &str) -> Result<(), Error> {
        self.client
            .query(
                &format!("CREATE TABLE {} (brand VARCHAR(255));", table_name),
                &[],
            )
            .await?;

        Ok(())
    }

    pub async fn drop_table(&self, table_name: &str) -> Result<(), Error> {
        self.client
            .query(&format!("DROP TABLE {};", table_name), &[])
            .await?;

        Ok(())
    }
}
