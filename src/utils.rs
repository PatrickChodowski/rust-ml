
use polars::prelude::*;

pub fn read_csv(csv_path: &str) -> PolarsResult<DataFrame> {
    CsvReader::from_path(csv_path)?
            .has_header(true)
            .finish()
}


pub fn preprocess(){
  
}