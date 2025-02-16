use std::str::FromStr;
use serde::{Deserialize, Serialize};
use serde_json::json;
use yaxp_core::xsdp::parser::parse_xsd_string;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Arrow,
    Avro,
    DuckDB,
    Json,
    JsonSchema,
    Polars,
    Spark,
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "arrow" => Ok(OutputFormat::Arrow),
            "avro" => Ok(OutputFormat::Avro),
            "duckdb" => Ok(OutputFormat::DuckDB),
            "json" => Ok(OutputFormat::Json),
            "jsonschema" => Ok(OutputFormat::JsonSchema),
            "polars" => Ok(OutputFormat::Polars),
            "spark" => Ok(OutputFormat::Spark),
            _ => Err(format!("Unknown output format: {}", s)),
        }
    }
}

pub fn transform_xsd(xsd_input: &str, output_format: &str, lowercase: Option<bool>) -> Result<String, Box<dyn std::error::Error>> {
    let xsd_schema = parse_xsd_string(xsd_input, None, lowercase)?;
    // web_sys::console::log_1(&format!("output_format at transforming: {:?}", &output_format).into());


    let res_output = output_format.parse::<OutputFormat>();
    match res_output {
        Err(e) => {
            web_sys::console::log_1(&format!("Error parsing output format: {:?}", e).into());
            Err(json!({"error": e.to_string()}).to_string().into())
        },
        Ok(format) => {
            match format {
                OutputFormat::Spark => {
                    let spark_schema = xsd_schema
                        .to_spark()?
                        .to_json()?;

                    Ok(serde_json::to_string_pretty(&spark_schema)?)
                },
                OutputFormat::Arrow => {
                    let arrow_schema = xsd_schema
                        .to_arrow()?;

                    Ok(arrow_schema.to_string())
                },
                OutputFormat::Json => {
                    let json_pretty = serde_json::to_string_pretty(&xsd_schema)?;
                    Ok(json_pretty)
                },
                OutputFormat::JsonSchema => {
                    let json_schema = xsd_schema.to_json_schema();
                    Ok(serde_json::to_string_pretty(&json_schema)?)
                },
                OutputFormat::Avro => {
                    let avro_schema = xsd_schema.to_avro()?;
                    Ok(serde_json::to_string_pretty(&avro_schema)?)
                },
                OutputFormat::DuckDB => {
                    let duckdb_schema = xsd_schema.to_duckdb_schema();
                    Ok(serde_json::to_string_pretty(&duckdb_schema)?)
                },
                OutputFormat::Polars => {
                    let polars_schema = xsd_schema.to_polars();
                    let fields: Vec<_> = polars_schema.iter().map(|(name, dtype)| {
                        json!({
                            "name": name.to_string(),
                            "dtype": format!("{:?}", dtype.to_string())
                        })
                    }).collect();
                    Ok(format!("{:?}", fields))

                }

            }
        }
    }



}