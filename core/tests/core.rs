use std::net::TcpListener;
use core::server::serve;
use core::views;

pub struct TestApp {
    pub address: String
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let server = serve(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp { address }
}

#[actix_rt::test]
async fn server_check() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    template_rendering(app, client).await;
}

async fn template_rendering(app: TestApp, client: reqwest::Client){
    // define routes as tuple with address and local template for variables
    let routes = &[
        ("admin", views::admin::test()),
        ("admin/entities", views::admin::entities::test()),
        ("admin/fields", views::admin::fields::test())
    ];
    
    for route in routes {
        let uri = &format!("{}/{}", &app.address, route.0);
        let response = client.get(uri).send().await
            .expect("Failed to execute request.");
        let response_html = response.text().await.unwrap();
        //let template_html = route.1();
        
        assert_eq!(route.1, response_html);
    }
}