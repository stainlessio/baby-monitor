#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate juniper;
extern crate juniper_rocket;
extern crate rocket;
extern crate chrono;

use juniper::FieldResult;
use rocket::response::content;
use rocket::State;

struct Context {}
impl juniper::Context for Context { }

struct Query;
graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
        "0.1"
    }
});

struct Mutation;
graphql_object!(Mutation: Context |&self| {
    field apiVersion() -> &str {
        "No"
    }
});

type Schema = juniper::RootNode<'static, Query, Mutation>;

#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[post("/graphql", data="<request>")]
fn post_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    rocket::ignite()
        .manage(Context {})
        .manage(Schema::new(
            Query {},
            Mutation {}
        ))
        .mount("/", routes![graphiql, post_graphql_handler])
        .launch();
}
