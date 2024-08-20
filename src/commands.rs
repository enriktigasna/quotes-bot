use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::{Context, Error};
use crate::models::{NewPerson, Person, Quote};

#[poise::command(slash_command, prefix_command)]
pub async fn quote(
    ctx: Context<'_>,
    #[description = "Who said it?"] quotee: String,
    #[description = "What did they say?"] quote: String,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    use crate::schema::people::dsl::people;
    use crate::schema::people::{name, id};
    use crate::schema::quotes;
    use crate::models::NewQuote;

    // Person to append the quote to
    let person = people
        .filter(name.eq(&quotee))
        .select(id)
        .first::<i32>(&mut conn)?;

    let new_quote = NewQuote {person_id: person, content: &quote};

    diesel::insert_into(quotes::table)
        .values(&new_quote)
        .execute(&mut conn)?;


    let response = format!("{}\"\n \\- {}", quote, quotee);
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn new_person(
    ctx: Context<'_>,
    #[description = "What is the name of the new person?"] person: String,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let new_person = NewPerson { name: &person };

    use crate::schema::people;
    diesel::insert_into(people::table)
        .values(&new_person)
        .execute(&mut conn)?;

    let response = format!("Created new person {}", person);
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn people(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    use crate::schema::people::dsl::people;
    let results: Vec<String> = people
        .load::<Person>(&mut conn)?
        .iter().map(|elem| format!("{} (ID: {})", elem.name, elem.id))
        .collect();

    let response = format!("All people: \n{}", results.join("\n"));

    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn get_quotes(
    ctx: Context<'_>,
    #[description = "Who said it?"] quotee: String,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    use crate::schema::people::name;
    use crate::schema::quotes::person_id;
    use crate::schema::quotes::dsl::quotes;
    use crate::schema::people::dsl::people;

    let person = people
        .filter(name.eq(&quotee))
        .first::<Person>(&mut conn)?;

    let found_quotes: Vec<String> = quotes
        .filter(person_id.eq(person.id))
        .load::<Quote>(&mut conn)?
        .iter()
        .map(|elem| format!("\"{}\" - {}", elem.content, person.name))
        .collect();

    let mut response = found_quotes.join("\n");


    if response.is_empty() {
        response.push_str("No quotes found attributed to this user.")
    }
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn get_quotes_id(
    ctx: Context<'_>,
    #[description = "Who said it?"] quotee: String,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    use crate::schema::people::name;
    use crate::schema::quotes::person_id;
    use crate::schema::quotes::dsl::quotes;
    use crate::schema::people::dsl::people;

    let person = people
        .filter(name.eq(&quotee))
        .first::<Person>(&mut conn)?;

    let found_quotes: Vec<String> = quotes
        .filter(person_id.eq(person.id))
        .load::<Quote>(&mut conn)?
        .iter()
        .map(|elem| format!("\"{}\" - {} (ID: {})", elem.content, person.name, elem.id))
        .collect();

    let mut response = found_quotes.join("\n");


    if response.is_empty() {
        response.push_str("No quotes found attributed to this user.")
    }
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn all_quotes(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    use crate::schema::people;
    use crate::schema::quotes;

    let all_quotes: Vec<String> = quotes::table
        .inner_join(people::table)
        .select((quotes::id, quotes::content, people::name))
        .load::<(i32, String, String)>(&mut conn)?
        .into_iter()
        .map(|(quote_id, quote_content, quote_name)| format!("{}. \"{}\" - {}", quote_id, quote_content, quote_name))
        .collect();

    let mut response = all_quotes.join("\n");


    if response.is_empty() {
        response.push_str("No quotes found attributed to this user.")
    }
    ctx.say(response).await?;
    Ok(())
}


#[poise::command(prefix_command)]
pub async fn drop_quote(
    ctx: Context<'_>,
    #[description = "ID of the quote to delete"] quote_id: i32,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    use crate::schema::quotes::dsl::quotes;
    use crate::schema::quotes::id;

    let num_deleted = diesel::delete(quotes.filter(id.eq(quote_id)))
        .execute(&mut conn)?;

    let response = if num_deleted == 0 {
        format!("No quote found with ID: {}", quote_id)
    } else {
        format!("Deleted {} quote(s)", num_deleted)
    };

    ctx.say(response).await?;
    Ok(())
}