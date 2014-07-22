use physfs::PhysFSContext;
use physfs::file;

#[test]
fn read_file_from_directory() {
    let con = match PhysFSContext::new() {
        Err(msg) => fail!(msg),
        Ok(con) => con
    };

    assert!(con.is_init());

    match con.mount(super::path_to_here.to_string(), "/test/".to_string(), true) {
        Err(msg) => fail!(msg),
        _ => {}
    }

    let file = match file::File::open(&con, "/test/directory/read.txt".to_string(), file::Read) {
        Ok(f) => f,
        Err(msg) => fail!(msg)
    };

    let mut bytes = [0u8, ..32];
    let buf = bytes.as_mut_slice();

    match file.read(buf, 1, 32) {
        Err(msg) => fail!(msg),
        _ => {}
    }

    let mut msg = String::new();
    for byte in buf.iter() {
        if *byte == 0 { break }
        msg.push_char(*byte as char);
    }

    assert!(msg.as_slice() == "Read from me.");
}