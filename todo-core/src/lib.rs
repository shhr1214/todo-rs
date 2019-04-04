#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Task {
    name: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
