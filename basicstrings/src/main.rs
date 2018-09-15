/*
struct Word {
    elem: String,
    occurs: u32,
}
*/
// Word methods
/*
impl Word {
    pub fn incOccurs(&self) {
        self.occurs = self.occurs + 1;
    }
    pub fn equals(&self, compare: String) -> bool {
        if self.elem.len() == compare.len()
            && self.elem.contains(compare) {
                return true;
        }else{
            return false;
        }
    }
}
*/
// Functions of Word
/*
pub fn buildWord(elem: String) -> Word {
    Word {
        elem: elem,
        occurs: 1,
    }
}
*/

fn main() {

    let mut word = String::from("Hello");
    println!("len = {}", word.len());
    println!("is_empty = {}", word.is_empty());
    println!("contains lo? {}\n\n", word.contains("lo"));

    word.push_str(". This is a string in rust.");
    word.push_str("I have a feeling it will be useful!");
    word.push_str("All it took was the right tutorial.");
    word.push_str("Now I'm on to the basics...");

    println!("len = {}", word.len());
    println!("is_empty = {}", word.is_empty());
    println!("contains lo? {}", word.contains("lo"));

    for token in word.split_whitespace(){
        println!("{}", token);
    }

    // replace
    {
        let s1 = String::from("Rust is fantastic!");
        println!("After replace: {}", s1.replace("is","IS"));
    }
    // lines
    {
        let s1 = String::from("It's\nstorming\noutside.");
        for line in s1.lines(){
            println!("{}", line);
        }
    }
    // split
    {
        let s1 = String::from("it_is_storming_out_side__");
        let tokens: Vec<&str> = s1.split("_").collect();

        println!("token[2]: {}", tokens[2]);
    }
    // trim
    {
        let s1 = String::from("   hello   my   name is   james\n\r");
        println!("Before trim: {}", s1);
        s1.trim();
        println!("After trim: {}", s1);
    }
    // chars
    {
        let s1 = String::from("Here are a bunch of chars");

        // get character at index
        match s1.chars().nth(4) {
            Some(c) => println!("Character at index 4: {}", c),
            None => println!("No character at index 4."),
        }
    }
}
