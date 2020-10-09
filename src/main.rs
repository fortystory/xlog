use colored::Colorize;
use sqlite::*;

fn main() {
    let connection = sqlite::open("/Users/nice/.xlog.db").unwrap();
    // println!("{:?}",connection);
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

    connection.iterate("SELECT * FROM xlog order by id desc limit 20", |pairs| {
        for &(column, value) in pairs.iter() {
            // match value {
            //     Some(v) => println!("{:?},{:?}", column,v)
            //     _ => (),
            // }
            // println!("{:?},{:?}", column,value.unwrap());
            println!("{},{}", column.blue(),value.unwrap().red());
            // println!("{} = {}", column.blue(), value.unwrap());
        }
        true
    })
    .unwrap();
    println!("{}", "abc".to_string().red());
}

#[derive(Debug)]
struct Xlog {
    id: u64,
    content: String,
    log_type: String,
    log_level: u8,
    appointment_time: u64,
    reminder_time: u64,
    extra: String,
    add_stamp: u64,   //毫秒时间戳
    add_time: String, //2020-10-03 20:29:23
    add_year: u8,     //年
    add_month: u8,    //月
    add_date: u8,     //日
    add_day: u8,      //星期
    add_hour: u8,     //小时
    add_minute: u8,   //分钟
    add_second: u8,   //秒
}

impl Xlog{
    // fn get_last()
}

#[derive(Debug)]
struct XlogType{
    log_type: String, //类型
    type_desc:String  //描述
}

