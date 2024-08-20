use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(table_name = crate::schema::people)]
pub struct Person {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::people)]
pub struct NewPerson<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(Person))]
#[diesel(table_name = crate::schema::quotes)]
pub struct Quote {
    pub id: i32,
    pub person_id: i32,
    pub content: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::quotes)]
pub struct NewQuote<'a> {
    pub person_id: i32,
    pub content: &'a str,
}