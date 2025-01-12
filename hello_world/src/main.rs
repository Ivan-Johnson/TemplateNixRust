#![forbid(unsafe_code)]
use hello_world::cli::parse_args;

pub fn main() {
	let args = parse_args();
	args.main();
}
