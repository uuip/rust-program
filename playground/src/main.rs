use log::info;

/// `hello` is a function that prints a greeting that is personalized based on
/// the name given.
///
/// # Arguments
///
/// * `name` - The name of the person you'd like to greet.
///
/// # Example
///
/// ```rust
/// let name = "Steve";
/// hello(name); // prints "Hello, Steve!"
/// ```
fn main() {
    // comment
    /*
    comment
    ...
    comment
     */
    info!("ppp");
}

/**
`add_two` 将指定值加2


```
let arg = 5;
let answer = my_crate::add_two(arg);

assert_eq!(7, answer);
```
*/
pub fn add_two(x: i32) -> i32 {
    x + 2
}

pub mod kinds {
    //! 模块文件的头部, 定义颜色的类型
    /*! ... */

    /// [`Pool`] configuration.
    ///
    /// [`Pool`]: super::Pool
    #[derive(Clone, Copy, Debug)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    pub struct PoolConfig {
        /// Maximum size of the [`Pool`].
        ///
        /// [`Pool`]: super::Pool
        pub max_size: usize,
    }
}
