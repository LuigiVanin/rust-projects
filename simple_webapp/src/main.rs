use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;

#[derive(Deserialize, Debug)]
struct Post<'a> {
    content: &'a str,
    ip: &'a str,
}

#[derive(Deserialize, Serialize, Debug)]
struct PostResult {
    status: String,
}

#[get("/health")]
fn index() -> &'static str {
    "100% health!"
}

#[post("/", data = "<data>")]
fn get_json(data: Json<Post<'_>>) -> String {
    println!("Passou por aqui!");
    println!("{:?}", data);

    let mut res: String = data.0.content.into();
    res = format!("your content is: {}", res);
    res
}

#[post("/fail", data = "<data>")]
fn fail_or_sucess(data: Json<Post<'_>>) -> Result<Json<PostResult>, BadRequest<String>> {
    if data.0.ip == "i am a ip" {
        return Ok(Json(PostResult {
            status: "sucess".to_string(),
        }));
    }
    return Err(BadRequest(Some("Erro bobo rapaz".to_string())));
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_json, fail_or_sucess])
}
