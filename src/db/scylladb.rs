use scylla::{Session, SessionBuilder};
use std::fs;

pub struct ScyllaDbService {
    pub db_session: Session,
 }

impl ScyllaDbService {
    pub async fn new(host: String, schema_file: String) -> Self {

        let session: Session = SessionBuilder::new()
            .known_node(host.clone())
            .build()
            .await
            .expect("Error Connecting to ScyllaDB");
      
        let schema = fs::read_to_string(&schema_file)
        .expect(("Error Reading Schema File".to_owned() + schema_file.as_str()).as_str());

        let schema_query = schema.trim().replace("\n", "");

        for q in schema_query.split(";") {
            let query = q.to_owned() + ";";
            if query.len() > 1 {
                session.query(query, &[]).await.expect("Error creating schema!");
            }
        }

        ScyllaDbService{
            db_session: session,
        }
    }
}