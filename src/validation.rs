use crate::error::{NombaError, Result};
use openapiv3::{OpenAPI, PathItem, ReferenceOr, RequestBody, Schema, SchemaKind, Type};
use std::collections::HashMap;
use std::sync::OnceLock;

static SPEC: OnceLock<OpenAPI> = OnceLock::new();

fn load_spec() -> &'static OpenAPI {
    SPEC.get_or_init(|| {
        let spec_json = include_str!("../openapi/nomba_openapi.json");
        serde_json::from_str(spec_json).expect("Failed to parse OpenAPI spec")
    })
}

fn resolve_path_item<'a>(
    spec: &'a OpenAPI,
    path_ref: &'a ReferenceOr<PathItem>,
) -> Option<&'a PathItem> {
    match path_ref {
        ReferenceOr::Reference { reference } => {
            let name = reference.split('/').last()?;
            spec.paths.paths.get(name).and_then(|p| match p {
                ReferenceOr::Item(item) => Some(item),
                ReferenceOr::Reference { .. } => None,
            })
        }
        ReferenceOr::Item(item) => Some(item),
    }
}

fn resolve_schema<'a>(spec: &'a OpenAPI, schema: &'a ReferenceOr<Schema>) -> Option<&'a Schema> {
    match schema {
        ReferenceOr::Reference { reference } => {
            let name = reference.split('/').last()?;
            spec.components
                .as_ref()?
                .schemas
                .get(name)
                .and_then(|s| match s {
                    ReferenceOr::Item(item) => Some(item),
                    ReferenceOr::Reference { .. } => None,
                })
        }
        ReferenceOr::Item(schema) => Some(schema),
    }
}

fn resolve_schema_boxed<'a>(
    spec: &'a OpenAPI,
    schema: &'a ReferenceOr<Box<Schema>>,
) -> Option<&'a Schema> {
    match schema {
        ReferenceOr::Reference { reference } => {
            let name = reference.split('/').last()?;
            spec.components
                .as_ref()?
                .schemas
                .get(name)
                .and_then(|s| match s {
                    ReferenceOr::Item(item) => Some(item),
                    ReferenceOr::Reference { .. } => None,
                })
        }
        ReferenceOr::Item(schema) => Some(schema.as_ref()),
    }
}

fn resolve_request_body<'a>(
    spec: &'a OpenAPI,
    rb_ref: &'a ReferenceOr<RequestBody>,
) -> Option<&'a RequestBody> {
    match rb_ref {
        ReferenceOr::Reference { reference } => {
            let name = reference.split('/').last()?;
            spec.components
                .as_ref()?
                .request_bodies
                .get(name)
                .and_then(|r| match r {
                    ReferenceOr::Item(item) => Some(item),
                    ReferenceOr::Reference { .. } => None,
                })
        }
        ReferenceOr::Item(item) => Some(item),
    }
}

fn check_required_fields(
    spec: &OpenAPI,
    schema: &Schema,
    value: &serde_json::Value,
    path: &str,
    missing: &mut Vec<String>,
) {
    let schema_ref = ReferenceOr::Item(schema.clone());
    let schema = match resolve_schema(spec, &schema_ref) {
        Some(s) => s,
        None => return,
    };

    match &schema.schema_kind {
        SchemaKind::Type(Type::Object(obj)) => {
            if !value.is_object() {
                if !value.is_null() {
                    missing.push(format!("{} (expected object)", path));
                }
                return;
            }

            let obj_value = value.as_object().unwrap();
            for required in &obj.required {
                if !obj_value.contains_key(required) || obj_value[required].is_null() {
                    let field_path = if path.is_empty() {
                        required.clone()
                    } else {
                        format!("{}.{}", path, required)
                    };
                    missing.push(field_path);
                }
            }

            for (key, sub_value) in obj_value {
                if let Some(prop_schema_ref) = obj.properties.get(key) {
                    let prop_schema = match resolve_schema_boxed(spec, prop_schema_ref) {
                        Some(s) => s,
                        None => continue,
                    };
                    let sub_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };
                    check_required_fields(spec, prop_schema, sub_value, &sub_path, missing);
                }
            }
        }
        SchemaKind::Type(Type::Array(arr)) => {
            if !value.is_array() {
                return;
            }
            if let Some(items_schema_ref) = &arr.items {
                let items_schema = match resolve_schema_boxed(spec, items_schema_ref) {
                    Some(s) => s,
                    None => return,
                };
                for (i, item) in value.as_array().unwrap().iter().enumerate() {
                    let sub_path = format!("{}[{}]", path, i);
                    check_required_fields(spec, items_schema, item, &sub_path, missing);
                }
            }
        }
        _ => {}
    }
}

pub fn validate_body(verb: &str, path_template: &str, body: &serde_json::Value) -> Result<()> {
    let spec = load_spec();
    let path_item_ref = spec.paths.paths.get(path_template);
    if path_item_ref.is_none() {
        return Ok(());
    }

    let path_item = match resolve_path_item(spec, path_item_ref.unwrap()) {
        Some(p) => p,
        None => return Ok(()),
    };

    let operation = match verb.to_lowercase().as_str() {
        "get" => path_item.get.as_ref(),
        "post" => path_item.post.as_ref(),
        "put" => path_item.put.as_ref(),
        "delete" => path_item.delete.as_ref(),
        "patch" => path_item.patch.as_ref(),
        _ => return Ok(()),
    };

    if operation.is_none() {
        return Ok(());
    }

    let request_body_ref = operation.as_ref().unwrap().request_body.as_ref();
    if request_body_ref.is_none() {
        return Ok(());
    }

    let request_body = match resolve_request_body(spec, request_body_ref.unwrap()) {
        Some(r) => r,
        None => return Ok(()),
    };

    let content = request_body.content.get("application/json");
    if content.is_none() {
        return Ok(());
    }

    let schema = content.unwrap().schema.as_ref();
    if schema.is_none() {
        return Ok(());
    }

    let schema = match resolve_schema(spec, schema.unwrap()) {
        Some(s) => s,
        None => return Ok(()),
    };

    let mut missing = Vec::new();
    check_required_fields(spec, schema, body, "", &mut missing);

    if !missing.is_empty() {
        return Err(NombaError::validation(
            format!(
                "Missing required field(s) in request body: {}",
                missing.join(", ")
            ),
            missing,
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_validate_body_valid() {
        let body = json!({
            "accountRef": "test-ref",
            "accountName": "Test Account"
        });
        let result = validate_body("post", "/v1/accounts/virtual", &body);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_body_missing_required() {
        let body = json!({
            "accountName": "Test Account"
        });
        let result = validate_body("post", "/v1/accounts/virtual", &body);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("accountRef"));
        }
    }
}
