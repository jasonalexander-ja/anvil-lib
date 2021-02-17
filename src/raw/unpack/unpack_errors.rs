use std::fmt;

/// This is just a helper enum for denoting what type of error has occured when unpacking a file, 
/// this allows for more custom error handling and better debugging. 
/// 
/// * The file size is less than the minimum valid size 
/// 
///     `RegionParseError::FileSizeErr(file_length, minimum_length)`
/// 
/// * An encountered chunk has an end point beyond the end of the file 
/// 
///     `RegionParseError::ChunkParseErr(chunk_number, file_length, chunk_end)`
/// 
/// * A chunk header shows a chunk length greater than the rounded-up rough length shown by the posistion table
/// 
///     `RegionParseError::ChunkHeaderErr(chunk_number, max_length, indicated_length)`
/// 
/// # Examples
/// 
/// `fmt::Debug` has been implemented for this so that error can be displayed directly to the console.
/// 
/// ```
/// use anvil_lib::raw::get_region_raw;
/// 
/// let file: Vec<u8> = Vec::new();
/// match get_region_raw(&file) { // returns Result<Vec<(Vec<u8>, u8)>, RegionParseError>
///     Ok(_) => (),
///     Err(error) => println!("{:?}", error)
/// }
/// ```
pub enum RegionParseError {
    FileSizeErr(usize, usize), // (file_length, minimum_length)
    ChunkPosErr(usize, usize, usize), // (chunk_no, file_length, chunk_end) 
    ChunkHeaderErr(usize, usize, usize) // (chunk_no, max_len, indicated_len)
}
impl fmt::Debug for RegionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegionParseError::FileSizeErr(file_size, min_file_size) => 
                f.write_fmt(format_args!("The supplied file is smaller than the minimum size, minimum size: {}; size: {};",
                    file_size, min_file_size)),
            RegionParseError::ChunkPosErr(chunk_no, file_length, chunk_end) => 
                f.write_fmt(format_args!("The chunk end is beyond the end of the end of the file, chunk table index: {}; file size: {}; chunk end: {};",
                    chunk_no, file_length, chunk_end)),
            RegionParseError::ChunkHeaderErr(chunk_no, max_len, indicated_len) => 
                f.write_fmt(format_args!("The chunk size as indicated by the header is larger than the max length indciated by the posistion table, chunk table index: {}; max length: {}; indicated length: {}",
                    chunk_no, max_len, indicated_len))
        }
    }
}
