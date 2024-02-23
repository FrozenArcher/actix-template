use crate::db::{AppDB, DBError};

#[actix_rt::test]
async fn test_db() {
    let db = AppDB::mock();
    let res = db.test_db().await;
    if let Err(e) = res {
        if let DBError::Unimplemented = e {
            // test success
        } else {
            panic!("Error should be `Unimplemented`, got {}", e)
        }
    } else {
        panic!("`db.test_db().await` should be `Err(Unimplemented)`, not `Ok()`.")
    }
}
