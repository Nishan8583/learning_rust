use crate::api::api::CreateUserRequest;
use postgres::Error;
use tokio_postgres::{Client, Connection, NoTls};

pub struct DBConn {
    pub client: Client,
}

impl DBConn {
    pub async fn new() -> Result<DBConn, Error> {
        let (client, connection) = tokio_postgres::connect(
            "user=postgres password=mysecretpassword host=localhost",
            NoTls,
        )
        .await
        .unwrap();

        tokio::spawn(async move {
            if let Err(err) = connection.await {
                eprintln!("connection error {}", err);
            };
        });

        return Ok(DBConn { client: client });
    }
    pub async fn create_table(&mut self) {
        self.client
            .batch_execute(
                "
            CREATE TABLE IF NOT EXISTS app_user (
                id              SERIAL PRIMARY KEY,
                username        VARCHAR UNIQUE NOT NULL,
                password        VARCHAR NOT NULL,
                email           VARCHAR UNIQUE NOT NULL
                )
        ",
            )
            .await
            .unwrap();
    }

    pub async fn register_user(&mut self, user: &CreateUserRequest) -> Result<(), Error> {
        if let Err(err) = self
            .client
            .execute(
                "INSERT INTO app_user (username, password, email) VALUES ($1, $2, $3)",
                &[&user.username, &user.password, &user.email],
            )
            .await
        {
            return Err(err);
        }

        Ok(())
    }
}
