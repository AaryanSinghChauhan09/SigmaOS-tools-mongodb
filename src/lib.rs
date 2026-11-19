#![forbid(unsafe_code)]
#![allow(non_snake_case)]
//! MongoDB Document Store Engine for SigmaOS
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct MongoDocument {
    pub id: String,
    pub fields: HashMap<String, String>,
}

pub struct MongoCollection {
    pub name: String,
    docs: HashMap<String, MongoDocument>,
}

impl MongoCollection {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            docs: HashMap::new(),
        }
    }

    pub fn insert_one(&mut self, doc: MongoDocument) -> Result<(), &'static str> {
        if self.docs.contains_key(&doc.id) {
            return Err("Duplicate key error: _id already exists");
        }
        self.docs.insert(doc.id.clone(), doc);
        Ok(())
    }

    pub fn find(&self, query_key: &str, query_val: &str) -> Vec<&MongoDocument> {
        self.docs
            .values()
            .filter(|doc| doc.fields.get(query_key).map(|v| v.as_str()) == Some(query_val))
            .collect()
    }

    pub fn delete_one(&mut self, id: &str) -> bool {
        self.docs.remove(id).is_some()
    }

    pub fn count(&self) -> usize {
        self.docs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mongo_collection_crud_operations_aaa() {
        // Arrange
        let mut col = MongoCollection::new("users");
        let mut f1 = HashMap::new();
        f1.insert("name".into(), "Alice".into());
        f1.insert("role".into(), "admin".into());
        let doc1 = MongoDocument {
            id: "u1".into(),
            fields: f1,
        };

        let mut f2 = HashMap::new();
        f2.insert("name".into(), "Bob".into());
        f2.insert("role".into(), "user".into());
        let doc2 = MongoDocument {
            id: "u2".into(),
            fields: f2,
        };

        // Act
        col.insert_one(doc1).unwrap();
        col.insert_one(doc2).unwrap();
        let admins = col.find("role", "admin");

        // Assert
        assert_eq!(col.count(), 2);
        assert_eq!(admins.len(), 1);
        assert_eq!(admins[0].fields.get("name").unwrap(), "Alice");

        // Act 2: delete
        let deleted = col.delete_one("u1");
        assert!(deleted);
        assert_eq!(col.count(), 1);
    }
}


#[cfg(test)]
mod mongodb_extra_tests {
    use super::*;

    #[test]
    fn test_mongodb_find_non_existent_key_aaa() {
        // Arrange
        let col = MongoCollection::new("logs");
        // Act
        let res = col.find("missing_key", "val");
        // Assert
        assert!(res.is_empty());
    }

    #[test]
    fn test_mongodb_delete_non_existent_returns_false_aaa() {
        // Arrange
        let mut col = MongoCollection::new("logs");
        // Act
        let deleted = col.delete_one("ghost_id");
        // Assert
        assert!(!deleted);
        assert_eq!(col.count(), 0);
    }

    #[test]
    fn test_mongodb_duplicate_insert_error_aaa() {
        // Arrange
        let mut col = MongoCollection::new("logs");
        let doc = MongoDocument { id: "1".into(), fields: HashMap::new() };
        col.insert_one(doc.clone()).unwrap();
        // Act
        let res = col.insert_one(doc);
        // Assert
        assert!(res.is_err());
    }
}
