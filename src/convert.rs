use std::path::Path;
use subprocess::Exec;

pub fn check_dep() {
    let file_npm = Path::new("/usr/bin/npm");

    if file_npm.exists() {
        println!("npm exist")
    } else {
        Exec::shell("sudo pacman -S npm --nocofirm").join().unwrap();
    }

    let file_node = Path::new("/usr/bin/node");

    if file_node.exists() {
        println!("nodejs exist")
    } else {
        Exec::shell("sudo pacman -S nodejs --nocofirm")
            .join()
            .unwrap();
    }
}
pub fn po2csv(p2c_input_po: String, p2c_output_csv: String) {
    let convert = format!("node index.js {} > {}", p2c_input_po, p2c_output_csv);
    println!("po2csv : {}", convert);
    //-----------------(use index.js)---------------(Folder's po)------------(where to convert into)
    Exec::shell(convert).join().unwrap();
}
pub fn csv2po(c2p_input_po: String, c2p_output_csv: String, c2p_output_po: String) {
    let convert = format!(
        "node index.js {} {} > {}",
        c2p_input_po, c2p_output_csv, c2p_output_po
    );
    println!("csv2po : {}", convert);
    //-----------------(use index.js)---------------(Folder's po)------------(where to convert into)
    Exec::shell(convert).join().unwrap();
}
