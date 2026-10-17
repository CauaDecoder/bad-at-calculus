//! Native half of the shared assessment parity check; fixtures are authored and public.

use std::error::Error;

use serde_json::{json, Value};
use vx_api::{commands::ASSESSMENT_CHECK, dispatch};

fn main() -> Result<(), Box<dyn Error>> {
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("../tests/fixtures/assessment-cases.json"))?;
    let reports: Vec<Value> = cases
        .into_iter()
        .map(|case| {
            let result = dispatch(ASSESSMENT_CHECK, case["request"].clone())
                .map_err(|error| format!("{error:?}"))?;
            if result["status"] != case["status"] {
                return Err(format!("Unexpected verdict for {}", case["name"]));
            }
            Ok(json!({"name":case["name"], "result":result}))
        })
        .collect::<Result<_, String>>()?;
    println!("{}", serde_json::to_string(&reports)?);
    Ok(())
}
