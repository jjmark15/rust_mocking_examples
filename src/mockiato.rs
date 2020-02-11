#[cfg(test)]
use mockiato::mockable;

#[cfg_attr(test, mockable)]
trait Greeter {
    fn greet(&self, name: &str) -> String;
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn greet_the_world() {
        let mut greeter = GreeterMock::new();

        greeter
            .expect_greet(|arg| arg.partial_eq("world"))
            .times(1..2)
            .returns(String::from("Hello world"));

        assert_that(&greeter.greet("world")).is_equal_to("Hello world".to_string());
    }
}
