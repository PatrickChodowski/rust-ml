
use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TitanicRecord {
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


fn read_data(data_path: &str) -> Result<Vec<TitanicRecord>, Box<dyn Error>> {
  let mut rdr = csv::Reader::from_path(data_path)?;
  let headers = rdr.headers();
  println!("CSV Data headers: {:?}", headers);
  let mut v: Vec<TitanicRecord> = Vec::new();

  for result in rdr.deserialize() {
    let record: TitanicRecord = result?;
    // println!("{:?}", record);
    v.push(record);
  }

  Ok(v)
}

fn print_data(data: Vec<TitanicRecord>) {
  for record in data {
    println!("{:?}", record);
  }
}

fn main() {
  // If htere is no data it might as well fail
  // let res = read_data("./data/train2.csv").unwrap();

  // Handle result properly
  let res_data = read_data("./data/train.csv");
  if let Ok(e) = res_data {
    print_data(e);
  } else if let Err(e) = res_data {
    eprintln!("{}", e);
  }

}
