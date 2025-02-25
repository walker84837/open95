const DIGIT_RANGE: u32 = 10;
const OEM_DAY_RANGE: u32 = 366;

pub struct KeyGenerator;

impl KeyGenerator {
    pub fn generate_oem_key() -> String {
        let oem_key_day = Self::generate_oem_day();
        let oem_key_year = Self::generate_oem_year();
        let oem_key_part_1 = Self::generate_5digits_oem();
        let oem_key_part_2 = Self::generate_5digits_random_oem();

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
        let retail_key_part_1 = Self::generate_3digits_retail();
        let retail_key_part_2 = Self::generate_7digits_retail();

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

    fn generate_7digits_retail() -> [u32; 7] {
        let mut retail_key_part_2 = [0; 7];
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

    fn generate_3digits_retail() -> [u32; 3] {
        let mut retail_key_part_1 = [0; 3];
        loop {
            for digit in retail_key_part_1.iter_mut() {
                *digit = fastrand::u32(0..DIGIT_RANGE);
            }
            if ![
                3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 9,
            ]
            .contains(&retail_key_part_1[0])
            {
                break;
            }
        }
        retail_key_part_1
    }

    fn generate_5digits_random_oem() -> [u32; 5] {
        let mut oem_key_part_2 = [0; 5];
        for digit in oem_key_part_2.iter_mut() {
            *digit = fastrand::u32(0..DIGIT_RANGE);
        }
        oem_key_part_2
    }

    fn generate_5digits_oem() -> [u32; 5] {
        let mut oem_key_part_1 = [0; 5];
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

    fn generate_oem_day() -> u32 {
        loop {
            let oem_key_day = fastrand::u32(1..=OEM_DAY_RANGE);
            if oem_key_day != 0 {
                return oem_key_day;
            }
        }
    }

    fn generate_oem_year() -> [u32; 2] {
        let mut oem_key_year = [0; 2];
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retail_key_format() {
        let key = KeyGenerator::generate_retail_key();
        assert_eq!(key.len(), 11, "Retail key has wrong length");
        let parts: Vec<&str> = key.split('-').collect();
        assert_eq!(parts.len(), 2, "Retail key should have two parts");
        assert_eq!(parts[0].len(), 3, "First part must be 3 digits");
        assert_eq!(parts[1].len(), 7, "Second part must be 7 digits");

        let first_digit = parts[0].chars().next().unwrap().to_digit(10).unwrap();
        assert!(
            first_digit <= 2,
            "First digit of retail key must be 0,1,2. Got {}",
            first_digit
        );

        let sum: u32 = parts[1].chars().map(|c| c.to_digit(10).unwrap()).sum();
        assert_eq!(sum % 7, 0, "Sum of digits is not divisible by 7");
    }

    #[test]
    fn test_oem_key_format() {
        let key = KeyGenerator::generate_oem_key();
        assert_eq!(key.len(), 23, "OEM key has wrong length");
        let parts: Vec<&str> = key.split('-').collect();
        assert_eq!(parts.len(), 4, "OEM key should have four parts");
        assert_eq!(
            parts[0].len(),
            5,
            "First part must be 5 digits (day + year)"
        );
        assert_eq!(parts[1], "OEM", "Second part must be 'OEM'");
        assert!(
            parts[2].starts_with("00"),
            "Third part must start with '00'"
        );
        assert_eq!(parts[3].len(), 5, "Fourth part must be 5 digits");

        let day_str = &parts[0][0..3];
        let day = day_str.parse::<u32>().expect("Failed to parse day");
        assert!(
            (1..=366).contains(&day),
            "Day {} is out of range (1-366)",
            day
        );

        let year_str = &parts[0][3..5];
        let year = year_str.parse::<u32>().expect("Failed to parse year");
        let valid_year = (95..=99).contains(&year) || (0..=2).contains(&year);
        assert!(
            valid_year,
            "Year {} is invalid (must be 95-99 or 00-02)",
            year
        );

        let part1_str = &parts[2][2..];
        let part1_sum: u32 = part1_str.chars().map(|c| c.to_digit(10).unwrap()).sum();
        assert_eq!(
            part1_sum % 7,
            0,
            "Part1 sum {} is not divisible by 7",
            part1_sum
        );
    }

    #[test]
    fn test_multiple_retail_keys() {
        for _ in 0..1000 {
            let key = KeyGenerator::generate_retail_key();
            validate_retail_key(&key);
        }
    }

    #[test]
    fn test_multiple_oem_keys() {
        for _ in 0..1000 {
            let key = KeyGenerator::generate_oem_key();
            validate_oem_key(&key);
        }
    }

    fn validate_retail_key(key: &str) {
        assert_eq!(key.len(), 11);
        let parts: Vec<&str> = key.split('-').collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0].len(), 3);
        assert_eq!(parts[1].len(), 7);

        let first_digit = parts[0].chars().next().unwrap().to_digit(10).unwrap();
        assert!(first_digit <= 2);

        let sum: u32 = parts[1].chars().map(|c| c.to_digit(10).unwrap()).sum();
        assert_eq!(sum % 7, 0);
    }

    fn validate_oem_key(key: &str) {
        assert_eq!(key.len(), 23);
        let parts: Vec<&str> = key.split('-').collect();
        assert_eq!(parts.len(), 4);
        assert_eq!(parts[0].len(), 5);
        assert_eq!(parts[1], "OEM");
        assert!(parts[2].starts_with("00"));
        assert_eq!(parts[3].len(), 5);

        let day_str = &parts[0][0..3];
        let day = day_str.parse::<u32>().unwrap();
        assert!((1..=366).contains(&day));

        let year_str = &parts[0][3..5];
        let year = year_str.parse::<u32>().unwrap();
        assert!((95..=99).contains(&year) || (0..=2).contains(&year));

        let part1_str = &parts[2][2..];
        let part1_sum: u32 = part1_str.chars().map(|c| c.to_digit(10).unwrap()).sum();
        assert_eq!(part1_sum % 7, 0);
    }
}
