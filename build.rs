use log::{info, warn};
use std::cmp::min;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use toml::Table;

fn get_database_url() -> Result<String, Box<dyn Error>> {
    let mut config = String::new();
    File::open("app_config.toml")?.read_to_string(&mut config)?;
    let table: Table = config.parse()?;
    Ok(String::from(
        table
            .get("database")
            .ok_or("Key not found")?
            .get("url")
            .ok_or("Key not found")?
            .as_str()
            .ok_or("Key not String")?,
    ))
}

fn main() {
    if !env::var("DATABASE_URL").is_ok_and(|url| url.len() > 0) {
        let database_url = get_database_url().unwrap_or_else(|err|{
            println!("cargo:warning=Database url not found.\n未找到数据库连接配置，建议配置此项目以进行编译期的SQL动态检查。\n\n\
                    配置方法：\n\
                    1. 配置app.config.toml并填写相关字段\n\
                    2. 配置环境变量DATABASE_URL\n");
            panic!("{}", err);
        });
        println!("cargo:rustc-env=DATABASE_URL={}", database_url);
        println!(
            "正在使用Database Url: {}... 进行编译期检查",
            database_url.get(0..database_url.len().min(10)).unwrap()
        );
    }
}
