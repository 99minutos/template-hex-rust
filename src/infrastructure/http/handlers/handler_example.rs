use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{
    infrastructure::http::{dto::example_dto::ExampleDto, response::GenericApiResponse},
    AppContext,
};

#[tracing::instrument(skip_all)]
#[get("")]
pub async fn get_examples(ctx: web::Data<AppContext>) -> impl Responder {
    let result = ctx.example_srv.get_examples().await.map(|list| {
        list.into_iter()
            .map(ExampleDto::from)
            .collect::<Vec<ExampleDto>>()
    });

    HttpResponse::Ok().json(GenericApiResponse::from(result))
}

#[tracing::instrument(skip_all)]
#[post("/random")]
pub async fn add_random_example(ctx: web::Data<AppContext>) -> impl Responder {
    let result = ctx
        .example_srv
        .add_random_example()
        .await
        .map(ExampleDto::from);
    HttpResponse::Ok().json(GenericApiResponse::from(result))
}

#[tracing::instrument(skip_all)]
#[get("/error")]
pub async fn get_examples_with_error(ctx: web::Data<AppContext>) -> impl Responder {
    let result = ctx.example_srv.get_examples_with_error().await.map(|list| {
        list.into_iter()
            .map(ExampleDto::from)
            .collect::<Vec<ExampleDto>>()
    });
    HttpResponse::Ok().json(GenericApiResponse::from(result))
}
