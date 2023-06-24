use std::collections::HashMap;

use pem::Block;

lazy_static::lazy_static! {
  pub static ref PEM_PRIVATE_KEY2: String = {
    const RAW:&'static str = r#"-----BEGIN RSA TESTING KEY-----
Proc-Type: 4,ENCRYPTED
Content-Domain: RFC822
DEK-Info: AES-128-CBC,BFCD243FEDBB40A4AA6DDAA1335473A4

qDXMK7nLIavAnXZhAPSBrWnSwEJBO+Q8r1lebSo8nKGkXmg3xIxwHKkY5sIripHc
LR8IIznxS4sbL0YLsia6T0CAOcSxyzu0ZT8bsvcI0sbVqJ8jabY9+awcsxOHZAQ3
20DIggzQ+CF83L0JBCAWsJfibVYd4+zw/OJWraQDcG1jPAG+Pig4b8Dm/YXRU6ib
y9QEsXO5czLWesYpJaXaF5N6EOhB+6UXIPhO6eOPUSATu963k64TivYJ9KZB4CtR
GjA4DbE7Z4dk9coyZ9HIpT0jcsQGr497Jqw8dZGhABPGXEnVPOeyspng1SX64hKA
N4XPksobn/NO2IDvPM7N9ZCe+aeyDEkE8QmP6mPScLuGvzSrsgOxWTMWF7Dbdzj0
tJQLJRZ+ItT5Irl4owSEBNLahC1j3fhQavbj9WVAfKk=
-----END RSA TESTING KEY-----
"#;

    testing_key(RAW)
  };

  pub static ref PRIVATE_KEY2: Block = {
    let type_ = "RSA PRIVATE KEY".to_string();

    let mut headers = HashMap::default();
        headers.insert(  "Proc-Type".to_string(), "4,ENCRYPTED".to_string());
        headers.insert("DEK-Info".to_string(), "AES-128-CBC,BFCD243FEDBB40A4AA6DDAA1335473A4".to_string());
        headers.insert("Content-Domain".to_string(), "RFC822".to_string());

    Block { type_, headers, bytes: vec![
      0xa8, 0x35, 0xcc, 0x2b, 0xb9, 0xcb, 0x21, 0xab, 0xc0,
        0x9d, 0x76, 0x61, 0x0, 0xf4, 0x81, 0xad, 0x69, 0xd2,
        0xc0, 0x42, 0x41, 0x3b, 0xe4, 0x3c, 0xaf, 0x59, 0x5e,
        0x6d, 0x2a, 0x3c, 0x9c, 0xa1, 0xa4, 0x5e, 0x68, 0x37,
        0xc4, 0x8c, 0x70, 0x1c, 0xa9, 0x18, 0xe6, 0xc2, 0x2b,
        0x8a, 0x91, 0xdc, 0x2d, 0x1f, 0x8, 0x23, 0x39, 0xf1,
        0x4b, 0x8b, 0x1b, 0x2f, 0x46, 0xb, 0xb2, 0x26, 0xba,
        0x4f, 0x40, 0x80, 0x39, 0xc4, 0xb1, 0xcb, 0x3b, 0xb4,
        0x65, 0x3f, 0x1b, 0xb2, 0xf7, 0x8, 0xd2, 0xc6, 0xd5,
        0xa8, 0x9f, 0x23, 0x69, 0xb6, 0x3d, 0xf9, 0xac, 0x1c,
        0xb3, 0x13, 0x87, 0x64, 0x4, 0x37, 0xdb, 0x40, 0xc8,
        0x82, 0xc, 0xd0, 0xf8, 0x21, 0x7c, 0xdc, 0xbd, 0x9, 0x4,
        0x20, 0x16, 0xb0, 0x97, 0xe2, 0x6d, 0x56, 0x1d, 0xe3,
        0xec, 0xf0, 0xfc, 0xe2, 0x56, 0xad, 0xa4, 0x3, 0x70,
        0x6d, 0x63, 0x3c, 0x1, 0xbe, 0x3e, 0x28, 0x38, 0x6f,
        0xc0, 0xe6, 0xfd, 0x85, 0xd1, 0x53, 0xa8, 0x9b, 0xcb,
        0xd4, 0x4, 0xb1, 0x73, 0xb9, 0x73, 0x32, 0xd6, 0x7a,
        0xc6, 0x29, 0x25, 0xa5, 0xda, 0x17, 0x93, 0x7a, 0x10,
        0xe8, 0x41, 0xfb, 0xa5, 0x17, 0x20, 0xf8, 0x4e, 0xe9,
        0xe3, 0x8f, 0x51, 0x20, 0x13, 0xbb, 0xde, 0xb7, 0x93,
        0xae, 0x13, 0x8a, 0xf6, 0x9, 0xf4, 0xa6, 0x41, 0xe0,
        0x2b, 0x51, 0x1a, 0x30, 0x38, 0xd, 0xb1, 0x3b, 0x67,
        0x87, 0x64, 0xf5, 0xca, 0x32, 0x67, 0xd1, 0xc8, 0xa5,
        0x3d, 0x23, 0x72, 0xc4, 0x6, 0xaf, 0x8f, 0x7b, 0x26,
        0xac, 0x3c, 0x75, 0x91, 0xa1, 0x0, 0x13, 0xc6, 0x5c,
        0x49, 0xd5, 0x3c, 0xe7, 0xb2, 0xb2, 0x99, 0xe0, 0xd5,
        0x25, 0xfa, 0xe2, 0x12, 0x80, 0x37, 0x85, 0xcf, 0x92,
        0xca, 0x1b, 0x9f, 0xf3, 0x4e, 0xd8, 0x80, 0xef, 0x3c,
        0xce, 0xcd, 0xf5, 0x90, 0x9e, 0xf9, 0xa7, 0xb2, 0xc,
        0x49, 0x4, 0xf1, 0x9, 0x8f, 0xea, 0x63, 0xd2, 0x70,
        0xbb, 0x86, 0xbf, 0x34, 0xab, 0xb2, 0x3, 0xb1, 0x59,
        0x33, 0x16, 0x17, 0xb0, 0xdb, 0x77, 0x38, 0xf4, 0xb4,
        0x94, 0xb, 0x25, 0x16, 0x7e, 0x22, 0xd4, 0xf9, 0x22,
        0xb9, 0x78, 0xa3, 0x4, 0x84, 0x4, 0xd2, 0xda, 0x84,
        0x2d, 0x63, 0xdd, 0xf8, 0x50, 0x6a, 0xf6, 0xe3, 0xf5,
        0x65, 0x40, 0x7c, 0xa9,
    ] }
  };
}

pub fn testing_key(s: &str) -> String {
    s.replace("TESTING KEY", "PRIVATE KEY")
}
