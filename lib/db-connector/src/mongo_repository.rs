use mongodb::{
    bson::Document,
    error::Error as MongoError,
    options::{DeleteOptions, FindOneOptions, FindOptions, InsertManyOptions, InsertOneOptions},
    results::{InsertManyResult, InsertOneResult},
    Collection, Database,
};
use std::fmt::Debug;

pub struct MongoRepository<T> {
    collection: Collection<T>,
}

impl<T> MongoRepository<T> {
    pub fn new(db: &Database, collection_name: &str) -> Self {
        let collection = db.collection(collection_name);
        MongoRepository { collection }
    }

    pub async fn insert(
        &self,
        doc: &T,
        options: Option<InsertOneOptions>,
    ) -> Result<InsertOneResult, MongoError>
    where
        T: serde::Serialize,
    {
        match self.collection.insert_one(doc, options).await {
            Ok(result) => {
                log::info!("Document inserted successfully: {:?}", result);
                Ok(result)
            }
            Err(err) => {
                log::error!("Failed to insert document: {}", err);
                Err(err.into())
            }
        }
    }

    pub async fn insert_many(
        &self,
        docs: &Vec<T>,
        options: Option<InsertManyOptions>,
    ) -> Result<InsertManyResult, MongoError>
    where
        T: serde::Serialize,
    {
        match self.collection.insert_many(docs, options).await {
            Ok(result) => {
                log::info!("Documents inserted successfully: {:?}", result);
                Ok(result)
            }
            Err(err) => {
                log::error!("Failed to insert documents: {}", err);
                Err(err.into())
            }
        }
    }

    pub async fn find_one(
        &self,
        filter: Document,
        options: Option<FindOneOptions>,
    ) -> Result<Option<T>, MongoError>
    where
        T: serde::de::DeserializeOwned + Debug + Unpin + Send + Sync,
    {
        match self.collection.find_one(filter, options).await? {
            Some(document) => {
                log::info!("Document found: {:?}", document);
                Ok(Some(document))
            }
            None => {
                log::info!("Document not found");
                Ok(None)
            }
        }
    }

    pub async fn find(
        &self,
        filter: Document,
        options: Option<FindOptions>,
    ) -> Result<Vec<T>, MongoError>
    where
        T: serde::de::DeserializeOwned + Debug + Unpin + Send + Sync,
    {
        let mut cursor = self.collection.find(filter, options).await?;
        let mut result = vec![];

        while cursor.advance().await? {
            let document = cursor.deserialize_current()?;
            log::info!("Document found: {:?}", document);
            result.push(document);
        }

        Ok(result)
    }

    pub async fn delete_many(
        &self,
        query: Document,
        options: Option<DeleteOptions>,
    ) -> Result<(), MongoError> {
        let result = self.collection.delete_many(query, options).await?;
        log::info!("Deleted {} documents", result.deleted_count);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use mongodb::{bson::doc, Database};
    use serde::{Deserialize, Serialize};

    use crate::mongo::connect;

    use super::MongoRepository;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
    struct TestDocument {
        name: String,
        age: i32,
    }

    fn create_test_repository(
        db: Database,
        collection_name: &str,
    ) -> MongoRepository<TestDocument> {
        // Create a new instance of the MongoDB repository
        MongoRepository::new(&db, collection_name)
    }

    async fn setup() -> mongodb::Database {
        let client = connect().await.unwrap();
        let db = client.database("test_db");
        let collection = db.collection::<TestDocument>("test_collection");
        collection.drop(None).await.unwrap(); // Drop the collection to start fresh for each test
        db
    }

    #[tokio::test]
    async fn test_insert_and_find_one() {
        let db = setup().await;
        let repo = create_test_repository(db, "test_db");

        // Insert a test document
        let test_doc = TestDocument {
            name: "John".to_string(),
            age: 30,
        };

        repo.insert(&test_doc, None).await.unwrap();

        // Find the inserted document
        let found_doc = repo
            .find_one(doc! { "name": "John" }, None)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(found_doc, test_doc);
    }

    #[tokio::test]
    async fn test_find_multiple_documents() {
        let db = setup().await;
        let repo = create_test_repository(db, "test_db");

        // Insert test documents
        let test_docs = vec![
            TestDocument {
                name: "John".to_string(),
                age: 30,
            },
            TestDocument {
                name: "Alice".to_string(),
                age: 28,
            },
            TestDocument {
                name: "Jane".to_string(),
                age: 25,
            },
        ];
        for doc in &test_docs {
            repo.insert(doc, None).await.unwrap();
        }

        // Find all documents with age greater than 26
        let filter = doc! { "age": { "$gt": 26 } };
        let found_docs = repo.find(filter, None).await.unwrap();

        // Convert vectors to HashSet for easy comparison
        let found_set: HashSet<_> = found_docs.iter().collect();
        let expected_set: HashSet<_> = test_docs[0..2].iter().collect();

        assert_eq!(found_set, expected_set);
    }
}
