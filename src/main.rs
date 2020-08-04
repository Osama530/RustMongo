// use bson::doc;
// 	use mongodb::{options::ClientOptions, Client};
// 	use serde_json::{json, Value};
// 	//
// 	fn main() {
// 	    //connect with mongodb
// 	    let client= Client::with_uri_str("mongodb://localhost:27017/?readPreference=primary&appname=MongoDB%20Compass&ssl=false").expect("Failed to connect");
// 	    //make database and a collection
// 	    let db = client.database("test").collection("tblStudent");
// 	    // inserting a document
// 		let docs = doc! { "user_name": "John123" };
// 	    let data = db.insert_one(docs, None).unwrap();
// 	    println!("id is : {:#?}", data);
// 	    // find a document
// 	    let docs1 = doc! { "user_name": "John123" };
// 	    let data = db.find_one(docs1,None).unwrap();
// 	    match data {
// 	        Some(data) => {
// 	            let data: Value = json!(data);
// 	            println!("data is : {}", data);
// 	        }
// 	        None => println!("No record Found"),
// 	    }

// 	    //updating a doc
// 	    let filter = doc! { "user_name": "John123" };
// 	    let replacement = doc! { "user_name": "umer123" };
// 	    db.find_one_and_replace(filter,replacement,None);

// 	    //for confirming
// 	    let docs1 = doc! { "user_name": "umer123" };
// 	    let data = db.find_one(docs1,None).unwrap();
// 	    match data {
// 	        Some(data) => {
// 	            let data: Value = json!(data);
// 	            println!("data is : {}", data["user_name"]);
// 	        }
// 	        None => println!("No record Found"),
// 	    }

// 	    //deleting a document
// 	    let query = doc! { "user_name": "John123" };
// 	     db.delete_one(query,None);
// 	    //for confirming
// 	    let docs2 = doc! { "user_name": "John123" };
// 	    let data = db.find_one(docs2,None).unwrap();
// 	    match data {
// 	        Some(data) => {
// 	            let data: Value = json!(data);
// 	            println!("data is : {}", data["user_name"]);
// 	        }
// 	        None => println!("No record Found"),
// 	    }

// 	}

extern crate bson;
extern crate mongodb;
extern crate serde_json;

use bson::doc;
use mongodb::{options::ClientOptions, Client};
use serde_json::{json, Value};

fn main() {
    //makinfg a connecton wi mongodb
    let client = Client::with_uri_str(
        "mongodb://localhost:27017/?readPreference=primary&appname=MongoDB%20Compass&ssl=false",
    )
    .expect("enable to connect");

    //making a database and a collection
    let db = client.database("practice").collection("family_members");

    //creating a document
    let document = doc!(
        "name" : "osama",
        "age" : 27,
        "Hobbies" : ["prgramming", "reading", "football"]
    );

    //inserting a document in a collection
    db.insert_one(document, None).unwrap();
    println!("document inserted succesfully");

    //find and print the data in a terminal
    let filter = doc!( "name" : "osama");
    let result = db.find_one(filter, None).unwrap();

    ////printing as a string
    //println!("search result in string format :: {:?}", result);

    //printing as a json key value pair
    match result {
        Some(result) => {
            let result: Value = json!(result); //converting into a json object
            println!("In json format:: {}", result["name"]); //accecssing object by key(name)
        }
        None => println!("no record found"),
    };

    //updating the document
    let doc = doc!( "name": "osama");
    let replace_with = doc!( "name": "khizar" );
    db.find_one_and_replace(doc, replace_with, None);
    println!("replace succesfully");

    //confirming update
    let doc_updated = doc!("name" : "khizar");
    let update_result = db.find_one(doc_updated, None).unwrap();
    match update_result {
        Some(update_result) => {
            let result: Value = json!(update_result);
            println!("updated result: {}", update_result);
        }
        None => println!("no record found"),
    };

    //deleting a document
    let query = doc!("name" : "khizar");
    db.delete_one(query, None);
    println!("succesfully deleted ");

    //confirming
    let doc_deleted = doc!("name" : "khizar");
    let deleted_result = db.find_one(doc_deleted, None).unwrap();
    match deleted_result {
        Some(deleted_result) => {
            let result: Value = json!(deleted_result);
            println!("updated result: {}", deleted_result);
        }
        None => println!("no record found"),
    };
}
