use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;
use std::io::Read;
use anyhow::Result;

use super::parts_master::PartData;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddPartData {
    name: String,
    model: String,
    manufacturer: String,
    tag: Option<String>,
    // 空白の場合falseとする
    is_discontinued: Option<bool>,
    update_date: String,
    // カンマ区切りの入力も受け付ける
    estimated_price: Option<String>,
    url: Option<String>,
    remarks: Option<String>,
}

#[derive(Debug)]
pub struct AddParts{
    inner: Vec<AddPartData>
}


impl AddParts {
    pub fn load(path:&str)-> Result<Self>{
        let mut add_parts_file = File::open(path)?;
        let mut add_parts_contents = String::new();
        add_parts_file.read_to_string(&mut add_parts_contents)?;

        let add_parts: Vec<AddPartData> = if add_parts_contents.is_empty() {
            Vec::new()
        } else {
            serde_yaml::from_str(&add_parts_contents)?
        };

        Ok(Self { inner: add_parts })
    }

    pub fn take_inner(self)-> Vec<AddPartData>{
        self.inner
    }


}


impl AddPartData {
    pub fn convert(self,uuid:u32)-> Result<PartData>{
        let is_discontinued = self.is_discontinued.unwrap_or(false);
        let estimated_price:Option<f32>= match self.estimated_price{
            Some(string_price)   => {
                let string_price= string_price.replace(",", "");
                let num_price:Result<f32> = string_price.parse().map_err(anyhow::Error::new);
                match num_price {
                    Ok(t)=>Some(t),
                    Err(_)=> None
                }
            },
            None => None
        };

        let parts_data = PartData::new(
            uuid,
            self.name,
            self.model,
            self.manufacturer,
            self.tag, is_discontinued,
            self.update_date,
            estimated_price,
            self.url,
            self.remarks
        );


        Ok(parts_data)
    }

    pub fn serialize_input(model:String)->String{
        format!(
            "- name: \n  model: {}\n  manufacturer: \n  tag: \n  is_discontinued: \n  update_date: \n  estimated_price: \n  url: \n  remarks: \n\n",
            model
        )
    }
}