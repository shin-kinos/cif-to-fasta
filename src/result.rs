
use std::fs::File;
use std::io::Write;

pub fn show_result(
	res_list_1 : &Vec<char>,
	arg_t      : &String,
	arg_o      : &String
) {

	println!( "\nResult :\n" );

	let title_name : &String = if ( *arg_t ).is_empty() { arg_o } else { arg_t };
	//println!( "Name of title line : {}", title_name );

	print!( ">{}\n", *title_name );
	for i in 1 .. ( ( *res_list_1 ).len() + 1 ) {
		if i % 60 == 0 {
			print!( "{}", ( *res_list_1 )[ i - 1 ] );
			print!( "\n" );
		} else {
			print!( "{}", ( *res_list_1 )[ i - 1 ] );
		}
	}
	print!( "\n\n" );

}

pub fn save_result(
	res_list_1 : &Vec<char>,
	arg_t      : &String,
	arg_o      : &String
) {

	let mut fout = File::create( ( *arg_o ).as_str() ).expect( "FAILED to open output file" );

	let title_name : &String = if ( *arg_t ).is_empty() { arg_o } else { arg_t };
	//println!( "Name of title line : {}", title_name );

	write!( fout, ">{}\n", *title_name ).expect( "FAILED to write" );
	for i in 1 .. ( ( *res_list_1 ).len() + 1 ) {
		if i % 60 == 0 {
			write!( fout, "{}", ( *res_list_1 )[ i - 1 ] ).expect( "FAILED to write" );
			write!( fout, "\n" ).expect( "FAILED to write" );
		} else {
			write!( fout, "{}", ( *res_list_1 )[ i - 1 ] ).expect( "FAILED to write" );
		}
	}
	write!( fout, "\n" ).expect( "FAILED to write" );


	println!( "\nThe output file was correctly written.\n" );
}
