use std::collections::HashMap;

struct Name(u32);

struct File {
    pub structs: Vec<Struct>,
}

struct Struct {
    pub name: Name,
    pub fields: Vec<Field>,
}

struct Field {
    pub name: Name,
    pub ty: Name,
}
