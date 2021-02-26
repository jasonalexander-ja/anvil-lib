mod unpack_errors;
use super::AnvilSchema;

pub use unpack_errors::*;
pub use unpack_errors::*;

pub fn get_region_raw(file: &[u8], schema: AnvilSchema) -> Result<Vec<(Vec<u8>, Vec<u8>)>, RegionParseError> {
    if file.len() < schema.min_anvil_file_size { 
        return Err(RegionParseError::throw_file_size_err(file.len(), schema.min_anvil_file_size)) 
    }
    let chunk_posistions = get_posistion_table(&file, &schema)?; 
    get_regions_from_headers(&file, &chunk_posistions, &schema)
}

fn get_regions_from_headers(file: &[u8], headers: &[(usize, usize)], schema: &AnvilSchema) 
    -> Result<Vec<(Vec<u8>, Vec<u8>)>, RegionParseError> 
{
    let mut output: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
    for (iter, (pos, size)) in headers.iter().enumerate() {
        if pos != &0 {
            let end_pos = pos + size;
            let chunk = get_chunk(&file, *pos, end_pos, iter, &schema)?;
            output.push(chunk);
        } else {
            output.push((Vec::new(), Vec::new()));
        }
    }
    Ok(output)
}

// Parses the first table in the file 
fn get_posistion_table(file: &[u8], schema: &AnvilSchema) -> Result<Vec<(usize, usize)>, RegionParseError> {
    let mut output_vec: Vec<(usize, usize)> = Vec::new();
    for iter in 0..schema.chunks_per_region {
        let record = get_pos_record(file, iter, schema);
        output_vec.push(record);
    }
    Ok(output_vec)
}

// Gets a record from the first table in the file
fn get_pos_record(file: &[u8], rec_no: usize, schema: &AnvilSchema) -> (usize, usize) {
    let offset = rec_no * schema.posistion_table_record_len;
    let pos_data_slice = &file[schema.pos_table_start_bytes.start + offset..schema.pos_table_start_bytes.end + offset];
    let pos_data = make_num_from_bytes(pos_data_slice) * schema.pos_multiplier;
    let size_raw = &file[schema.pos_table_size_bytes.start  + offset..schema.pos_table_size_bytes.end +  offset];
    let size = make_num_from_bytes(size_raw) * schema.size_multiplier;
    (pos_data, size)
}

// Finds the relavant chunk from the file 
fn get_chunk(file: &[u8], start: usize, end: usize, chunk_index: usize, schema: &AnvilSchema) 
        -> Result<(Vec<u8>, Vec<u8>), RegionParseError> 
{
    if file.len() < end { // Ensures we aren't about to index ouside the Vec `file`
        return Err(RegionParseError::throw_chunk_pos_err(chunk_index, file.len(), end)) 
    }
    let chunk = &file[start..end];
    let (size, compression) = parse_chunk_header(&chunk, &schema)?;
    if size > end { // Make sure we're not about to indesx outside of chunk 
        return Err(RegionParseError::throw_chunk_header_err(chunk_index, end, size)); 
    }
    let output_vec = chunk[schema.chunk_starts_from..size].to_vec();
    Ok((output_vec, compression))
}

// Parses the first few bytes of a chunk
fn parse_chunk_header(chunk: &[u8], schema: &AnvilSchema) -> Result<(usize, Vec<u8>), RegionParseError> {
    let compression = chunk[schema.chunk_header_compr_bytes.start..schema.chunk_header_compr_bytes.end].to_vec();
    let size_raw = &chunk[schema.chunk_header_size_bytes.start..schema.chunk_header_size_bytes.end];
    let size = make_num_from_bytes(&size_raw) + schema.chunk_starts_from;
    Ok((size, compression))
}

// Helper funtion to turn arrays of bytes read from files into full numbers 
fn make_num_from_bytes(bytes: &[u8]) -> usize {
    let mut output: usize = 0; 
    for (iter, val) in bytes.into_iter().rev().enumerate() {
        output += (*val as usize) << (iter * 8); 
    }
    output
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use std::fs;
    // Testing the function calls in reverse order they would be called when get_region_raw is called 
    // First start off with make_num_from_bytes as this is used all over the place as a general helper 
    #[test]
    fn check_num_making() {
        let test_vec = vec![0x1, 0x0];
        assert_eq!(make_num_from_bytes(&test_vec), 256);
    }
    // Check the chunk header parser 
    #[test]
    fn check_header_parse() {
        let schema = AnvilSchema::default();
        let header = vec![0x0, 0x0, 0x0, 0x1, 0x2];
        match parse_chunk_header(&header, &schema) {
            Ok(val) => {
                assert_eq!(val.0, 6);
                assert_eq!(val.1, vec![2]);
            },
            Err(error) => panic!("{:?}", error),
        };
    }
    // Check the chunk parse functionality 
    #[test]
    fn test_get_chunk() {
        let schema = AnvilSchema::default(); 
        let chunk = fs::read("data/chunks/ok.bin").expect("Failed to open file."); 
        match get_chunk(&chunk, 0, 4096, 0, &schema) {
            Ok(val) => { 
                let last_byte = val.0[val.0.len() - 1];
                let middle_byte = val.0[165];
                let first_byte = val.0[0];
                assert_eq!(last_byte, 112); 
                assert_eq!(first_byte, 120);
                assert_eq!(middle_byte, 181);
        }, 
            Err(error) => panic!("{:?}", error), 
        };
    }
    // Check the parsing of the posistion table 
    #[test]
    fn test_get_posistion_table() {
        let schema = AnvilSchema::default();
        let file = fs::read("data/test.bin").expect("Failed to open file."); 
        match get_posistion_table(&file, &schema) {
            Ok(val) => {
                assert_eq!(val[0].1, 4096); 
            }, 
            Err(error) => panic!("{:?}", error), 
        };
    }
    // Check the parsing of the file as a whole 
    #[test]
    fn check_region_raw() {
        let schema = AnvilSchema::default(); 
        let file = fs::read("data/test.bin").expect("Failed to open file."); 
        match get_region_raw(&file, schema) {
            Ok(val) => { 
                let chunk = &val[512].0;
                assert_eq!(120, chunk[0]); 
                assert_eq!(201, chunk[304]); 
                assert_eq!(97, chunk[608]); 
            }, 
            Err(error) => panic!("{:?}", error), 
        } 
    }
}
