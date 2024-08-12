#![allow(dead_code)]

use std::{sync::Arc, collections::{HashMap, HashSet}, cell::RefCell};

pub enum  Variable {
    Bounded(usize), 
    Unbounded()
}


pub enum DataCellX {
    // <REF, k>
    Var(Variable),
    // <STR, k>
    StructIdx(usize),
    // Name, Arity
    StructDef(String, usize),
}

pub type DataCell = Arc<DataCellX>;


pub struct RegisterX {
    num: usize, 
    data: Option<DataCell>
}

impl RegisterX {
    fn load(&self) -> Option<DataCell> {
        self.data.clone()
    }

    fn store(&mut self, val: DataCell) {
        self.data = Some(val);
    }
}

pub type Register = Arc< RefCell<RegisterX>>;


pub struct FlatQuery {
   name: String,
   args: Vec<Register>
}

impl FlatQuery {
    fn size(&self) -> usize {
        1
    }
}



pub struct Machine {
    cells: Vec<DataCell>,
    variables: HashMap<Arc<String>, Register>,
    registers: HashSet<Register>, 
    register_num: usize, 
    h: usize
}

impl Machine {
    fn fresh_reg(&mut self) -> Register {
        let r: Arc<RefCell<RegisterX>> = RefCell::new(RegisterX{num: self.register_num, data: None.into()}).into();
        self.register_num += 1;
        r
    }



    fn put_structure(&mut self, q: FlatQuery, reg: Register) {
        // put structure
        // (STR, H + 1)
        let struct_idx = DataCellX::StructIdx(self.h + 1).into();
        // HEAP[H] = f/n
        self.cells[self.h] = struct_idx;
        
        //Xi <- HEAP[h]
        let reg_val = self.cells[self.h].clone();
        reg.borrow_mut().store(reg_val);

        // H <- H + 2
        self.h = self.h + 2;
    }

    fn set_variable(&mut self, reg: Register) {
        // HEAP[H] <- (REF, H)
        let var = DataCellX::Var(Variable::Unbounded()).into();
        self.cells[self.h] = var;

        // Xi <- HEAP[h];
        let var = self.cells[self.h].clone();
        reg.borrow_mut().store(var);
        self.h += 1;
    }

    fn set_value(&mut self, reg: Register) {
        // HEAP[H] <- Xi; // correctness condition lies on caller
        let data = reg.borrow().load().unwrap();
        self.cells[self.h] = data;
        // H <- H + 1;
        self.h += 1;
    }

    fn get_structure() {}

    fn unify_variable() {}

    fn unify_value() {}

    fn deref(&mut self, addr: usize) -> usize {
        let cell = self.cells[addr].clone();
        match &*cell {
            DataCellX::Var(_) => {},
            DataCellX::StructIdx(_) => {},
            DataCellX::StructDef(_, _) => {}
        };

        let mut addr = addr;

        loop {
            let cell = self.cells[addr].clone(); 
            let var = match &*cell {
                DataCellX::Var(var) => var,
                _ => return addr
            };
            match var {
                Variable::Bounded(a) => {
                    addr = *a;
                },
                Variable::Unbounded() => return addr
            };
        }
    }


    fn compile_query() {}
}

fn main() {
    println!("Hello, world!");

}
