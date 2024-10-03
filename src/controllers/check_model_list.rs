use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::io::{Write,BufWriter};


use crate::models::parts_master::PartsMaster;
use crate::models::add_parts::AddPartData;


pub fn check_model_list(
    list_path:&str,
    output_yaml_path:&str,
    parts_master_path:&str
)->Result<()>{
    let parts_master = PartsMaster::load(parts_master_path)?;
    let (master_model_list,_)= parts_master.get_vec_model_and_max_uuid();

    // 調査を行う型式を抽出
    let file = File::open(list_path)?;
    let buf = io::BufReader::new(file);
    let models: Vec<String> = buf
        .lines()
        .map_while(Result::ok)
        .filter(|line| !line.trim().is_empty())
        .collect();

    let set: HashSet<_> = models.into_iter().collect();
    let mut models: Vec<String> = set.into_iter().collect();
    models.retain(|line| !master_model_list.contains(line));

    // 型式だけ入れた空のYAMLを作成

    let file = File::create(output_yaml_path)?;
    let mut writer = BufWriter::new(file);


    for model in models {
        let yaml_string = AddPartData::serialize_input(model);
        writer.write_all(yaml_string.as_bytes())?;
    }


    Ok(())
}