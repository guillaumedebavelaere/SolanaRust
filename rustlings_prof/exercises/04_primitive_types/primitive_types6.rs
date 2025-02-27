fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3); // t.0 t.1 t.2

        // TODO: Use a tuple index to access the second element of `numbers`
        // and assign it to a variable called `second`.
        let second: i32 = numbers.1;

        // Même chose
        //let (premier, second, troisieme) = numbers; ou let (_, second, _) = numbers;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
