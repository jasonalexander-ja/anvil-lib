/// This is a helper type to store all the settings for how the file is formatted. 
/// It's fields are used when parsing an anvil file format region. This will have implemented a 
/// default constructor. Due to the number of fields, if custom parsing is desired,
/// it is recommended to explicitly define the value for each field or use the default constructor
/// and modify the desired values.
/// 
pub struct AnvilSchema {
    /// The number of chunks in a region file (used when paring the records in tables)
    pub chunks_per_region: usize,
    /// The length in bytes of each record in the posision table
    pub posistion_table_record_len: usize,
    /// The minimum size that the file needs to be to have the pos. table parsed
    pub min_anvil_file_size: usize,
    /// The amount that the posistion in the pos. table is multiplied by to get the start
    pub pos_multiplier: usize,
    /// The amount the size in the pos. table is multiplied by to get the rough size
    pub size_multiplier: usize,
    /// Which bytes in the pos. table record is the posistion of the chunk
    pub pos_table_start_bytes: (usize, usize),
    /// Which bytes in the pos. table us the rough size of the chunk
    pub pos_table_size_bytes: (usize, usize),
    /// Number of bytes from the start of the chunk where the body starts
    pub chunk_starts_from: usize,
    /// Which bytes in the chunk header is the absolute size
    pub chunk_header_size_bytes: (usize, usize),
    /// Which bytes in the heaer is the compression scheme
    pub chunk_header_compr_bytes: (usize, usize),
}

impl AnvilSchema {
    /// This will get the default schema for the anvil file format. 
    /// Correct as of last build. 
    /// 
    /// # Example
    /// ```
    /// use anvil_lib::raw;
    /// use std::fs;
    /// 
    /// fn main() {
    ///     let file = fs::read("./TestData/chunk.mca").unwrap();
    ///     
    ///     let schema = raw::AnvilSchema::default();
    ///     raw::RawRegion::from_file(&file, &schema);
    /// }
    /// ```
    pub fn default() -> AnvilSchema {
        AnvilSchema {
            chunks_per_region: 1024,
            posistion_table_record_len: 4,
            min_anvil_file_size: 8192,
            pos_multiplier: 4096,
            size_multiplier: 4096,
            pos_table_start_bytes: (0, 3),
            pos_table_size_bytes: (3, 4),
            chunk_starts_from: 5,
            chunk_header_size_bytes: (0, 4),
            chunk_header_compr_bytes: (4, 5), 
        }
    }
}
