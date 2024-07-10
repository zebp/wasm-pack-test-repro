use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn whatever() {}

#[cfg(test)]
mod tests {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn a_very_unique_name_to_grep() {
        // This function is never called, but it's name is unique enough to grep
    }
}
