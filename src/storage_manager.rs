use crate::class::{BootstrapClassLoader, Class};
use std::collections::HashMap;

// How the openjdk does it:
// http://openjdk.java.net/groups/hotspot/docs/StorageManagement.html

struct Storage {
    method_area: MethodArea,
    heap: Heap,
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

    fn create_class(&mut self, name: String) {
        self.classes
            .insert(name.clone(), self.bootstrap_class_loader.load_class(name));
    }
}

struct Heap {}

impl Heap {}
