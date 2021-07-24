use super::{ 
    AnvilSchema, 
    RawError,
    RawChunk,
    Compression
};

pub fn get_posistion_table(file: &[u8], schema: &AnvilSchema) -> Vec<(usize, usize)>
{
    let mut output_vec: Vec<(usize, usize)> = Vec::new();
    for iter in 0..schema.chunks_per_region {
        let record = get_pos_record(file, iter, schema);
        output_vec.push(record);
    }
    output_vec
}

fn get_pos_record(file: &[u8], rec_no: usize, schema: &AnvilSchema) -> (usize, usize) {
    let offset = (rec_no * schema.posistion_table_record_len) as usize;

    let (pos_data_start, pos_data_end) = schema.pos_table_start_bytes;
    let (size_data_start, size_data_end) = schema.pos_table_size_bytes;

    let pos_data_bytes = &file[pos_data_start + offset..pos_data_end + offset];
    let size_data_bytes = &file[size_data_start + offset..size_data_end + offset];
    let pos_index = make_usize_from_bytes(pos_data_bytes) * schema.pos_multiplier;
    let size_index = make_usize_from_bytes(size_data_bytes) * schema.size_multiplier;
    (pos_index, size_index)
}

pub fn get_timestamp_table(file: &[u8], schema: &AnvilSchema) -> Vec<u32> {
    let mut output = Vec::new();
    let timestamp_table_start = schema.chunks_per_region * schema.posistion_table_record_len;
    for iter in 0..schema.chunks_per_region {
        let offset = (timestamp_table_start + iter * 4) as usize;
        let timestamp_bytes = &file[offset..offset + 4];
        let timestamp = make_u32_from_bytes(timestamp_bytes);
        output.push(timestamp);
    }
    output
} 

pub fn get_chunks(file: &[u8], headers: &[(usize, usize)], timestamp_table: &[u32], schema: &AnvilSchema) -> Result<Vec<RawChunk>, RawError> {
    let mut chunks: Vec<RawChunk> = Vec::new();
    for (iter, (pos, size)) in headers.iter().enumerate() {
        let timestamp = timestamp_table[iter];
        if *pos != 0 {
            let end_pos = pos + size;
            let chunk = get_chunk(&file, *pos, end_pos, iter, timestamp, &schema)?;
            chunks.push(chunk);
        } else {
            chunks.push((Compression::None, Vec::new(), timestamp));
        }
    }
    Ok(chunks)
}

fn get_chunk(file: &[u8], start: usize, end: usize, chunk_index: usize, timestamp: u32, schema: &AnvilSchema) -> Result<RawChunk, RawError> {
    if file.len() < end {
        return Err(RawError::throw_chunk_pos_err(chunk_index, file.len(), end))
    }
    let chunk = &file[start..end];
    let (size, compression) = parse_chunk_header(&chunk, &schema);
    let output_vec = if size > end {
        return Err(RawError::throw_chunk_header_err(chunk_index, end, size)); 
    } else {
        chunk[schema.chunk_starts_from..size].to_vec()
    };
    Ok((compression, output_vec, timestamp))
}

// Parses the first few bytes of a chunk
fn parse_chunk_header(chunk: &[u8], schema: &AnvilSchema) -> (usize, Compression) 
{
    let (compression_byte_start, compression_byte_end) = schema.chunk_header_compr_bytes;
    let (size_bytes_start, size_bytes_end) = schema.chunk_header_size_bytes;

    let compression_bytes = &chunk[compression_byte_start..compression_byte_end];
    let size_bytes = &chunk[size_bytes_start..size_bytes_end];

    let size = make_usize_from_bytes(&size_bytes) + schema.chunk_starts_from;
    let compression = match make_usize_from_bytes(compression_bytes) {
        1 => Compression::GZip,
        2 => Compression::ZLib,
        _ => Compression::None
    };
    (size, compression)
}

// Helper funtion to turn arrays of bytes read from files into full numbers 
fn make_usize_from_bytes(bytes: &[u8]) -> usize {
    let mut output: usize = 0; 
    for (iter, val) in bytes.into_iter().rev().enumerate() {
        output += (*val as usize) << (iter * 8); 
    }
    output
}
// Helper funtion to turn arrays of bytes read from files into full numbers 
fn make_u32_from_bytes(bytes: &[u8]) -> u32 {
    let mut output: u32 = 0; 
    for (iter, val) in bytes.into_iter().rev().enumerate() {
        output += (*val as u32) << (iter * 8); 
    }
    output
}
