use crate::cli::OutputFormat;

/// Output a serializable value in the requested format.
///
/// For table/CSV, flattens the JSON into rows. Objects become single-row tables,
/// arrays become multi-row tables. Nested objects are JSON-stringified.
pub fn output(value: &impl serde::Serialize, format: &OutputFormat) -> anyhow::Result<()> {
    let json = serde_json::to_value(value)?;

    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        OutputFormat::Table => {
            print_table(&json)?;
        }
        OutputFormat::Csv => {
            print_csv(&json)?;
        }
    }
    Ok(())
}

fn print_table(json: &serde_json::Value) -> anyhow::Result<()> {
    let rows = extract_rows(json);
    if rows.is_empty() {
        println!("(no data)");
        return Ok(());
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

    println!("{table}");
    Ok(())
}

fn print_csv(json: &serde_json::Value) -> anyhow::Result<()> {
    let rows = extract_rows(json);
    if rows.is_empty() {
        return Ok(());
    }

    let headers = collect_headers(&rows);
    let mut wtr = csv::Writer::from_writer(std::io::stdout());
    wtr.write_record(&headers)?;

    for row in &rows {
        let record: Vec<String> = headers
            .iter()
            .map(|h| format_cell(row.get(h.as_str())))
            .collect();
        wtr.write_record(&record)?;
    }
    wtr.flush()?;
    Ok(())
}

/// Extract a flat list of JSON objects from the value.
/// Handles: single object, array of objects, or objects with a nested array field.
fn extract_rows(json: &serde_json::Value) -> Vec<&serde_json::Map<String, serde_json::Value>> {
    match json {
        serde_json::Value::Array(arr) => arr
            .iter()
            .filter_map(|v| v.as_object())
            .collect(),
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
fn collect_headers(
    rows: &[&serde_json::Map<String, serde_json::Value>],
) -> Vec<String> {
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
