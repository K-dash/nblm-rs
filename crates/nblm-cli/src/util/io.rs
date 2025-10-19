use anyhow::Result;
use nblm_core::models::{
    BatchCreateSourcesResponse, ListRecentlyViewedResponse, Notebook, ShareResponse,
};
use serde_json::json;

pub fn emit_notebook(notebook: &Notebook, json_mode: bool) {
    let notebook_id = notebook
        .notebook_id
        .as_deref()
        .or_else(|| {
            notebook
                .name
                .as_deref()
                .and_then(|name| name.rsplit('/').next())
        })
        .unwrap_or_default();
    let payload = json!({
        "notebook_id": notebook_id,
        "notebook": notebook,
    });
    emit_json(payload, json_mode);
}

pub fn emit_recent(response: &ListRecentlyViewedResponse, json_mode: bool) -> Result<()> {
    if json_mode {
        emit_json(json!(response), true);
    } else {
        if response.notebooks.is_empty() {
            println!("No recently viewed notebooks.");
        } else {
            for notebook in &response.notebooks {
                println!("{}", serde_json::to_string_pretty(notebook)?);
            }
        }
        if let Some(token) = &response.next_page_token {
            println!("next_page_token: {token}");
        }
    }
    Ok(())
}

pub fn emit_sources(
    notebook_id: &str,
    response: &BatchCreateSourcesResponse,
    json_mode: bool,
) -> Result<()> {
    let payload = json!({
        "notebook_id": notebook_id,
        "sources": response.sources,
        "error_count": response.error_count,
    });
    emit_json(payload, json_mode);
    Ok(())
}

pub fn emit_share(response: &ShareResponse, json_mode: bool) -> Result<()> {
    emit_json(json!(response), json_mode);
    Ok(())
}

pub fn emit_json(value: serde_json::Value, json_mode: bool) {
    if json_mode {
        println!("{}", serde_json::to_string_pretty(&value).unwrap());
    } else {
        match value {
            serde_json::Value::Object(map) => {
                for (key, val) in map {
                    println!("{key}: {val}");
                }
            }
            other => println!("{}", other),
        }
    }
}
