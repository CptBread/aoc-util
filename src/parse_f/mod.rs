use std::str::{FromStr};
use crate::parse_t::ParseTUtil;

// So that we don't need to do a seperate use for the macro when using this
pub use crate::parse_f;

#[macro_export]
macro_rules! parse_f{
	($s:expr, $($prefix:literal,)? $(($p:expr, $sep:literal)),+) => {
		{
			// Create a lambda to be called so that we can shortcirtuit at any point
			let mut f = || {
				#[allow(unused)]
				let mut ss: &str = ($s).as_ref();
				$(ss = ss.strip_prefix($prefix)?;)?
				Some(($(parse_f!(step ss, $p, $sep)),*, ss))
			};
			f()
		}
	};

	(step $s:ident, $p:expr, $sep:literal) => {
		{
			if $sep == "" {
				if let Some(v) = $p($s) {
					$s = "";
					v
				}
				else {
					return None;
				}
			}
			else if let Some((front, end)) = $s.split_once($sep) {
				$s = end;
				if let Some(v) = $p(front) {
					v
				}
				else {
					return None;
				}
			}
			else {
				return None;
			}
		}
	};
}

pub fn from_str<'a, T: FromStr>(s: &'a str) -> Option<T> {
	s.parse::<T>().ok()
}

pub fn from_adaptor<'a, T: ParseTUtil<'a>>(s: &'a str) -> Option<T::Res> {
	T::parse(s)
}

pub fn passtrough(s: &str) -> Option<&str> {
	Some(s)
}

pub fn seperated_f<'b, 'a, T, F: FnMut(&'a str) -> Option<T> + 'b>(sep: &'b str, mut func: F) -> impl FnMut(&'a str) -> Option<Vec<T>> + 'b {
	move |s: &'a str| -> Option<Vec<T>> {
		let mut res = Vec::new();
		for s in s.split(sep) {
			res.push(func(s)?);
		}
		Some(res)
	}
}

pub fn byte_mapping<'a, T, F: FnMut(char) -> Option<T>>(mut func: F) -> impl FnMut(&'a str) -> Option<Vec<T>> {
	move |s: &'a str| -> Option<Vec<T>> {
		let mut res = Vec::new();
		for c in s.chars() {
			res.push(func(c)?);
		}
		Some(res)
	}
}

pub fn byte_mapping2d<'a, T, F>(sep: &'a str, mut func: F) -> impl FnMut(&'a str) -> Option<(Vec<T>, usize)> 
	where F: FnMut(char) -> Option<T>
{
	move |s: &'a str| {
		let mut res = Vec::new();
		let mut w = None;
		for s in s.split(sep) {
			let mut l = 0;
			for c in s.chars() {
				res.push(func(c)?);
				l += 1;
			}
			if w.is_none() {
				w = Some(l);
			}
			else if w.unwrap() != l {
				return None;
			}
		}
		Some((res, w.unwrap_or(0)))
	}
}

pub fn fixed_size<'a, T, F, const N: usize>(sep: &'a str, mut func: F) -> impl FnMut(&'a str) -> Option<[T; N]> 
	where 
		F: FnMut(&'a str) -> Option<T>,
		T: Default + Copy,
{
	move |s: &'a str| {
		let mut res = [T::default(); N];
		let mut last_n = 0;
		for (s, n) in s.split(sep).zip(0..N) {
			res[n] = func(s)?;
			last_n = n;
		}
		if last_n + 1 == N {
		Some(res)
		}
		else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::parse_f;
	use crate::parse_t::Csv;

	#[test]
	fn parse_f_from_str() {
		assert_eq!(parse_f!("1, 2, 3", (from_str::<u8>, ", "), (from_str::<u8>, ", ")), Some((1, 2, "3")));
	}

	#[test]
	fn parse_f_adaptors() {
		assert_eq!(parse_f!("1, 2, 3", (from_adaptor::<Csv<u8>>, "")), Some((vec![1, 2, 3], "")));
	}

	#[test]
	fn parse_f_passthrough() {
		assert_eq!(parse_f!("1, 2, 3", (passtrough, "")), Some(("1, 2, 3", "")));
	}

	#[test]
	fn parse_f_closure() {
		let n = 1;
		assert_eq!(parse_f!("1", (|s: &str|{ Some(s.parse::<u8>().ok()? + n) }, "")), Some((2u8, "")));
	}

	#[test]
	fn parse_f_closure_mut() {
		let mut n = 0;
		let mut f = |s: &str| -> Option<u8> { n += 1; Some(s.parse::<u8>().ok()? + n) };
		assert_eq!(parse_f!("0,0", (f, ","), (f, "")), Some((1u8, 2u8, "")));
	}

	#[test]
	fn parse_f_seperated_f() {
		let mut n = 0;
		let f = |s: &str| -> Option<u8> { n += 1; Some(s.parse::<u8>().ok()? + n) };
		assert_eq!(parse_f!("0, 0", (seperated_f(", ", f), "")), Some((vec![1u8, 2u8], "")));
		assert_eq!(parse_f!("0, 0", (seperated_f(", ", from_str), "")), Some((vec![0u8, 0u8], "")));
	}

	#[test]
	fn parse_f_byte_mapping() {
		let f = |c: char| -> Option<bool> { 
			match c {
				'#' => Some(true),
				'.' => Some(false),
				_ => None,
			}
		};
		assert_eq!(parse_f!("#.", (byte_mapping(f), "")), Some((vec![true, false], "")));
		assert_eq!(parse_f!("#. ## .#", (byte_mapping2d(" ", f), "")), Some(((vec![true, false, true, true, false, true], 2), "")));
	}

	#[test]
	fn parse_f_prefix() {
		assert_eq!(parse_f!("test 12.11.10;", "test ", (seperated_f(".", from_str), ";")), Some((vec![12, 11, 10], "")));
	}
}