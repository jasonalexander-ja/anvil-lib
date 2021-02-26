use std::fmt;

/// This is a helper struct that stores information on errors in a clean way, including type, 
/// index (as it appeared in the posistion table) of the chunk at which the error occured, and the 
/// minimum, maximum, and specified values that caused the error. 
/// 
/// # Examples
/// 
/// `fmt::Debug` has been implemented for this so that error can be displayed directly to the console.
/// 
/// ```
/// use anvil_lib::raw::*;
/// 
/// let file: Vec<u8> = Vec::new();
/// match get_region_raw(&file) { // returns Result<Vec<(Vec<u8>, u8)>, RegionParseError>
///     Ok(_) => (),
///     Err(error) => println!("{:?}", error)
/// }
/// ```
pub struct RegionParseError {
    chunk_index: usize,
    specified_val: usize,
    min_val: usize,
    max_val: usize,
    err_type: RegionParseErrorType
}
impl RegionParseError  {
    /// Returns a new error denoting a file that is below the minimum required 
    /// length to parse the posistion and timestamp tables
    /// 
    /// # Examples
    /// 
    /// ```
    /// use anvil_lib::raw::*;
    /// use std::fs;
    /// 
    /// const MIN_SIZE: usize = 8192; // 2 x 4 kibibyte tables 
    /// 
    /// fn unpack() -> Result<Vec<u8>, RegionParseError> {
    ///     let file = fs::read("test/data.bin").expect("Error opening file.");
    ///     if file.len() < MIN_SIZE {  
    ///         return Err(RegionParseError::throw_file_size_err(MIN_SIZE, file.len()));
    ///     }
    ///     Ok(file)
    /// }
    /// ```
    pub fn throw_file_size_err(min_size: usize, size: usize) -> RegionParseError {
        RegionParseError {
            chunk_index: 0,
            specified_val: size,
            min_val: min_size,
            max_val: 0,
            err_type: RegionParseErrorType::FileSizeErr
        }
    }
    /// Returns a new error denoting a chunks indicated end posistion is beyond the end of the file,
    /// takes the index of the chunk as it appears in the posistion table, the size of the file,
    /// and the end point of the chunk. 
    /// 
    /// # Examples
    /// 
    /// ```
    /// use anvil_lib::raw::*;
    /// use std::fs;
    /// 
    /// fn get_chunk(file: &[u8], index: usize, start_offset: usize, size: usize) -> Result<Vec<u8>, RegionParseError> {
    ///     if file.len() < start_offset + size {
    ///         return Err(RegionParseError::throw_chunk_pos_err(index, file.len(), start_offset + size));
    ///     }
    ///     let output = file[start_offset..=(start_offset + size)].to_vec();
    ///     Ok(output)
    /// }
    /// ```
    pub fn throw_chunk_pos_err(index: usize, size: usize, chunk_end: usize) -> RegionParseError {
        RegionParseError {
            chunk_index: index,
            specified_val: chunk_end,
            min_val: 0,
            max_val: size,
            err_type: RegionParseErrorType::ChunkPosErr
        }
    } 
    /// Returns a new error denoting that the specified size for a chunk in it's header
    /// is larger than the size specified in the posistion table. Where the size in the
    /// posistion table should be the chunk size rounded to the nearest 4 kibitytes (4096 bytes). 
    /// 
    /// # Examples
    /// 
    /// ```
    /// use anvil_lib::raw::*;
    /// use std::fs;
    /// 
    /// fn parse_chunk(chunk: &[u8], index: usize, max_size: usize, size: usize) -> Result<Vec<u8>, RegionParseError> {
    ///     if max_size < size {
    ///         return Err(RegionParseError::throw_chunk_header_err(index, max_size, size));
    ///     }
    ///     let output = chunk[index..=(size)].to_vec();
    ///     Ok(output)
    /// }
    /// ```
    pub fn throw_chunk_header_err(index: usize, max_size: usize, chunk_size: usize) -> RegionParseError {
        RegionParseError {
            chunk_index: index, 
            specified_val: chunk_size, 
            min_val: 0,
            max_val: max_size,
            err_type: RegionParseErrorType::ChunkHeaderErr
        }
    } 
}
impl fmt::Debug for RegionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.err_type {
            RegionParseErrorType::FileSizeErr => 
                f.write_fmt(format_args!("The supplied file is smaller than the minimum size, minimum size: {}; size: {};",
                    self.min_val, self.specified_val)),
            RegionParseErrorType::ChunkPosErr => 
                f.write_fmt(format_args!("The chunk end is beyond the end of the end of the file, chunk table index: {}; file size: {}; chunk end: {};",
                    self.chunk_index, self.max_val, self.specified_val)),
            RegionParseErrorType::ChunkHeaderErr => 
                f.write_fmt(format_args!("The chunk size as indicated by the header is larger than the max length indciated by the posistion table, chunk table index: {}; max length: {}; indicated length: {}",
                    self.chunk_index, self.max_val, self.specified_val)),
        }
    }
}
/// This is a helper enum for denoting what type of error has occured when unpacking a file, 
/// allowing for more custom error handling and better debugging. 
/// 
/// * The file size is less than the minimum valid size 
/// 
///     `RegionParseError::FileSizeErr`
/// 
/// * An encountered chunk has an end point beyond the end of the file 
/// 
///     `RegionParseError::ChunkParseErr`
/// 
/// * A chunk header shows a chunk length greater than the rounded-up rough length shown by the posistion table
/// 
///     `RegionParseError::ChunkHeaderErr`
/// 
pub enum RegionParseErrorType {
    FileSizeErr,
    ChunkPosErr,
    ChunkHeaderErr,
}
