use anyhow::Result;

use crate::models::parts_master::PartsMaster;


pub fn convert_to_csv(parts_master_path:&str,csv_path:&str)->Result<()>{
    let parts_master = PartsMaster::load(parts_master_path)?;
    // csvはexcelで開くことを考えてshift-jis
    // parts_master.convert_to_csv_utf8(csv_path)?;
    parts_master.convert_to_csv_jis(csv_path)?;
    Ok(())
}