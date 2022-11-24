use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MLData {
    pub nodes: Vec<Node>,
    pub tree: Vec<TreeNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Node {
    pub i: String,
    #[serde(default = "default_fnz_id")]
    fnz_id: String,
    pub a: HashMap<String, String>,
}

fn default_fnz_id() -> String {
    String::from("-1")
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TreeNode {
    pub i: String,
    pub c: Option<Vec<TreeNode>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MLDataContainer {
    element_statistics: MLData,
}

pub fn read_ml_json(path: &Path) -> MLDataContainer{

    let json_str = fs::read_to_string(path).unwrap();

    let mut deserializer = serde_json::Deserializer::from_str(&json_str);
    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);

    MLDataContainer::deserialize(deserializer).unwrap()
}

fn calc_val(v1: f32, v2:f32) -> Option<f32>{
    if v2 == 0.0{
        None
    }else {
        Some(v1/v2)
    }
}

fn sum_rate(v1: f32, v2:f32, val: f32) -> Option<f32>{
    let rate = calc_val(v1, v2)?;
    //match rate {
    //    Some(r) =>{
    //        Some(r + val)
    //    },
    //    None => {
    //        None
    //    }
    //}

    Some(rate + val)
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
    height :f32
}

#[cfg(test)]
mod test{
    use std::path::Path;
    use std::slice::range;
    use crate::ml_data::{Person, read_ml_json};
    use crate::Node;

    #[test]
    fn json_test(){
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let p: Person = serde_json::from_str(data).unwrap();

        // Do things just like with any other Rust data structure.
        println!("Please call {} at the number {}", p.name, p.phones[0]);
    }


    #[test]
    fn load_json_test(){
        let path = Path::new("resources/1663154348643_8ZGUJJLLWV/ml_data/1663154348643_8ZGUJJLLWV.json");
        let data = read_ml_json(&path);
        let mut xx_node: Node = data.element_statistics.nodes[0].clone();
        let mut correlacion = Vec::new();

        for node in data.element_statistics.nodes.iter() {
            if node.a.contains_key("XX") {
                xx_node = node.clone();
                break;
            }
        }
        println!("Nodo XX {}:  {:?}", xx_node.i, xx_node.a);

        for node in data.element_statistics.nodes.iter() {
            let mut suma = 0.0;
            let mut num_nodos = 0.0;
            for (k, v) in xx_node.a.iter() {
                suma += node.a.iter()
                            .filter(|(gk, gv)| *gk == k && *gv == v && *gk != "XX" && *gk != "HT" && *gk != "WH"
                                && *gk != "LT" && *gk != "RT")
                            .count() as f64;
            }
            num_nodos = node.a.iter()
                            .filter(|(gk, gv)| *gk != "XX" && *gk != "HT" && *gk != "WH"
                                && *gk != "LT" && *gk != "RT")
                            .count() as f64;

            correlacion.push(suma / num_nodos);
        }
        println!("Correlacion: {:?}", correlacion);
        println!("Maximo: {}", correlacion.iter().cloned().fold(0./0., f64::max));

        /*
        let mut iter = data.element_statistics.nodes.iter();
        //let cuantos = iter.len();
        //println!("{}", cuantos);
        let mut cual = "no".to_string();
        for num in iter{
            //println!("{}", num.i);
            //println!("{}", num.a.len());
            for key in num.a.keys(){
                //println!("{}",key);
                if key == "XX" && num.a[key] == "true"{
                    cual = num.i;
                    break
                }
            }
        }
        let my_int: i32 = cual.parse().unwrap();
        println!("{}",my_int);
        println!("{}", data.element_statistics.nodes[513].a["XX"]);
         */
    }
}
