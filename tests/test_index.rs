mod fixtures;

#[actix_web::test]
async fn test_index() {
    let app = fixtures::spawn_app().await;

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("{}/", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    let page_str = &resp.text().await.unwrap();

    let page = fixtures::get_page_element(page_str, "h1");

    assert_eq!(page, "Hello world!")
}
