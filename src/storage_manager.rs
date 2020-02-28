use crate::class::{BootstrapClassLoader, Class};
use std::collections::HashMap;

// How the openjdk does it:
// http://openjdk.java.net/groups/hotspot/docs/StorageManagement.html

pub struct Storage {
    method_area: MethodArea,
    heap: Heap,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            method_area: MethodArea::new(),
            heap: Heap::new(),
        }
    }

    pub fn create_class(&mut self, name: &str) {
        let class = self.method_area.bootstrap_class_loader.load_class(name);
        
        self.method_area.classes.insert(name.to_owned(), class);
        println!("{:#?}", self.method_area.classes);
    }

    pub fn resolve_class(&mut self, name: &str) -> &Class {
        if !self.method_area.classes.contains_key(name) {
            self.create_class(name);
        }
        self.method_area.classes.get(name).unwrap()
    }
}

struct MethodArea {
    bootstrap_class_loader: BootstrapClassLoader,
    classes: HashMap<String, Class>,
}

impl MethodArea {
    fn new() -> MethodArea {
        MethodArea {
            bootstrap_class_loader: BootstrapClassLoader::new(),
            classes: HashMap::new(),
        }
    }
}

struct Heap {}

impl Heap {
    fn new() -> Heap {
        Heap {}
    }
}
