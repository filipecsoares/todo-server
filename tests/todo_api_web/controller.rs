#[cfg(test)]
mod ping_readiness {
    use todo_server::todo_api_web::controller::{ping, readiness};

    use actix_web::{test, body, App, http::StatusCode, dev::Service, HttpResponse, web};

    #[actix_web::test]
    async fn test_ping_pong() {
        let mut app = test::init_service(App::new().service(ping)).await;

        let req = test::TestRequest::get().uri("/ping").to_request();
        let resp = test::call_service(&mut app, req).await;
        let body = resp.into_body();
        let bytes = body::to_bytes(body).await.unwrap();

        assert_eq!(bytes, web::Bytes::from_static(b"pong"));
    }

    #[actix_web::test]
    async fn test_readiness() {
        let mut app = test::init_service(App::new().service(readiness)).await;

        let req = test::TestRequest::get().uri("/ready").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::ACCEPTED);
    }

    #[actix_web::test]
    async fn not_found_route() {
        let app = test::init_service(
            App::new()
                .service(readiness)
                .service(ping)
                .default_service(web::to(|| HttpResponse::NotFound())),
        )
        .await;

        let req = test::TestRequest::with_uri("/crazy-path").to_request();

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}