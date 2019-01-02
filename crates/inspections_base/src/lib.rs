mod fixes;
mod problem;
mod inspection_impl;


struct InspectionRegistrar {

}

trait Inspection {
    fn register(holder: InspectionRegistrar);
}

struct A {

}

impl A {
    fn foo(&mut self) -> i32 { 32 }
}

fn main() {
    let a = A {};
    let c = A::foo;
}