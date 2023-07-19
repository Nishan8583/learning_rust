use database::DB::DBConn;

pub mod database;
fn main() {
    let mut conn = DBConn::new().unwrap();
    conn.create_table();
    conn.register_user(String::from("nishan")).unwrap();
}
