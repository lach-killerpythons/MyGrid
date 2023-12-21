// create a matrix Vec<Vec<T>> of generic type T
// allow these MyCells to be colored when they print
// create, read, update, delete
#![allow(warnings)]

use colored::{Colorize, ColoredString};
use core::sync::atomic::Ordering;
use std::vec;
use std::{io, collections::HashSet, sync::atomic::AtomicUsize, fmt::Debug};
use std::fmt::Error;

#[derive(PartialEq, Debug, Clone)]
enum HolderType{
    A,
    B,
    Inserted,
    Empty,
    Updated,
    Deleted,
}
#[derive(Debug, PartialEq, Clone)]
struct Holder{
    myType: HolderType,
    index: u32,
}
impl Holder{
    fn new() -> Holder{
        Holder{
            myType: HolderType::Empty,
            index: 0,
        }
    }
    fn print(&self) -> String {
        let output = format!("this is a {:?} Holder",&self.myType);
        output
    }
    fn update(&mut self) {
        self.myType = HolderType::Updated;
    }

    fn copy(&mut self) -> Holder {
        Holder {
            myType: self.myType.clone(),
            index: self.index,
        }
    }
 
}

trait Inserted{
    fn new(n:u32) -> Self;
    //fn print() -> ();
}

impl Inserted for Holder{
    fn new(n: u32) -> Holder{
        Holder { 
            myType: HolderType::Inserted,
            index: n,
        }
    }
}

trait Updated{}

trait A{
    fn new() -> Self;
    fn printA() -> &'static str;
}
impl A for Holder{
    fn new() -> Holder{
        Holder{
            myType: HolderType::A,
            index: 0,
        }
    }
    fn printA() -> &'static str {
        let output = "this is an A_Holder";
        output
    }
}
trait B {
    fn new() -> Self;
    fn printB() -> &'static str;
}
impl B for Holder{
    fn new() -> Holder{
        Holder{
            myType: HolderType::B,
            index: 0,
        }
    }
    fn printB() -> &'static str {
        let output = "this is a B_Holder";
        output
    }
}

static COUNTER: AtomicUsize = AtomicUsize::new(1);
pub fn get_id() -> usize { COUNTER.fetch_add(1, Ordering::Relaxed) }

pub fn box_grid(x: u32, y: u32) -> Vec<Vec<String>> {
    let mut vertical: Vec<String> = Vec::new();
    let mut grid: Vec<Vec<String>> = Vec::new();
    for i in 1..=x {
        vertical.push("[ ]".to_string());
    }
    for i in 1..=y {
        grid.push(vertical.clone());
    }
    grid


}

pub fn cell_grid(x: u32, y: u32) -> Vec<Vec<MyCell>> {
    let mut vertical = Vec::new();
    let mut grid = Vec::new(); //Vec<Vec<MyCell<T>>>
/*     for i in 1..=x {
        let new_cell = MyCell::new();
        vertical.push(new_cell);
    }
    for i in 1..=y {
        grid.push(vertical.clone());
    } */
    for i in 1..=y {
        for i in 1..=x {
            let new_cell = MyCell::new();
            vertical.push(new_cell);
        }
        grid.push(vertical);
        vertical = vec![];
    }
    grid

}

pub fn short_string(input: String, n: i32) -> String{
    let mut output: String = "".to_string();
    let mut count : i32 = 0;
    for c in input.chars() {
            output.push(c);
            count += 1;
            if count == n {break}   
    }
    output 

}

#[derive(Debug)]
//pub struct MyGrid<T> {
pub struct MyGrid {
    mycell_grid :Vec<Vec<MyCell>>,
    //holder_vec : Vec<Box<T>>, // generic type T is set on intialization -> cannot iterate unknown structs
    holder_vec : Vec<Holder>,
    max_x: u32,
    max_y: u32,
}
//impl<T> MyGrid<T> {
impl MyGrid {

    //fn new(x:u32,y:u32) -> MyGrid<T> {
    fn new(x:u32,y:u32) -> MyGrid {
        //MyGrid::<T> {
        MyGrid {
            mycell_grid : cell_grid(x, y),
            holder_vec: vec![],
            max_x : x,
            max_y : y,
        }

    }
/* 
    fn add_object(&mut self, t: T) {
        self.holder_vec.push(Box::new(t));
    } */


    fn add_object(&mut self, myHolder: Holder) {
        self.holder_vec.push(myHolder);
    }

    fn get_object_n(&mut self, n:u32) -> Result<&mut Holder, Error>{
        for h in self.holder_vec.iter_mut() {
            if &h.index == &n {
                return Ok(h)
            }
        }
        return Err(Error)
        /* for &mut h in self.holder_vec {
            if h.index == n {
                return Ok(h)
            }
        }
        Err(Error) */
    }

    // use this to return a flattened vec
    fn print_cells(self) -> u32{
        let mut count: u32 = 0;
        for mycell_line in self.mycell_grid {
            for mycell in mycell_line {
                count+=1;
                println!("{:?}",mycell);
            }
        }
        count
    }

    fn read_holder_n(self, n: u32) -> Result<Holder, Error> {
        for object in self.holder_vec {
            if object.index == n {
                return Ok(object)
            } 
        }
        return Err(Error)
    }

    fn read_cell_n<'a>(&'a self, n: u32) -> Result<&'a MyCell, Error> {
        for mycell in self.mycell_grid.iter().flatten() {
            if mycell.index == n {
                return Ok(&mycell);
                
            }
        }
        return Err(Error)
    }

    fn n_READ(&self, n:u32) -> String {
        let result = &self.read_cell_n(n);
        let mut output: String = "".to_string();
        match result {
            Ok(c) => {
                let r = *c;
                output = format!("Index({}) = '{}'",r.index, r.val).to_string(); 
            }
            Err(e) => {
                output = format!("n_Read failed! {}", e)
            }
        }
        output
    }

    fn n_READ_val(&self, n:u32) -> String {
        let result = &self.read_cell_n(n);
        let mut output: String = "".to_string();
        match result {
            Ok(c) => {
                let r = *c;
                output = format!("{}", r.val).to_string(); 
            }
            Err(e) => {
                output = format!("n_Read failed! {}", e)
            }
        }
        output
    }

    fn n_MOVE(&mut self, n_current: u32, n_new: u32)
    {
        
        if n_current != n_new {
            let result = self.get_object_n(n_current);
            match result {
                Ok(h) => {
                    h.index = n_new ;
                    let contents = self.n_READ_val(n_current);
                    self.n_UPDATE(n_new, contents.as_str());
                    self.n_UPDATE(n_current, "")

                },
                Err(e) => println!("{}",e),

            }
        }
    }

    fn n_UPDATE(&mut self, n: u32, input:&str) {
        let input = input.to_string();
        match self.update_n_result(n, input) {
            Ok(_) => {
                println!("UPDATE {} SUCCESS!", n);
                let checkExists = self.get_object_n(n);
                match checkExists {
                    Ok(h) => h.update(), // update current + previous value
                    Err(e) => {
                        let newHolder: Holder = Inserted::new(n);
                        self.add_object(newHolder);
                    }
                }

                
            }
            Err(E) => println!("UPDATE {} ERROR - {}", n, E),
            _ => panic!(),
        }
    }

    fn update_n_result(&mut self, n: u32, input: String) -> Result<&mut MyCell, Error> {
        let i = input.clone();
        for mycell in self.mycell_grid.iter_mut().flatten() {
            if mycell.index == n {
                mycell.update_val(input);
                if i == "".to_string() {
                    mycell.contains_val = false;
                }
                return Ok(mycell);
            }
        }
        return Err(Error);
    }

    // remove holders from holder_vec where myType = Deleted
    fn remove_deleted(&mut self) {
        //let holder = h;
        let mut v2: Vec<Holder> = vec![];
        let mut ii = 0;
        for i in self.holder_vec.iter_mut() {
            if i.myType != HolderType::Deleted {
                ii = i.index;
                let p = i.copy();
                v2.push(p);
            }
        }
        self.holder_vec = v2;

    }

    // do these as separate steps to keep the borrow checker happy
    pub fn n_DELETE(&mut self, n: u32) {
        self.n_UPDATE(n, ""); // update the cell value first
        self.delete_holder_n(n); // check n exists update myType 
        self.remove_deleted(); // delete from the holder_vec
    }

    fn delete_holder_n(&mut self, n:u32) {
        let r = self.get_object_n(n);
        //let d: &mut Holder;
        let mut outcome = false;
        match r { // Holder is in holder_vec -> delete!
            Ok(m) => {
                
                //println!("successfully deleted - {:?}", &m); 
                //self.remove_holder(m);
                m.myType = HolderType::Deleted;
                outcome = true;
                

            }
            Err(E) => {
                println!("could not find index:{} -- Error code {}",n, E);
            }
        }


    }

    // fix this for for array

    //fn insert_cell_n(&mut self, n:u32, input:String) -> bool {

    fn print_objects(&self){

        for p in &self.holder_vec {
            //let h = *p;
            match p.myType {
                HolderType::A => println!("{}",Holder::printA()),
                HolderType::B => println!("{}",Holder::printB()),
                HolderType::Empty=> println!("{}",Holder::print(p)),
                _ => println!("{}",Holder::print(p)),

            }
        }
    }


    fn print_grid(&self) { // [XX] format
        let n = 0;
        for cell in &self.mycell_grid {
            //rows
            let mut v1 = vec![];
            for c in cell {
                if c.val == "" {
                    let mut f: String = "".to_string();
                    // minimum 2 digits
                    if c.index < 10 { 
                        f = format!("0{}", c.index); // make same size
                    }
                    else {
                        f = format!("{}", c.index);
                    }

                    v1.push(f);
                }
                else {
                    let c = short_string(c.val.clone(), 2);
                    v1.push(c); 
                }
            }
            let mut output_str: String = "".to_string();
            for s in v1 {
                //let c = s.green();
                let c: ColoredString;
                let second_char = s.chars().nth(1).unwrap();
                let num = second_char.is_digit(10);
                if num {
                    c = s.bright_yellow();
                }
                else {
                    match second_char {
                        'M' => {c = s.blue()}
                        'F' => {c = s.bright_magenta()}
                        _ => {c = s.green()}
                    }
                }


                let ss = format!("-[{}]", c);
                output_str.push_str(ss.as_str());
            }
            println!("{}",output_str);

        }
    }

/*     fn print_objects(self) {
        for i in self.holder_vec {
            let output = format!("{:?}",i);

        }
    } */
}

#[derive(Debug, PartialEq, Clone)]
pub struct MyCell {
    index: u32,
    contains_val: bool,
    val: String,
    //parent_grid: &'a Beach,
}


impl MyCell {
    fn new() -> MyCell {
        MyCell {
            index: get_id() as u32,
            contains_val: false,
            val : "".to_string(),        }
    }

    fn update_val(&mut self, input: String) {
        self.val = input;
        self.contains_val = true;
    }
    fn lazy_n_UPDATE(&mut self, n:u32, input: String) -> bool{
        if self.index == n {
            self.update_val(input);
            return true;
        }
        false
    } 

    
/*     fn print(&self){
        let output = format!("[{}]",&self.label);
        println!("{output}");
        println!("{:?}",&self.xyn);
    } */


}

#[test]
fn test_CRUD() {

    //CREATE
    let mut c1: MyGrid = MyGrid::new(2, 2); // do not edit this
    
    let s = c1.mycell_grid.iter().flatten().count();
    assert!(
        4 == s,
        "CREATE FAILED : something went wrong creating 2x2 grid - length is `{s}` not 4"
    );
    // READ
    let s = c1.n_READ(2);
    assert!(
        s == "Index(2) = ''".to_string(),
        "READ FAILED : returned non empty value {s}"
    );
    // UPDATE
    c1.n_UPDATE(2, "TEST");
    c1.n_UPDATE(3, "TEST2");
    let s = c1.n_READ(2);
    
    assert!{
        s == "Index(2) = 'TEST'".to_string(),
        "UPDATE FAILED: Index(2) != 'TEST'"
    };
    //DELETE
    c1.n_DELETE(2);
    let s = c1.n_READ(2);
    assert!(
        s == "Index(2) = ''".to_string(),
        "DELETE FAILED : Index(2) returned non empty value {s}"
    );
    // check holder_vec is correct
    let s = c1.holder_vec.iter().count();
    assert!(
        s == 1 as usize,
        "DELETE FAILED : holder_vec has the wrong length {s}"

    );

    c1.print_grid();
    
    //println!("All CRUD tests passed!");

    
}

fn gameLoop(target: &mut MyGrid) {
    let mut playing = true;
    println!("Welcome to MyGrid!");
    println!("valid commands : READ, MOVE, UPDATE, DELETE, PRINT, QUIT");

    while playing {
        let mut my_input = String::new();
        io::stdin()
            .read_line(&mut my_input)
            .expect("failed to read line");
        //println!("You my_inputed: {my_input}")
        let my_input: String = match my_input.trim().parse() {
            Ok(str) => str,
            _ => "INVALID".to_string() // correct answer cannot be zero, but continue 
        };
        let cut: Vec<&str> = my_input.split(' ').collect();
        let c1: String = cut[0].to_string();
        let mut c2: u32 = 0; // always u32 - all commands
        let mut c3: String = "".to_string(); // could be String or u32
        // parse string to u32 (can fail)
        if cut.len() > 1 { 
            let t: u32 = match cut[1].parse::<u32>() {
                Ok(i) => {
                    println!("P{}", i);
                    i
                },
                _ => 0,
            };
            c2 = t;
        }
        if cut.len() > 2 {
            c3.push_str(cut[2]);
        }
            
        match c1.as_str() {
            "PRINT" => {
                target.print_grid();
            }

            "READ" | "read" => {
                if c2 == 0 {
                    println!("READ syntax : READ 1")
                }
                else {
                    let read = target.n_READ(c2);
                    println!("READ({}):", c2);
                    println!("{read}");
                }

            }
            "DELETE" | "delete" => {
                if c2 == 0 {
                    println!("DELETE syntax : DELTE 1")
                }
                else {
                    println!("DELETE({}):", c2);
                    target.n_DELETE(c2);
                }
            }
            "MOVE" | "move" => {
                println!("moving");
                if c3 != "".to_string() {
                    let t: u32 = match c3.parse::<u32>() {
                        Ok(i) => i,
                        _ => 0,
                    };
                    if c2 != 0 && t != 0 {
                        target.n_MOVE(c2, t);
                        target.print_grid();
                    }
                }

            }
            "UPDATE" => {
                if c2 != 0 && c3 != "" {
                    println!("Updating({}) -> {}",c2, c3);
                    target.n_UPDATE(c2, c3.as_str());
                }
                else {
                    println!("UPDATE syntax : UPDATE 1 new_str")
                }
            }
            "EXIT" | "QUIT" => {
                println!("quitting");
                playing = false;
            }

            _ => {
                println!("invalid command {}", my_input);
                //false
            }        
        }
    
    }
}

// TODO - fix n commands to result
// TODO - fix MOVE - invalid ranges

fn main() {

    let mut c1: MyGrid = MyGrid::new(8, 8);
    c1.n_UPDATE(3, "_P");
    c1.n_UPDATE(20, "_M");
    c1.n_UPDATE(63, "_F");
    c1.n_UPDATE(64, "_F");
    c1.n_DELETE(64);
    c1.n_MOVE(20, 28);
    c1.print_grid();
    //println!("{:?}",c1);
    gameLoop(&mut c1);

}