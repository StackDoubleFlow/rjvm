use crate::class::Class;
use std::collections::HashMap;

// How the openjdk does it:
// http://openjdk.java.net/groups/hotspot/docs/StorageManagement.html

struct Storage {
    method_area: MethodArea,
    heap: Heap,
}

struct MethodArea {
    classes: HashMap<String, Class>,
}

impl MethodArea {
    fn new() -> MethodArea {
        MethodArea {
            classes: HashMap::new(),
        }
    }
}

struct Heap {}

impl Heap {}
