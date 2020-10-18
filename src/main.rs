use colored::Colorize;
use clap::{App,load_yaml};

//cargo run -- input --type type
fn main(){
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    if let Some(o) = matches.value_of("type") {
        println!("Value for output: {}", o.red());
    }
    if let Some(input) = matches.value_of("INPUT") {
        println!("Value for input: {}", input.green());
    }

    if let Some(v) = matches.value_of("version") {
        println!("Value for version: {}", v.red());
    }


    let connection = sqlite::open("/tmp/.xlog.db").unwrap();
    //CREATE TABLE users (name TEXT, age INTEGER);

    // connection
    // .execute(
    //     "
    //     INSERT INTO users VALUES ('Alice', 42);
    //     INSERT INTO users VALUES ('Bob', 69);
    //     ",
    //     )
    //     .unwrap();

    //let list = connection.execute("SELECT * FROM xlog order by id desc limit 20");

    let mut s = String::from("");
    connection.iterate("SELECT * FROM xlog order by id desc limit 20", |xlog| {
        for &(column, value) in xlog.iter() {
            //todo
            // if(){
            //     continue;
            // }

            match value {
                Some(v) => {
                    if s == ""{
                        s = format!("{}{:16} {}",s,column.green(),v.to_string().blue());
                    }else{
                        s = format!("{}\n{:16} {}",s,column.green(),v.to_string().blue());
                    }
                },
                _ => {
                    s = format!("{}\n{:16} {}",s,column.green(),"--".blue());
                }
            }
            // println!("{:?},{:?}", column,value.unwrap());
            //println!("{},{}", column.blue(),value.unwrap().red());
            // println!("{} -> {}", column.blue(),value.unwrap().red());
            // println!("{} = {}", column.blue(), value.unwrap());
        }
        true
    }).unwrap();
    println!("{}",s);

}

#[derive(Debug)]
struct Xlog {
    id              : u64,
    content         : String,
    log_type        : String,
    log_level       : u8,
    appointment_time: u64,
    reminder_time   : u64,
    extra           : String,
    add_stamp       : u64,      //毫秒时间戳
    add_time        : String,   //2020-10-03 20:29:23
    add_year        : u8,       //年
    add_month       : u8,       //月
    add_date        : u8,       //日
    add_day         : u8,       //星期
    add_hour        : u8,       //小时
    add_minute      : u8,       //分钟
    add_second      : u8,       //秒
}

impl Xlog{
    // fn get_last()
}

#[derive(Debug)]
struct XlogType{
    log_type : String,   //类型
    type_desc: String    //描述
}
