use anvil_lib::raw::get_region_raw;
use std::fs;


#[test]
fn pack_test_ok() {
    let file = fs::read("data/test.bin").expect("Can't open file.");
    match get_region_raw(&file) {
        Ok(val) => { 
            let s = &val[0].0; // 78 9c ed 
            for i in 0..10 {
                println!("{}", s[i]);
            }
        }, 
        Err(error) => panic!("{:?}", error) 
    }
}
