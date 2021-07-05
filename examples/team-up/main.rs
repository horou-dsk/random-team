use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::error::Error;
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    name: String,
    team: u8,
    wz: String,
    hero: String,
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let users = ["陈刚", "陈清川", "刘相成", "小毛驴", "周光强", "于茂"];
    let client = reqwest::Client::new();
    loop {
        for index in 0..users.len() {
            println!("{}.{}", index, users[index]);
        }
        println!("请输入名称编号：");
        let mut index = String::new();
        std::io::stdin().read_line(&mut index).expect("读取编号错误");
        if let Ok(index) = &index[..1].parse::<usize>() {
            if *index < users.len() {
                let mut map = HashMap::new();
                map.insert("name", users[*index]);
                let res = client.post("http://47.108.64.61:9699/team_up")
                    .json(&map)
                    .send()
                    .await?;
                match print_result(res).await {
                    Ok(_) => {},
                    Err(_) => {
                        eprintln!("数据解析错误！");
                    }
                }
                continue
            }
        }
        eprintln!("编号错误，请重新输入！");
    }
}

async fn print_result(res: reqwest::Response) -> Result<(), Box<dyn Error>> {
    let data: Value = res.json().await?;
    let user: User = serde_json::from_value(data["data"].clone())?;
    println!("姓名：{}，队伍：{}, 位置：{}, 英雄：{}",
             user.name, user.team + 1, user.wz, user.hero);
    Ok(())
}
