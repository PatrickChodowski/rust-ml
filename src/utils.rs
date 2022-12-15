use serde::Deserialize;
use std::{error::Error};

// PassengerId,Fare,Age,Embarked_C,Embarked_Q,Embarked_S,Pclass_1,Pclass_2,Pclass_3,Sex_female,Sex_male,Survived,No_Cabin,No_Family

#[derive(Debug, Deserialize)]
pub struct TitanicRecord {
  passenger_id: u32,
  survived: f32,
  fare: f32,
  age: f32,
  embarked_c: f32,
  embarked_q: f32,
  embarked_s: f32,
  pclass1: f32,
  pclass2: f32,
  pclass3: f32,
  sex_female: f32,
  sex_male: f32,
  no_cabin: f32,
  no_family: f32
}

pub struct TitanicData {
  passenger_id: Vec<u32>,
  survived: Vec<f32>,
  fare: Vec<f32>,
  age: Vec<f32>,
  embarked_c: Vec<f32>,
  embarked_q: Vec<f32>,
  embarked_s: Vec<f32>,
  pclass1: Vec<f32>,
  pclass2: Vec<f32>,
  pclass3: Vec<f32>,
  sex_female: Vec<f32>,
  sex_male: Vec<f32>,
  no_cabin: Vec<f32>,
  no_family: Vec<f32>
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


pub fn read_data2(data_path: &str) -> Result<TitanicData, Box<dyn Error>> {
  let mut rdr = csv::Reader::from_path(data_path)?;
  let headers = rdr.headers();
  println!("CSV Data headers: {:?}", headers);

  let mut data: TitanicData = TitanicData{passenger_id: Vec::new(),
                                          survived: Vec::new(),
                                          fare: Vec::new(),
                                          age: Vec::new(),
                                          embarked_c: Vec::new(),
                                          embarked_q: Vec::new(),
                                          embarked_s: Vec::new(),
                                          pclass1: Vec::new(),
                                          pclass2: Vec::new(),
                                          pclass3: Vec::new(),
                                          sex_female: Vec::new(),
                                          sex_male: Vec::new(),
                                          no_cabin: Vec::new(),
                                          no_family: Vec::new()
                              };

  for result in rdr.deserialize() {
    let record: TitanicRecord = result?;
    data.passenger_id.push(record.passenger_id);
    data.survived.push(record.survived);
    data.fare.push(record.fare);
    data.age.push(record.age);
    data.embarked_c.push(record.embarked_c);
    data.embarked_q.push(record.embarked_q);
    data.embarked_s.push(record.embarked_s);
    data.pclass1.push(record.pclass1);
    data.pclass2.push(record.pclass2);
    data.pclass3.push(record.pclass3);
    data.sex_female.push(record.sex_female);
    data.sex_male.push(record.sex_male);
    data.no_cabin.push(record.no_cabin);
    data.no_family.push(record.no_family);
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



// pub fn print_data2(data: TitanicData, limit: u32) {
//   for (index, record) in data.PassengerId.enumerate() {
//     if index < limit.try_into().unwrap() {
//       println!("{:?}", record);
//     }
//   }
// }
