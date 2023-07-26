use crate::api::api::{CreateUserRequest, DeleteUserrequest, UserLogin};
use postgres::{Error, Row};
use tokio_postgres::{Client, NoTls};

// DBConn will handle db connection
pub struct DBConn {
    pub client: Client,
}

impl DBConn {
    // new creates new DBConn instance and returns Its variant or an Error
    // it creates a new db connection, spawns a new connection.await,
    // moves client to the field of DBconn and returns it
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

    // create_table creates a table
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

    // register_user takes in user creation request and creates a new user in the table
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

    pub async fn delete_user(&mut self, user: &DeleteUserrequest) -> Result<(), Error> {
        if let Err(err) = self
            .client
            .execute("DELETE FROM app_user WHERE username=$1", &[&user.username])
            .await
        {
            println!("ERROR while trying to delete a user");
            return Err(err);
        }
        Ok(())
    }

    pub async fn login_user(&mut self, user_login: UserLogin) -> Result<(), Error> {
        let value = self
            .client
            .query(
                "SELECT id WHERE username=$1 AND password=$2",
                &[&user_login.username, &user_login.password],
            )
            .await;

        let mut row: Vec<Row> = vec![];
        if let Err(err) = value {
            println!("Error while authenticating user");
            return Err(err);
        }

        row = value.unwrap();
        Ok(())
    }
}
