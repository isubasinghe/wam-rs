

pub enum  Variable {
    Bounded(usize), 
    Unbounded()
}


pub enum DataCell {
    // <REF, k>
    Var(String, Variable),
    // <STR, k>
    StructIdx(usize),
    // Name, Arity
    StructDef(String, usize),
}


pub struct Heap {
    cells: Vec<DataCell>
}

impl Heap {
    fn index(&self, idx: usize) -> &DataCell {
        &self.cells[idx] // maybe panic
    }
}

fn main() {
    println!("Hello, world!");

}
