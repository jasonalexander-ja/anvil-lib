/// An error type used when unpacking/packing an anvil format file, allows for custom error handling 
#[derive(Debug)]
pub enum RawError {
    Pack,
    PackNoChunksErr(RawErrData),

    UnpackFileSizeErr(RawErrData),
    UnpackChunkPosErr(RawErrData),
    UnpackChunkHeaderErr(RawErrData),
}

impl RawError {
    // Unpacking errors 

    pub fn throw_file_size_err(min_size: usize, size: usize) -> RawError {
        let info = RawErrData {
            chunk_index: 0,
            specified_val: size,
            min_val: min_size,
            max_val: 0,
        };
        RawError::UnpackFileSizeErr(info)
    }
    pub fn throw_chunk_pos_err(index: usize, size: usize, chunk_end: usize) -> RawError {
        let info = RawErrData {
            chunk_index: index,
            specified_val: chunk_end,
            min_val: 0,
            max_val: size,
        };
        RawError::UnpackChunkPosErr(info)
    }
    pub fn throw_chunk_header_err(index: usize, max_size: usize, chunk_size: usize) -> RawError {
        let info = RawErrData {
            chunk_index: index, 
            specified_val: chunk_size, 
            min_val: 0,
            max_val: max_size,
        };
        RawError::UnpackChunkHeaderErr(info)
    }

    // Packing errors 

    pub fn throw_no_chunks_err(no_chunks: usize, req_number: usize) -> RawError {
        let info = RawErrData {
            chunk_index: 0,
            specified_val: no_chunks,
            min_val: req_number as usize,
            max_val: req_number as usize,
        };
        RawError::PackNoChunksErr(info)
    }
}

/// A helper structure to store information on potential errors when processing 
/// a raw anvil file 
#[derive(Debug)]
pub struct RawErrData {
    chunk_index: usize,
    specified_val: usize,
    min_val: usize,
    max_val: usize,
}
