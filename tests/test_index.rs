use actix_minimal::routes::HelloData;
use reqwest::header;
mod fixtures;

#[actix_web::test]
async fn test_index_returns_html() {
    let app = fixtures::spawn_app().await;

    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Accept",
        header::HeaderValue::from_static(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
        ),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

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

    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Accept",
        header::HeaderValue::from_static("application/json"),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let resp = client
        .get(&format!("{}/", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    let data = &resp.json::<HelloData>().await.unwrap();

    assert_eq!(data.text, "Hello world!")
}
