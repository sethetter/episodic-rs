// use diesel::prelude::*;

// use crate::schema::shows;

// #[derive(Serialize, Queryable)]
// pub struct Show {
//     pub id: i32,
//     pub name: String,
//     pub tmdb_id: i32,
//     pub user_id: i32,
// }

// #[derive(Insertable)]
// #[table_name = "shows"]
// pub struct NewShow {
//     pub name: String,
//     pub tmdb_id: i32,
//     pub user_id: i32,
// }

// pub fn create_show(
//     conn: &PgConnection,
//     user_id: i32,
//     tmdb_id: i32,
//     name: String,
// ) -> QueryResult<usize> {
//     diesel::insert_into(shows::table)
//         .values(NewShow {
//             user_id,
//             tmdb_id,
//             name,
//         })
//         .execute(conn)
// }

// pub fn shows_for_user(
//     conn: &PgConnection,
//     user_id: i32,
// ) -> QueryResult<Vec<Show>> {
//     shows::table.filter(
//         shows::columns::user_id.eq(user_id)
//     ).load::<Show>(conn)
// }
