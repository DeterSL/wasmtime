use crate::I32Exit;
use crate::p2::LogLevel;
use crate::p2::{WasiImpl, WasiView, bindings::cli::exit};

pub enum ExitEvent {
    Exit
}

impl<T> exit::Host for WasiImpl<T>
where
    T: WasiView,
{
    fn exit(&mut self, status: Result<(), ()>) -> anyhow::Result<()> {
        self.ctx().logger.log(LogLevel::DEBUG, "calling exit (exit) function.".into());
        self.ctx().event_handler.accepts(&ExitEvent::Exit)?;
        let status = match status {
            Ok(()) => 0,
            Err(()) => 1,
        };
        Err(anyhow::anyhow!(I32Exit(status)))
    }

    fn exit_with_code(&mut self, status_code: u8) -> anyhow::Result<()> {
        self.ctx().logger.log(LogLevel::DEBUG, "calling exit (exit) function.".into());
        self.ctx().event_handler.accepts(&ExitEvent::Exit)?;
        Err(anyhow::anyhow!(I32Exit(status_code.into())))
    }
}
