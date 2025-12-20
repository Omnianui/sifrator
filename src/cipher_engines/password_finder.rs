use hex_literal::hex;
use sha1::{Sha1, Digest};
use rand::{Rng,seq::{IndexedRandom, SliceRandom}};

static  SPECIAL_CHARS:[&str;7] = ["@","!","?","_","-",".",","];
static STRING_DATA:[&str;5] = ["zari","Mici","Maruska","dort","ChocoFans"];
static NUMBER_DATA:[&str;7] = ["19","9","1990","2015","90","1","0"];

pub fn find_password(){
    let mut hash:[u8; 20] = [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];
    let mut random_pass:String = String::new();
    let mut pokus = 1;
    let mut rng = rand::rng();
    
    while hash.as_slice() != hex!("ead02c3700a95dd1f7e34ecea894abfe6f0c87bc") { 
        let mut random_pass_vec :Vec<String> = 
            SPECIAL_CHARS.choose_multiple(&mut rand::rng(), rng.random_range(1..3)).map(|item| item.to_string())
            .chain::<Vec<_>>(STRING_DATA.choose_multiple(&mut rand::rng(), rng.random_range(1..3)).map(|item| item.to_string()).collect())
            .chain::<Vec<_>>(NUMBER_DATA.choose_multiple(&mut rand::rng(), rng.random_range(1..3)).map(|item| item.to_string()).collect()).collect();
        random_pass_vec.shuffle(&mut rng);
        random_pass = random_pass_vec.iter().map(|item| item.to_string()).collect();
        
        let mut hasher = Sha1::new();
        hasher.update(&random_pass);
        hash = hasher.finalize().into();
        pokus += 1;
    }

    println!("Na {:x} pokusu bylo získáno heslo {}",pokus,random_pass);
}