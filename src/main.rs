use aoc_util::parse_t::*;

fn main() {
	println!("Hello, world!");
	println!("{:?}", parse_t!("testing 12 and, all, the rest", "testing ", u32, " ", Csv<PassStr>, " rest"));

	println!("{:?}", parse_t!("test 12,11, 10", "test ", Csv<u32>, ""));
	println!("{:?}", parse_t!("test 12,11, 10;", "test ", Csv<u32>, ";"));
	println!("{:?}", parse_t!("test 12, 11,10; 12 ", "test ", Csv<u32>, "; ", u32, " "));
}
