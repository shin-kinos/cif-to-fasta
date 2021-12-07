# cif-to-fasta 

A Rust program that convert `mmCIF` format into `FASTA` format by using `atom_site` Data Category. 

## Description 

* It converts `mmCIF` to `FASTA`. 
* It uses `atom_site` Data Category. 

The detailed information for `atom_site` Data Category in `mmCIF` format ( https://mmcif.wwpdb.org/dictionaries/mmcif_pdbx_v50.dic/Categories/atom_site.html ).

## Implementation 

You can compile this program by using `Cargo`. 🦀📦 

[e.g.] 

```
% cd cif-to-fasta-main 
% cargo build --release
``` 

Then the object file is generated in `./target/release` directory. 

## Input file format 

`mmCIF` format. ⚠️ NOTE that `PDB` format is not supported. 

See some example input files in `demo` directory. 

## Usage 

Options: 

* `-i` : Input filename, REQUIRED. 
* `-o` : Output filename, REQUIRED. 
* `-c` : ChainID, REQUIRED. 
* `-a` : Whether it uses the Author ID information or not ( 'yes' or 'no', default 'yes' ). If 'yes', `auth_asym_id`, `auth_atom_id`, `auth_comp_id` and `auth_seq_id` are used for converting from `mmCIF` into `FASTA`. If 'no', `label_asym_id`, `label_atom_id`, `label_comp_id` and `label_seq_id` are used instead. 
* `-t` : Name of the title line of the output FASTA file.  

[e.g.] 

```
% cif-to-fasta -i 6y2e.cif -o output.fasta -c A -a no
```  

## Output file format 

FASTA format. 
