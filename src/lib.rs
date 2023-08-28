#[macro_export]
macro_rules! chain_functions {
    ($input:expr, $($operation:expr),+ $(,)?) => {
        {
            {
                let temp = $input;
                $(
                    let temp = {
                        #[cfg(feature = "tracing")]
                        let span = tracing::span!(tracing::Level::INFO,stringify!($operation));
                        #[cfg(feature = "tracing")]
                        let _enter = span.enter();
                        $operation(temp)?
                    };
                )+
                Ok(temp)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    // Nice and clean.  Macro cleans up extra temp and error handling.
    // Downside is that it's not as easy to debug.
    fn test_chain(input: i32) -> anyhow::Result<i32> {
        chain_functions!(input, add_one, multiply_by_two, subtract_three)
    }

    // lambdas
    fn chain_lambda(input: i32) -> anyhow::Result<i32> {
        chain_functions!(
            input,
            add_one,
            |x| Ok::<_, anyhow::Error>(x * 2),
            subtract_three
        )
    }

    // Extra "noise" of temp and error handling, but easier to debug.
    fn manual_chain(input: i32) -> Result<i32> {
        let temp = input;

        let temp = add_one(temp)?;
        let temp = multiply_by_two(temp)?;
        let temp = subtract_three(temp)?;

        Ok(temp)
    }

    fn manual_chain2(input: i32) -> Result<i32> {
        let temp = add_one(input)?;
        let temp = multiply_by_two(temp)?;
        Ok(subtract_three(temp)?)
    }

    fn manual_chain_inline(input: i32) -> Result<i32> {
        Ok(subtract_three(multiply_by_two(add_one(input)?)?)?)
    }

    // Sample functions for demonstration
    fn add_one(x: i32) -> Result<i32> {
        Ok(x + 1)
    }

    fn multiply_by_two(x: i32) -> Result<i32> {
        Ok(x * 2)
    }

    fn subtract_three(x: i32) -> Result<i32> {
        Ok(x - 3)
    }

    #[test]
    fn test_chain_functions() {
        assert_eq!(test_chain(1).unwrap(), 1);
        assert_eq!(test_chain(2).unwrap(), 3);
        assert_eq!(test_chain(3).unwrap(), 5);
        assert_eq!(test_chain(4).unwrap(), 7);
        assert_eq!(test_chain(5).unwrap(), 9);
    }

    #[test]
    fn test_manual_chain() {
        assert_eq!(manual_chain(1).unwrap(), 1);
        assert_eq!(manual_chain(2).unwrap(), 3);
        assert_eq!(manual_chain(3).unwrap(), 5);
        assert_eq!(manual_chain(4).unwrap(), 7);
        assert_eq!(manual_chain(5).unwrap(), 9);
    }

    #[test]
    fn test_manual_chain2() {
        assert_eq!(manual_chain2(1).unwrap(), 1);
        assert_eq!(manual_chain2(2).unwrap(), 3);
        assert_eq!(manual_chain2(3).unwrap(), 5);
        assert_eq!(manual_chain2(4).unwrap(), 7);
        assert_eq!(manual_chain2(5).unwrap(), 9);
    }

    #[test]
    fn test_manual_chain_inline() {
        assert_eq!(manual_chain_inline(1).unwrap(), 1);
        assert_eq!(manual_chain_inline(2).unwrap(), 3);
        assert_eq!(manual_chain_inline(3).unwrap(), 5);
        assert_eq!(manual_chain_inline(4).unwrap(), 7);
        assert_eq!(manual_chain_inline(5).unwrap(), 9);
    }

    #[test]
    fn test_chain_lambda() {
        assert_eq!(chain_lambda(1).unwrap(), 1);
        assert_eq!(chain_lambda(2).unwrap(), 3);
        assert_eq!(chain_lambda(3).unwrap(), 5);
        assert_eq!(chain_lambda(4).unwrap(), 7);
        assert_eq!(chain_lambda(5).unwrap(), 9);
    }

    #[test]
    fn inline_lambda() {
        let res = || -> Result<_, anyhow::Error> {
            chain_functions!(5, add_one, multiply_by_two, subtract_three)
        };
        assert_eq!(res().unwrap(), 9);
    }

    
}
