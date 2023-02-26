use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::helios_lightclient::HeliosLightClient,
        starknet::StarkNetLightClientImpl,
    },
};
use beerus_rest_api::api::{ethereum, starknet};
use log::info;
use rocket::{Build, Rocket};
#[macro_use]
extern crate rocket;
#[get("/")]
fn index() -> &'static str {
    "Hakai!"
}
#[launch]
async fn rocket() -> Rocket<Build> {
    bar().await
}
async fn bar() -> Rocket<Build> {
    env_logger::init();
    info!("starting Beerus Rest API...");
    let config = Config::default();
    let ethereum_lightclient = HeliosLightClient::new(config.clone()).unwrap();
    let starknet_lightclient = StarkNetLightClientImpl::new(&config).unwrap();
    let mut beerus = BeerusLightClient::new(
        config,
        Box::new(ethereum_lightclient),
        Box::new(starknet_lightclient),
    );
    info!("starting the Beerus light client...");
    beerus.start().await.unwrap();
    info!("Beerus light client started and synced.");
    rocket::build().manage(beerus).mount(
        "/",
        routes![
            index,
            ethereum::endpoints::query_balance,
            starknet::endpoints::query_starknet_state_root
        ],
    )
}
#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::{http::Status, local::asynchronous::Client};
    #[doc = " Test the `query_balance` endpoint."]
    #[doc = " `/ethereum/balance/<address>`"]
    #[tokio::test]
    #[ignore]
    async fn given_normal_conditions_when_query_balance_then_ok() {
        let client = Client::tracked(rocket().await)
            .await
            .expect("valid rocket instance");
        let response = client
            .get(uri!(
                "/ethereum/balance/0xc24215226336d22238a20a72f8e489c005b44c4a"
            ))
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), "Hello, world!");
    }
}
