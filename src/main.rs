use shamir_secret_sharing::ShamirSecretSharing as SSS;
use num_bigint::{BigInt};
use hex;

fn main() {
let sss = SSS {
    threshold: 3,
    share_amount: 5,
    prime: BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",16).unwrap()
    };

let text = "Hello World".as_bytes();
let hex = hex::encode(text);

//let secret = BigInt::parse_bytes(b"ffffffffffffffffffffffffffffffffffffff", 16).unwrap();
let secret = BigInt::parse_bytes(hex.as_bytes(), 16).unwrap();

 let shares = sss.split(secret.clone());

 println!("shares: {:?}", shares);
 assert_eq!(secret, sss.recover(&shares[0..sss.threshold as usize]));

 let xxx = sss.recover(&shares[0..sss.threshold as usize]);
 let yyy = xxx.to_str_radix(16);
 
 
 let zzz = hex::decode(yyy.as_bytes()).unwrap();

 let string = String::from_utf8(zzz).unwrap();
  println!("{}", string);
 
}