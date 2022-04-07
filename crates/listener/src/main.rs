use listener::startup::Listener;
use listener_core::blockchain::blockchain::Blockchain;
use listener_core::configuration::get_configuration;
use listener_core::tracing::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("listener".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration(None).expect("Failed to read configuration.");

    let listener = Listener::build(&configuration).await;
    let _res = listener.run_until_stopped().await;
}
