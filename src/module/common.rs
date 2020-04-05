pub fn i64_to_u8(value: i64) -> u8 {
    let cnv_value: u8;
    if value > 255i64 {
        cnv_value = 255u8;
    }
    else if value < 0 {
        cnv_value = 0u8;
    }
    else {
        cnv_value = value as u8;
    }
    cnv_value
}