extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;


#[derive(Serialize, Deserialize, Debug)]
pub struct TomlFile {
    pub info: Info,
    pub components: Vec<Component>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Component {
    pub pause: Pause,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pause {
    pub fixed: Option<PauseFixed>,
    pub random: Option<PauseRandom>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PauseFixed {
    pub duration: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PauseRandom {
    normal: Option<PauseRandomNormal>,
    uniform: Option<PauseRandomUniform>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PauseRandomNormal {
    pub duration: f64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PauseRandomUniform {
    pub duration: f64,
}


fn main() {
    println!("Hello, world!");
}
