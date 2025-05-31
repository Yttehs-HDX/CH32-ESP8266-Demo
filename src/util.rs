use core::fmt::{Display, Error, Write};
use heapless::String;

pub fn parse_to_str<T: Display, const N: usize>(input: T) -> Result<(String<N>, usize), Error> {
    let mut s = String::<N>::new();
    write!(&mut s, "{}", input)?;

    let len = s.chars().filter(|&c| c != '\0').count();
    Ok((s, len))
}
