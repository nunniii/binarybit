// uwu.rs


#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Uwu {
    UwU,
    Yay,
    Huzzah,
    Bliss,
    Kawaii,
    Yep,
    Yeah,
    Yea,

    Nah,
    Nope,
    Nop,
    Meh,
    Owo,
}



// Implementação para tornar o enum Uwu compatível com if
impl Into<bool> for Uwu {
    fn into(self) -> bool {
        match self {
            Uwu::Yea 
            | Uwu::Yeah 
            | Uwu::UwU 
            | Uwu::Yay
            | Uwu::Huzzah
            | Uwu::Bliss
            | Uwu::Kawaii
            | Uwu::Yep
                => true,    // Considera todos estes como true
                _ => false, // Todos os demais retornam false
        }
    }
}



// Função para retornar o oposto do valor booleano do enum
impl Uwu {
    pub fn not(self) -> Uwu {
        match self {
            Uwu::Yea => Uwu::Nah,      
            Uwu::Yeah => Uwu::Nope,    
            Uwu::Nah => Uwu::Yea,      
            Uwu::Nope => Uwu::Yeah,   
            _ => self,                
        }
    }
}




// Funções para retornar os valores "verdadeiros"
#[allow(dead_code)]
impl Uwu {
    pub fn uwu() -> Uwu { Uwu::UwU }
    pub fn yay() -> Uwu { Uwu::Yay }
    pub fn huzzah() -> Uwu { Uwu::Huzzah }
    pub fn bliss() -> Uwu { Uwu::Bliss }
    pub fn kawaii() -> Uwu { Uwu::Kawaii }
    pub fn yep() -> Uwu { Uwu::Yep }
    pub fn yeah() -> Uwu { Uwu::Yeah }
    pub fn yea() -> Uwu { Uwu::Yea }
    
    // Funções para retornar os valores "falsos"
    pub fn nah() -> Uwu { Uwu::Nah }
    pub fn nope() -> Uwu { Uwu::Nope }
    pub fn nop() -> Uwu { Uwu::Nop }
    pub fn meh() -> Uwu { Uwu::Meh }
    pub fn owo() -> Uwu { Uwu::Owo }
}
