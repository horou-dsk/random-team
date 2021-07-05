use actix_web::{web, HttpResponse, post};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use actix::Addr;
use crate::team::{Team, TeamUp};
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct ResultOk<T> {
    code: u16,
    data: T,
}

impl<T> ResultOk<T> {
    fn new(data: T) -> Self {
        ResultOk { code: 200, data }
    }
}

#[derive(Serialize, Deserialize)]
struct ResultErr {
    code: u16,
    err_msg: String,
}

impl ResultErr {
    fn new(code: u16, err_msg: String) -> Self {
        ResultErr { code, err_msg }
    }
}


pub struct ResultJson;

impl ResultJson {
    fn ok<T>(data: T) -> ResultOk<T> {
        ResultOk::new(data)
    }

    fn err<S: Into<String>>(code: u16, err_msg: S) -> ResultErr {
        ResultErr::new(code, err_msg.into())
    }
}

type AddrTeam = web::Data<Addr<Team>>;

#[post("/team_up")]
async fn team_up(info: web::Json<HashMap<String, String>>, team: AddrTeam) -> Result<HttpResponse, Box<dyn Error>> {
    let name = info.0["name"].clone();
    if let Some(user) = team.get_ref().send(TeamUp(name)).await? {
        Ok(HttpResponse::Ok().json(ResultJson::ok(user)))
    } else {
        Ok(HttpResponse::Ok().json(ResultJson::err(500, "参数错误")))
    }
}

pub fn router_config(cfg: &mut web::ServiceConfig) {
    cfg.service(team_up);
}