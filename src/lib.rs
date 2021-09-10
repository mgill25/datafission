mod parquet_reader;
mod array;
mod schema;
mod datatype;

#[derive(Debug, PartialEq)]
struct RecordBatch {}

#[allow(dead_code)]
impl RecordBatch {
    fn empty() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use crate::RecordBatch;

    #[test]
    fn it_can_create_empty_batch() {
        let empty_batch = RecordBatch::empty();
        assert_eq!(empty_batch, RecordBatch {});
    }
}
