use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use ynab_client::YnabClient;

async fn client_for(server: &MockServer) -> YnabClient {
    YnabClient::with_base_url("test-token".into(), format!("{}/v1", server.uri())).unwrap()
}

#[tokio::test]
async fn raw_get_with_v1_prefix_does_not_double_the_segment() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/plans"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"data": {"plans": []}})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = client_for(&server).await;
    let result = client.raw_request("GET", "/v1/plans", None).await.unwrap();
    assert_eq!(result["data"]["plans"], serde_json::json!([]));
}

#[tokio::test]
async fn raw_get_without_v1_prefix_also_works() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/plans"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"data": {"plans": []}})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = client_for(&server).await;
    client.raw_request("GET", "/plans", None).await.unwrap();
}

#[tokio::test]
async fn api_error_is_surfaced_as_api_variant() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/plans/nope"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "error": {"id": "404.2", "name": "resource_not_found", "detail": "Resource not found"}
        })))
        .mount(&server)
        .await;

    let client = client_for(&server).await;
    let err = client
        .raw_request("GET", "/v1/plans/nope", None)
        .await
        .unwrap_err();
    assert!(
        matches!(err, ynab_client::YnabError::Api { status: 404, .. }),
        "got {err:?}"
    );
}
