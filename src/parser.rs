use nom::{digit, alpha, alphanumeric};
use std::str;
use std::str::FromStr;

named!(pub integer<i64>,
  map_res!(
    map_res!(
      digit,
      str::from_utf8
    ),
    FromStr::from_str
  )
);

named!(pub identifier<String>,
    chain!(
        a: alpha ~
        b: many0!(alphanumeric),
        || {
            print!("a: {:?} b: {:?}\n", a, b);
            let b = b.iter(|a|);
            let mut s = String::from_utf8_lossy(b).into_owned();
            s.insert(0, a[0] as char);
            s
        }
    )
);

/*
named!(expression, delimited!(char!('('), is_not!(")"), char!(')')));
named!(statement, delimited!(char!('('), is_not!(")"), char!(')')));
*/

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;

    #[test]
    fn parses_ints() {
        let remaining: &[u8] = &[];
        assert_eq!(integer(&b"3"[..]), IResult::Done(remaining, 3));

        assert!(integer(&b"$100%"[..]).is_err());
    }

    #[test]
    fn parses_identifiers() {
        println!("{:?}", identifier(&b"abc"[..]));
        println!("{:?}", identifier(&b"abc_42"[..]));
        println!("{:?}", identifier(&b"11elf"[..]));
        println!("{:?}", identifier(&b"a_b-c?d"[..]));
        unimplemented!();
    }
}
