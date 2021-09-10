use parquet::file::reader::SerializedFileReader;
use std::fs::File;
use std::path::Path;

/**
 * Read rows from a given parquet file.
**/
#[allow(dead_code)]
fn create_parquet_reader(file_path: &str) -> SerializedFileReader<File> {
    let file = File::open(&Path::new(file_path)).unwrap();
    SerializedFileReader::new(file).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::parquet_reader::create_parquet_reader;
    use parquet::file::reader::{FileReader, SerializedFileReader};
    use std::fs::File;
    use parquet::record::Row;

    #[test]
    fn it_can_read_parquet_file() {
        let filename = "test_data/yellow_tripdata_2020-12_tiny.parquet";
        let reader: SerializedFileReader<File> = create_parquet_reader(filename);
        let mut iter = reader.get_row_iter(None).unwrap();
        while let Some(row) = iter.next() {
            println!("{}", row);
        }
    }
}
