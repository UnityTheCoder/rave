fn main() {
    let adr = Alloc::string("Hello").expect("Panic"); // Alloc String
    let converted = rave::StrOut::decode(rave::Read::string(webhook).as_str()).expect("This is not a string") // Read & convert to String  
    println!("{}", converted);
    Dealloc::string(adr); // Delete from memory
}
