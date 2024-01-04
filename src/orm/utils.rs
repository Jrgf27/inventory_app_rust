use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub enum GenericModel {
    IntegerField(i64),
    CharField(String),
}

pub trait ModelMethods: Serialize {
    fn model_name(&self) -> String {
        let model = std::any::type_name::<Self>().to_string();
        let model_split = model.split("::");
        model_split.last().unwrap().to_string()
    }

    fn db_init(&self) -> String {
        let model_name = self.model_name();

        let json_value: Value = serde_json::to_value(self).expect("Failed to serialize struct");

        let mut res: String = "CREATE TABLE IF NOT EXISTS ".to_string()
            + &model_name
            + " (id INTEGER PRIMARY KEY NOT NULL,";

        if let Value::Object(fields) = json_value {
            for (field_name, field_value) in fields {
                // Match the field value with GenericModel type
                match serde_json::from_value::<GenericModel>(field_value) {
                    Ok(generic_model) => match generic_model {
                        GenericModel::IntegerField(_value) => {
                            if field_name == "id" {
                                continue;
                            } else {
                                res += &format!("{} INTEGER NOT NULL,", field_name);
                            }
                        }
                        GenericModel::CharField(_value) => {
                            res += &format!("{} VARCHAR(250) NOT NULL,", field_name);
                        }
                    },
                    Err(_) => println!("Unknown field type"),
                }
            }
        }
        let mut chars = res.chars();
        chars.next_back();
        res = chars.as_str().to_string().clone();

        res += ");";

        res
    }

    fn insert_values<T: Serialize>(&self, payload: T) -> String {
        let model_name = self.model_name();

        let json_value: Value = serde_json::to_value(self).expect("Failed to serialize struct");

        let json_payload: Value =
            serde_json::to_value(payload).expect("Failed to serialize struct");

        let mut res: String = "INSERT INTO ".to_string() + &model_name + " (";
        let mut values_to_insert: String = "VALUES (".to_string();

        if let Value::Object(fields) = json_value {
            for (field_name, _field_value) in fields {
                if let Value::Object(fields_payload) = &json_payload {
                    for (field_name_payload, field_value_payload) in fields_payload {
                        if field_name == "id" {
                            continue;
                        } else if field_name_payload == &field_name {
                            res += &format!(" {},", &field_name);

                            values_to_insert += &format!(" {},", &field_value_payload);
                        }
                    }
                }
            }
        }
        let mut chars = res.chars();
        chars.next_back();
        res = chars.as_str().to_string().clone();

        let mut chars = values_to_insert.chars();
        chars.next_back();
        values_to_insert = chars.as_str().to_string().clone();

        res += ")";
        res += &values_to_insert;
        res += ");";

        res
    }

    fn return_last_id(&self) -> String {
        let model_name = &self.model_name();
        format!("SELECT id FROM {model_name} ORDER BY id DESC LIMIT 1")
    }

    fn db_retrieve_all(&self) -> String {
        let model_name = &self.model_name();
        format!("SELECT * FROM {model_name}")
    }

    fn db_retrieve_by_id(&self, id: i64) -> String {
        let model_name = &self.model_name();
        format!("SELECT * FROM {model_name} WHERE id = {id}")
    }

    fn db_retrieve_by_field(&self, field: &str, value: &str) -> String {
        let model_name = &self.model_name();
        format!("SELECT * FROM {model_name} WHERE {field} = {value}")
    }

    fn db_update_by_id<T: Serialize>(&self, id: i64, payload: T) -> String {
        let model_name = self.model_name();

        let json_value: Value = serde_json::to_value(self).expect("Failed to serialize struct");

        let json_payload: Value =
            serde_json::to_value(payload).expect("Failed to serialize struct");

        let mut res: String = "UPDATE ".to_string() + &model_name + " SET";

        if let Value::Object(fields) = json_value {
            for (field_name, _field_value) in fields {
                if let Value::Object(fields_payload) = &json_payload {
                    for (field_name_payload, field_value_payload) in fields_payload {
                        if field_name == "id" {
                            continue;
                        } else if field_name_payload == &field_name {
                            res += &format!(" {field_name} = {field_value_payload},");
                        }
                    }
                }
            }
        }
        let mut chars = res.chars();
        chars.next_back();
        res = chars.as_str().to_string().clone();

        res += &format!(" WHERE id = {id};");

        res
    }

    fn db_delete_by_id(&self, id: i64) -> String {
        let model_name = &self.model_name();
        format!("DELETE FROM {model_name} WHERE id = {id}")
    }
}
