use argh::FromArgs;

/// This is a basic "Hello, World" program written in Rust that supports Nix.
#[derive(FromArgs)]
pub struct MyParsedArgs {
	#[argh(subcommand)]
	subcommand: SubcommandCLI,
}

impl MyParsedArgs {
	pub fn main(self) -> ! {
		self.subcommand.main()
	}
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum SubcommandCLI {
	HelloWorld(HelloWorldData),
}

impl SubcommandCLI {
	pub fn main(self) -> ! {
		match self {
			SubcommandCLI::HelloWorld(foo) => foo.main(),
		}
	}
}

/// This subcommand is used to launch the program.
#[derive(FromArgs)]
#[argh(subcommand, name = "run")]
struct HelloWorldData {
	/// the name of the person/entity that you want to greet
	#[argh(option)]
	name: Option<String>,
}

impl HelloWorldData {
	pub fn main(self) -> ! {
		let name = self.name.unwrap_or("World".to_string());
		println!("Hello, {name}!");
		std::process::exit(0);
	}
}

pub fn parse_args() -> MyParsedArgs {
	let args: MyParsedArgs = argh::from_env();
	args
}
