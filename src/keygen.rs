use fastrand;

const DIGIT_RANGE: u32 = 10;
const OEM_DAY_RANGE: u32 = 366;

pub fn generate_oem_key() -> String {
    let oem_key_day = generate_oem_day();
    let oem_key_year = generate_oem_year();
    let oem_key_part_1 = generate_5digits_oem();
    let oem_key_part_2 = generate_5digits_random_oem();

    format!(
        "{:03}{}{}-OEM-00{}{}{}{}{}-{}{}{}{}{}",
        oem_key_day,
        oem_key_year[0],
        oem_key_year[1],
        oem_key_part_1[0],
        oem_key_part_1[1],
        oem_key_part_1[2],
        oem_key_part_1[3],
        oem_key_part_1[4],
        oem_key_part_2[0],
        oem_key_part_2[1],
        oem_key_part_2[2],
        oem_key_part_2[3],
        oem_key_part_2[4]
    )
}

pub fn generate_retail_key() -> String {
    let retail_key_part_1 = generate_3digits_retail();
    let retail_key_part_2 = generate_7digits_retail();

    format!(
        "{}{}{}-{}{}{}{}{}{}{}",
        retail_key_part_1[0],
        retail_key_part_1[1],
        retail_key_part_1[2],
        retail_key_part_2[0],
        retail_key_part_2[1],
        retail_key_part_2[2],
        retail_key_part_2[3],
        retail_key_part_2[4],
        retail_key_part_2[5],
        retail_key_part_2[6]
    )
}

pub fn generate_7digits_retail() -> [u32; 7] {
    let mut retail_key_part_2 = [0, 0, 0, 0, 0, 0, 0];
    loop {
        for digit in retail_key_part_2.iter_mut() {
            *digit = fastrand::u32(0..DIGIT_RANGE);
        }
        let sum7: u32 = retail_key_part_2.iter().sum();
        if sum7 % 7 == 0 {
            break;
        }
    }
    retail_key_part_2
}

pub fn generate_3digits_retail() -> [u32; 3] {
    let mut retail_key_part_1 = [0, 0, 0];
    loop {
        for digit in retail_key_part_1.iter_mut() {
            *digit = fastrand::u32(0..DIGIT_RANGE);
        }
        if ![3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 9]
            .contains(&retail_key_part_1[0])
        {
            break;
        }
    }
    retail_key_part_1
}

pub fn generate_5digits_random_oem() -> [u32; 5] {
    let mut oem_key_part_2 = [0, 0, 0, 0, 0];
    for digit in oem_key_part_2.iter_mut() {
        *digit = fastrand::u32(0..DIGIT_RANGE);
    }
    oem_key_part_2
}

pub fn generate_5digits_oem() -> [u32; 5] {
    let mut oem_key_part_1 = [0, 0, 0, 0, 0];
    loop {
        for digit in oem_key_part_1.iter_mut() {
            *digit = fastrand::u32(0..DIGIT_RANGE);
        }
        let sum7: u32 = oem_key_part_1.iter().sum();
        if sum7 % 7 == 0 {
            break;
        }
    }
    oem_key_part_1
}

pub fn generate_oem_day() -> u32 {
    loop {
        let oem_key_day = fastrand::u32(1..=OEM_DAY_RANGE);
        if oem_key_day != 0 {
            return oem_key_day;
        }
    }
}

pub fn generate_oem_year() -> [u32; 2] {
    let mut oem_key_year = [0, 0];
    loop {
        for digit in oem_key_year.iter_mut() {
            *digit = fastrand::u32(0..DIGIT_RANGE);
        }
        if (oem_key_year[0] == 9 && oem_key_year[1] >= 5)
            || (oem_key_year[0] == 0 && oem_key_year[1] < 3)
        {
            break;
        }
    }
    oem_key_year
}

