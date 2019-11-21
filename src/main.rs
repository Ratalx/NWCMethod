use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
pub mod task_data;
use task_data::{TaskData, Field};
// Metoda tablicowa, pierwsze przybliżenie metoda północno zachodniego wierzchołka, dla problemu pośrednika

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

fn temp_config<P: AsRef<Path>> (path: P) -> Result<TaskData, Box< dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);  

    let config = serde_json::from_reader(reader)?; 
    
    Ok(config)

}

fn get_dual_vars(fields: &Vec<Field>, suppliers_size: usize, recipents_size: usize ) -> (Vec<DualVar>, Vec<DualVar>) {
    let mut aflas: Vec<DualVar> = vec![DualVar::new(0); suppliers_size];
    let mut betas: Vec<DualVar> = vec![DualVar::new(0); recipents_size];
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

fn make_cycle(fields: &Vec<Field>, min_val_index: usize) -> Option<(Vec<Field>, Vec<Field>)> {

    let start_of_cycle = &fields[min_val_index];
    let mut base_fields: Vec<&Field> = fields.iter().filter(|field| field.value != 0).collect();
    let mut row: Vec<&Field> = Vec::new();
    let mut column: Vec<&Field> = Vec::new();
    let mut other: Vec<&Field> = Vec::new();

    if base_fields.contains(&start_of_cycle) {
        return None
    }

    println!("Start of cycle = {:?}", start_of_cycle);
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

    println!("row Fields = {:?}", row);
    println!("column Fields = {:?}", column);
    println!("other Fields = {:?}", other);


    for row_field in &row {
        let mut supplier_id = row_field.supplier_id;
        for column_field in &column {
            let mut recipient_id = column_field.recipient_id;
            let mut positive_fields = vec![start_of_cycle.clone()];
            let mut negative_fields = vec![(*row_field).clone(), (*column_field).clone()];
            let mut positive = true;
            loop {
                match other.iter().find(|other_field| (other_field.recipient_id == recipient_id 
                                                && other_field.supplier_id == supplier_id)) {
                    Some(field) => {
                        if positive {
                            positive_fields.push((*field).clone());   
                        }
                        else {
                            negative_fields.push((*field).clone());
                        }
                        if (negative_fields.len() + positive_fields.len()) % 2 == 0 {
                            return Some((positive_fields, negative_fields))              
                        }
                        break                        
                    }
                    None => { 
                        let tab = if positive {&mut positive_fields} else {&mut negative_fields}; 
                        let mut count = 0;
                        let temp_supplier_id = supplier_id.clone();
                        let temp_recipient_id = recipient_id.clone();

                        for other_field in &other {
                            if other_field.supplier_id == temp_supplier_id {
                                recipient_id = other_field.recipient_id;
                                tab.push((*other_field).clone());
                                count += 1;
                            }
                            if other_field.recipient_id == temp_recipient_id {
                                supplier_id = other_field.supplier_id;
                                tab.push((*other_field).clone());
                                count += 1;
                            }
                        }
                        positive = !positive;
                        if count != 2 {
                            break
                        }
                    }
                }
            }
                                            
       }
    }
    None
}


fn optimize_iteration(mut fields: Vec<Field>, suppliers_size: usize, recipents_size: usize) -> Vec<Field> {
    let mut optimized = true;
    let mut temp_id: usize = 0;
    let (alfas, betas) = get_dual_vars(&fields, suppliers_size, recipents_size);
    let mut largest_field = (fields[0].cost - alfas[fields[0].supplier_id].value - betas[fields[0].recipient_id].value, 0);

        println!();
        println!("Afla = {:?}", alfas);
        println!();
        println!("Beta = {:?}", betas);

    for  field in &mut fields.clone() {
        field.cost -= alfas[field.supplier_id].value + betas[field.recipient_id].value;

        if field.cost > 0 {
            println!("Field = {:?}", field);
            optimized = false;
            if largest_field.0 < field.cost {
                largest_field = (field.cost, temp_id);
            }
        }
        temp_id += 1;
    }
    
    let cycle: (Vec<Field>,Vec<Field>);
    match make_cycle(&fields, largest_field.1) {
        Some(c) => {cycle = c}
        None => { println!("Returned None");
                return fields}
    }

    println!("Cycle = {:?}", cycle);
    let cycle_min_val = cycle.1.iter().min_by(|x,y| x.value.cmp(&y.value)).unwrap().value;

    for i in 0..cycle.0.len(){
        let field_to_add: &mut Field = fields.iter_mut().find(|field| **field == cycle.0[i]).unwrap();
        field_to_add.value += cycle_min_val;
    }

    for i in 0..cycle.1.len(){
        let field_to_sub: &mut Field = fields.iter_mut().find(|field| **field == cycle.1[i]).unwrap();
        field_to_sub.value -= cycle_min_val;
    }

    if !optimized {
        fields = optimize_iteration(fields, suppliers_size, recipents_size);   
        println!("Not Optimized");
    }

    fields
}

// fn export_to_json(config: &Config, cost: &i32) {
//     let mut json = serde_json::to_string_pretty(config).unwrap();
//     let cost_json = serde_json::to_string_pretty(cost).unwrap();

//     json.push_str(cost_json.as_str());
//     println!("{}",json);
// }

fn main() {

    let mut config = temp_config("tempJson/data3.json").unwrap(); 

    config.ballance();
    config.calculate_gains();

    let gains = config.nw_method();

    println!("Gains = {:}", gains);
    let new_fields = optimize_iteration(config.fields, config.suppliers.len(), config.recipients.len());
    println!("New Fileds \n {:?}", new_fields);
    let mut new_gains = 0;

    for field in &new_fields {
        new_gains += field.cost * field.value;
    }

    println!("New Gain = {:?}", new_gains);

}


