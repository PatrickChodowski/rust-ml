use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct TitanicRecord {
  PassengerId: u32,
  Survived: String,
  Pclass: u32,
  Name: String,
  Sex: String, 
  Age: String,
  SibSp: u32,
  Parch: u32,
  Ticket: String,
  Fare: f32,
  Cabin: String,
  Embarked: String
}



pub fn read_data(data_path: &str) -> Result<Vec<TitanicRecord>, Box<dyn Error>> {
  let mut rdr = csv::Reader::from_path(data_path)?;
  let headers = rdr.headers();
  println!("CSV Data headers: {:?}", headers);
  let mut v: Vec<TitanicRecord> = Vec::new();

  for result in rdr.deserialize() {
    let record: TitanicRecord = result?;
    v.push(record);
  }

  Ok(v)
}

pub fn print_data(data: Vec<TitanicRecord>, limit: u32) {
  for (index, record) in data.iter().enumerate() {
    if index < limit.try_into().unwrap() {
      println!("{:?}", record);
    }
  }
}
