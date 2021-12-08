
use std::env;
use std::process;

pub struct Options {
	pub input   : String,
	pub output  : String,
	pub chainid : String,
	pub author  : String,
	pub title   : String,
}

impl Options {
	pub fn new() -> Options
	{
		let argv : Vec<String> = env::args().collect();
		let argc : usize = argv.len();

		let mut arg_i : &String = &String::new();
		let mut arg_o : &String = &String::new();
		let mut arg_c : &String = &String::new();
		let mut arg_a : &String = &String::from( "yes" );
		let mut arg_t : &String = &String::new();

		if argc < 5 { show_usage( &argv[ 0 ] ) };

		let mut i : usize = 1;
		while i < argc {
			match argv[ i ].as_str() {
				"-i" => { i += 1; arg_i = &argv[ i ]; }
				"-o" => { i += 1; arg_o = &argv[ i ]; }
				"-c" => { i += 1; arg_c = &argv[ i ]; }
				"-a" => { i += 1; arg_a = &argv[ i ]; }
				"-t" => { i += 1; arg_t = &argv[ i ]; }
				"-h" => { show_usage( &argv[ 0 ] );   }
				_    => { show_usage( &argv[ 0 ] );   }
			}
			i += 1;
		}

		if ( *arg_c ).is_empty() { show_usage( &argv[ 0 ] ); }

		match ( *arg_a ).as_str() {
			"yes" | "no" => (),
			_            => show_usage( &argv[ 0 ] ),
		}

		Options {
			input   : arg_i.to_string(),
			output  : arg_o.to_string(),
			chainid : arg_c.to_string(),
			author  : arg_a.to_string(),
			title   : arg_t.to_string(),
		}
	}

	pub fn show_parameter( &self ) {

		println!( "\nParameter set :"                           );
		println!( "===========================================" );
		println!( "Input filename  : {}", self.input            );
		println!( "Onput filename  : {}", self.output           );
		println!( "Author Seq ID   : {}", self.author           );
		println!( "Chain ID        : {}", self.chainid          );
		println!( "Title line name : {}", self.title            );
		println!( "===========================================" );
	}
}

fn show_usage( arg : &String ) {

	println!( "Usage: {} [Options] \n\nOptions :\n\n", *arg );
	println!( "    -i    Input filename, REQUIRED." );
	println!( "    -o    Output filename, REQUIRED." );
	println!( "    -a    Use the Author Sequence ID ('yes' or 'no', default 'yes').
          If 'yes',
              * auth_comp_id
              * auth_asym_id
              * auth_atom_id
          in 'atom_site' Data Category are used.
          If 'no',
              * label_comp_id
              * label_asym_id
              * label_atom_id
          are used instead." );
	println!( "    -c    Chain ID, REQUIRED." );
	println!( "    -t    Name of title line in the output FASTA file.
          If no title line name set, it will be same as the output filename." );
	println!( "    -h    Print this help, ignore all other arguments." );
	println!( "\n" );

	process::exit( 1 );
}
