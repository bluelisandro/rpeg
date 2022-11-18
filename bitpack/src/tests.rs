#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod tests {
    use crate::{bitpack::{self, newu, news, gets}};

    #[test]
    fn fitsu_test() {
        assert_eq!(bitpack::fitsu(107, 5), false);
        assert_eq!(bitpack::fitsu(15, 5), true);
        assert_eq!(bitpack::fitsu(39, 9), true);
    }

    #[test]
    fn fitss_test() {
        assert_eq!(bitpack::fitss(40, 5), false);
        assert_eq!(bitpack::fitss(-5, 5), true);
        assert_eq!(bitpack::fitss(-83, 9), true);
    }

    #[test]
    fn newu_test() {
        let mut word: Option<u64> = Some(0);

        let a: u64 = 1; // lsb = 0
        let b: u64 = 2; // lsb = 16
        let c: u64 = 3; // lsb = 32
        let d: u64 = 4; // lsb = 48

        word = bitpack::newu(word.unwrap(), 16, 48, d);
        word = bitpack::newu(word.unwrap(), 16, 32, c);
        word = bitpack::newu(word.unwrap(), 16, 16, b);
        word = bitpack::newu(word.unwrap(), 16, 0, a);

        // a = 0000000000000001
        // b = 0000000000000010
        // c = 0000000000000011
        // d = 0000000000000100
        let expected_word: u64 = 0b_000000000000100_0000000000000011_0000000000000010_0000000000000001;

        assert_eq!(word.unwrap(), expected_word);
    }

    #[test]
    fn newu_test2() {
        let mut word: Option<u64> = Some(0);

        let a: u64 = 4927; // lsb = 0
        let b: u64 = 473; // lsb = 16
        let c: u64 = 39563; // lsb = 32
        let d: u64 = 8561; // lsb = 48

        word = bitpack::newu(word.unwrap(), 16, 48, d);
        word = bitpack::newu(word.unwrap(), 16, 32, c);
        word = bitpack::newu(word.unwrap(), 16, 16, b);
        word = bitpack::newu(word.unwrap(), 16, 0, a);

        // a = 0001001100111111
        // b = 0000000111011001
        // c = 1001101010001011
        // d = 0010000101110001
        let expected_word: u64 = 0b_0010000101110001_1001101010001011_0000000111011001_0001001100111111;

        assert_eq!(word.unwrap(), expected_word);
    }

    #[test]
    fn news_test1() {
        let mut word: Option<u64> = Some(0);

        word = bitpack::news(word.unwrap(), 16, 48, -1);

        let expected_word: u64 = 0b_1111111111111111_0000000000000000_0000000000000000_0000000000000000;

        assert_eq!(word.unwrap(), expected_word);
    }

    #[test]
    fn newu_a_test() {
        let word: u64 = 0;
        let value: u64 = 253;
        let width: u64 = 9;
        let lsb: u64 = 23;

        // let mut new_word: u64 = word;
        // new_word = word | (value << lsb); // Add value to word, then shift by lsb bits

        assert_eq!(bitpack::newu(word, 9, 23, 253), Some(word | (value << lsb)));
    }

    #[test]
    fn news_b_test() {
        let word: u64 = 0;
        assert_eq!(bitpack::news(word, 5, 18, 7), bitpack::newu(word, 5, 18, 7));
    }

    #[test]
    fn news_c_test() {
        let word: u64 = 0;
        let value: i64 = -3;
        let width = 5;
        let lsb: u64 = 13;

        // Create mask
        let mut mask: u64 = 0;
        for i in 0..(64 - width) {
            mask = mask | 1 << (64 - i - 1);
        }

        // Convert signed value to unsigned value using XOR operator with mask
        let unsigned_bits: u64 = (value as u64) ^ mask;
        // println!("{:b}", unsigned_bits);

        // Shift bits
        let new_word: u64 = word | (unsigned_bits << lsb);

        assert_eq!(bitpack::news(word, 5, 13, -3), Some(new_word));
    }

    #[test]
    fn news_d_test() {
        let word: u64 = 0;
        let value: i64 = -1;
        let width = 5;
        let lsb: u64 = 8;

        // Create mask
        let mut mask: u64 = 0;
        for i in 0..(64 - width) {
            mask = mask | 1 << (64 - i - 1);
        }

        // Convert signed value to unsigned value using XOR operator with mask
        let unsigned_bits: u64 = (value as u64) ^ mask;
        // println!("{:b}", unsigned_bits);

        // Shift bits
        let new_word: u64 = word | (unsigned_bits << lsb);

        assert_eq!(bitpack::news(word, 5, 8, -1), Some(new_word));
    }

    #[test]
    fn pack_abcd_test() {
        let mut word: Option<u64> = Some(0);
        
        // Pack a
        word = bitpack::newu(word.unwrap(), 9, 23, 253);

        // Pack b
        word = bitpack::news(word.unwrap(), 5, 18, 7);

        // Pack c
        word = bitpack::news(word.unwrap(), 5, 13, -3);

        // Pack d
        word = bitpack::news(word.unwrap(), 5, 8, -1);

        // Pack pb
        word = bitpack::newu(word.unwrap(), 4, 4, 10);

        // Pack pr
        word = bitpack::newu(word.unwrap(), 4, 0, 14);
        
        let expected_word: u64 = 0b_00000000000000000000000000000000_01111110100111111011111110101110;

        assert_eq!(word.unwrap(), expected_word);
    }

    #[test]
    fn gets_b_test() {
        let coded_word = bitpack::news(0, 5, 18, 7);
        assert_eq!(7, gets(coded_word.unwrap(), 5, 18));
    }

    #[test]
    fn gets_c_test() {
        let coded_word = bitpack::news(0, 5, 13, -3);
        assert_eq!(-3, gets(coded_word.unwrap(), 5, 13));
    }

    #[test]
    fn gets_d_test() {
        let coded_word = bitpack::news(0, 5, 8, -1);
        assert_eq!(-1, gets(coded_word.unwrap(), 5, 8));
    }

    #[test]
    fn unpack_word_test() {
        let mut word: Option<u64> = Some(0);
        
        // Pack a, b, c, d, index_pb, index_pr
        word = bitpack::newu(word.unwrap(), 9, 23, 253); // a
        word = bitpack::news(word.unwrap(), 5, 18, 7); // b
        word = bitpack::news(word.unwrap(), 5, 13, -3); // c
        word = bitpack::news(word.unwrap(), 5, 8, -1); // d
        word = bitpack::newu(word.unwrap(), 4, 4, 10); // index_pb
        word = bitpack::newu(word.unwrap(), 4, 0, 14); // index_pr
        
        // Unpack a, b, c, d, index_pb, index_pr
        let a = bitpack::getu(word.unwrap(), 9, 23);
        let b = bitpack::gets(word.unwrap(), 5, 18);
        let c = bitpack::gets(word.unwrap(), 5, 13);
        let d = bitpack::gets(word.unwrap(), 5, 8);
        let index_pb = bitpack::getu(word.unwrap(), 4, 4);
        let index_pr = bitpack::getu(word.unwrap(), 4, 0);

        let expected_a = 253;
        let expected_b = 7;
        let expected_c = -3;
        let expected_d = -1;
        let expected_index_pb = 10;
        let expected_index_pr = 14;
       
        assert_eq!(expected_a, a);
        assert_eq!(expected_b, b);
        assert_eq!(expected_c, c);
        assert_eq!(expected_d, d);
        assert_eq!(expected_index_pb, index_pb);
        assert_eq!(expected_index_pr, index_pr);
    }

}