
mod convert;
mod mmcif;
mod options;
mod result;

fn main() {

	println!( "A program that convert mmCIF format into FASTA format by using 'atom_site' Data Category." );

	/* Set options. */
	let opts = options::Options::new();
	opts.show_parameter();

	/* Read mmCIF file and get information. */
	let mut data = mmcif::MmCif::new();
	data.read_cif_info( &( opts.input ) );
	//println!( "{:?}", data.category_list );

	/*
	for i in 0 .. ( data.atom_site_data ).len() {
		println!( "{:?}", ( data.atom_site_data )[ i ] );
	}
	*/

	/* Read 'atom_site' Data Category and set it's order. */
	data.define_category_order();
	//println!( "{:?}", data.category_order );

	/* Get 'atom_site' data category information. */
	data.set_data_vector();

	/* Remove 'label_alt_id'. */
	data.remove_alt_id();

	/* Remove HETATM record line. */
	data.remove_hetatm();

	/*
	print!( "group_PDB\t"     );
	print!( "label_atom_id\t" );
	print!( "label_alt_id\t"  );
	print!( "label_comp_id\t" );
	print!( "label_asym_id\t" );
	print!( "auth_comp_id\t"  );
	print!( "auth_asym_id\t"  );
	print!( "auth_atom_id"    );
	print!( "\n" );
	for i in 0 .. ( data.group_pdb ).len() {
		print!( "{}\t", ( data.group_pdb )[ i ]     );
		print!( "{}\t", ( data.label_atom_id )[ i ] );
		print!( "{}\t", ( data.label_alt_id )[ i ]  );
		print!( "{}\t", ( data.label_comp_id )[ i ] );
		print!( "{}\t", ( data.label_asym_id )[ i ] );
		print!( "{}\t", ( data.auth_comp_id )[ i ]  );
		print!( "{}\t", ( data.auth_asym_id )[ i ]  );
		print!( "{}"  , ( data.auth_atom_id )[ i ]  );
		print!( "\n" );
	}
	*/

	/* Get residue sequence information from mmCIF format. */
	let res_list_3 : Vec<String> = data.get_fasta_from_cif( &( opts.author ), &( opts.chainid ) );
	//println!( "{:?}", res_list_3 );

	/* Convert three letter into one letter. */
	let res_list_1 : Vec<char> = convert::convert_residue( &res_list_3 );

	/*
	for i in 1 .. ( res_list_1.len() + 1 ) {
		if i % 60 == 0 {
			print!( "{}", res_list_1[ i - 1 ] );
			print!( "\n" );
		} else {
			print!( "{}", res_list_1[ i - 1 ] );
		}
	}
	print!( "\n" );
	*/

	/* Show result. */
	result::show_result(
		&res_list_1,
		&( opts.title ),
		&( opts.output )
	);

	/* Save result. */
	result::save_result(
		&res_list_1,
		&( opts.title ),
		&( opts.output )
	);

	println!( "Program completed !!!" );

}
