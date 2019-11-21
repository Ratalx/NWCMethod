use serde::{Deserialize, Serialize};
use std::cmp::min;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TaskData {
    pub fields: Vec<Field>,
    pub suppliers: Vec<Etnities>,
    pub recipients: Vec<Etnities>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Field {
    pub supplier_id: usize,
    pub recipient_id: usize,
    pub cost: i32,
    pub value: i32
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Etnities {
    pub id: usize,
    pub value: i32,
    pub price: i32,
}

impl TaskData {

    pub fn ballance(self: &mut Self) {
        let mut suppliers_sum = 0;
        let mut recipients_sum = 0;

        for supplier in &mut self.suppliers {
            suppliers_sum += supplier.value;
        }

        for recipient in &mut self.recipients {
            recipients_sum += recipient.value;
        }

        if suppliers_sum == recipients_sum {
            return ;
        }

        if suppliers_sum < recipients_sum {
            let id = self.suppliers.len();

            self.suppliers.push(Etnities::new(id, recipients_sum, 0));
            for recipient in &self.recipients {
                self.fields.push(Field::new(id, recipient.id))
            }
        }
        else {
            let id = self.recipients.len();
            let mut index = self.recipients.len();

            self.recipients.push(Etnities::new(id, suppliers_sum, 0));
            for supplier in &self.suppliers {
                self.fields.insert(index, Field::new(supplier.id, id));
                index += self.recipients.len();
            }
        }
    }

    pub fn calculate_gains(self: &mut Self) {
        
        for field in &mut self.fields {
            if field.cost != 0 {
                field.cost = self.recipients[field.recipient_id].price - self.suppliers[field.supplier_id].price - field.cost;
            }
        }
    }

    pub fn nw_method(self: &mut Self) -> i32 {
        let mut cost: i32 = 0;

        for  field in &mut self.fields {
            field.value = min(self.suppliers[field.supplier_id as usize].value, self.recipients[field.recipient_id as usize].value);
            self.suppliers[field.supplier_id as usize].value -= field.value;
            self.recipients[field.recipient_id as usize].value -= field.value;
            cost += field.cost * field.value;
        }
        cost
    }
}

impl Field {
    pub fn new(supplier_id: usize, recipient_id: usize) -> Field {
        Field {
            supplier_id: supplier_id,
            recipient_id: recipient_id,
            value: 0,
            cost: 0,
        }
    }
}

impl Etnities {
    pub fn new(id: usize, value: i32, price: i32) -> Etnities {
        Etnities {
            id: id,
            value: value,
            price: price,
        }
    }
}

