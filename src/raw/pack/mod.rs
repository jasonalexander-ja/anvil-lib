mod pack_errors;
pub use pack_errors::*;


pub fn pack_region(region: &[(Vec<u8>, u8)]) -> Result<Vec<u8>, PackErrors> {
    let mut output: Vec<u8> = Vec::new();
    println!("Start.");
    if region.len() > 1024 {
        return Err(PackErrors::TooLittleChunks(1024, region.len()))
    }

    for iter in 0..1024 {
        let mut chunk: Vec<u8> = region[iter].0.to_vec();
        let compresseion = region[iter].1;
        let chunk_len = chunk.len();
        let chunk_rounded_len = (chunk_len as f64 / 4096_00f64).ceil() as usize * 4096;
        let length_to_pack = chunk_rounded_len - chunk.len();
        chunk.append(&mut vec![0; length_to_pack]);
        println!("{}", chunk.len());
    }
    println!("End.");
    return Ok(output);
}
