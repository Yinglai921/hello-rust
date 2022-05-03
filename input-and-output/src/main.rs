mod read_cmd_args;
mod read_file;
mod write_file;

fn main() {
    /*
     * Read args from command line
     * run: cargo run Moon 1990 --all
     */
    read_cmd_args::read_input();
    let arg_2 = read_cmd_args::read_nth_input(2);
    assert_eq!(arg_2, "1990");

    /*
     * Read and write local files
     */
    // read_file::read_fika();
    // write_file::write_fika();
}
