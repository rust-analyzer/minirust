use crate::Token;

pub struct File;
impl File {
    pub fn strukt(&mut self) -> Struct { todo!() }
}

pub struct Struct;
impl Struct {
    pub fn struct_token(&mut self, _token: Option<Token>) { todo!() }
    pub fn name(&mut self, _token: Option<Token>) { todo!() }
    pub fn l_curly_token(&mut self, _token: Option<Token>) { todo!() }
    pub fn field(&mut self) -> Field { todo!() }
    pub fn r_curly_token(&mut self, _token: Option<Token>) { todo!() }
}

pub struct Field;
impl Field {
    pub fn name(&mut self, _token: Option<Token>) { todo!() }
    pub fn colon_token(&mut self, _token: Option<Token>) { todo!() }
    pub fn ty(&mut self, _token: Option<Token>) { todo!() }
    pub fn comma_token(&mut self, _token: Option<Token>) { todo!() }
}

