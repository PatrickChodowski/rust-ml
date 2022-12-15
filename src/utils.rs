use serde::Deserialize;
use std::{error::Error, collections::HashMap};
use std::ops::{Index, IndexMut};

// PassengerId,Fare,Age,Embarked_C,Embarked_Q,Embarked_S,Pclass_1,Pclass_2,Pclass_3,Sex_female,Sex_male,Survived,No_Cabin,No_Family

#[derive(Debug, Deserialize)]
pub struct TitanicRecord {
  PassengerId: u32,
  Fare: f32,
  Age: f32,
  Embarked_C: f32,
  Embarked_Q: f32,
  Embarked_S: f32,
  Pclass_1: f32,
  Pclass_2: f32,
  Pclass_3: f32,
  Sex_female: f32,
  Sex_male: f32,
  Survived: f32,
  No_Cabin: f32,
  No_Family: f32
}

pub struct TitanicData {
  PassengerId: Vec<u32>,
  Fare: Vec<f32>,
  Age: Vec<f32>,
  Embarked_C: Vec<f32>,
  Embarked_Q: Vec<f32>,
  Embarked_S: Vec<f32>,
  Pclass_1: Vec<f32>,
  Pclass_2: Vec<f32>,
  Pclass_3: Vec<f32>,
  Sex_female: Vec<f32>,
  Sex_male: Vec<f32>,
  Survived: Vec<f32>,
  No_Cabin: Vec<f32>,
  No_Family: Vec<f32>
}



// Table: Hashmap of vectors

// Vector<Hashmap<column_name, vector>
// let mut hmap: HashMap<&str, Vec<u8>> = HashMap::new();




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


// pub fn read_data2(data_path: &str) -> Result<Vec<TitanicRecord>, Box<dyn Error>> {
//   let mut rdr = csv::Reader::from_path(data_path)?;
//   let headers = rdr.headers();
//   println!("CSV Data headers: {:?}", headers);
//   let mut v: Vec<TitanicRecord> = Vec::new();

//   for result in rdr.deserialize() {
//     let record: TitanicRecord = result?;
//     v.push(record);
//   }

//   Ok(v)
// }


pub fn print_data(data: Vec<TitanicRecord>, limit: u32) {
  for (index, record) in data.iter().enumerate() {
    if index < limit.try_into().unwrap() {
      println!("{:?}", record);
    }
  }
}
