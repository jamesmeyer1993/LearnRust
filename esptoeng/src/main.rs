enum VerbType {
    PreteritReg,
    PreteritSpell,
    PreteritStem,
    PreteritIrreg,
    Imperfect_Ar,
    Imperfect_Er_Ir,
    Imperfect_Ir,
    Imperfect_Ser,
    Imperfect_Ver,
}

struct VerbChange {
    verbtype: u32,
    perspective: String,
}

struct VerbChanges {
    reg: Vec<String>,
    spell: Vec<String>,
    stem: Vec<String>,
    irreg: Vec<String>,
}

fn main()
{
    println!("Hello,world!..Hola, mundo!");
}
