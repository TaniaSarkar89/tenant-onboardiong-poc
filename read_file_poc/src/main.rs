//use std::fs::File;
use std::io::Read;
//use std::io::{BufRead, BufReader};
extern crate yaml_rust;
use yaml_rust::{YamlLoader};
//use yaml_rust::{YamlLoader, YamlEmitter};

fn main(){

    let mut file = std::fs::File::open("config.yaml").unwrap();
    let mut contents = String::new();
    
    file.read_to_string(&mut contents).unwrap();
    let docs = YamlLoader::load_from_str(&contents).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];

    // Debug support
    //println!("{:?}", doc);

    // Index access for map & array
    println!("{:?}", doc["tenantName"].as_str().unwrap());
    println!("{:?}", doc["prettyTenantName"].as_str().unwrap());
    println!("{:?}", doc["github"]["repoName"].as_str().unwrap());
    println!("{:?}", doc["github"]["teamName"].as_str().unwrap());
    println!("{:?}", doc["studio"]["active"].as_bool().unwrap());
    println!("{:?}", doc["studio"]["beta"].as_bool().unwrap());
    println!("{:?}", doc["studio"]["internal"].as_bool().unwrap());
    println!("{:?}", doc["studio"]["solutions"]["merchants"].as_bool().unwrap());
    println!("{:?}", doc["studio"]["solutions"]["financialInstitutions"].as_bool().unwrap());
    println!("{:?}", doc["studio"]["runbox"]["sandbox"].as_bool().unwrap());
    println!("{:?}", doc["studio"]["runbox"]["sandboxType"].as_str().unwrap());
    

    // Chained key/array access is checked and won't panic,
    // return BadValue if they are not exist.
    assert!(doc["INVALID_KEY"][100].is_badvalue());

    // print the values in a MDB script
    // let db_start_string =
    // "
    // db.tenants.insertOne({ 
    //     title: '<tenantName>',
    //     name: '<prettyTenantName>'
    //     github: '<github>'
    // });
    // ";
    // let db_end_string = "";
}


