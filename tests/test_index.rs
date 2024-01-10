use actix_minimal::routes::HelloData;
mod fixtures;

#[actix_web::test]
async fn test_index_returns_html() {
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

#[actix_web::test]
async fn test_index_returns_json() {
    let app = fixtures::spawn_app().await;

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("{}/", &app.address))
        .header("Accept", "application/json")
        .send()
        .await
        .expect("Failed to execute request");

    let data = &resp.json::<HelloData>().await.unwrap();

    assert_eq!(data.text, "Hello world!")
}
