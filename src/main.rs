#[allow(dead_code)]

use std::fs;

fn format_address(address: usize) -> String
{
    let mut address = format!("{:X}", address);
    if address.len() < 8
    {
        for _i in 0..(8 - address.len())
        {
            address = format!("{}{}", 0, address);
        }
    }
    return address;
}

fn uint_to_mem(byte: u8) -> (String,char)
{
    let mut c = byte as char;
    let mut hex = format!("{:X}", byte);
    
    if c == '\n' {c = 'n';}
    if hex.len() == 1 {hex = format!("{}{}", 0, hex);}
    return (hex,c);
}

fn main()
{
    let path: &str = "mcs/test.txt";
    let file: Vec<u8> = fs::read(path).unwrap();
    
    println!("ADDRESS   00 01 02 03 04 05 06 07   08 09 0A 0B 0C 0D 0E 0F");
    for i in 0..file.len()
    {
        if i % 8 == 0 && i != 0 { print!("  "); }
        if i % 16 == 0
        {
            if i != 0 { print!("\n"); }
            print!("{}: ",format_address(i));
        }
        print!("{} ",uint_to_mem(file[i]).0);
    }
}