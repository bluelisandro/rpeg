#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod tests {
    use crate::codec::decompress;
    use array2::array2::Array2;
    use csc411_image::Rgb;
    use std::vec;

    #[test]
    fn test_trim_u32_words() {
        let mut words_array2: Array2<u32> =
            Array2::from_row_major(3, 3, vec![1, 1, 1, 2, 2, 2, 3, 3, 3]).unwrap();

        let old_width = words_array2.width();
        let old_height = words_array2.height();

        decompress::trim::trim_img(&mut words_array2);

        assert_eq!(old_width - 1, words_array2.width());
        assert_eq!(old_height - 1, words_array2.height());
    }

    #[test]
    fn test_reverse_dct() {
        let a =  0.4960625;
        let b = 0.1303125;
        let c =  -0.0636875;
        let d = -0.0164375;
        let index_pb = 10 as f32;
        let index_pr = 14 as f32;

        let abcd_array2 = Array2::new(1, 1, (a, b, c, d, index_pb, index_pr));

        let y_array2 = decompress::reverse_dct::reverse_dct(&abcd_array2);

        let expected_y1 = 0.413;
        let expected_y2 = 0.3185;
        let expected_y3 =  0.7065;
        let expected_y4 = 0.54625;

        let y1 = y_array2.get(0, 0).unwrap().0;
        let y2 = y_array2.get(0, 0).unwrap().1;
        let y3 = y_array2.get(0, 0).unwrap().2;
        let y4 = y_array2.get(0, 0).unwrap().3;

        let benchmark = 0.00005;

        if approx::relative_eq!(expected_y1, y1, epsilon = benchmark) == false {
            assert!(false);
        }

        if approx::relative_eq!(expected_y2, y2, epsilon = benchmark) == false {
            assert!(false);
        }

        if approx::relative_eq!(expected_y3, y3, epsilon = benchmark) == false {
            assert!(false);
        }

        if approx::relative_eq!(expected_y4, y4, epsilon = benchmark) == false {
            assert!(false);
        }
    }

    #[test]
    fn test_component_video_color() {
        let a =  0.4960625;
        let b = 0.1303125;
        let c =  -0.0636875;
        let d = -0.0164375;
        let index_pb = 0.0657096 as f32;
        let index_pr = 0.2078728 as f32;

        let abcd_array2 = Array2::new(1, 1, (a, b, c, d, index_pb, index_pr));

        let y_array2 = decompress::reverse_dct::reverse_dct(&abcd_array2);

        let yi_pb_pr = decompress::component_video_color::component_video_color(&y_array2, &abcd_array2);

        let yi = yi_pb_pr.get(0, 0).unwrap().0;
        let pb = yi_pb_pr.get(0, 0).unwrap().1;
        let pr = yi_pb_pr.get(0, 0).unwrap().2;

        let expected_yi = 0.413;
        let expected_pb =  0.0657096 as f32;
        let expected_pr =   0.2078728 as f32;

        let benchmark = 0.00005;

        if approx::relative_eq!(expected_yi, yi, epsilon = benchmark) == false {
            assert!(false);
        }

        if approx::relative_eq!(expected_pb as f64, pb as f64, epsilon = benchmark) == false {
            assert!(false);
        }

        if approx::relative_eq!(expected_pr as f64, pr as f64, epsilon = benchmark) == false {
            assert!(false);
        }
    }

    #[test]
    fn test_component_video_color2() {
        let a =  0.4960625;
        let b = 0.1303125;
        let c =  -0.0636875;
        let d = -0.0164375;
        let index_pb = 0.0657096 as f32;
        let index_pr = 0.2078728 as f32;
        
        // let a2 =  0.4960625;
        // let b2 = 0.1303125;
        // let c2 =  -0.0636875;
        // let d2 = -0.0164375;
        // let index_pb2 = 0.0657096 as f32;
        // let index_pr2 = 0.2078728 as f32;

        let temp_vec = vec![(a, b, c, d, index_pb, index_pr), (a, b, c, d, index_pb, index_pr)];

        let abcd_array2 = Array2::from_row_major(2, 1, temp_vec).unwrap();

        let y_array2 = decompress::reverse_dct::reverse_dct(&abcd_array2);

        let yi_pb_pr = decompress::component_video_color::component_video_color(&y_array2, &abcd_array2);

        let yi = yi_pb_pr.get(0, 0).unwrap().0;
        let pb = yi_pb_pr.get(0, 0).unwrap().1;
        let pr = yi_pb_pr.get(0, 0).unwrap().2;

        let yi2 = yi_pb_pr.get(1, 0).unwrap().0;
        let pb2 = yi_pb_pr.get(1, 0).unwrap().1;
        let pr2 = yi_pb_pr.get(1, 0).unwrap().2;

        let expected_yi_1 = 0.413;
        let expected_pb_1 =  0.0657096 as f32;
        let expected_pr_1 =   0.2078728 as f32;

        let expected_yi_2 = 0.3185;
        let expected_pb_2 = 0.0657096 as f32;
        let expected_pr_2 =  0.2078728 as f32;

        let benchmark = 0.00005;

        if approx::relative_eq!(expected_yi_1, yi, epsilon = benchmark) == false {
            assert!(false);
        }

        if approx::relative_eq!(expected_pb_1 as f64, pb as f64, epsilon = benchmark) == false {
            assert!(false);
        }

        if approx::relative_eq!(expected_pr_1 as f64, pr as f64, epsilon = benchmark) == false {
            assert!(false);
        }

        if approx::relative_eq!(expected_yi_2, yi2, epsilon = benchmark) == false {
            assert!(false);
        }

        if approx::relative_eq!(expected_pb_2 as f64, pb2 as f64, epsilon = benchmark) == false {
            assert!(false);
        }

        if approx::relative_eq!(expected_pr_2 as f64, pr2 as f64, epsilon = benchmark) == false {
            assert!(false);
        }


    }
}
