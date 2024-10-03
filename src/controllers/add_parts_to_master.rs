use anyhow::Result;

use crate::models::parts_master::PartsMaster;
use crate::models::add_parts::AddParts;


pub fn add_parts(add_parts_path:&str,parts_master_path:&str)->Result<()>{
    let mut parts_master = PartsMaster::load(parts_master_path)?;
    let add_parts = AddParts::load(add_parts_path)?;


    parts_master.add_parts_and_write(add_parts)?;
    
    // println!("{:?}",parts_master);


    Ok(())
}