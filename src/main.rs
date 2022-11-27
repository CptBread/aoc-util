use aoc_util::*;

fn main() {
	println!("Hello, world!");
	println!("{:?}", parse_util!("test 12,11, 10", "test ", Csv<u32>, ""));
	println!("{:?}", parse_util!("test 12,11, 10;", "test ", Csv<u32>, ";"));
	println!("{:?}", parse_util!("test 12, 11,10; 12 ", "test ", Csv<u32>, "; ", u32, " "));
}
