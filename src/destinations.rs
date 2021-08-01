use csv::{ReaderBuilder, Trim};
use std::collections::HashMap;

pub fn get_destinations(
    file_path: &str,
    simulate: bool,
) -> Result<Vec<HashMap<String, String>>, String> {
    let destinations = if simulate {
        get_fixed_destinations()?
    } else {
        get_destionations_from_file(file_path)?
    };
    Ok(destinations)
}

fn get_destionations_from_file(file_path: &str) -> Result<Vec<HashMap<String, String>>, String> {
    let rdr = csv::Reader::from_path(file_path)
        .map_err(|err| format!("Error when opening file: {}", err))?;
    get_destionations_from_reader(rdr)?
}

fn get_fixed_destinations() -> Result<Vec<HashMap<String, String>>, String> {
    let rdr = ReaderBuilder::new().trim(Trim::All).from_reader(
        "email_address,name,other_property,x,y
    john@giggio.net,John Doe,one value,a,b
    mary@giggio.net,Mary Doe,another value,c,d"
            .as_bytes(),
    );
    get_destionations_from_reader(rdr)?
}

fn get_destionations_from_reader<T: std::io::Read>(
    mut rdr: csv::Reader<T>,
) -> Result<Result<Vec<HashMap<String, String>>, String>, String> {
    let mut results = vec![];
    let headers_record = {
        rdr.headers()
            .map_err(|err| format!("Error when opening header: {}", err))?
            .clone()
    };
    let headers: Vec<&str> = headers_record.iter().collect();
    for result in rdr.records() {
        let record = result.map_err(|err| format!("Error when opening record: {}", err))?;
        let mut destination = HashMap::new();
        for (i, header) in headers.iter().enumerate() {
            destination.insert(header.to_string(), record.get(i).expect("").to_owned());
        }
        printlnv!("Destination is {:?}", &destination);
        results.push(destination);
    }
    Ok(Ok(results))
}
