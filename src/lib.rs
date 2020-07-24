mod utils;
use tinysegmenter;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn tokenize(text: &str) -> String {
    let tokens = run_tokenizer(text);
    return tokens.join(" / ");
}


fn run_tokenizer(text: &str) -> Vec<String> {
    return tinysegmenter::tokenize(text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input1 = "私は蛙";
        let expect1 = "私 / は / 蛙";
        let actual1 = tokenize(input1);
        assert_eq!(expect1, actual1);
    }
}