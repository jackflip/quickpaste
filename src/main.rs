use std::{fs::exists, io::Result};
use std::fs;
use rocket::fs::{FileServer, Options, relative};
use rocket::http::uri::Absolute;
use rocket::response::Redirect;
use rocket::tokio::io::{AsyncReadExt, AsyncWriteExt};
use rocket::form::{Context, Form, FromForm};
use rocket::{Data, tokio::fs::File};
use rocket_dyn_templates::{Template, context};

use crate::paste::Paste;
use crate::paste_id::PasteId;
use crate::storage::storage;

#[macro_use] extern crate rocket;

mod paste_id;
mod paste;
pub mod storage;

const HOST: Absolute<'static> = uri!("http://localhost:8000");

#[launch]
fn rocket() -> _ {
    if let Ok(result) = exists(storage()) && !result {
        fs::create_dir(storage());
    }
    rocket::build()
        .mount("/", routes![index, serve, upload])
        .mount("/recent", routes![recent_pastes])
        .mount("/", FileServer::new(relative!("static"), Options::None))
        .attach(Template::fairing())
}

#[get("/")]
fn index() -> Template {
    Template::render("index", Context::default())
}

#[get("/<id>")]
async fn serve(id: PasteId<'_>) -> Template {
    let contents = fs::read(id.path()).unwrap_or(Vec::new());

    let content = String::from_utf8(contents).unwrap();

    Template::render("paste", context! {
        content
    })
}

#[post("/", data="<form>")]
async fn upload(form: Form<Paste>) -> Result<Redirect> {
    let id = PasteId::new();

    File::create_new(id.path()).await?
        .write_all(form.content.as_bytes()).await?;

    Ok(Redirect::to(uri!(HOST, serve(id)).to_string()))
}

#[get("/")]
async fn recent_pastes() -> Template {

    let mut pastes: Vec<String> = Vec::new();

    if let Ok(storage_read) = fs::read_dir(storage()) {
        for paste in storage_read {
            if let Ok(paste) = paste {
                pastes.push(paste.file_name().into_string().unwrap());
            }
        }
    }

    Template::render("recent", context! {
        pastes
    })
}