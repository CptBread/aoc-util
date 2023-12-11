use std::str::{FromStr};
use core::marker::PhantomData;

pub use crate::parse_t;

#[macro_export]
macro_rules! parse_t{
	($s:ident, $t:ty, $end:literal) => {
		{
			// Create a lambda to be called so that we can shortcirtuit at any point
			let mut f = || {
				#[allow(unused)]
				let mut ss: &str = $s.as_ref();
				Some(parse_t!(step ss, $t, $end))
			};
			f()
		}
	};

	($s:literal $(,$prefix:literal)? $(, $more_t:ty, $more_s:literal)*) => {
		{
			#[allow(unused)]
			let mut ss: &str = $s.as_ref();
			parse_t!(ss $(,$prefix)? $(, $more_t, $more_s)*)
		}
	};

	($s:ident, $prefix:literal $(, $more_t:ty, $more_s:literal)*) => {
		{
			#[allow(unused)]
			if let Some(mut rest) = $s.strip_prefix($prefix) {
				parse_t!(rest $(, $more_t, $more_s)*)
			}
			else {
				None
			}
		}
	};

	($s:ident $(, $more_t:ty, $more_s:literal)*) => {
		{
			#[allow(unused)]
			let mut f = || {
				#[allow(unused)]
				let mut ss: &str = $s.as_ref();
				Some(($(parse_t!(step ss, $more_t, $more_s),)*))
			};
			f()
		}
	};

	(step $s:ident, $t:ty, $end:literal) => {
		if $end == "" {
			if let Some(v) = <$t>::long_name_for_macro_calling_parse($s) {
				$s = "";
				v
			}
			else {
				return None;
			}
		}
		else if let Some((front, end)) = $s.split_once($end) {
			$s = end;
			if let Some(v) = <$t>::long_name_for_macro_calling_parse(front) {
				v
			}
			else {
				return None;
			}
		}
		else
		{
			return None;
		}
	};
}

pub trait ParseTUtil<'a> {
	type Res;
	fn parse(s: &'a str) -> Option<Self::Res>;

	fn long_name_for_macro_calling_parse(s: &'a str) -> Option<Self::Res> {
		Self::parse(s)
	}
}

// Seperate trait as because we are using macros so lets use that to deal with pass through &str
pub struct PassStr<'a>(PhantomData<&'a str>);

impl<'a> ParseTUtil<'a> for PassStr<'a> {
	type Res = &'a str;
	fn parse(s: &'a str) -> Option<&'a str> {
		Some(s)
	}
}

impl<'a, T> ParseTUtil<'a> for T where T: FromStr {
	type Res = Self;
	fn parse(s: &'a str) -> Option<Self> {
		s.parse().ok()
	}
}

pub type Csv<T> = CsvStict<Trim<T>>;

pub struct CsvStict<T>(PhantomData<T>);

impl<'a, T> ParseTUtil<'a> for CsvStict<T> where T: ParseTUtil<'a>{
	type Res = Vec<T::Res>;
	fn parse(s: &'a str) -> Option<Self::Res> {
		let mut res = Vec::new();
		// TODO: Properly do csv parsing
		for s in s.split(",") {
			res.push(T::parse(s)?);
		}
		Some(res)
	}
}

pub struct Seperated<T, const SEP: char>(PhantomData<T>);

impl<'a, T, const SEP: char> ParseTUtil<'a> for Seperated<T, SEP> where T: ParseTUtil<'a> {
	type Res = Vec<T::Res>;
	fn parse(s: &'a str) -> Option<Self::Res> {
		let mut res = Vec::new();
		for s in s.split(SEP) {
			res.push(T::parse(s)?);
		}
		Some(res)
	}
}

pub struct Trim<T>(PhantomData<T>);
impl<'a, T> ParseTUtil<'a> for Trim<T> where T: ParseTUtil<'a> {
	type Res = <T as ParseTUtil<'a>>::Res;
	fn parse(s: &'a str) -> Option<Self::Res> {
		<T>::parse(s.trim())
	}
}

pub struct Seperated2d<T, const SEP: char, const LSEP: char>(PhantomData<T>);
impl<'a, T, const SEP: char, const LSEP: char> ParseTUtil<'a> for Seperated2d<T, SEP, LSEP> where T: ParseTUtil<'a> {
	type Res = (Vec<T::Res>, usize);
	fn parse(s: &'a str) -> Option<Self::Res> {
		let mut res = Vec::new();
		let mut w = None;
		for s in s.split(LSEP) {
			let mut len = 0;
			for s in s.split(SEP) {
				res.push(T::parse(s)?);
				len += 1;
			}
			if let Some(w) = w {
				assert_eq!(w, len, "Parsing Seperated2d requires that each line contains the same amount of items");
			}
			else {
				w = Some(len);
			}
		}
		Some((res, w.unwrap_or(0)))
	}
}

pub struct Chars<const NUM: usize>();
impl<'a, const NUM: usize> ParseTUtil<'a> for Chars<NUM> {
	type Res = [char; NUM];
	fn parse(s: &'a str) -> Option<Self::Res> {
		if s.len() != NUM {
			return None;
		}
		let v = s.chars().collect::<Vec<_>>();
		v.try_into().ok()
	}
}

#[cfg(test)]
mod tests {
	use crate::parse_t;
	use super::*;

	#[test]
	fn parse_t_many() {
		assert_eq!(parse_t!("test 1 2 3 h all", "test ", u8, " ", u8, " ", u8, " ", char, " ", String, ""), Some((1, 2, 3, 'h', "all".into())));
	}

	#[test]
	fn parse_t_str() {
		assert_eq!(parse_t!("test 1 2 3 h all", PassStr, " ", Seperated<PassStr, ' '>, " h", Trim<PassStr>, ""), Some(("test", vec!["1", "2", "3"], "all")));
	}

	#[test]
	fn parse_t_last() {
		assert_eq!(parse_t!("test 12", "test ", u8, ""), Some(12));
	}

	#[test]
	fn parse_t_trim() {
		assert_eq!(parse_t!("test 12", "test", Trim<u8>, ""), Some(12));
	}

	#[test]
	fn parse_t_csv() {
		assert_eq!(parse_t!("test 12, 11,10;", "test ", Csv<u32>, ";"), Some(vec![12, 11, 10]));
	}

	#[test]
	fn parse_t_seperator() {
		assert_eq!(parse_t!("test 12. 11.10;", "test ", Seperated<Trim<u32>, '.'>, ";"), Some(vec![12, 11, 10]));
	}

	#[test]
	fn parse_t_from_string() {
		let s = String::from("test 12 1");
		assert_eq!(parse_t!(s, "test ", u32, " ", u32, ""), Some((12, 1)));
	}

	#[test]
	fn parse_t_from_string2() {
		let l = "12,13 -> 12,13".to_string();
		assert_eq!(parse_t!(l, i32, ",", i32, " -> ", i32, ",", i32, ""), Some((12,13,12,13)));
		assert_eq!(l.as_str(), "12,13 -> 12,13");
	}

	#[test]
	fn parse_t_2d() {
		assert_eq!(parse_t!("test 0. 0;1.1;2.2", "test ", Seperated2d<Trim<u32>, '.', ';'>, ""), Some((vec![0, 0, 1, 1, 2, 2], 2)));
	}

	#[test]
	fn parse_t_chars() {
		assert_eq!(parse_t!("test 123", Chars<4>, " ", Chars<3>, ""), Some((['t', 'e', 's', 't'], ['1', '2', '3'])));
	}
}