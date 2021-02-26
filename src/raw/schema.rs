
/*
    This is a struct to store info on the anvil file format to allow for easy parsing. 

    The fields are as follows: 
*/
pub struct AnvilSchema {
    pub chunks_per_region: usize,           // The number of chunks in a region file (used when paring the records in tables)
    pub posistion_table_record_len: usize,  // The length in bytes of each record in the posision table 
    pub min_anvil_file_size: usize,         // The minimum size that the file needs to be to have the pos. table parsed
    pub pos_multiplier: usize,              // The amount that the posistion in the pos. table is multiplied by to get the start
    pub size_multiplier: usize,             // The amount the size in the pos. table is multiplied by to get the rough size
    pub pos_table_start_bytes: Range,       // Which bytes in the pos. table record is the posistion of the chunk
    pub pos_table_size_bytes: Range,        // Which bytes in the pos. table us the rough size of the chunk 
    pub chunk_starts_from: usize,           // Number of bytes from the start of the chunk where the body starts 
    pub chunk_header_size_bytes: Range,     // Which bytes in the chunk header is the absolute size
    pub chunk_header_compr_bytes: Range     // Which bytes in the heaer is the compression scheme 
}

impl AnvilSchema {
    /*  This is the format that currently works as of 23/02/2021 
        Should it change, then the default values used by this 
        method should change */
    pub fn default() -> AnvilSchema {
        AnvilSchema {
            chunks_per_region: 1024,
            posistion_table_record_len: 4,
            min_anvil_file_size: 8192,
            pos_multiplier: 4096,
            size_multiplier: 4096,
            pos_table_start_bytes: Range{ start: 0, end: 3 },
            pos_table_size_bytes: Range{ start: 3, end: 4 },
            chunk_starts_from: 5,
            chunk_header_size_bytes: Range{ start: 0, end: 4 },
            chunk_header_compr_bytes: Range{ start: 4, end: 5 }, 
        }
    }
}
// This is a helper struct to store the ranges used to slice into 
// a vector of bytes read from a file 
pub struct Range {
    pub start: usize, 
    pub end: usize
}
