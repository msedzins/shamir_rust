use num_bigint::BigInt;
use shamir_secret_sharing::ShamirSecretSharing as SSS;
use hex;
use wasm_bindgen::prelude::*;

const PRIME: &[u8;64] = b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f";

// Called when the wasm module is instantiated
// Can't be named "main:
//https://github.com/rustwasm/wasm-bindgen/issues/2206
#[wasm_bindgen(start)]
fn main_js() -> Result<(), JsValue> {
    set_panic_hook();

    Ok(())
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn get_shamir_secret(threshold: usize, share_amount: usize, secret: String) -> Vec<String> {
    let sss = SSS {
        threshold: threshold,
        share_amount: share_amount,
        prime: BigInt::parse_bytes(
            PRIME,
            16,
        )
        .unwrap(),
    };

    let hex = hex::encode(secret.as_bytes());
    let secret = BigInt::parse_bytes(hex.as_bytes(), 16).unwrap();
    let shares = sss.split(secret.clone());

    shares.iter().map(|x| x.1.to_string()).collect()
    // alert(&format!("Hello, {}!", value));
}

//shares is a vector of strings, each string is a share
//current limitation: the shares must be in order, starting from the first one!
//this is because we now Y, but we don't know corresponding X
//the solution is to take touples of (X, Y) to recover the secret
#[wasm_bindgen]
pub fn recover(threshold: usize, shares: Vec<String>) -> String {
    let sss = SSS {
        threshold: threshold, //this must be known so that we know the degree of the polynomial
        share_amount: 0, //this is not used in the recovery process
        prime: BigInt::parse_bytes( //this is the prime number used in the polynomial
            PRIME,
            16,
        )
        .unwrap(),
    };

    //We need to check this, otherwise sss.recover will panic
    if threshold > shares.len() {
        return "Threshold is greater than the number of shares".to_string();
    }

    let shares: Vec<BigInt> = shares.iter().map(|x| BigInt::parse_bytes(x.as_bytes(), 10).unwrap()).collect();
    let numbers: Vec<usize> = (1..=shares.len()).collect();

    let value: Vec<(usize, BigInt)> = numbers.iter().cloned().zip(shares.iter().cloned()).collect();

    let recovered = sss.recover(&value[0..threshold]);
    
    // Convert the recovered BigInt to bytes
    let bytes = recovered.to_bytes_be().1;

    // Convert the bytes to a UTF-8 string
    String::from_utf8(bytes).unwrap_or_else(|_| "Invalid UTF-8".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_shamir_secret() {
        let threshold = 2;
        let share_amount = 4;
        let secret = "mysecret".to_string();

        let shares = get_shamir_secret(threshold, share_amount, secret.clone());

        assert_eq!(shares.len(), share_amount);
        
        // Recover the secret from the shares
        let recovered = recover(threshold, shares.clone());
        assert_eq!(secret, recovered);

        // Recover the secret from subset of shares, should still work
        let recovered = recover(threshold, shares.iter().cloned().take(2).collect());
        assert_eq!(secret, recovered);

        // Recover the secret from subset of shares, should still work
        let recovered = recover(threshold, shares.iter().cloned().take(3).collect());
        assert_eq!(secret, recovered);

        // Recover the secret from subset of shares, won't work
        let recovered = recover(threshold, shares.iter().cloned().take(1).collect());
        assert_ne!(secret, recovered);
    }
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
