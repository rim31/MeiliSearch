use serde_json::json;

mod common;

// Test issue https://github.com/meilisearch/MeiliSearch/issues/519
#[test]
fn check_add_documents_with_primary_key_param() {
    let mut server = common::Server::with_uid("movies");

    // 1 - Create the index with no primary_key

    let body = json!({
        "uid": "movies",
    });
    let (response, status_code) = server.create_index(body);
    assert_eq!(status_code, 201);
    assert_eq!(response["primaryKey"], json!(null));

    // 2 - Add documents

    let body = json!([{
      "title": "Test",
      "comment": "comment test"
    }]);

    let url = "/indexes/movies/documents?primaryKey=title";
    let (response, status_code) = server.post_request(&url, body);
    eprintln!("{:#?}", response);
    assert_eq!(status_code, 202);
    let update_id = response["updateId"].as_u64().unwrap();
    server.wait_update_id(update_id);

    // 3 - Check update sucess

    let (response, status_code) = server.get_update_status(update_id);
    assert_eq!(status_code, 200);
    assert_eq!(response["status"], "processed");
}
