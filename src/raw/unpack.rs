use std::fmt;

/// This is just a helper type for denoting what type of error has occured when unpacking a file 
/// 
/// # Examples
/// 
/// * The file size is less than the minimum valid size 
/// `RegionParseError::FileSizeErr(file_length, minimum_length)`
/// * An encountered chunk has an end point beyond the end of the file 
/// `ChunkParseErr(chunk_number, file_length, chunk_end)`
/// 
/// `fmt::Debug` has been implemented for this so that error can be displayed directly to the console
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
    ChunkParseErr(usize, usize, usize) // (chunk_no, file_length, chunk_end) 
}
impl fmt::Debug for RegionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegionParseError::FileSizeErr(file_size, min_file_size) => 
                f.write_fmt(format_args!("The supplied file is smaller than the minimum size, minimum size: {}; size: {};",
                    file_size, min_file_size)),
            RegionParseError::ChunkParseErr(chunk_no, file_length, chunk_end) => 
                f.write_fmt(format_args!("The chunk end is beyond the end of the end of the file, chunk No: {}; file size: {}; chunk end: {};",
                    chunk_no, file_length, chunk_end)),
        }
    }
}

/// This function will take an anvil format file (`.mca`) in a slice of bytes (`u8`) and attempts to 
/// parse it into a vector of tuples, that contains the exctracted chunk (in order of how they appear 
/// in the files posistion table) and the compression scheme. Where item 0 is a vector of `u8` 
/// containing the uncompressed chunk and the second item is a u8 and denotes the compression scheme. 
/// 
/// A compression scheme of 1 is gzip, and 2 is zlib. A scheme of 0 or anything else indicates that the
/// chunk has not been generated yet and the chunk will be a blank vector. 
/// 
/// If an error is found in the structure of the file that can not be recovered, the function
/// will return an `Err(RegionParseErr)`, a enum denoting whether the error is with the file as a
/// whole or an individual chunk.
/// 
/// # Examples
/// ```
/// use std::fs;
/// use anvil_lib::raw::get_region_raw;
/// 
/// fn main() {
///     let file = fs::read("data/test.bin").expect("Failed to open file.");
///     let mut raw_region = match get_region_raw(&file) {
///         Ok(val) => val,
///         Err(error) => panic!("{:?}", error) // fmt debug is implemented for RegionParseError
///     };
/// }
/// ```
pub fn get_region_raw(file: &[u8]) -> Result<Vec<(Vec<u8>, u8)>, RegionParseError> {
    // Stores the data itself and the compression scheme for each chunk (data, compression_scheme) 
    let mut output: Vec<(Vec<u8>, u8)> = Vec::new(); 

    /*
        Region files contain 32 * 32 chunks (1024 chunks) 

        They begin with 2 fixed sized tables, each 4096 bytes each to a total of 8192 

        The first table holds 1024 4 byte table entries, with the 1st 3 bytes of each entry stating 
        the posistion of the beginning of the corresponding chunk in 4096 bytes intervals from 
        the start of the file, the remaining byte gives the size of the chunk in intervals of 4096 
        bytes, with each chunk is rounded up to the nearest multiple of 4096 

        The next table contains 4096 4 byte (32 bit) timestamps denoting the modified date of each 
        chunk 

        A chunk position and or size of 0 indicates the chunk has not been created yet 
    */

    // Make sure that the file is definitely above 8192 bytes (both the fixed sized tables) 
    if file.len() < 8192 { return Err(RegionParseError::FileSizeErr(file.len(), 8192)) } 

    // Iterate over the 1024 entries 
    for iter in 0..1024 {

        // Multiply the iteration No by 4 bytes the get the beginning of the 4 byte entry 
        let offset = iter * 4; 

        // Converts the 3 bytes, largest to smallest into a single number 
        let third_byte = (file[offset] as usize) << 16; 
        let second_byte = (file[offset + 1] as usize) << 8; 
        let first_byte = file[offset + 2] as usize; 
        // The entry posistion is indicated in intervals of 4096 bytes 
        let entry_pos = (third_byte + second_byte + first_byte) * 4096; 
        // The rough size is also indicated in 4096 byte intervals 
        let rough_size = (file[offset + 3] as usize) * 4096; 

        /*
            The first 5 bytes of the chunk is the header, the first 4 bytes is the exact 
            size in bytes after the first 4 bytes 

            The last one is the compression scheme; 1 = gzip, 2 = zlib 
        */

        if entry_pos != 0 {

            // If the file length is shorter than the start posistion + rough end posistion 
            // return an error and indicate which chunk errored 
            if file.len() < entry_pos + rough_size { 
                return Err(RegionParseError::ChunkParseErr(iter, file.len(), entry_pos + rough_size)) 
            } 

            // Converts the 4 bytes, largest to smallest into a single number 
            let fourth_byte = (file[entry_pos] as usize) << 24; 
            let third_byte = (file[entry_pos + 1] as usize) << 16; 
            let second_byte = (file[entry_pos + 2] as usize) << 8; 
            let first_byte = file[entry_pos + 3] as usize; 
            let size = fourth_byte + third_byte + second_byte + first_byte; 

            // Slice from the first 5 bytes and the first 5 bytes + the size 
            let raw_chunk = file[(entry_pos + 5)..(entry_pos + 5 + size)].to_vec();
            
            // Append the slice as a tuple into the output vec 
            output.push((raw_chunk, file[entry_pos + 4]));
        }
        else {
            // If the chunk doesn't exist, return an empty vec and a compression scheme of 0
            // that can be used as a marker for non existant chunks 
            output.push((Vec::new(), 0));
        }
    }
    return Ok(output);
}
