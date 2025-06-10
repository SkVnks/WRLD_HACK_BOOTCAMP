use ic_cdk::api::management_canister::http_request::http_request;
use ic_cdk::api::management_canister::http_request::CanisterHttpRequestArgument;
use ic_cdk::api::management_canister::http_request::HttpMethod;
use ic_cdk::api::management_canister::http_request::HttpHeader;

#[ic_cdk::update]
async fn translate(text: String) -> String {
    let token = "";
    let (res,) = http_request(
        CanisterHttpRequestArgument {
            url: "https://api-inference.huggingface.co/models/google-t5/t5-base".to_string(),
            max_response_bytes: None,
            method: HttpMethod::POST,
            headers: vec![
                HttpHeader {
                    name: "Authorization".to_string(),
                    value: format!("Bearer {}", token),
                },
                 HttpHeader {
                    name: "Content-Type".to_string(),
                    value: "application/json".to_string(),
                }
            ],
            body: Some(format!(r#"{{"inputs": "{}"}}"#, text).into()),
            transform: None,
        },
    ).await.unwrap();

    String::from_utf8(res.body).unwrap()
}