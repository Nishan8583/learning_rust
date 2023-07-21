use postgres::Error;
use postgres::{Client, NoTls};
pub struct DBConn {
    pub client: Client,
}

impl DBConn {
    pub fn new() -> Result<DBConn, Error> {
        match Client::connect(
            "postgresql://postgres:mysecretpassword@127.0.0.1:5432/postgres",
            NoTls,
        ) {
            Ok(client) => Ok(DBConn { client: client }),
            Err(err) => Err(err),
        }
    }
    pub fn create_table(&mut self) {
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
            .unwrap();
    }

    pub fn register_user(&mut self, username: String) -> Result<(), Error> {
        if let Err(err) = self.client.execute(
            "INSERT INTO app_user (username, password, email) VALUES ($1, $2, $3)",
            &[&username, &"mypass", &"user@test.com"],
        ) {
            return Err(err);
        }

        Ok(())
    }
}
