use std::path::PathBuf;
use coren::*;

fn main() {
  match packs::get_packs(&PathBuf::from("/home/kono/.minecraft/resourcepack")) {
    Ok(..) => (),
    Err(err) => println!("{:?}", err)
  }
  match packs::get_packs(&PathBuf::from("/home/kono/.minecraft/resourcepacks")) {
    Ok(..) => (),
    Err(err) => println!("{:?}", err)
  }
  match packs::get_packs(&PathBuf::from("/home/kono/.minecraft/resourcepaack")) {
    Ok(..) => (),
    Err(err) => println!("{:?}", err)
  }
}
