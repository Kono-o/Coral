use std::path::PathBuf;
use coren::*;

fn main() {
  println!("LINUX");
  match packs::get_packs(&PathBuf::from("/home/kono/.minecraft/resourcepacks")) {
    Ok(..) => println!("worked"),
    Err(err) => println!("{:?}", err)
  }
  println!("WINDOWS");
  match packs::get_packs(&PathBuf::from("C:/Users/aryaa/AppData/Roaming/.minecraft/resourcepacks")) {
    Ok(..) => println!("worked"),
    Err(err) => println!("{:?}", err)
  }
}
