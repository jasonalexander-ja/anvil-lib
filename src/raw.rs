/*!
A module for handling unpacking and packing raw anvil format file `.mca` into the raw uncompressed chunks. 
*/
mod unpack;
mod pack;
mod schema;

use schema::*;
pub use unpack::*;

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
pub fn get_region_raw(file: &[u8]) -> Result<Vec<(Vec<u8>, usize)>, RegionParseError> {
    let schema = AnvilSchema::default();
    unpack::get_region_raw(file, schema)
}
