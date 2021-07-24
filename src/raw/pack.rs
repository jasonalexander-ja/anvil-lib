use super::{
    AnvilSchema,
    RawError,
    RawChunk,
    Compression
};

pub fn pack_chunks(chunks: &[RawChunk], schema: &AnvilSchema) -> Result<Vec<Vec<u8>>, RawError> {
    let mut output = Vec::new();
    for iter in 0..schema.chunks_per_region as usize {
        let (compression, chunk, _timestamp) = &chunks[iter];
        let new_chunk = format_chunk_data(&chunk, &compression, &schema);
        output.push(new_chunk);
    }
    Ok(output)
}

fn format_chunk_data(chunk: &[u8], compression: &Compression, schema: &AnvilSchema) -> Vec<u8> {
    if chunk.len() == 0 {
        return Vec::new();
    }
    let new_chunk_data = get_packed_chunk(chunk, &schema);
    let mut header = make_header(chunk.len(), compression, schema);
    header.extend(new_chunk_data);
    header
}

fn get_packed_chunk(chunk: &[u8], schema: &AnvilSchema) -> Vec<u8> {
    let final_size = next_multiple(chunk.len(), schema.size_multiplier);
    let amount_to_add = final_size - chunk.len();
    let mut packed_chunk = chunk.to_vec();
    packed_chunk.extend(vec![0; amount_to_add]);
    packed_chunk
}

fn make_header(size: usize, compression: &Compression, schema: &AnvilSchema) -> Vec<u8> {
    // Make the vec we'll splice the data into later 
    let mut output_vec = vec![0; schema.chunk_starts_from];
    // Get the positions for where to splice the data into 
    let (size_start, size_end) = schema.chunk_header_size_bytes;
    let (compr_start, compr_end) = schema.chunk_header_compr_bytes;
    // Get the data 
    let size_bytes = make_byte_arr(size, size_start - size_end);
    let compr_bytes = make_compr_bytes(compression);
    // Splice the data in and return 
    output_vec.splice(size_start..size_end, size_bytes);
    output_vec.splice(compr_start..compr_end, compr_bytes);
    output_vec
}



pub fn create_header_table(chunks: &[RawChunk], packed_chunks: &[Vec<u8>], schema: &AnvilSchema) -> Vec<u8> {
    let mut pos_from_start = 0;
    let mut new_pos_table: Vec<u8> = Vec::new();
    let mut new_date_table: Vec<u8> = Vec::new();
    for iter in 0..chunks.len() {
        let (_compression, _chunk, timestamp) = &chunks[iter];
        let packed_chunk = &packed_chunks[iter];
        pos_from_start += schema.min_anvil_file_size + packed_chunk.len();
        let mut new_record = vec![0; schema.posistion_table_record_len];
        let (start_byte_start, start_byte_end) = schema.pos_table_start_bytes;
        let (size_byte_start, size_byte_end) = schema.pos_table_size_bytes;
        let start_bytes = make_byte_arr(pos_from_start, start_byte_start - start_byte_end);
        let size_bytes = make_byte_arr(packed_chunk.len(), size_byte_start - size_byte_end);
        new_record.splice(start_byte_start..start_byte_end, start_bytes);
        new_record.splice(size_byte_start..size_byte_end, size_bytes);
        new_pos_table.extend(new_record);

        let new_date = make_byte_arr(*timestamp as usize, 4);
        new_date_table.extend(new_date);
    }
    new_pos_table.extend(new_date_table);
    new_pos_table
}



fn make_compr_bytes(compr: &Compression) -> Vec<u8> {
    match compr {
        Compression::GZip => vec![1],
        Compression::ZLib => vec![2],
        Compression::None => vec![0]
    }
}

fn make_byte_arr(num: usize, length: usize) -> Vec<u8> {
    let mut output = Vec::new();
    for iter in 0..length {
        let new_byte = (num >> (iter * 8)) & 255;
        output.push(new_byte as u8);
    }
    output.reverse();
    output
}

fn next_multiple(num: usize, base: usize) -> usize {
    let multiple = num as f64 / base as f64;
    return base * (multiple.ceil() as usize);
}
