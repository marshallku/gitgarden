#[cfg(test)]
mod tests {
    use crate::render::objects::Objects;

    #[test]
    fn test_size_of_iter() {
        let objects = Objects::iter().collect::<Vec<_>>();
        assert_eq!(objects.len(), Objects::Dirt as usize + 1);
    }
}
