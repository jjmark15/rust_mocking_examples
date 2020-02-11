#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
}

#[cfg(test)]
mod tests {
    use mockall::predicate::*;
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn my_test() {
        let mut mock = MockMyTrait::new();
        mock.expect_foo().with(eq(4)).times(1).returning(|x| x + 1);

        assert_that(&mock.foo(4)).is_equal_to(5);
    }
}
