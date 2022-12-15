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


pub fn read_data2(data_path: &str) -> Result<TitanicData, Box<dyn Error>> {
  let mut rdr = csv::Reader::from_path(data_path)?;
  let headers = rdr.headers();
  println!("CSV Data headers: {:?}", headers);

  let mut data: TitanicData = TitanicData{PassengerId: Vec::new(),
                                          Fare: Vec::new(),
                                          Age: Vec::new(),
                                          Embarked_C: Vec::new(),
                                          Embarked_Q: Vec::new(),
                                          Embarked_S: Vec::new(),
                                          Pclass_1: Vec::new(),
                                          Pclass_2: Vec::new(),
                                          Pclass_3: Vec::new(),
                                          Sex_female: Vec::new(),
                                          Sex_male: Vec::new(),
                                          Survived: Vec::new(),
                                          No_Cabin: Vec::new(),
                                          No_Family: Vec::new()
                              };

  for result in rdr.deserialize() {
    let record: TitanicRecord = result?;
    data.PassengerId.push(record.PassengerId);
    data.Fare.push(record.Fare);
    data.Age.push(record.Age);
    data.Embarked_C.push(record.Embarked_C);
    data.Embarked_Q.push(record.Embarked_Q);
    data.Embarked_S.push(record.Embarked_S);
    data.Pclass_1.push(record.Pclass_1);
    data.Pclass_2.push(record.Pclass_2);
    data.Pclass_3.push(record.Pclass_3);
    data.Sex_female.push(record.Sex_female);
    data.Sex_male.push(record.Sex_male);
    data.Survived.push(record.Survived);
    data.No_Cabin.push(record.No_Cabin);
    data.No_Family.push(record.No_Family);
  }

  Ok(data)
}





pub fn print_data(data: Vec<TitanicRecord>, limit: u32) {
  for (index, record) in data.iter().enumerate() {
    if index < limit.try_into().unwrap() {
      println!("{:?}", record);
    }
  }
}
