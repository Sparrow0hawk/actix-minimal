mod fixtures;

#[actix_web::test]
async fn test_page_not_found() {
    let app = fixtures::spawn_app().await;

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("{}/foo", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    let status = resp.status().clone();

    let page_str = &resp.text().await.unwrap();

    let page = fixtures::get_page_element(page_str, "h1");

    assert_eq!(page, "Page not found");
    assert_eq!(status, 404)
}
