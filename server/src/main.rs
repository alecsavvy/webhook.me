use darkredis::ConnectionPool;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use rusoto_core::credential::StaticProvider;
use rusoto_core::request::HttpClient;
use rusoto_sqs::Sqs;
use rusoto_sqs::SqsClient;
use std::convert::Infallible;
use std::net::SocketAddr;

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

#[tokio::main]
async fn main() -> darkredis::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello_world)) });

    let server = Server::bind(&addr).serve(make_svc);

    let pool = ConnectionPool::create("redis:6379".into(), None, num_cpus::get()).await?;
    let mut conn = pool.get().await;

    //And away!
    conn.set("secret_entrance", b"behind the bookshelf").await?;
    let secret_entrance = conn.get("secret_entrance").await?;
    assert_eq!(secret_entrance, Some("behind the bookshelf".into()));

    // gather aws credentials
    let provider =
        StaticProvider::new_minimal("local_access_key".into(), "local_secret_access_key".into());

    // elastic mq client
    let region = rusoto_core::region::Region::Custom {
        name: "local_sqs".to_owned(),
        endpoint: "http://elasticmq:9324".to_owned(),
    };
    let client = SqsClient::new_with(
        HttpClient::new().expect("couldnt make http client 🤷‍♀️"),
        provider,
        region,
    );
    let req = rusoto_sqs::ListQueuesRequest::default();
    let queues = client.list_queues(req).await;

    println!("queues: {:?}", queues);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
