mod convert;
mod read_filename;
mod readwrite_translate;
//mod storing;
fn main() {
    let mut read_filename = vec![];
    read_filename::read_fname(&mut read_filename);

    for loop_file in read_filename.into_iter() {
        println!("loop and check file name for translate {:?}", loop_file);
        let input_po = format!("{}{}", "data/import_po/", loop_file); //import po file
        let output_csv = String::from("data/csv_cache/non_translate_csv/file.csv"); //export csv file
        let input_tran_csv = String::from("data/csv_cache/translated_csv/file.csv");
        let output_po = format!("{}{}", "data/export_po/", loop_file);
        println!("po file import name {}", input_po);
        convert::check_dep();
        convert::po2csv(input_po.clone(), output_csv);
        readwrite_translate::main();
        convert::csv2po(input_po, input_tran_csv, output_po);
    }
}
