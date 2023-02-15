use std::time::Duration;

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::ExchangeDeclareArguments,
    connection::Connection,
};
use tokio::time;
use tracing::info;

mod common;

#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_net_io_err_handling() {
    common::setup_logging();

    // open a connection to RabbitMQ server
    let conn_args = common::build_conn_args();

    let connection = Connection::open(&conn_args).await.unwrap();
    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();
    // open a channel on the connection
    let channel = connection.open_channel(None).await.unwrap();
    channel
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();

    let exchange_name = "amq.topic";
    let exchange_type = "topic";

    // create arguments for exchange_declare
    let args = ExchangeDeclareArguments::new(exchange_name, exchange_type)
        .passive(true)
        .finish();

    // declare exchange
    channel.exchange_declare(args).await.unwrap();

    // wait on io failure
    // if return `true`, it means got notification due to network failure, we try to recreate connection
    // if return `false`, the connection may already be closed, or closed later by client or server.
    if connection.listen_network_io_failure().await {
        loop {
            time::sleep(Duration::from_secs(10)).await;

            if let Ok(recover_conn) = Connection::open(&conn_args).await {
                // ... do some recovery...
                let recover_ch = recover_conn.open_channel(None).await.unwrap();
                assert!(true == recover_ch.is_open());
                assert!(true == recover_conn.is_open());
                info!("recover: {}, {}", recover_conn, recover_ch);
                break;
            };
        }
    };
    // here, old connection should be closed no matter due to network failure or closed by server
    assert!(connection.is_open() == false);
}