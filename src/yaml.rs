extern crate yaml_rust;

use yaml_rust::Yaml;

pub fn get_yaml_str(yaml: &Yaml, first: &str, second: &str, third: &str) -> Option<String> {
    let h = yaml.as_hash()?;
    if first != "" {
        let value = h.get(&Yaml::from_str(first))?;
        match value {
            Yaml::String(s) => Some(s.to_string()),
            Yaml::Hash(_h) => get_yaml_str(value, second, third, ""),
            _ => None,
        }
    } else {
        None
    }
}
