use super::modpow::ModPow;
use num::BigInt;
use num::bigint::Sign;
use std::io::Write;
use std::io;


struct Test {
    data: Vec<TestCase>,
}

struct TestCase {
    modulus:  BigInt,
    base:     BigInt,
    exponent: BigInt,
    expected: BigInt,
}

// Test cases for small positive integers.
fn test_cases_small_positive_integers() -> Test {
    Test {
        data: vec![
            TestCase {
                modulus:  BigInt::from(53),
                base:     BigInt::from(11),
                exponent: BigInt::from(13),
                expected: BigInt::from(52),
            },
            TestCase {
                modulus:  BigInt::from(17),
                base:     BigInt::from(4),
                exponent: BigInt::from(3),
                expected: BigInt::from(13),
            },
            TestCase {
                modulus:  BigInt::from(509),
                base:     BigInt::from(808),
                exponent: BigInt::from(454),
                expected: BigInt::from(9),
            },
            TestCase {
                modulus:  BigInt::from(610),
                base:     BigInt::from(131), 
                exponent: BigInt::from(870),
                expected: BigInt::from(1),
            },
            TestCase {
                modulus:  BigInt::from(802),
                base:     BigInt::from(923),
                exponent: BigInt::from(483),
                expected: BigInt::from(721),
            },
            TestCase {
                modulus:  BigInt::from(800),
                base:     BigInt::from(596),
                exponent: BigInt::from(240),
                expected: BigInt::from(576),
            },
            TestCase {
                modulus:  BigInt::from(833),
                base:     BigInt::from(365),
                exponent: BigInt::from(915),
                expected: BigInt::from(155),
            },
            TestCase {
                modulus:  BigInt::from(461),
                base:     BigInt::from(343), 
                exponent: BigInt::from(115),
                expected: BigInt::from(48),
            }
        ]
    }
}

// Test cases for computing modular powers on extremely large positive integers.
fn test_cases_large_positive_integers() -> Test {
    let sign = Sign::Plus;

    Test {
        data: vec![
            TestCase {
                modulus:  BigInt::new(sign, 
                            vec![
                                    951, 513, 512, 519, 190, 153, 896, 127, 026, 161, 901, 
                                    947, 005, 214, 253, 905, 913, 323, 532, 408, 721, 094,  
                                    937, 822, 871, 789, 564, 043, 889, 316, 666, 273, 417
                                ]),
                base:     BigInt::new(sign, 
                            vec![
                                    644, 460, 366, 195, 816, 178, 219, 591, 590, 508, 150, 
                                    968, 198, 237, 239, 596, 876, 393, 130, 701, 055, 223,
                                    328, 534, 507, 319, 941, 551, 230, 679, 253, 905, 913
                                ]),
                exponent: BigInt::new(sign, 
                            vec![
                                    160, 930, 252, 280, 866, 611, 777, 587, 124, 521, 594, 
                                    735, 046, 810, 933, 665, 226, 114, 760, 013, 707, 799, 
                                    308, 160, 016, 331, 386, 938, 878, 342, 545, 632, 260
                                ]),
                expected: BigInt::new(sign, 
                            vec![
                                    756, 662, 359, 370, 394, 705, 718, 012, 922, 814, 343,
                                    729, 259, 130, 373, 245, 996, 861, 511, 525, 422, 096,
                                    106, 263, 953, 466, 374, 877, 172, 763, 695, 317, 644
                                ]),
            },
            TestCase {
                modulus:  BigInt::new(sign, 
                            vec![
                                    586, 710, 102, 019, 209, 576, 630, 666, 547, 187, 610,
                                    143, 984, 426, 456, 410, 293, 179, 774, 795, 020, 978,
                                    698, 952, 189, 195, 364, 142, 476, 018, 066, 394, 839
                                ]),
                base:     BigInt::new(sign, 
                            vec![
                                    005, 994, 690, 336, 495, 925, 843, 986, 047, 696, 580,
                                    161, 606, 653, 438, 315, 593, 228, 503, 417, 437, 559,
                                    348, 066, 551, 637, 628, 942, 078, 884, 514, 782, 688
                                ]),
                exponent: BigInt::new(sign, 
                            vec![
                                    620, 163, 966, 216, 732, 528, 880, 396, 978, 013, 207,
                                    564, 885, 579, 471, 787, 503, 025, 394, 472, 844, 660,
                                    065, 284, 117, 085, 405, 352, 185, 342, 856, 474, 304
                                ]),
                expected: BigInt::new(sign, 
                            vec![
                                    878, 186, 050, 209, 379, 253, 423, 174, 433, 099, 258,
                                    649, 240, 802, 411, 146, 972, 483, 647, 737, 808, 487,
                                    648, 900, 112, 244, 066, 304, 736, 361, 589, 795, 605
                                ]),
            },
            TestCase {
                modulus:  BigInt::new(sign, 
                            vec![
                                    911, 473, 469, 338, 652, 841, 193, 564, 752, 660, 669,
                                    999, 356, 604, 627, 107, 003, 777, 664, 012, 322, 693,
                                    339, 876, 242, 921, 931, 924, 661, 396, 621, 870, 212
                                ]),
                base:     BigInt::new(sign, 
                            vec![
                                    229, 803, 841, 993, 625, 747, 698, 679, 724, 455, 161,
                                    383, 044, 020, 559, 949, 678, 118, 267, 160, 896, 767,
                                    845, 321, 696, 921, 691, 009, 756, 712, 100, 116, 995
                                ]),
                exponent: BigInt::new(sign, 
                            vec![
                                    410, 758, 197, 134, 355, 683, 244, 470, 315, 048, 406,
                                    545, 074, 967, 474, 482, 424, 021, 192, 891, 911, 345,
                                    576, 718, 877, 409, 373, 797, 009, 648, 938, 087, 138
                                ]),
                expected: BigInt::new(sign, 
                            vec![
                                    867, 535, 804, 209, 611, 182, 823, 420, 110, 019, 328,
                                    043, 809, 795, 633, 858, 437, 669, 124, 272, 850, 463,
                                    809, 480, 994, 041, 352, 403, 560, 594, 162, 339, 903
                                ]),
            },
            TestCase {
                modulus:  BigInt::new(sign, 
                            vec![
                                    433, 007, 622, 210, 136, 994, 168, 143, 386, 418, 902,
                                    019, 530, 951, 990, 317, 996, 843, 871, 190, 791, 064,
                                    910, 522, 941, 644, 049, 697, 177, 589, 725, 883, 432
                                ]),
                base:     BigInt::new(sign, 
                            vec![
                                    553, 347, 006, 857, 360, 945, 368, 402, 181, 507, 459,
                                    111, 701, 792, 259, 123, 886, 171, 282, 209, 692, 271,
                                    618, 895, 335, 711, 498, 235, 535, 396, 011, 668, 519
                                ]),
                exponent: BigInt::new(sign, 
                                vec![
                                        732, 815, 172, 657, 699, 242, 261, 287, 852, 665, 248,
                                        538, 619, 084, 790, 599, 825, 346, 663, 191, 342, 919,
                                        877, 222, 591, 239, 588, 069, 601, 998, 714, 046, 873
                                    ]),
                expected: BigInt::new(sign, 
                            vec![
                                    593, 780, 595, 016, 569, 438, 291, 531, 506, 048, 933,
                                    434, 642, 177, 027, 417, 180, 691, 693, 753, 506, 053,
                                    095, 603, 893, 350, 890, 459, 416, 852, 074, 963, 410
                                ]),
            }
        ]
    }
}

fn test_cases_modulo_one() -> Test {
    Test {
        data: vec![
            TestCase {
                modulus:  BigInt::from(1),
                base:     BigInt::from(596),
                exponent: BigInt::from(240),
                expected: BigInt::from(0),
            },
            TestCase {
                modulus:  BigInt::from(1),
                base:     BigInt::new(Sign::Plus, 
                            vec![
                                    005, 994, 690, 336, 495, 925, 843, 986, 047, 696, 580,
                                    161, 606, 653, 438, 315, 593, 228, 503, 417, 437, 559,
                                    348, 066, 551, 637, 628, 942, 078, 884, 514, 782, 688
                                ]),
                exponent: BigInt::new(Sign::Plus, 
                            vec![
                                    620, 163, 966, 216, 732, 528, 880, 396, 978, 013, 207,
                                    564, 885, 579, 471, 787, 503, 025, 394, 472, 844, 660,
                                    065, 284, 117, 085, 405, 352, 185, 342, 856, 474, 304
                                ]),
                expected: BigInt::from(0),
            },
            TestCase {
                modulus:  BigInt::from(1),
                base:     BigInt::from(596),
                exponent: BigInt::from(-240),
                expected: BigInt::from(0),
            },
            TestCase {
                modulus:  BigInt::from(1),
                base:     BigInt::from(596),
                exponent: BigInt::from(0),
                expected: BigInt::from(0),
            }
        ]
    }
}

fn run_test(test: &Test) {
    for test_case in test.data.iter() {
        let result = <BigInt as ModPow>::mod_pow(&test_case.base, &test_case.exponent, &test_case.modulus);
        assert_eq!(result, test_case.expected);
    }
}

#[test]
#[should_panic]
fn test_modpow_should_panic_with_zero_modulus() {
    let modulus  = BigInt::from(0);
    let exponent = BigInt::from(53);
    let base     = BigInt::from(11);

    let result   = <BigInt as ModPow>::mod_pow(&base, &exponent, &modulus);

    assert_eq!(result, result);
    // mod_pow failed.
    assert!(false);
}

#[test]
fn test_run_small_positive_integers() {
    run_test(&test_cases_small_positive_integers());
}

#[test]
fn test_run_large_positive_integers() {
    run_test(&test_cases_large_positive_integers());
}

#[test]
fn test_run_integers_modulo_one() {
    run_test(&test_cases_modulo_one());
}

fn print_test_cases(test: &Test) {
    for test_case in test.data.iter() {
        let result = <BigInt as ModPow>::mod_pow(&test_case.base, &test_case.exponent, &test_case.modulus);
        writeln!(&mut io::stderr(), "\nmodulus:  {}", test_case.modulus);
        writeln!(&mut io::stderr(), "base:     {}", test_case.base);
        writeln!(&mut io::stderr(), "exponent: {}", test_case.exponent);
        writeln!(&mut io::stderr(), "expected: {}", test_case.expected);
        writeln!(&mut io::stderr(), "result:   {}\n", result);
    }
}

#[test]
fn test_print_test_cases() {
    print_test_cases(&test_cases_large_positive_integers());
}