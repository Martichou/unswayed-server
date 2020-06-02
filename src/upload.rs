use crate::diesel::BoolExpressionMethods;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::diesel::QueryDsl;

use super::models::NewImage;
use super::schema::images::dsl::*;
use super::s3;

use diesel::dsl::{insert_into, exists, select};
use actix_multipart::{Field, Multipart};
use serde::{Deserialize, Serialize};
use diesel::r2d2::ConnectionManager;
use diesel::prelude::PgConnection;
use actix_web::{web, Error};
use futures::StreamExt;
use std::convert::From;
use std::io::Write;
use nanoid::nanoid;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UploadFile {
    pub filename: String,
    pub url: String,
}

impl From<Tmpfile> for UploadFile {
    fn from(tmp_file: Tmpfile) -> Self {
        UploadFile {
            filename: tmp_file.name,
            url: tmp_file.s3_url,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tmpfile {
    pub name: String,
    pub realname: String,
    pub tmp_path: String,
    pub s3_url: String,
}

impl Tmpfile {
    fn new(filename: std::string::String, real_name: &str) -> Tmpfile {
        Tmpfile {
            name: filename.to_string(),
            realname: real_name.to_string(),
            tmp_path: format!("./tmp/{}", real_name),
            s3_url: "".to_string(),
        }
    }

    async fn s3_upload_and_tmp_remove(&mut self) {
        self.s3_upload().await;
        self.tmp_remove();
    }

    async fn s3_upload(&mut self) {
        let path = format!("{}", &self.name);
        let url: String = s3::Client::new().put_object(&self.tmp_path, &path.clone()).await;
        self.s3_url = url;
    }

    fn tmp_remove(&self) {
        std::fs::remove_file(&self.tmp_path).unwrap();
    }
}

pub async fn split_payload(
    user_id_f: i32,
    pool: &web::Data<Pool>,
    payload: &mut Multipart
) -> Vec<Tmpfile> {
    let conn = pool.get().unwrap();
    let mut tmp_files: Vec<Tmpfile> = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let content_type = field.content_disposition().unwrap();
        let name = content_type.get_name().unwrap();
        if name == "images" {
            match content_type.get_filename() {
                Some(filename) => {
                    // Check if the filename already exist for that user
                    let item_exist: std::result::Result<bool, diesel::result::Error> = select(exists(images.filter(realname.eq(filename).and(user_id.eq(user_id_f))))).get_result(&conn);
                    // If found or error skip
                    if item_exist.is_err() || item_exist.unwrap() {
                        continue ;
                    // Else process
                    } else {
                        let tmp_file = Tmpfile::new(nanoid!(128), &sanitize_filename::sanitize(&filename));
                        let tmp_path = tmp_file.tmp_path.clone();
                        let mut f = web::block(move || std::fs::File::create(&tmp_path)).await.unwrap();
                        while let Some(chunk) = field.next().await {
                            let data = chunk.unwrap();
                            f = web::block(move || f.write_all(&data).map(|_| f)).await.unwrap();
                        }
                        tmp_files.push(tmp_file.clone());
                    }
                }
                None => {
                    println!("file none");
                }
            }
        }
    }
    tmp_files
}

pub async fn save_file(
    user_id_f: i32,
    pool: &web::Data<Pool>,
    tmp_files: Vec<Tmpfile>,
) -> Result<Vec<UploadFile>, Error> {
    let conn = pool.get().unwrap();
    let mut arr: Vec<UploadFile> = Vec::with_capacity(tmp_files.len());

    for item in tmp_files {
        let mut tmp_file: Tmpfile = item.clone();
        tmp_file.s3_upload_and_tmp_remove().await;
        // Add to database (here?) Maybe check for error but need to investigate more on this
        let new_image = NewImage {
            user_id: user_id_f,
            realname: tmp_file.realname.clone(),
            fakedname: tmp_file.name.clone(),
            created_at: chrono::Local::now().naive_local(),
        };
        let res = insert_into(images).values(&new_image).execute(&conn);
        if res.is_ok() {
            arr.push(UploadFile::from(tmp_file));
        }
    }
    Ok(arr)
}

#[allow(unused)]
pub async fn delete_object(list: Vec<String>) {
    for key in list {
        s3::Client::new().delete_object(key).await;
    }
}