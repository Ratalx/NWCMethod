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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

fn optimize_iteration(mut fields: Vec<Field>) -> Vec<Field> {
    let mut optimized = true;
    let mut smallest_field = (fields[0].cost, 0);
    let mut temp_id: usize = 0;
    let (alfas, betas) = get_dual_vars(&fields);

    for  field in &mut fields.clone() {
        field.cost -= alfas[field.supplier_id].value + betas[field.recipient_id].value;
        if field.cost < 0{
            optimized = false;
            if smallest_field.0 > field.cost {
                smallest_field = (field.cost,temp_id);
            }
        }
        temp_id += 1;
    }
    
    let cycle: (Vec<Field>,Vec<Field>);
    match make_cycle(&fields, smallest_field.1) {
        Some(c) => {cycle = c}
        None => {return fields}
    }

    let cycle_min_val = cycle.1.iter().min_by(|x,y| x.value.cmp(&y.value)).unwrap().value;

    for i in 0..2{
        let field_to_add:&mut Field = fields.iter_mut().find(|field| **field == cycle.0[i]).unwrap();
        field_to_add.value += cycle_min_val;
    }

    for i in 0..2{
        let field_to_sub:&mut Field = fields.iter_mut().find(|field| **field == cycle.1[i]).unwrap();
        field_to_sub.value -= cycle_min_val;
    }


    // if !optimized {
    //     fields = optimize_iteration(fields);   
    // }

    fields
}

fn make_cycle(fields: &Vec<Field>, min_val_index: usize) -> Option<(Vec<Field>, Vec<Field>)> {

    let start_of_cycle = &fields[min_val_index];
    let mut base_fields: Vec<&Field> = fields.iter().filter(|field| field.value != 0).collect();
    let mut row: Vec<&Field> = Vec::new();
    let mut column: Vec<&Field> = Vec::new();
    let mut other: Vec<&Field> = Vec::new();

    loop {
        match base_fields.pop() {
            Some(field) => {

                if field.recipient_id == start_of_cycle.recipient_id {
                    row.push(field);
                }
                else if field.supplier_id == start_of_cycle.supplier_id {
                    column.push(field);
                }
                else {
                    other.push(field);
               }
            }
            None => break
        }
    }

    for row_field in &row {
        let supplier_id = row_field.supplier_id;
        for column_field in &column {
            let recipient_id = column_field.recipient_id;
            match other.iter().find(|other_field| (other_field.recipient_id == recipient_id 
                                            && other_field.supplier_id == supplier_id)) {
                                                Some(field) => {
                                                    // Aparently Linter won't enable *field.clone() as it suspect a move operations. 
                                                    let f: &Field = *field;
                                                    let r: &Field = *row_field;
                                                    let c: &Field = *column_field;
                                                   return Some((vec![start_of_cycle.clone(), f.clone()], vec![r.clone(), c.clone()]))
                                                }
                                                None =>{}
                                            }
       }
    }
    None
}

fn export_to_json(config: &Config, cost: &i32) {
    let mut json = serde_json::to_string_pretty(config).unwrap();
    let cost_json = serde_json::to_string_pretty(cost).unwrap();

    json.push_str(cost_json.as_str());
    println!("{}",json);
}

fn main() {

    let mut config = temp_config("tempJson/data.json").unwrap(); 

    let cost = nw_method(&mut config);

    let new_fields = optimize_iteration(config.fields);
    println!("New Fileds \n {:?}", new_fields);

}


