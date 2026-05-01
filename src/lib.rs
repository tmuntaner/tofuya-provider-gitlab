use crate::bindings::exports::component::tofuya_wasi_gitlab::tofu::Guest;
use crate::models::GraphQlResponse;
use waki::Client;

#[allow(warnings)]
mod bindings;
pub(crate) mod models;

struct Component;

impl Guest for Component {
    fn get_state_names(
        api_url: String,
        project_path: String,
        auth_token: Option<String>,
    ) -> Result<Vec<String>, String> {
        let query = format!(
            r#"query {{ project(fullPath: "{project_path}") {{ terraformStates {{ nodes {{ name }} }} }} }}"#
        );

        let payload = serde_json::json!({
            "query": query
        })
        .to_string();

        let mut headers_list = vec![("Content-Type", b"application/json".to_vec())];

        if let Some(token) = auth_token {
            headers_list.push(("Authorization", format!("Bearer {}", token).into_bytes()));
        }

        let response = Client::new()
            .post(api_url.as_str())
            .headers(headers_list)
            .body(payload.as_bytes())
            .send()
            .map_err(|_| "failed to send request")?;

        if response.status_code() < 200 || response.status_code() >= 300 {
            return Err(format!(
                "GitLab returned HTTP status: {}",
                response.status_code()
            ));
        }

        let body = response.body().map_err(|_| "Failed to decode body")?;
        let body = String::from_utf8(body).map_err(|_| "failed to decode body into utf8 string")?;

        let parsed: GraphQlResponse = serde_json::from_str(body.as_str())
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        let mut state_names = Vec::new();

        if let Some(data) = parsed.data
            && let Some(project) = data.project
                && let Some(states) = project.terraform_states {
                    for node in states.nodes {
                        state_names.push(node.name);
                    }
                }

        Ok(state_names)
    }
}

bindings::export!(Component with_types_in bindings);
