use bincode;
use serde::{Deserialize, Serialize};
use crypto::digest::Digest;
use crypto::sha3::Sha3;


pub fn get_serialize<T: ?Sized>(value: &T) -> Vec<u8>
    where
    T: Serialize, 
{
    let serialized = bincode::serialize(value).unwrap();
    serialized
}

pub fn get_unserialize<'a, T>(bytes: &'a[u8]) -> T
    where T:Deserialize<'a>,
{
    let desrialized = bincode::deserialize(bytes).unwrap();
    desrialized
}

pub fn get_hash(value: &[u8]) -> String{
    let mut hasher = Sha3::keccak256();
    hasher.input(value);
    hasher.result_str()
}


#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct Person{
    name: String,
    age: u8
}
#[cfg(test)]
mod test {
    use crate::util_cryto::Person;
    use crate::util_cryto::{get_serialize, get_unserialize};

    #[test]
    fn it_works(){
        let person = Person{name: String::from("gary"), age: 18};
        let se = get_serialize(&person);
        let de: Person = get_unserialize(&se);

        assert_eq!(de, person);
    }
}