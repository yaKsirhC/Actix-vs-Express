use mysql::{*, prelude::Queryable};
use std::{hash::{Hash, Hasher}, collections::hash_map};

fn connect_mysql() -> Result<PooledConn, Box<dyn std::error::Error>> {
    let url = "mysql://root:mongoDB@localhost:3306/research";
    let pool = Pool::new(url)?;
    let conn = pool.get_conn()?;
  
    Ok(conn)
}

fn get_user_hash(str: String) -> u64{
    let mut hasher = hash_map::DefaultHasher::new();
    str.hash(&mut hasher);
    hasher.finish()
  
  }

pub fn create_user() -> Result<(), Box<dyn std::error::Error> > {
    let mut conn = connect_mysql()?;
    
    let _ = conn.exec_drop("insert into rs_user(id, name, email, password) values (NULL,:name, :email, :password);", params! {
        "name" => "Chris Kay",
        "email" => "chrisuser199@gmail.com",
        "password" => get_user_hash("1234567890-1234567890".to_string()).to_string(),
      }).unwrap();

    Ok(())
}

pub fn create_post(uid: u16) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = connect_mysql()?;

    let _ = conn.exec_drop("insert into rs_post(pid, uid, title, body, tags) values (NULL,:uid, :title, :body, :tags);", params! {
        "uid" => uid,
        "title" => "Hello world",
        "body" => "hello this is a sample, please follow me...",
        "tags" => ["programming", "web servers", "software development"].join(" "),
      }).unwrap();

    Ok(())
}