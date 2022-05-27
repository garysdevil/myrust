use bincode;
use serde::{Deserialize, Serialize};
use crypto::digest::Digest;
use crypto::sha3::Sha3;


pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8>
    where
    T: Serialize, 
{
    let serialized = bincode::serialize(value).unwrap();
    serialized
}

pub fn my_unserialize<'a, T>(bytes: &'a[u8]) -> T
    where T:Deserialize<'a>,
{
    let desrialized = bincode::deserialize(bytes).unwrap();
    desrialized
}

pub fn get_hash(value: &[u8], mut out: &mut [u8]){
    let mut hasher = Sha3::keccak256();
    hasher.input(value);
    hasher.result(&mut out);
}




#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct Person{
    name: String,
    age: u8
}
#[cfg(test)]
mod test {
    use crate::cryto::Person;
    use crate::cryto::{my_serialize, my_unserialize};

    #[test]
    fn it_works(){
        let person = Person{name: String::from("gary"), age: 18};
        let se = my_serialize(&person);
        let de: Person = my_unserialize(&se);

        assert_eq!(de, person);
    }
}