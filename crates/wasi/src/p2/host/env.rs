use crate::p2::bindings::cli::environment;
use crate::p2::{WasiImpl, WasiView, LogLevel};

impl<T> environment::Host for WasiImpl<T>
where
    T: WasiView,
{
    fn get_environment(&mut self) -> anyhow::Result<Vec<(String, String)>> {
        self.ctx().logger.log(LogLevel::DEBUG, "calling environment (get_environment) function.".into());
        Ok(self.ctx().env.clone())
    }
    fn get_arguments(&mut self) -> anyhow::Result<Vec<String>> {
        self.ctx().logger.log(LogLevel::DEBUG, "calling environment (get_arguments) function.".into());
        Ok(self.ctx().args.clone())
    }
    fn initial_cwd(&mut self) -> anyhow::Result<Option<String>> {
        self.ctx().logger.log(LogLevel::DEBUG, "calling environment (intial_cmd) function.".into());
        // FIXME: expose cwd in builder and save in ctx
        Ok(None)
    }
}
