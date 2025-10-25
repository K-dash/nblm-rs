use anyhow::Result;
use nblm_core::models::{
    BatchCreateSourcesResponse, ListRecentlyViewedResponse, Notebook, ShareResponse,
    UploadSourceFileResponse,
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
    } else if response.notebooks.is_empty() {
        println!("No recently viewed notebooks.");
    } else {
        for notebook in &response.notebooks {
            println!("{}", serde_json::to_string_pretty(notebook)?);
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

pub fn emit_uploaded_source(
    notebook_id: &str,
    file_name: &str,
    content_type: &str,
    response: &UploadSourceFileResponse,
    json_mode: bool,
) -> Result<()> {
    let payload = json!({
        "notebook_id": notebook_id,
        "file_name": file_name,
        "content_type": content_type,
        "source_id": response.source_id,
        "extra": response.extra,
    });
    emit_json(payload, json_mode);
    if !json_mode {
        if let Some(source_id) = response.source_id.as_ref().and_then(|id| id.id.as_deref()) {
            println!("Created source: {source_id}");
        } else {
            println!("Upload request accepted (source ID unavailable)");
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_notebook_id(notebook: &Notebook) -> String {
        notebook
            .notebook_id
            .as_deref()
            .or_else(|| {
                notebook
                    .name
                    .as_deref()
                    .and_then(|name| name.rsplit('/').next())
            })
            .unwrap_or_default()
            .to_string()
    }

    #[test]
    fn extract_notebook_id_from_notebook_id_field() {
        let notebook = Notebook {
            name: None,
            title: "Test".to_string(),
            notebook_id: Some("nb123".to_string()),
            emoji: None,
            metadata: None,
            sources: Vec::new(),
            extra: Default::default(),
        };
        assert_eq!(extract_notebook_id(&notebook), "nb123");
    }

    #[test]
    fn extract_notebook_id_from_name_field() {
        let notebook = Notebook {
            name: Some("projects/123/locations/global/notebooks/nb456".to_string()),
            title: "Test".to_string(),
            notebook_id: None,
            emoji: None,
            metadata: None,
            sources: Vec::new(),
            extra: Default::default(),
        };
        assert_eq!(extract_notebook_id(&notebook), "nb456");
    }

    #[test]
    fn extract_notebook_id_prefers_notebook_id_field() {
        let notebook = Notebook {
            name: Some("projects/123/locations/global/notebooks/from-name".to_string()),
            title: "Test".to_string(),
            notebook_id: Some("from-field".to_string()),
            emoji: None,
            metadata: None,
            sources: Vec::new(),
            extra: Default::default(),
        };
        assert_eq!(extract_notebook_id(&notebook), "from-field");
    }

    #[test]
    fn extract_notebook_id_when_both_missing() {
        let notebook = Notebook {
            name: None,
            title: "Test".to_string(),
            notebook_id: None,
            emoji: None,
            metadata: None,
            sources: Vec::new(),
            extra: Default::default(),
        };
        assert_eq!(extract_notebook_id(&notebook), "");
    }

    #[test]
    fn extract_notebook_id_when_name_ends_with_slash() {
        let notebook = Notebook {
            name: Some("projects/123/locations/global/notebooks/".to_string()),
            title: "Test".to_string(),
            notebook_id: None,
            emoji: None,
            metadata: None,
            sources: Vec::new(),
            extra: Default::default(),
        };
        assert_eq!(extract_notebook_id(&notebook), "");
    }

    #[test]
    fn extract_notebook_id_with_consecutive_slashes() {
        let notebook = Notebook {
            name: Some("projects/123/locations/global/notebooks//weird".to_string()),
            title: "Test".to_string(),
            notebook_id: None,
            emoji: None,
            metadata: None,
            sources: Vec::new(),
            extra: Default::default(),
        };
        // rsplit('/').next() will return "weird"
        assert_eq!(extract_notebook_id(&notebook), "weird");
    }
}
