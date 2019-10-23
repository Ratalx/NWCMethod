use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Config{
    fields: Vec<Field>,
    suppliers: Vec<Etnities>,
    recipients: Vec<Etnities>,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
struct Field{
    supplier_id: i32,
    recipient_id: i32,
    cost: i32,
    value: i32
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Etnities {
    id: i32,
    value: i32
}

fn temp_config<P: AsRef<Path>> (path: P) -> Result<Config, Box< dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);  

    let config = serde_json::from_reader(reader)?; 
    
    Ok(config)

}

fn nw_method(config: &mut Config) -> i32 {
    let fields: &mut Vec<Field> = &mut config.fields;
    let recipients: &mut Vec<Etnities> = &mut config.recipients;
    let suppliers: &mut Vec<Etnities> = &mut config.suppliers;
    let mut cost: i32 = 0;

    for mut field in fields
    {
      field.value = min(suppliers[field.supplier_id as usize].value, recipients[field.recipient_id as usize].value);
      suppliers[field.supplier_id as usize].value -= field.value;
      recipients[field.recipient_id as usize].value -= field.value;
      cost += field.cost*field.value;
    }
    cost
}

fn export_to_json(config: &Config, cost: &i32)
{
    let mut json = serde_json::to_string_pretty(config).unwrap();
    let cost_json = serde_json::to_string_pretty(cost).unwrap();

    json.push_str(cost_json.as_str());
    println!("{}",json);


}

fn main() {

    let mut config = temp_config("tempJson/data.json").unwrap(); 

    let cost = nw_method(&mut config);

    export_to_json(&config, &cost);

}


