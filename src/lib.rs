use std::str::{FromStr};
use core::marker::PhantomData;

#[macro_export]
macro_rules! parse_util{
	($s:ident, $t:ty, $end:literal) => {
		{
			// Create a lambda to be called so that we can shortcirtuit at any point
			let f = |s: &str| {
				#[allow(unused)]
				let mut ss = s;
				Some(parse_util!(step ss, $t, $end))
			};
			f($s)
		}
	};

	($s:ident, $t:ty) => {
		<$t>::parse($s)
	};

	($s:literal $(,$prefix:literal)? $(, $more_t:ty, $more_s:literal)*) => {
		{
			#[allow(unused)]
			let mut ss = $s;
			parse_util!(ss $(,$prefix)? $(, $more_t, $more_s)*)
		}
	};

	($s:ident, $prefix:literal $(, $more_t:ty, $more_s:literal)*) => {
		#[allow(unused)]
		if let Some(mut rest) = $s.strip_prefix($prefix) {
			parse_util!(rest $(, $more_t, $more_s)*)
		}
		else {
			None
		}
	};

	($s:ident $(, $more_t:ty, $more_s:literal)*) => {
		{
			let mut f = || {
				// _ss so that we don't get faulty "value assigned to `ss` is never read" warnings
				// let mut _ss = s;
				Some(($(parse_util!(step $s, $more_t, $more_s),)*))
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

pub trait ParseUtil {
	type Res;
	fn parse(s: &str) -> Option<Self::Res>;

	fn long_name_for_macro_calling_parse(s: &str) -> Option<Self::Res> {
		Self::parse(s)
	}
}

impl<T> ParseUtil for T where T: FromStr {
	type Res = Self;
	fn parse(s: &str) -> Option<Self> {
		s.parse().ok()
	}
}

// Seperate trait as because we are using macros so lets use that to deal with pass through &str
pub trait ParseUtilStr<'a> {
	fn parse(s: &'a str) -> Option<&'a str>;

	fn long_name_for_macro_calling_parse(s: &'a str) -> Option<&'a str> {
		Self::parse(s)
	}
}

impl<'a> ParseUtilStr<'a> for &'a str {
	fn parse(s: &'a str) -> Option<&'a str> {
		Some(s)
	}
}

pub type Csv<T> = CsvStict<Trim<T>>;

pub struct CsvStict<T>(PhantomData<T>);

impl<T> ParseUtil for CsvStict<T> where T: ParseUtil{
	type Res = Vec<T::Res>;
	fn parse(s: &str) -> Option<Self::Res> {
		let mut res = Vec::new();
		// TODO: Properly do csv parsing
		for s in s.split(",") {
			res.push(T::parse(s)?);
		}
		Some(res)
	}
}

pub struct Seperated<T, const SEP: char>(PhantomData<T>);

impl<T, const SEP: char> ParseUtil for Seperated<T, SEP> where T: ParseUtil{
	type Res = Vec<T::Res>;
	fn parse(s: &str) -> Option<Self::Res> {
		let mut res = Vec::new();
		for s in s.split(SEP) {
			res.push(T::parse(s)?);
		}
		Some(res)
	}
}

pub struct Trim<T>(PhantomData<T>);
impl<T> ParseUtil for Trim<T> where T: ParseUtil {
	type Res = <T as ParseUtil>::Res;
	fn parse(s: &str) -> Option<Self::Res> {
		<T>::parse(s.trim())
	}
}

impl<'a, T> ParseUtilStr<'a> for Trim<T> where T: ParseUtilStr<'a> {
	fn parse(s: &'a str) -> Option<&'a str> {
		<T>::parse(s.trim())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_many() {
		assert_eq!(parse_util!("test 1 2 3 h all", "test ", u8, " ", u8, " ", u8, " ", char, " ", String, ""), Some((1, 2, 3, 'h', "all".into())));
	}

	#[test]
	fn parse_str() {
		assert_eq!(parse_util!("test 1 2 3 h all", "test ", u8, " ", u8, " ", u8, " ", char, " ", &str, ""), Some((1, 2, 3, 'h', "all")));
	}

	#[test]
	fn parse_last() {
		assert_eq!(parse_util!("test 12", "test ", u8, ""), Some(12));
	}

	#[test]
	fn parse_trim() {
		assert_eq!(parse_util!("test 12", "test", Trim<u8>, ""), Some(12));
	}

	#[test]
	fn parse_csv() {
		assert_eq!(parse_util!("test 12, 11,10;", "test ", Csv<u32>, ";"), Some(vec![12, 11, 10]));
	}

	#[test]
	fn parse_seperator() {
		assert_eq!(parse_util!("test 12. 11.10;", "test ", Seperated<Trim<u32>, '.'>, ";"), Some(vec![12, 11, 10]));
	}
}