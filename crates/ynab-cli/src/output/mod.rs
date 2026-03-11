use std::io::Write;

use crate::cli::OutputFormat;

/// Fields that contain milliunit monetary values and should be converted with --dollars.
const MILLIUNIT_FIELDS: &[&str] = &[
    "amount",
    "balance",
    "budgeted",
    "activity",
    "income",
    "to_be_budgeted",
    "cleared_balance",
    "uncleared_balance",
    "goal_target",
    "goal_under_funded",
    "goal_overall_funded",
    "goal_overall_left",
];

/// Bundles all output options for consistent rendering across commands.
pub struct OutputConfig<'a> {
    pub format: &'a OutputFormat,
    pub dollars: bool,
    pub fields: Option<&'a str>,
    pub output_path: Option<&'a str>,
}

/// Output a serializable value with all post-processing options applied.
pub fn output(value: &impl serde::Serialize, cfg: &OutputConfig) -> anyhow::Result<()> {
    let mut json = serde_json::to_value(value)?;

    // Apply dollar conversion
    if cfg.dollars {
        convert_dollars(&mut json);
    }

    // Apply field filtering
    if let Some(field_list) = cfg.fields {
        let field_names: Vec<&str> = field_list.split(',').map(|s| s.trim()).collect();
        filter_fields(&mut json, &field_names);
    }

    // Determine output destination
    if let Some(path) = cfg.output_path {
        let content = match cfg.format {
            OutputFormat::Json => serde_json::to_string_pretty(&json)?,
            OutputFormat::Table => format_table(&json)?,
            OutputFormat::Csv => format_csv(&json)?,
        };
        let mut file = std::fs::File::create(path)?;
        file.write_all(content.as_bytes())?;
        file.write_all(b"\n")?;
        eprintln!("Output written to: {path}");
    } else {
        match cfg.format {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&json)?);
            }
            OutputFormat::Table => {
                print!("{}", format_table(&json)?);
            }
            OutputFormat::Csv => {
                print!("{}", format_csv(&json)?);
            }
        }
    }
    Ok(())
}

/// Convert milliunit fields to dollar amounts in-place.
fn convert_dollars(json: &mut serde_json::Value) {
    match json {
        serde_json::Value::Object(map) => {
            for (key, value) in map.iter_mut() {
                if MILLIUNIT_FIELDS.contains(&key.as_str()) {
                    if let Some(n) = value.as_i64() {
                        let dollars = n as f64 / 1000.0;
                        *value = serde_json::json!(dollars);
                    }
                } else {
                    convert_dollars(value);
                }
            }
        }
        serde_json::Value::Array(arr) => {
            for item in arr.iter_mut() {
                convert_dollars(item);
            }
        }
        _ => {}
    }
}

/// Filter JSON to only include specified fields.
fn filter_fields(json: &mut serde_json::Value, fields: &[&str]) {
    match json {
        serde_json::Value::Object(map) => {
            // Check if this object has array values (response wrapper like {transactions: [...]})
            let has_array_child = map.values().any(|v| v.is_array());
            if has_array_child {
                for value in map.values_mut() {
                    filter_fields(value, fields);
                }
            } else {
                map.retain(|k, _| fields.contains(&k.as_str()));
            }
        }
        serde_json::Value::Array(arr) => {
            for item in arr.iter_mut() {
                filter_fields(item, fields);
            }
        }
        _ => {}
    }
}

fn format_table(json: &serde_json::Value) -> anyhow::Result<String> {
    let rows = extract_rows(json);
    if rows.is_empty() {
        return Ok("(no data)\n".to_string());
    }

    let headers = collect_headers(&rows);
    let mut table = comfy_table::Table::new();
    table.set_header(&headers);
    table.load_preset(comfy_table::presets::UTF8_FULL_CONDENSED);

    for row in &rows {
        let cells: Vec<String> = headers
            .iter()
            .map(|h| format_cell(row.get(h.as_str())))
            .collect();
        table.add_row(cells);
    }

    Ok(format!("{table}\n"))
}

fn format_csv(json: &serde_json::Value) -> anyhow::Result<String> {
    let rows = extract_rows(json);
    if rows.is_empty() {
        return Ok(String::new());
    }

    let headers = collect_headers(&rows);
    let mut wtr = csv::Writer::from_writer(Vec::new());
    wtr.write_record(&headers)?;

    for row in &rows {
        let record: Vec<String> = headers
            .iter()
            .map(|h| format_cell(row.get(h.as_str())))
            .collect();
        wtr.write_record(&record)?;
    }
    wtr.flush()?;
    Ok(String::from_utf8(wtr.into_inner()?)?)
}

/// Extract a flat list of JSON objects from the value.
/// Handles: single object, array of objects, or objects with a nested array field.
fn extract_rows(json: &serde_json::Value) -> Vec<&serde_json::Map<String, serde_json::Value>> {
    match json {
        serde_json::Value::Array(arr) => arr.iter().filter_map(|v| v.as_object()).collect(),
        serde_json::Value::Object(obj) => {
            // If the object has a single array field that contains objects, use that
            let array_fields: Vec<_> = obj
                .iter()
                .filter(|(_, v)| matches!(v, serde_json::Value::Array(_)))
                .collect();

            if array_fields.len() == 1
                && let serde_json::Value::Array(arr) = array_fields[0].1
            {
                let inner: Vec<_> = arr.iter().filter_map(|v| v.as_object()).collect();
                if !inner.is_empty() {
                    return inner;
                }
            }

            // Otherwise treat the object itself as a single row
            vec![obj]
        }
        _ => vec![],
    }
}

/// Collect all unique keys from the rows, preserving insertion order.
fn collect_headers(rows: &[&serde_json::Map<String, serde_json::Value>]) -> Vec<String> {
    let mut headers = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for row in rows {
        for key in row.keys() {
            if seen.insert(key.clone()) {
                headers.push(key.clone());
            }
        }
    }
    headers
}

fn format_cell(value: Option<&serde_json::Value>) -> String {
    match value {
        None | Some(serde_json::Value::Null) => String::new(),
        Some(serde_json::Value::String(s)) => s.clone(),
        Some(serde_json::Value::Bool(b)) => b.to_string(),
        Some(serde_json::Value::Number(n)) => n.to_string(),
        Some(other) => other.to_string(),
    }
}
