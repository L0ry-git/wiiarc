mod wiifs;
mod archive;

fn main() {

    let arc = archive::WiiArchive::new(String::from("res/ExampleArchive.arc"));
    let arc = arc.read_borrow().populate_root();

    for obj in arc.get_root().children() {
        let obj_type = if obj.is_root() {"root"} else if obj.can_have_children() {"dir"} else {"file"};
        println!("Name: {}, Type: {}", obj.name(),obj_type);
    }

}