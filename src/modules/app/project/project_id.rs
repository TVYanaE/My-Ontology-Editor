use uuid::Uuid;

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
pub struct ProjectID(Uuid); 

impl ProjectID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    } 
    pub fn get_str(&self) -> String {
        self.0.to_string()
    }
}
