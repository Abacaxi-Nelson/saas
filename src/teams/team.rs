use diesel;
use diesel::prelude::*;
use crate::db::schema::teams;
use crate::db::schema::in_teams;

#[table_name = "in_teams"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct InTeam {
    pub id: Option<i32>,
    pub id_user: Option<i32>,
    pub id_team: Option<i32>
}

impl InTeam {
    pub fn add_user_to_team(inteam: InTeam, connection: &SqliteConnection) -> InTeam {
        diesel::insert_into(in_teams::table)
          .values(&inteam)
          .execute(connection)
          .expect("Error creating new in team");

          in_teams::table.order(in_teams::id.desc()).first(connection).unwrap()
    }

    pub fn remove_user_from_team(id: i32, connection: &SqliteConnection) -> bool {
        diesel::delete(in_teams::table.find(id)).execute(connection).is_ok()
    }

    pub fn get_users_from_team(id_team: i32,connection: &SqliteConnection) -> Vec<Option<i32>> {
        let users = in_teams::table
            .filter(in_teams::id_user.eq(id_team))
            .select(in_teams::id_user)
            .load::<Option<i32>>(connection)
            .unwrap()
        ;
        

        for user in users.iter() {
            println!("{}", user.unwrap());
        }

        users
    }
}

#[table_name = "teams"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct Team {
    pub id: Option<i32>,
    pub name: String
}

impl Team {
  pub fn create(team: Team, connection: &SqliteConnection) -> Team {
      diesel::insert_into(teams::table)
          .values(&team)
          .execute(connection)
          .expect("Error creating new team");

      teams::table.order(teams::id.desc()).first(connection).unwrap()
  }

  pub fn read(connection: &SqliteConnection) -> Vec<Team> {
      teams::table.order(teams::id.asc()).load::<Team>(connection).unwrap()
  }

  pub fn update(id: i32, team: Team, connection: &SqliteConnection) -> bool {
      diesel::update(teams::table.find(id)).set(&team).execute(connection).is_ok()
  }

  pub fn delete(id: i32, connection: &SqliteConnection) -> bool {
      diesel::delete(teams::table.find(id)).execute(connection).is_ok()
  }
}