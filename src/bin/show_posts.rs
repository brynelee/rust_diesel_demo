use self::models::*;
use self::db::*;
use diesel::prelude::*;
use test_diesel::*;


/* We can run our script with cargo run --bin show_posts. */

fn main(){

    /* The use self::schema::posts::dsl::* line imports a bunch of aliases
    so that we can say posts instead of posts::table, and published instead
    of posts::published. It’s useful when we’re only dealing with a single
    table, but that’s not always what we want. */
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("------------\n");
        println!("{}", post.body);
    }

}