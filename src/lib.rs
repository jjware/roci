#[macro_use]
extern crate log;

mod bindings;

use bindings::*;
use ::std::ptr;

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::raw::c_void;

    #[test]
    fn can_create_env() {
        let env = Env::new(Mode::Threaded).unwrap();
    }

    /*
    #[test]
    fn can_free_handle() {
        let env = Env::new(Mode::Threaded).unwrap();
        env.close().unwrap();
    }
    */

    #[test]
    fn can_drop_cpool() {
        let env = Env::new(Mode::Threaded).unwrap();
        let error = Error::new(&env).unwrap();
        let cpool = CPool::new(&env).unwrap();
        drop(cpool)
    }

    #[test]
    fn can_create_connection_pool() {
        let env = Env::new(Mode::Threaded).unwrap();
        let error = Error::new(&env).unwrap();
        let cpool = CPool::new(&env).unwrap();
        let result = connection_pool_create(
            &env,
            &error,
            &cpool,
            "",
            1,
            3,
            1,
            "",
            "",
            Mode::Default,
        ).unwrap();
    }
}
