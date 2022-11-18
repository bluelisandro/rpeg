#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod tests {
    use approx;
    use array2::array2::Array2;
    use round::round;

    use crate::codec::compress;

    // TIM COLANERI'S EXAMPLE
    #[test]
    fn test_trim_img() {
        let values = vec![
            csc411_image::Rgb {
                red: 100,
                green: 0,
                blue: 100,
            },
            csc411_image::Rgb {
                red: 40,
                green: 30,
                blue: 20,
            },
            csc411_image::Rgb {
                red: 100,
                green: 50,
                blue: 100,
            },
            csc411_image::Rgb {
                red: 75,
                green: 50,
                blue: 25,
            },
            csc411_image::Rgb {
                red: 50,
                green: 40,
                blue: 75,
            },
            csc411_image::Rgb {
                red: 25,
                green: 15,
                blue: 5,
            },
        ];
        let mut input_array2 = Array2::from_row_major(2, 3, values).unwrap();
        let old_height = input_array2.height();
        compress::trim::trim_img(&mut input_array2);

        assert_eq!(input_array2.height(), old_height - 1);
    }

    #[test]
    fn test_changing_to_floating_point() {
        // test
        let values = vec![
            csc411_image::Rgb {
                red: 100,
                green: 0,
                blue: 100,
            },
            csc411_image::Rgb {
                red: 40,
                green: 30,
                blue: 20,
            },
            csc411_image::Rgb {
                red: 100,
                green: 50,
                blue: 100,
            },
            csc411_image::Rgb {
                red: 75,
                green: 50,
                blue: 25,
            },
        ];
        let mut input_array2 = Array2::from_row_major(2, 2, values).unwrap();
        let input_floating_points =
            compress::change_to_floating_point::change_to_floating_point(&mut input_array2, 100);

        // expected
        let expected_vec = vec![
            (1.0, 0.0, 1.0),
            (0.4, 0.3, 0.2),
            (1.0, 0.5, 1.0),
            (0.75, 0.5, 0.25),
        ];
        let expected_array2 = Array2::from_row_major(2, 2, expected_vec).unwrap();

        assert_eq!(input_floating_points, expected_array2);
    }

    #[test]
    fn test_convert_rgb_to_component() {
        let input_vec = vec![
            (1.0, 0.0, 1.0),
            (0.4, 0.3, 0.2),
            (1.0, 0.5, 1.0),
            (0.75, 0.5, 0.25),
        ];
        let input_array2 = Array2::from_row_major(2, 2, input_vec).unwrap();
        let output_array2 = compress::rgb_to_component::rgb_to_component(&input_array2);

        let expected_vec = vec![
            (0.413, 0.331264, 0.418668),
            (0.3185, -0.0668736, 0.0581312),
            (0.7065, 0.165632, 0.209334),
            (0.54625, -0.167184, 0.145328),
        ];
        let expected_array2 = Array2::from_row_major(2, 2, expected_vec).unwrap();

        let width = expected_array2.width();
        let height = expected_array2.height();
        let benchmark = 0.005;
        for i in 0..height {
            for j in 0..width {
                let y1 = output_array2.get(j, i).unwrap().0;
                let y2 = expected_array2.get(j, i).unwrap().0;
                if approx::relative_eq!(y1, y2, epsilon = benchmark) == false {
                    assert!(false);
                }

                let pb1 = output_array2.get(j, i).unwrap().1;
                let pb2 = expected_array2.get(j, i).unwrap().1;
                if approx::relative_eq!(pb1, pb2, epsilon = benchmark) == false {
                    assert!(false);
                }

                let pr1 = output_array2.get(j, i).unwrap().2;
                let pr2 = expected_array2.get(j, i).unwrap().2;
                if approx::relative_eq!(pr1, pr2, epsilon = benchmark) == false {
                    assert!(false);
                }
            }
        }

        assert!(true);
    }

    // FIX THIS
    #[test]
    fn test_avg_of_block() {
        let input_vec = vec![
            (0.413, 0.331264, 0.418668),
            (0.3185, -0.0668736, 0.0581312),
            (0.7065, 0.165632, 0.209334),
            (0.54625, -0.167184, 0.145328),
        ];
        let input_array2 = Array2::from_row_major(2, 2, input_vec).unwrap();
        let output_array2 = compress::avg_of_block::avg_of_block(&input_array2);

        let expected_vec = vec![(0.4960625, 0.0657096, 0.2078728)];
        let expected_array2 = Array2::from_row_major(1, 1, expected_vec).unwrap();

        let width = expected_array2.width();
        let height = expected_array2.height();
        let benchmark = 0.00005;
        for i in 0..height {
            for j in 0..width {
                let y_avg = output_array2.get(j, i).unwrap().0;
                let y2_avg = expected_array2.get(j, i).unwrap().0;

                if approx::relative_eq!(y_avg, y2_avg, epsilon = benchmark) == false {
                    assert!(false);
                }

                let pb_avg = output_array2.get(j, i).unwrap().1;
                let pb2_avg = expected_array2.get(j, i).unwrap().1;
                if approx::relative_eq!(pb_avg, pb2_avg, epsilon = benchmark) == false {
                    assert!(false);
                }

                let pr_avg = output_array2.get(j, i).unwrap().2;
                let pr2_avg = expected_array2.get(j, i).unwrap().2;

                if approx::relative_eq!(pr_avg, pr2_avg, epsilon = benchmark) == false {
                    assert!(false);
                }
            }
        }

        assert!(true);
    }

    #[test]
    fn test_convert_to_4bit() {
        let input_vec = vec![(0.4960625, 0.0657096, 0.2078728)];
        let input_array2 = Array2::from_row_major(1, 1, input_vec).unwrap();
        let output_array2 = compress::convert_to_4bit::convert_to_4bit(&input_array2);

        let expected_vec = vec![(10, 14)];
        let expected_array2 = Array2::from_row_major(1, 1, expected_vec).unwrap();

        assert_eq!(output_array2, expected_array2);
    }

    #[test]
    fn test_dct() {
        let input_vec = vec![
            (0.413, 0.331264, 0.418668),
            (0.3185, -0.0668736, 0.0581312),
            (0.7065, 0.165632, 0.209334),
            (0.54625, -0.167184, 0.145328),
        ];
        let input_array2 = Array2::from_row_major(2, 2, input_vec).unwrap();
        let output_array2 = compress::dct::discrete_cosine_transform(&input_array2);

        let expected_vec = vec![(0.4960625, 0.1303125, -0.0636875, -0.0164375)];
        let expected_array2 = Array2::from_row_major(1, 1, expected_vec).unwrap();

        let width = expected_array2.width();
        let height = expected_array2.height();
        let benchmark = 0.00005;
        for i in 0..height {
            for j in 0..width {
                let a1 = output_array2.get(j, i).unwrap().0;
                let a2 = expected_array2.get(j, i).unwrap().0;

                if approx::relative_eq!(a1, a2, epsilon = benchmark) == false {
                    assert!(false);
                }

                let b1 = output_array2.get(j, i).unwrap().1;
                let b2 = expected_array2.get(j, i).unwrap().1;

                if approx::relative_eq!(b1, b2, epsilon = benchmark) == false {
                    assert!(false);
                }

                let c1 = output_array2.get(j, i).unwrap().2;
                let c2 = expected_array2.get(j, i).unwrap().2;

                if approx::relative_eq!(c1, c2, epsilon = benchmark) == false {
                    assert!(false);
                }

                let d1 = output_array2.get(j, i).unwrap().3;
                let d2 = expected_array2.get(j, i).unwrap().3;

                if approx::relative_eq!(d1, d2, epsilon = benchmark) == false {
                    assert!(false);
                }
            }
        }

        assert!(true);
    }

    // -----------------------------------------------------------

    #[test]
    fn test_trim_uneven_img() {
        let mut img_array2 = array2::array2::Array2::new(
            3,
            2,
            csc411_image::Rgb {
                red: 0,
                green: 0,
                blue: 0,
            },
        );
        let old_width = img_array2.width();
        compress::trim::trim_img(&mut img_array2);
        assert_eq!(img_array2.width(), old_width - 1);
    }

    #[test]
    fn test_convert_rgb_to_component2() {
        // RGB to Component color space formulas
        // y = 0.299 * r + 0.587 * g + 0.114 * b
        // pb = -0.168736 * r - 0.331264 * g + 0.5 * b
        // pr = 0.5 * r - 0.418688 * g - 0.081312

        let rgb_array2 = array2::array2::Array2::new(1, 1, (0.75 as f64, 0.5 as f64, 0.25 as f64));

        let component_array2 = compress::rgb_to_component::rgb_to_component(&rgb_array2);

        let y = component_array2.get(0, 0).unwrap().0;
        let pb = component_array2.get(0, 0).unwrap().1;
        let pr = component_array2.get(0, 0).unwrap().2;

        let expected_y = 0.54625;
        let expected_pb = -0.167184;
        let expected_pr = 0.145328;

        let benchmark = 0.00005;

        if approx::relative_eq!(y, expected_y, epsilon = benchmark) == false {
            assert!(false);
        }

        if approx::relative_eq!(pb, expected_pb, epsilon = benchmark) == false {
            assert!(false);
        }

        if approx::relative_eq!(pr, expected_pr, epsilon = benchmark) == false {
            assert!(false);
        }
    }

    #[test]
    fn test_convert_to_4bit_1() {
        // test values
        let input_vec = vec![
            (0.5, 0.2, 0.5),
            (0.5, 0.5, 0.2),
            (0.5, 0.1, 0.2),
            (0.5, 0.25, 0.3),
        ];
        let input_avg_array2 = Array2::from_row_major(2, 2, input_vec);
        let output = compress::convert_to_4bit::convert_to_4bit(&input_avg_array2.unwrap());

        // expected values
        let test_vec: Vec<(f32, f32)> = vec![(0.2, 0.5), (0.5, 0.2), (0.1, 0.2), (0.25, 0.3)];
        let mut expected_4bit_vec = Vec::new();

        for input in test_vec.iter() {
            let pb_avg_4bit: usize = csc411_arith::index_of_chroma(input.0);
            let pr_avg_4bit: usize = csc411_arith::index_of_chroma(input.1);
            expected_4bit_vec.push((pb_avg_4bit, pr_avg_4bit));
        }

        let expected_array2 = Array2::from_row_major(2, 2, expected_4bit_vec);
        assert_eq!(output, expected_array2.unwrap());
    }

    #[test]
    fn test_convert_to_4bit_2() {
        // test values
        let input_vec = vec![
            (0.5, 0.1, 0.5),
            (0.5, 0.1, 0.2),
            (0.5, 0.125, 0.2),
            (0.5, 0.253, 0.35),
            (0.5, -0.2523, 0.351),
            (0.5, 0.253, 0.35),
        ];
        let input_avg_array2 = Array2::from_row_major(3, 2, input_vec);
        let output = compress::convert_to_4bit::convert_to_4bit(&input_avg_array2.unwrap());

        // expected values
        let test_vec: Vec<(f32, f32)> = vec![
            (0.1, 0.5),
            (0.1, 0.2),
            (0.125, 0.2),
            (0.253, 0.35),
            (-0.2523, 0.351),
            (0.253, 0.35),
        ];

        let mut expected_4bit_vec = Vec::new();

        for input in test_vec.iter() {
            let pb_avg_4bit: usize = csc411_arith::index_of_chroma(input.0);
            let pr_avg_4bit: usize = csc411_arith::index_of_chroma(input.1);
            expected_4bit_vec.push((pb_avg_4bit, pr_avg_4bit));
        }

        let expected_array2 = Array2::from_row_major(3, 2, expected_4bit_vec);
        assert_eq!(output, expected_array2.unwrap());
    }

    fn avg_of_2x2_blocks() {
        // Create Array2 of type Rgb struct as a 4x4 matrix, each value in
        let rgb_array2 = array2::array2::Array2::new(
            4,
            4,
            csc411_image::Rgb {
                red: 1,
                green: 2,
                blue: 3,
            },
        );
    }

    #[test]
    fn test_convert_to_32bit_test() {
        let a = 0.4960625;
        let b = 0.1303125;
        let c = -0.0636875;
        let d = -0.0164375;
        let index_pb = 10;
        let index_pr = 14;

        // Convert a, b, c, d, pb, pr to integers (multiply and round to nearest integer)
        let a_int = (a * 511 as f64) as u64;
        let b_int = round(b * 50 as f64, 0) as i64;
        let c_int = round(c * 50 as f64, 0) as i64;
        let d_int = round(d * 50 as f64, 0) as i64;

        dbg!(a_int);
        dbg!(b_int);
        dbg!(c_int);
        dbg!(d_int);

        // Create 32 bit word using a, b, c, d, pb, pr
        let word = compress::convert_to_32bit_word::convert_to_32bit_word(
            a_int,
            b_int,
            c_int,
            d_int,
            index_pb as u64,
            index_pr as u64,
        );

        let expected_word: u32 = 0b_01111110_10011111_10111111_10101110;

        assert_eq!(expected_word, word);
    }
}
