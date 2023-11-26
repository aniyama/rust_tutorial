pub struct InputValidator {}

impl InputValidator {
    pub fn validate_input_type(input_type: u8) {
        match input_type {
            0 | 1 => {}
            _ => panic!("不正な入力値"),
        }
    }
    pub fn validate_register_type(register_type: u8) {
        match register_type {
            0 | 1 => {}
            _ => panic!("不正な入力値"),
        }
    }
    pub fn validate_category_type(register_type: u8, category_type: u8) {
        if register_type == 0 {
            match category_type {
                0 | 1 | 2 => {}
                _ => panic!("不正な入力値"),
            }
        } else {
            match category_type {
                0 | 1 | 2 => {}
                _ => panic!("不正な入力値"),
            }
        }
    }
}
