use rocksdb::{DB,Options};
use std::fs;


pub fn put_data(hash : String, data : Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let mut opt = Options::default();
    opt.create_if_missing(true);

    let db = DB::open(&opt,"metadata.db").expect(" Failed to open the data base ");

    db.put(hash.as_bytes(),data.as_slice())?;

    Ok(())

}
pub fn get_data(hash : String) -> Result<Vec<u8>, Box<dyn std::error::Error>>{
    let mut opt = Options::default();
    opt.create_if_missing(true);

    let db = DB::open(&opt,"metadata.db").expect(" Failed to open the data base ");

    match db.get(hash.as_bytes()){
        Ok(Some(value)) => Ok(value.to_vec()),
        Ok(None) => Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound,"File not found"))),
        Err(e) => Err(Box::new(e))
    }
}

pub fn delete_data(hash : String) -> Result<(), Box<dyn std::error::Error>>{
        let mut opt = Options::default();
        opt.create_if_missing(true);

        let db = DB::open(&opt,"metadata.db").expect(" Failed to open the data base ");

        match db.delete(hash.as_bytes()){
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e))
        }
    }