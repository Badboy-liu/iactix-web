use actix_web::Responder;
use actix_web::{HttpRequest, HttpResponse, get, post};

use crate::err::CustomeError::MyError;
use crate::err::UserError::UserError;
use actix_web::error;
use actix_web::web::Path;
use actix_web::{Result, web};
use derive_more::derive::{Display, Error};
use imacro::{JSON, auto_config,body};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};



use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use std::collections::HashMap;
use sea_orm::EntityTrait;

#[auto_config]
mod routes {

    #[get("/")]
    async fn index() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }
    #[post("/update")]
    async fn update(path: Path<(u32)>) -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }
    #[derive(Deserialize)]
    struct User1 {
        id: u32,
    }
    #[post("/update2")]
    async fn update2(path: web::Path<User1>) -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }

    #[post("/update3")]
    async fn update3(req: HttpRequest) -> Result<String> {
        let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
        let userid: i32 = req.match_info().query("user_id").parse().unwrap();

        Ok(format!("Welcome {}, user_id {}!", name, userid))
    }

    #[post("/update4")]
    async fn update4(info: web::Query<User1>) -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }
    #[post("/update5")]
    async fn update5(info: web::Json<User1>) -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }
    #[post("/update6")]
    async fn update6(info: web::Form<User1>) -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }

    #[derive(Debug, Display, Error)]
    #[display("IError :{name}")]
    struct IError {
        name: &'static str,
    }
    impl error::ResponseError for IError {}
    #[post("/update7")]
    async fn update7(info: web::Form<User1>) -> Result<&'static str, IError> {
        Err(IError { name: "IError!" })
    }

    #[get("/index1")]
    async fn index1(req: HttpRequest) -> Result<&'static str, crate::err::CustomeError::MyError> {
        Err(crate::err::CustomeError::MyError::InternalError)
    }

    #[get("/index2")]
    async fn index2(req: HttpRequest) -> Result<&'static str, crate::err::UserError::UserError> {
        info!("index2");
        error!("index2");
        warn!("index2");
        Err(crate::err::UserError::UserError::InternalError)
    }

    #[derive(Debug, Display, Error, Deserialize, Serialize, JSON)]
    struct DDD {
        id: u32,
    }
    #[derive(Debug, Display, Error, Deserialize, Serialize)]
    struct DDD2 {
        id: u32,
    }

    #[get("/index3")]
    async fn index3(req: HttpRequest) -> DDD {
        DDD { id: 20 }
    }


    #[get("/index4")]
    #[body]
    async fn index4(req: HttpRequest) -> HashMap<u32,u32> {
        // DDD2 { id: 20 }
        let mut map = HashMap::new();
        map.insert(1, 2);
        map.insert(2, 2);
        map
    }
    // 数据结构：用户及文章
    #[derive(serde::Serialize,Default,JSON)]
    struct UserWithPosts {
        id: i32,
        username: String,
        email: String,
        created_at: chrono::DateTime<chrono::Utc>,
        posts: Vec<PostData>,
    }

    #[derive(serde::Serialize)]
    struct PostData {
        id: i32,
        title: String,
        content: String,
        created_at: chrono::DateTime<chrono::Utc>,
    }
    use crate::entity::{User,Post};
    #[get("/index5/{path}")]
    async fn index5(data: web::Data<sea_orm::DatabaseConnection>, path: web::Path<i32>) ->UserWithPosts {
        let user_id = path.into_inner();

        // 查询用户
        let userResult = User::Entity::find_by_id(user_id)
            .one(data.get_ref())
            .await;

        let user = match userResult {
            Ok(mut user) => {
                match user {
                    Some(u) => {
                        u
                    },
                    None => {
                        return UserWithPosts::default();
                    }
                }
            },
            Err(error) => {
                println!("{}", error);
                return UserWithPosts::default();
            }
        };


        // 查询该用户的所有文章
        let posts_result = Post::Entity::find()
            .filter(Post::Column::UserId.eq(user_id))
            .all(data.get_ref())
            .await
            ;

        if let Ok(posts)=posts_result{

            // 转换为响应格式
            let posts_data: Vec<PostData> = posts
                .into_iter()
                .map(|p| PostData {
                    id: p.id,
                    title: p.title,
                    content: p.content,
                    created_at: p.created_at,
                })
                .collect();

            let user_with_posts = UserWithPosts {
                id: user.id,
                username: user.username,
                email: user.email,
                created_at: user.created_at,
                posts: posts_data,
            };
            return user_with_posts;
        }else{
            return UserWithPosts::default();
        }
    }


}