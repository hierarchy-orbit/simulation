use crate::dispatch::{args, Handle, Result as DispatchResult};
use crate::Notion;

use rocket::response::status;
use rocket::{config, uri};
use rocket::{get, post, routes, State};
use rocket_contrib::json::Json;
use snafu::{ResultExt, Snafu};
use types::eth_spec::EthSpec;

/// Shorthand for result types returned from the API server.
pub type Result<V, E = Error> = std::result::Result<V, E>;

/// Errors arising from the API server.
#[derive(Debug, Snafu)]
pub enum Error {
    Config { source: config::ConfigError },
}

pub fn run<T: EthSpec>(notion: &Notion<T>, handle: Handle) -> Result<()> {
    let config = config::Config::build(config::Environment::Development)
        .address(format!("{}", notion.bind.ip()))
        .port(notion.bind.port())
        .finalize()
        .context(Config)?;

    rocket::custom(config)
        .mount(
            "/",
            routes![
               create_execution_environment,
//                create_shard_block,
//                get_execution_environment,
//                get_execution_environment_state,
//                get_shard_block,
//                get_shard_state,
            ],
        )
        .manage(handle)
        .launch();

    Ok(())
}

#[tokio::main] // TODO: Check efficiency of tokio::main. Does it create or reuse thread pools?
#[post("/create-execution-environment", data = "<args>")]
async fn create_execution_environment(
   args: Json<args::CreateExecutionEnvironment>,
   handle: State<Handle>,
) -> DispatchResult<Json<u64>> {
   let args = args.into_inner();

   let ee_index = handle.clone().create_execution_environment(args).await?;

   Ok(Json(ee_index))
}

// #[tokio::main]
// #[post("/get-execution-environments", data = "<args>")]
// async fn get_execution_environment(
//    args: Json<args::GetExecutionEnvironment>,
//    handle: State<Handle>,
// ) -> DispatchResult<Json<args::ExecutionEnvironment>> {
//    let arg = args::GetExecutionEnvironment {
//        ee_index,
//    };
//
//    let ee = handle.clone().get_execution_environment(arg).await?;
//    Ok(Json(ee))
// }


//#[tokio::main]
//#[post("/shards", data = "<shard>")]
//async fn create_shard_chain(
//    shard: Json<args::CreateShardChain>,
//    handle: State<Handle>,
//) -> DispatchResult<status::Created<()>> {
//    let shard = shard.into_inner();
//
//    let idx = handle.clone().create_shard_chain(shard).await?;
//    let location = uri!(show_shard_chain: idx);
//
//    Ok(status::Created(location.to_string(), None))
//}
//
//#[tokio::main]
//#[get("/shards/<index>")]
//async fn show_shard_chain(index: u32, handle: State<Handle>) -> DispatchResult<Json<args::ShardChain>> {
//    let arg = args::GetShardChain {
//        shard_chain_index: index,
//    };
//
//    let shard = handle.clone().shard_chain(arg).await?;
//
//    Ok(Json(shard))
//}
