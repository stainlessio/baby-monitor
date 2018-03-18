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
use std::collections::HashMap;

mod schema;

struct Context {
    pub branches: HashMap<String, schema::Branch>,
}
impl juniper::Context for Context { }
impl Context {
    fn new() -> Context {
        Context {
            branches: HashMap::new()
        }
    }
}

struct Query;
graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
        "0.1"
    }

    interfaces: [&schema::Branch]

    field branch(&executor, name: String) -> &schema::Branch {
        let context = executor.context();
        context.branches.get(&name).unwrap()
    }

    field branches(&executor) -> Vec<&schema::Branch> {
        let context = executor.context();
        context.branches.values().collect()
    }
});

struct Mutation;
graphql_object!(Mutation: Context |&self| {
    field setStatus(&executor, name: String, status: schema::BranchStatus) -> FieldResult<&schema::Branch> {
        let mut context = executor.context();
        let branch = context.branches.remove(&name).unwrap();
        let updated_branch = branch.set_status(&status).unwrap();
        context.branches.insert(name.clone(), updated_branch);
        Ok(context.branches.get(&name).unwrap())
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
    let mut context = Context::new();
    let master = schema::Branch::new("master".to_owned());
    let development = schema::Branch::new("development".to_owned());
    context.branches.insert(master.name.clone(), master);
    context.branches.insert(development.name.clone(), development);
    rocket::ignite()
        .manage(context)
        .manage(Schema::new(
            Query {},
            Mutation {}
        ))
        .mount("/", routes![graphiql, post_graphql_handler])
        .launch();
}
