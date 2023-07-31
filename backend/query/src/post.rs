use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uchat_domain::ids::{PostId, UserId};
use uuid::Uuid;

use crate::{schema, DieselError};

#[derive(Clone, Debug, DieselNewType, Serialize, Deserialize)]
pub struct Content(pub serde_json::Value);

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::posts)]
pub struct Post {
    pub id: PostId,
    pub user_id: UserId,
    pub content: Content,
    pub time_posted: DateTime<Utc>,
    pub direct_message_to: Option<UserId>,
    pub reply_to: Option<PostId>,
    pub created_at: DateTime<Utc>,
}

impl Post {
    pub fn new(
        posted_by: UserId,
        content: uchat_endpoint::post::Content,
        options: uchat_endpoint::post::NewPostOptions,
    ) -> Result<Self, serde_json::Error> {
        Ok(Self {
            id: Uuid::new_v4().into(),
            user_id: posted_by,
            content: Content(serde_json::to_value(content)?),
            time_posted: options.time_posted,
            direct_message_to: options.direct_message_to,
            reply_to: options.reply_to,
            created_at: Utc::now(),
        })
    }
}

pub fn new(conn: &mut PgConnection, post: Post) -> Result<PostId, DieselError> {
    conn.transaction::<PostId, DieselError, _>(|conn| {
        diesel::insert_into(schema::posts::table)
            .values(&post)
            .execute(conn)?;
        Ok(post.id)
    })
}

pub fn get(conn: &mut PgConnection, post_id: PostId) -> Result<Post, DieselError> {
    use crate::schema::posts::dsl::*;
    posts.filter(id.eq(post_id)).get_result(conn)
}

pub fn get_trending(conn: &mut PgConnection) -> Result<Vec<Post>, DieselError> {
    use crate::schema::posts::dsl::*;
    posts
        .filter(time_posted.lt(Utc::now()))
        .filter(direct_message_to.is_null())
        .order(time_posted.desc())
        .limit(30)
        .get_results(conn)
}

pub fn bookmark(
    conn: &mut PgConnection,
    user_id: UserId,
    post_id: PostId,
) -> Result<(), DieselError> {
    let uid = user_id;
    let pid = post_id;
    {
        use crate::schema::bookmarks::dsl::*;
        diesel::insert_into(bookmarks)
            .values((user_id.eq(uid), post_id.eq(pid)))
            .on_conflict((user_id, post_id))
            .do_nothing()
            .execute(conn)
            .map(|_| ())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeleteStatus {
    Deleted,
    NotFound,
}

pub fn delete_bookmark(
    conn: &mut PgConnection,
    user_id: UserId,
    post_id: PostId,
) -> Result<DeleteStatus, DieselError> {
    let uid = user_id;
    let pid = post_id;
    {
        use crate::schema::bookmarks::dsl::*;
        diesel::delete(bookmarks)
            .filter(user_id.eq(uid))
            .filter(post_id.eq(pid))
            .execute(conn)
            .map(|row_count| {
                if row_count > 0 {
                    DeleteStatus::Deleted
                } else {
                    DeleteStatus::NotFound
                }
            })
    }
}

pub fn get_bookmark(
    conn: &mut PgConnection,
    user_id: UserId,
    post_id: PostId,
) -> Result<bool, DieselError> {
    let uid = user_id;
    let pid = post_id;
    {
        use crate::schema::bookmarks::dsl::*;
        use diesel::dsl::count;
        bookmarks
            .filter(user_id.eq(uid))
            .filter(post_id.eq(pid))
            .select(count(post_id))
            .get_result(conn)
            .optional()
            .map(|n: Option<i64>| match n {
                Some(n) => n == 1,
                None => false,
            })
    }
}
