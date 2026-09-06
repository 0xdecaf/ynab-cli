use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};
use ynab_client::YnabClient;

async fn client_for(server: &MockServer) -> YnabClient {
    YnabClient::with_base_url("test-token".into(), format!("{}/v1", server.uri())).unwrap()
}

#[tokio::test]
async fn by_month_hits_the_month_scoped_endpoint() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/plans/p1/months/2026-03-01/money_movements"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": {"money_movements": [], "server_knowledge": 7}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = client_for(&server).await;
    client
        .get_money_movements_for_month("p1", "2026-03-01", None)
        .await
        .unwrap();
}

#[tokio::test]
async fn by_month_forwards_last_knowledge() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/plans/p1/months/2026-03-01/money_movement_groups"))
        .and(query_param("last_knowledge_of_server", "42"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": {"money_movement_groups": [], "server_knowledge": 43}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = client_for(&server).await;
    client
        .get_money_movement_groups_for_month("p1", "2026-03-01", Some(42))
        .await
        .unwrap();
}
