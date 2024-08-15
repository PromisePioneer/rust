fn main() {
    /*
    ------------------------------------------------------------------------------------------------------------
       variable are immutable by default when variable is immutable, once value is bound to a name, you cant change the value.
    ----------------------------------------------------------------------------------------------------------------
     */
    let x = 5;
    println!("The value of x is: {}", { x });

    /*
    --------------------------------------------
    this will return an error, x is immutable
    x = 6;
    println!("The value of x is:", { x });
    --------------------------------------------
     */

    /*
      ------------------------------------------------------------------------------------------------------------
     were allowed to change the value bound to y  from 6 to 10 when mut is used. Ultimately deciding whether to use mutability or not is up to you and depends on what you think is clearest in that particular situation.
     -------------------------------------------------------------------------------------------------------------
     */
    let mut y = 6;
    println!("The value of x is: {}", y);
    y = 10;
    println!("The value of x is: {}", y);

    /*
    -----------------------------------------------------------------------------------------------------------------
    Constants
       Like immutable variables, constants are value that are bound to a name and are not allowed to change, but there are a few difference between constants and variables.

       1. not allowed to use mut keyword on constants. constants are immutable by default and always immutable.
       2. constant can be declared in any scope, including the global scope, which makes them useful for value that many parts of code need to know about
       3. constants may be set only to a constant expression, not a result of a value that many parts of code need to know about.
       --------------------------------------------------------------------------------------------------------------
     */

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    /*
       Shadowing.
       1. we bind first to value 5. it creates a new variable first by repeating let first = first + 1, so the value first now is 6
     */
    let first = 5;
    let first = first + 1; // shadowed
    {
        let first = first * 2;
        println!("The value of first is {}", first); // 12 = 6 * 2
    }

    println!("the value of first is {}", first); // 6
}
