use serde::Deserialize;
use std::{error::Error};
use std::collections::HashMap;


pub type DataFrame<'a> = HashMap<&'a str, Vec<f32>>;


// PassengerId,Fare,Age,Embarked_C,Embarked_Q,Embarked_S,Pclass_1,Pclass_2,Pclass_3,Sex_female,Sex_male,Survived,No_Cabin,No_Family

#[derive(Debug, Deserialize)]
pub struct TitanicRecord {
  passenger_id: f32,
  survived:     f32,
  fare:         f32,
  age:          f32,
  embarked_c:   f32,
  embarked_q:   f32,
  embarked_s:   f32,
  pclass1:      f32,
  pclass2:      f32,
  pclass3:      f32,
  sex_female:   f32,
  sex_male:     f32,
  no_cabin:     f32,
  no_family:    f32
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


pub fn read_data3(data_path: &str) -> Result<DataFrame, Box<dyn Error>> {
  let mut data = HashMap::new();
  data.insert("passenger_id", Vec::new());
  data.insert("survived",     Vec::new());
  data.insert("fare",         Vec::new());
  data.insert("age",          Vec::new());
  data.insert("embarked_c",   Vec::new());
  data.insert("embarked_q",   Vec::new());
  data.insert("embarked_s",   Vec::new());
  data.insert("pclass1",      Vec::new());
  data.insert("pclass2",      Vec::new());
  data.insert("pclass3",      Vec::new());
  data.insert("sex_female",   Vec::new());
  data.insert("sex_male",     Vec::new());
  data.insert("no_cabin",     Vec::new());
  data.insert("no_family",    Vec::new());

  let mut rdr = csv::Reader::from_path(data_path)?;
  let headers = rdr.headers();
  // println!("CSV Data headers: {:?}", headers);

  let mut v: Vec<TitanicRecord> = Vec::new();
  for result in rdr.deserialize() {
    let record: TitanicRecord = result?;
    data.entry("passenger_id").or_default().push(record.passenger_id);
    data.entry("survived").or_default().push(record.survived);
    data.entry("fare").or_default().push(record.fare);
    data.entry("age").or_default().push(record.age);
    data.entry("embarked_c").or_default().push(record.embarked_c);
    data.entry("embarked_q").or_default().push(record.embarked_q);
    data.entry("embarked_s").or_default().push(record.embarked_s);
    data.entry("pclass1").or_default().push(record.pclass1);
    data.entry("pclass2").or_default().push(record.pclass2);
    data.entry("pclass3").or_default().push(record.pclass3);
    data.entry("sex_female").or_default().push(record.sex_female);
    data.entry("sex_male").or_default().push(record.sex_male);
    data.entry("no_cabin").or_default().push(record.no_cabin);
    data.entry("no_family").or_default().push(record.no_family);
  }

  return Ok(data);

}



pub fn print_data(data: Vec<TitanicRecord>, limit: u32) {
  for (index, record) in data.iter().enumerate() {
    if index < limit.try_into().unwrap() {
      println!("{:?}", record);
    }
  }
}

