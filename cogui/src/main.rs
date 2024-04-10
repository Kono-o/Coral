use coren::*;
use std::path::PathBuf;

fn main() {
    println!("LINUX");
    match packs::get(&PathBuf::from("/home/kono/.minecraft/resourcepacks")) {
        Ok(..) => println!("worked"),
        Err(err) => println!("{:?}", err),
    }
    println!("WINDOWS");
    match packs::get(&PathBuf::from(
        "C:/Users/aryaa/AppData/Roaming/.minecraft/resourcepacks",
    )) {
        Ok(..) => println!("worked"),
        Err(err) => println!("{:?}", err),
    }
    let color = atlas::Atlas::new(atlas::Map::Color, 2048);
    color.save("./pack","TEST");

}
