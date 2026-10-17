use serde_json::{json, Value};
use tauri::{
    ipc::{CallbackFn, InvokeBody},
    test::{get_ipc_response, mock_builder, mock_context, noop_assets, MockRuntime, INVOKE_KEY},
    webview::InvokeRequest,
    WebviewWindow,
};

fn invoke(webview: &WebviewWindow<MockRuntime>, request: Value) -> Result<Value, String> {
    let message = InvokeRequest {
        cmd: "engine".into(),
        callback: CallbackFn(0),
        error: CallbackFn(1),
        url: "http://tauri.localhost"
            .parse()
            .map_err(|error| format!("{error}"))?,
        body: InvokeBody::Json(json!({"command":"assessment.check", "request":request})),
        headers: Default::default(),
        invoke_key: INVOKE_KEY.into(),
    };
    get_ipc_response(webview, message)
        .map_err(|error| format!("{error:?}"))?
        .deserialize::<Value>()
        .map_err(|error| error.to_string())
}

#[test]
fn assessment_fixtures_cross_the_tauri_ipc_command_boundary(
) -> Result<(), Box<dyn std::error::Error>> {
    let app = mock_builder()
        .invoke_handler(tauri::generate_handler![super::engine])
        .build(mock_context(noop_assets()))?;
    let webview = tauri::WebviewWindowBuilder::new(&app, "main", Default::default()).build()?;
    let fixtures: Vec<Value> = serde_json::from_str(include_str!(
        "../../../../crates/vx-api/tests/fixtures/assessment-cases.json"
    ))?;
    for fixture in fixtures {
        let request = fixture["request"].clone();
        let native = vx_api::dispatch("assessment.check", request.clone())
            .map_err(|error| format!("{error:?}"))?;
        assert_eq!(native["status"], fixture["status"], "{}", fixture["name"]);
        assert_eq!(invoke(&webview, request)?, native, "{}", fixture["name"]);
    }
    Ok(())
}
