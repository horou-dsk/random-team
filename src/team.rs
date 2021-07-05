use rand::prelude::ThreadRng;
use actix::{Actor, Context};
use actix::prelude::*;
use rand::Rng;
use std::fs::File;
use std::io::Read;
use serde::{Serialize, Deserialize};

const USERS: [&str; 6] = ["陈刚", "陈清川", "刘相成", "小毛驴", "周光强", "于茂"];
// const WZ: [&str; 3] = ["上", "中", "下"];

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    name: String,
    team: u8,
    wz: String,
    hero: String,
}

pub struct Team {
    rng: ThreadRng,
    users: Vec<User>,
    heroes: Vec<String>,
    remaining_wz: [Vec<String>; 2],
}

impl Actor for Team {
    type Context = Context<Self>;
}

impl Team {
    pub fn new() -> std::io::Result<Self> {
        let mut heroes = String::new();
        let mut file = File::open("./assets/所有英雄.txt")?;
        file.read_to_string(&mut heroes)?;
        let mut lines = heroes.lines();
        let mut heroes = Vec::new();
        while let Some(hero) = lines.next() {
            heroes.push(hero.to_string());
        }
        Ok(Self {
            rng: rand::thread_rng(),
            users: Vec::new(),
            heroes,
            remaining_wz: [
                vec!["上".to_string(), "中".to_string(), "下".to_string()],
                vec!["上".to_string(), "中".to_string(), "下".to_string()],
            ],
        })
    }
}

#[derive(Message)]
#[rtype(result = "Option<User>")]
pub struct TeamUp(pub String);

impl Handler<TeamUp> for Team {
    type Result = Option<User>;

    fn handle(&mut self, msg: TeamUp, _: &mut Self::Context) -> Self::Result {
        let name = msg.0;
        let mut team_num = [0, 0];
        for user in self.users.iter() {
            team_num[user.team as usize] += 1;
            if user.name == name {
                return Some(user.clone())
            }
        }
        if USERS.contains(&name.as_str()) {
            let mut team = self.rng.gen_range(0..2);
            if team_num[0] == 3 {
                team = 1;
            } else if team_num[1] == 3 {
                team = 0;
            }
            let wz = &mut self.remaining_wz[team as usize];
            let wz_index = self.rng.gen_range(0..wz.len());
            let wz = wz.remove(wz_index);
            let hero_index = self.rng.gen_range(0..self.heroes.len());
            let hero = self.heroes.remove(hero_index);
            let user = User {
                name,
                team,
                wz,
                hero,
            };
            self.users.push(user.clone());
            Some(user)
        } else {
            None
        }
    }
}
