#![cfg(target_arch = "wasm32")]

use shamir_rust::*;
use wasm_bindgen_test::*;

//Uncomment to run tests in browser
//wasm-pack test --chrome --headless
//wasm_bindgen_test_configure!(run_in_browser);
//Note: testing in browser fails with the following error:
//error: http://127.0.0.1:49410/session/fdeaa5a7b92f04d662568146055c3f51/url: status code 404
//To investigate later

//This test looks similar to the one in src/lib.rs
//But it is different because it is using the wasm_bindgen_test macro and verifies that the wasm functions work as expected
//For example it can detect if entrophy was not initialized properly 
#[wasm_bindgen_test]
pub fn test_get_shamir_secret() {
    let threshold = 2;
    let share_amount = 4;
    let secret = "mysecret".to_string();

    let shares = get_shamir_secret(threshold, share_amount, secret.clone());

    assert_eq!(shares.len(), share_amount);
    
    // Recover the secret from the shares
    let recovered = recover(threshold, shares.clone());
    assert_eq!(secret, recovered);
}
