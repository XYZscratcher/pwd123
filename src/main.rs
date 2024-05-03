use clap::Parser;
use blake3;
#[derive(Parser, Debug)]
#[command(author="XYZscratcher , xyzscer@outlook.com", version=env!("CARGO_PKG_VERSION"), about, long_about = None)]
struct Args {
    /// Your username
    #[arg(short, long)]
    name: String,

    /// website address
    #[arg(short, long)]
    website: String,
    
    
    #[arg(short,long,default_value="")]
    secret_key: String,

    /// Length of your password (<=64 , had better >10 to keep your password strong)
    #[arg(short,long,default_value="20")]
    length: u8,

    #[arg(short,long,default_value="-10")]
    mov: i8,
}
fn main() {
    let mut hasher = blake3::Hasher::new();
    let args = Args::parse();
    hasher.update(args.name.as_bytes());
    hasher.update(args.website.as_bytes());
    hasher.update(args.secret_key.as_bytes());
    let mut hash2 = hasher.finalize().to_hex();
    hash2.truncate(args.length.into());
    let mut password=String::from("");
    let mov:i8=args.mov;
    for i in hash2.bytes(){
        password.push((i as i8 + mov) as u8 as char);        
    }
    println!("{}",password);
}
