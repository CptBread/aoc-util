use aoc_util::parse_t::*;
use aoc_util::parse_f::*;
use std::str::FromStr;

trait UntilT<'a> {
	fn until<T, F>(&self, sep: &'a str, func: F) -> Option<(T, &'a str)> 
		where F: FnMut(&'a str) -> Option<T>
	;

	fn parse_until<T>(&self, sep: &'a str) -> Option<(T, &'a str)>
		where T: FromStr
	;
}

impl<'a> UntilT<'a> for &'a str {
	fn until<T, F>(&self, sep: &'a str, mut func: F) -> Option<(T, &'a str)> 
		where F: FnMut(&'a str) -> Option<T>
	{
		let (s, r) = self.split_once(sep)?;
		Some((func(s)?, r))
	}

	fn parse_until<T>(&self, sep: &'a str) -> Option<(T, &'a str)> 
		where T: FromStr
	{
		let (s, r) = self.split_once(sep)?;
		Some((s.parse::<T>().ok()?, r))
	}
}

trait TryIntoVec<T> {
	fn try_vec(&mut self) -> Option<Vec<T>>;
}

impl<T, It> TryIntoVec<T> for It where It: Iterator<Item = Option<T>> {
	fn try_vec(&mut self) -> Option<Vec<T>> {
		let mut res = Vec::new();
		while let Some(v) = self.next() {
			res.push(v?);
		}
		Some(res)
	}

}

fn main() {
	println!("Hello, world!");
	println!("{:?}", parse_t!("testing 12 and, all, the rest", "testing ", u32, " ", Csv<PassStr>, " rest"));

	println!("{:?}", parse_t!("test 12,11, 10", "test ", Csv<u32>, ""));
	println!("{:?}", parse_t!("test 12,11, 10;", "test ", Csv<u32>, ";"));
	println!("{:?}", parse_t!("test 12, 11,10; 12 ", "test ", Csv<u32>, "; ", u32, " "));

	test();
}

fn test() -> Option<()>
{
	// let t = "test 12, 11,10; 12 ";
	// let (v, rest) = t.strip_prefix("test ")?.until(";", |s| Some(s))?;
	// println!("{} {}", v, rest);
	// let (v, rest) = ?;
	// assert_eq!(parse_t!("test 1 2 3 h all", "test ", u8, " ", u8, " ", u8, " ", char, " ", String, ""), Some((1, 2, 3, 'h', "all".into())));

	// let (v, rest) = t.strip_prefix("test ")?.until(";", seperated(",", trim(from_str::<u32>)))?;
	// let i: u32 = rest.trim().parse().ok()?;
	// println!("{:?} {:?} {:?}", v, i, rest);
	// println!("T: {:?}", t.strip_prefix("test ")?.until(";", |s| s.split(",").map(|s| u32::from_str(s.trim()).ok()).try_vec()));

	let t = "test 1 2 3 h all";
	let (i0, rest) = t.strip_prefix("test ")?.parse_until::<u8>(" ")?;
	let (i1, rest) = rest.parse_until::<u8>(" ")?;
	let (i2, rest) = rest.parse_until::<u8>(" ")?;
	let (c, rest) = rest.parse_until::<char>(" ")?;
	println!("{:?}", (i0, i1, i2, c, rest));

	let (i0, i1, i2, c, rest) = parse_f!(t, "test ", 
		// (from_str::<u8>, " "), (from_str::<u8>, " "), (from_str::<u8>, " "), (from_str::<char>, " ")
		(from_str::<u8>, " "), (from_str::<u8>, " "), (from_str::<u8>, " "), (from_str::<char>, " ")
	)?;
	println!("{:?}", (i0, i1, i2, c, rest));
	Some(())
}