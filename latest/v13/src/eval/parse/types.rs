use hashbrown::HashMap;

#[derive(Debug)]
pub struct Type {
  pub name: String,
  pub generics: HashMap<String, Type>,
}
