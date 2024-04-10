use coren::*;
use std::path::PathBuf;

//just api testing

fn main() {
    println!("LINUX");
    match pack::get(&PathBuf::from("/home/kono/.minecraft/resourcepacks")) {
        Ok(..) => println!("worked"),
        Err(err) => println!("{:?}", err),
    }
    println!("WINDOWS");
    match pack::get(&PathBuf::from(
        "C:/Users/aryaa/AppData/Roaming/.minecraft/resourcepacks",
    )) {
        Ok(..) => println!("worked"),
        Err(err) => println!("{:?}", err),
    }
    let color = atlas::Atlas::new(atlas::Map::Color, 2048);
    match color.save("./packs","TEST") {
        Ok(..) => println!("Ok"),
        Err(err) => println!("{:?}", err)
    }

}
