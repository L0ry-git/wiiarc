use std::fs;
use crate::wiifs;

pub mod read {

    pub struct ReadInfo {
        current_node: u32,
        string_table: Option<String>
    }
    
    #[allow(dead_code)]
    impl ReadInfo {
    
        pub fn new() -> ReadInfo {
            ReadInfo {
                current_node: 1, 
                string_table: None
            }
        }

        pub fn current_node(&self) -> u32 {self.current_node}
        pub fn increment_node(&mut self) {self.current_node += 1}
    
        pub fn string_table(&self) -> &Option<String> {&self.string_table}
        pub fn init_string_table(&mut self, string_table: String) {self.string_table = Some(string_table)}
    
    }

}

use read::*;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

#[allow(dead_code)]
const IO_ERR_MSG: &str = "Something went wrong while reading the archive (I/O Error).";
#[allow(dead_code)]
const POPULATE_ERR_MSG: &str = concat!( "Something went wronte while reading data from the archive: archive data has not been read, ", 
                                    "therefore its value is None. Call the read() method to solve this error.");   
#[allow(dead_code)]
const UTF8_ERR_MSG: &str = "Something went wrong while making an UTF-8 String from bytes (UTF8 Error).";
#[allow(dead_code)]
const FUNNY_ERR_MSG: &str = "God damn, are you serious? You managed to fuck it up. This is bad news, why not getting a social life instead of having a furry pfp and doing this crap all day.";

#[allow(dead_code)]
const WIIFS_FILE_ID: u32 = 0;
#[allow(dead_code)]
const WIIFS_DIR_ID: u32 = 1;

pub struct WiiArchive {
    filename: String,
    data: Option<Vec<u8>>,
    root: wiifs::WiiFSObject,
    read_info: ReadInfo
}

#[allow(dead_code)]
impl WiiArchive {

    pub fn new(filename: String) -> WiiArchive {
        WiiArchive {
            filename, 
            data: None, 
            root: wiifs::objs::new_empty_root(), 
            read_info: ReadInfo::new()}
    }

    pub fn read(&mut self) {
        self.data = Some(fs::read(self.filename.as_str()).expect(IO_ERR_MSG));
    }

    pub fn read_borrow(mut self) -> WiiArchive {
        self.read();
        self
    }

    pub fn populate_root(mut self) -> WiiArchive {
        let file_data = match self.data {
            Some(some_data) => some_data,
            None => {
                eprintln!("{}", POPULATE_ERR_MSG);
                return self;
            }
        };
        self.data = None;

        //cursor for reading the data
        let mut vec_read = Cursor::new(file_data);

        //Read fst
        vec_read.set_position(4);
        let fst_start = vec_read.read_u32::<BigEndian>().unwrap();
        let fst_size = vec_read.read_u32::<BigEndian>().unwrap();
        let save_pos = fst_start + 12;

        vec_read.set_position((save_pos - 4) as u64);
        let last_child = vec_read.read_u32::<BigEndian>().unwrap();

        //Read string table
        let str_table_start = (save_pos + ((last_child - 1) * 12)) as usize;
        let str_table_end = (save_pos + fst_size - 12) as usize;

        //Init string table
        self.read_info.init_string_table(String::from_utf8(vec_read.get_ref()
            [str_table_start..str_table_end].to_vec())
            .expect(UTF8_ERR_MSG));
            
        //Read root directory
        WiiArchive::read_dir(&mut self.root, &mut self.read_info, &mut vec_read, save_pos, last_child);
        
        self
    }

    fn read_dir(dir: &mut wiifs::WiiFSObject, info: &mut ReadInfo, vec_read: &mut Cursor<Vec<u8>>, 
            mut data_pointer: u32, last_child: u32) {        
        while info.current_node() < last_child {
            info.increment_node();

            //Read current file data
            vec_read.set_position(data_pointer as u64);
            let value = vec_read.read_u32::<BigEndian>().unwrap();
            let data_offset = vec_read.read_u32::<BigEndian>().unwrap() as usize;
            let size = vec_read.read_u32::<BigEndian>().unwrap() as usize;

            //get name
            let mut new_obj_name = String::new();
            for table_ch in info.string_table().as_ref()
                .unwrap().chars().into_iter().skip((value & 0xFFFFFF) as usize) {
                    if table_ch == '\0' {break}
                    new_obj_name.push(table_ch);
            }

            //istantiate new wiifs object
            let new_fs_obj = match value >> 24 {
                WIIFS_FILE_ID => {
                    let mut new_obj_data: Vec<u8> = vec![];

                    let mut idx = 0;
                    for b in vec_read.get_ref().iter().skip(data_offset) {
                        if idx >= size {break}

                        new_obj_data.push(*b);
                        idx += 1;
                    }

                    data_pointer += 12;
                    wiifs::objs::new_file(new_obj_name, new_obj_data)
                },
                WIIFS_DIR_ID => {
                    let mut filled_dir = wiifs::objs::new_empty_dir(new_obj_name);
                    WiiArchive::read_dir(&mut filled_dir, info, vec_read, data_pointer, size as u32);

                    filled_dir
                },
                err_id => {
                    panic!(format!("Error with FS ID: {}. {}", err_id, FUNNY_ERR_MSG));
                }
            };

            //finally push the object into the 
            dir.push_child(new_fs_obj);
        }

    }

    pub fn get_root(&self) -> &wiifs::WiiFSObject {&self.root}

}

