extern crate nbt;
extern crate serde_json;

use nbt::Blob;
use std::fs::File;
use std::path::Path;

use nbt::Result;

fn run() -> Result<()>
{
    let path: &Path = Path::new(".mcs/level.dat");
    let mut file = File::open(path).unwrap();
    
    println!("file > {:?}", file);
    
    println!("NBT Contents");
    let blob = Blob::from_gzip_reader(&mut file).unwrap();
    println!("{:?}", blob);
    
    return Ok(());
}

fn main()
{
    print!("{:?}", run());
}