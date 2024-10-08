use anyhow::Result;
use std::fs::File;
use std::io::{BufReader,BufRead, Write,BufWriter};
use encoding_rs::SHIFT_JIS;

use crate::models::parts_master::PartsMaster;



pub fn generate_parts_list(
    list_path:&str,
    csv_path:&str,
    parts_master_path:&str
)->Result<()>{
    let parts_master = PartsMaster::load(parts_master_path)?;

    // 調査を行う型式を抽出
    let file = File::open(list_path)?;
    let buf = BufReader::new(file);
    let parts_list: Vec<String> = buf
        .lines()
        .map_while(Result::ok)
        .filter(|line| !line.trim().is_empty())
        .collect();

    let mut counts:Vec<(String,u32)> = Vec::new();

    for part in parts_list {
        if let Some((_, count)) = counts.iter_mut().find(|(p, _)| *p == part) {
            *count += 1;
        } else {
            counts.push((part, 1));
        }
    }


    let file = File::create(csv_path)?;
    let mut wtr = BufWriter::new(file);
    
    let header: Vec<String>= vec![
            "名称".to_string(),
            "型式".to_string(),
            "メーカー".to_string(),
            "個数".to_string(),
            "備考".to_string(),
            ];
    let header = header.join(",") + "\n";
    let (encoded_header, _, _) = SHIFT_JIS.encode(&header);

    wtr.write_all(&encoded_header)?;

    for (part, count) in counts {
        let parts_data = match parts_master.find_by_model(&part){
            Some(t)=>t,
            None=> {
                println!("{} はデータベースに登録されていない",part);
                continue
            }
        };
        let remarks = match parts_data.remarks(){
            Some(t)=>t,
            None=> ""
        };
        let record: String= String::new() +
            parts_data.name() +","+
            parts_data.model() +","+
            parts_data.manufacturer() +","+
            &count.to_string() +","+
            remarks + "\n";
        let (encoded_record, _, _) = SHIFT_JIS.encode(&record);
        wtr.write_all(&encoded_record)?;
    }


    Ok(())
}