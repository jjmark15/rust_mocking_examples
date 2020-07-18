#[cfg(test)]
use std::fmt::Debug;

#[cfg(test)]
use mockall::{automock, mock, predicate::*};

#[cfg_attr(test, automock)]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
}

trait ComplexTrait: Clone + Default + Eq + PartialEq {
    fn returns_a_number() -> u8;
}

#[cfg(test)]
mock! {
    pub ComplexTraitImpl {}

    trait ComplexTrait {
        fn returns_a_number() -> u8;
    }

    trait Clone {
        fn clone(&self) -> Self;
    }

    trait PartialEq {
        fn eq(&self, other: &MockComplexTraitImpl) -> bool;
    }

    trait Eq {}

    trait Debug {
        fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result;
    }
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

    #[test]
    fn complex_trait_test() {
        let mut mock = MockComplexTraitImpl::default();
        let mock2 = MockComplexTraitImpl::default();
        // prime method
        mock.expect_clone().return_once(move || mock2);
        mock.expect_eq().returning(|_o| true);
        // prime static method
        let ctx = MockComplexTraitImpl::returns_a_number_context();
        ctx.expect().return_once(|| 2);

        let other_instance = mock.clone();

        assert_that(&mock).is_equal_to(other_instance);
        assert_that(&MockComplexTraitImpl::returns_a_number()).is_equal_to(2);
    }
}
