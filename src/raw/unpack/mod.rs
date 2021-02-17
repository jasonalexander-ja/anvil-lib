mod unpack_errors;
pub use unpack_errors::*;

/// This function will take a slice of bytes (`u8`), check to ensure that it is in the anvil file format,
/// and extracts each uncompressed chunk as a vector of bytes, with the corresponsing compression scheme. 
/// 
/// It does not check for valid chunk compression, it will only check that both the fixed size 4096 byte
/// tables are present and makes sure that there is data in the places specified by the posistion table
/// and chunk header, else it will return a `Err(RegionParseErr)` detailing the error. 
/// 
/// If a valid anvil format is detected and parsed correctly, a type `Ok(Vec<(Vec<u8>, u8)>)` will be returned, 
/// this is a vector of tuples containing the uncompressed chunk as a vector of bytes (item 0) 
/// and the compression scheme for the chunk as a byte (item 1), the data for each chunk appears in 
/// the vector in the same order as what it appears in the posistion table in the file. 
/// 
/// The compresseion scheme will either be `1` for gzip, or `2` for zlib, if the chunk has not been created 
/// then the compression scheme will be 0 and the vector will be empty, anything else must be treated as 
/// corrupt data. 
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
/// 
/// # Errors
///  
/// * If the 2 fixed size 4096 byte tables are not present (if the given slice is less than 2 * 4096 = 8192 bytes); Returns: 
///     
///     `Err(RegionParseError::FileSizeErr(file_length, minimum_length))`
/// 
/// * If the maximum posistion for a chunk indicated in the posistion table (the first 4096 byte table) is 
/// larger than the length of the slice; Returns: 
/// 
///     `Err(ChunkParseErr(chunk_number, file_length, chunk_end))`
/// 
/// * If the chunk length indicated by the chunk header is larger than the maximum langth indicated by the 
/// posistion table; Returns: 
/// 
///     `Err(RegionParseError::ChunkHeaderErr(chunk_number, max_length, indicated_length))`
/// 
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
    for iter in 0..super::CHUNKS_PER_REGION {

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
                return Err(RegionParseError::ChunkPosErr(iter, file.len(), entry_pos + rough_size)) 
            } 

            // Converts the 4 bytes, largest to smallest into a single number 
            let fourth_byte = (file[entry_pos] as usize) << 24; 
            let third_byte = (file[entry_pos + 1] as usize) << 16; 
            let second_byte = (file[entry_pos + 2] as usize) << 8; 
            let first_byte = file[entry_pos + 3] as usize; 
            let size = fourth_byte + third_byte + second_byte + first_byte; 

            // If the exact chunk size is greater than the rough size return an error
            // error states (chunk index in posistion table, rough size, size indicated by the header)
            if 4 + size > rough_size { 
                return Err(RegionParseError::ChunkHeaderErr(iter, rough_size, 4 + size)); 
            }

            // We dont need to check if the exact size size if beyond the end of the slice as we now know that: 
            // - The exact size is smaller than or equal to the rough size
            // - The rough size is within the length if the slice 

            // Slice from the first 5 bytes and the number of bytes indicated past that first 5 
            // We add 4 to the end slice as the size includes the 5th byte of the header (compression scheme)
            let raw_chunk = file[(entry_pos + 5)..=(entry_pos + 4 + size)].to_vec();
            
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
