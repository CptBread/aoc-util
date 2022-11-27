use std::str::{FromStr};

#[macro_export]
macro_rules! parse_util{
	($s:ident, $t:ty, $end:literal) => {
		{
			let f = |s: &str| {
				// _ss so that we don't get faulty "value assigned to `ss` is never read" warnings
				let mut _ss = s;
				Some(parse_util!(step _ss, $t, $end))
			};
			f($s)
		}
	};

	($s:ident, $t:ty) => {
		<$t>::parse($s)
	};

	($s:literal $(,$prefix:literal)? $(, $more_t:ty, $more_s:literal)*) => {
		{
			let ss = $s;
			parse_util!(ss $(,$prefix)? $(, $more_t, $more_s)*)
		}
	};

	($s:ident, $prefix:literal $(, $more_t:ty, $more_s:literal)*) => {
		if let Some(rest) = $s.strip_prefix($prefix) {
			parse_util!(rest $(, $more_t, $more_s)*)
		}
		else {
			None
		}
	};

	($s:ident $(, $more_t:ty, $more_s:literal)*) => {
		{
			let f = |s: &str| {
				// _ss so that we don't get faulty "value assigned to `ss` is never read" warnings
				let mut _ss = s;
				Some(($(parse_util!(step _ss, $more_t, $more_s),)*))
			};
			f($s)
		}
	};

	(step $s:ident, $t:ty, $end:literal) => {
		if $end == "" {
			if let Some(v) = <$t>::parse($s) {
				$s = "";
				v
			}
			else {
				return None;
			}
		}
		else if let Some((front, end)) = $s.split_once($end) {
			$s = end;
			if let Some(v) = <$t>::parse(front) {
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
}

impl<T> ParseUtil for T where T: FromStr {
	type Res = Self;
	fn parse(s: &str) -> Option<Self> {
		s.parse().ok()
	}
}

pub type Csv<T> = CsvStict<Trim<T>>;

pub struct CsvStict<T>(Vec<<T as ParseUtil>::Res>) where T: ParseUtil;

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

pub struct Seperated<T, const SEP: char>(Vec<<T as ParseUtil>::Res>) where T: ParseUtil;

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

pub struct Trim<T>(<T as ParseUtil>::Res) where T: ParseUtil;
impl<T> ParseUtil for Trim<T> where T: ParseUtil {
	type Res = <T as ParseUtil>::Res;
	fn parse(s: &str) -> Option<Self::Res> {
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