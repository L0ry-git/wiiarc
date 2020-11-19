pub enum WiiFSObjectType {
    
    File(Vec<u8>), //when it's file, specify data stored in it
    Folder,
    FolderRoot,
}

pub struct WiiFSObject {
    pub children: Vec<WiiFSObject>,
    pub name: String,
    pub obj_type: WiiFSObjectType
}

#[allow(dead_code)]
impl WiiFSObject {
    
    pub fn name(&self) -> &str {self.name.as_str()}
    pub fn obj_type(&self) -> &WiiFSObjectType {&self.obj_type}

    pub fn children(&self) -> &Vec<WiiFSObject> {&self.children}

    pub fn is_root(&self) -> bool {
        match self.obj_type {
            WiiFSObjectType::FolderRoot => true,
            _ => false
        }
    }
    pub fn can_have_children(&self) -> bool {
        match self.obj_type {
            WiiFSObjectType::File(_) => false,
            _ => true
        }
    }

    pub fn push_child(&mut self, new_child: WiiFSObject) {
        if !self.can_have_children() || new_child.is_root() {
            return;
        }

        self.children.push(new_child);
    }

}

pub mod objs {

    use super::*;

    const ROOT: &str = "<root>";

    pub fn new_file(name: String, data: Vec<u8>) -> WiiFSObject {
        WiiFSObject {children: vec![], name, obj_type: WiiFSObjectType::File(data)}
    }
    #[allow(dead_code)]
    pub fn new_empty_file(name: String) -> WiiFSObject {new_file(name, vec![])}

    pub fn new_dir(name: String, is_root: bool, children: Vec<WiiFSObject>) -> WiiFSObject {
        let obj_type = if is_root {WiiFSObjectType::FolderRoot} else {WiiFSObjectType::Folder};
        WiiFSObject {children, name, obj_type}
    }
    pub fn new_empty_dir(name: String) -> WiiFSObject {new_dir(name, false, vec![])}

    pub fn new_root(children: Vec<WiiFSObject>) -> WiiFSObject {new_dir(String::from(ROOT), true, children)}
    pub fn new_empty_root() -> WiiFSObject {new_root(vec![])}

}