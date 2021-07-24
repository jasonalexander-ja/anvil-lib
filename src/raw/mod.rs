mod raw_schema;
mod raw_error;
mod unpack;
mod pack;

pub use raw_schema::*;
pub use raw_error::*;

use std::result::Result;

/// Used to denote the compression in each chunk, will be `Compression::None` if the chunk is blank 
pub enum Compression {
    GZip,
    ZLib,
    None,
}

/// The compression format and along with the compressed chunk data. These make up the region. 
pub type RawChunk = (Compression, Vec<u8>, u32); 

/// A wrapper for a group of chunks in a region, these are usually grouped as such in region files. 
pub struct RawRegion {
    pub chunks: Vec<RawChunk>, 
}

impl RawRegion {
    pub fn from_file(file: &Vec<u8>, schema: &AnvilSchema) -> Result<RawRegion, RawError> {
        let min_size = schema.min_anvil_file_size;
        if file.len() < min_size {
            return Err(RawError::throw_file_size_err(min_size, file.len()));
        }
        let header_table = unpack::get_posistion_table(&file, &schema);
        let timestamp_table = unpack::get_timestamp_table(&file, &schema);
        let chunks = unpack::get_chunks(&file, &header_table, &timestamp_table, &schema)?;
        Ok(RawRegion { chunks })
    }
    pub fn to_file(&self, schema: &AnvilSchema) -> Result<Vec<u8>, RawError> {
        let chunks = &self.chunks;
        if chunks.len() != schema.chunks_per_region as usize {
            return Err(RawError::throw_no_chunks_err(chunks.len(), schema.chunks_per_region));
        }
        let packed_chunks = pack::pack_chunks(chunks, &schema)?;
        let header_table = pack::create_header_table(chunks, &packed_chunks, schema);
        Ok(Vec::new())
    }
}
