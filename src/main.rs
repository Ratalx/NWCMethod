use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// Metoda tablicowa, pierwsze przybliżenie metoda północno zachodniego wierzchołka, dla problemu pośrednika

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Config{
    fields: Vec<Field>,
    suppliers: Vec<Etnities>,
    recipients: Vec<Etnities>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Field{
    supplier_id: usize,
    recipient_id: usize,
    cost: i32,
    value: i32
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Etnities {
    id: i32,
    value: i32,
}

#[derive(Clone, Debug)]
struct DualVar {
    value: i32,//Afla - supplier,
    //Beta - recipents
}

impl DualVar {
    pub fn new (value: i32) -> DualVar {
        DualVar{ 
            value: value
        }
    }
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

fn get_dual_vars(fields: &Vec<Field>) -> (Vec<DualVar>, Vec<DualVar>) {
    let mut aflas: Vec<DualVar> = vec![DualVar::new(0), DualVar::new(0), DualVar::new(0)];
    let mut betas: Vec<DualVar> = vec![DualVar::new(0); 3];
    for field in fields {

        if 0 != field.value {
            let beta = &mut betas[field.recipient_id].value;
            let alfa = &mut aflas[field.supplier_id].value;
            let cost = & field.cost;

           if 0 == *beta { 
                *beta = *cost - *alfa;
           }
           if 0 == *alfa {
               *alfa = *cost - *beta;
           }
        }
    }
    (aflas, betas)
}



fn optimize(fields: Vec<Field>, alfas: Vec<DualVar>, betas: Vec<DualVar>) -> bool {
    let mut optimized = true;

    for mut field in fields {
        if 0 != field.value{
            field.cost = 0;
        } 
        else {
            field.cost -= alfas[field.supplier_id].value + betas[field.recipient_id].value;
            if field.cost < 0{
                optimized = false;     
            }
        }
    }

    if !optimized {
        {
            
        }        
    }

    optimized
}

fn iterate_optimize() {
    
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

    let (aflas, betas) = get_dual_vars(&config.fields);

    println!("{:?}", aflas);
    println!("{:?}", betas);



    //export_to_json(&config, &cost);

}


