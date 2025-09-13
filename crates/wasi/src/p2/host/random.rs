use crate::p2::bindings::random::{insecure, insecure_seed, random};
use crate::p2::{WasiImpl, WasiView, LogLevel};
use cap_rand::{Rng, distributions::Standard};

pub enum RndEvent {
    Rnd,
}

impl<T> random::Host for WasiImpl<T>
where
    T: WasiView,
{
    fn get_random_bytes(&mut self, len: u64) -> anyhow::Result<Vec<u8>> {
        self.ctx().logger.log(LogLevel::DEBUG, "calling secure random (get_random_bytes) function.".into());
        self.ctx().event_handler.accepts(&RndEvent::Rnd)?;
        Ok((&mut self.ctx().random)
            .sample_iter(Standard)
            .take(len as usize)
            .collect())
    }

    fn get_random_u64(&mut self) -> anyhow::Result<u64> {
        self.ctx().logger.log(LogLevel::DEBUG, "calling secure random (get_random_u64) function.".into());
        self.ctx().event_handler.accepts(&RndEvent::Rnd)?;
        Ok(self.ctx().random.sample(Standard))
    }
}

impl<T> insecure::Host for WasiImpl<T>
where
    T: WasiView,
{
    fn get_insecure_random_bytes(&mut self, len: u64) -> anyhow::Result<Vec<u8>> {
        self.ctx().logger.log(LogLevel::DEBUG, "calling insecure random (get_insecure_random_bytes) function.".into());
        self.ctx().event_handler.accepts(&RndEvent::Rnd)?;
        Ok((&mut self.ctx().insecure_random)
            .sample_iter(Standard)
            .take(len as usize)
            .collect())
    }

    fn get_insecure_random_u64(&mut self) -> anyhow::Result<u64> {
        self.ctx().logger.log(LogLevel::DEBUG, "calling insecure random (get_insecure_random_u64) function.".into());
        self.ctx().event_handler.accepts(&RndEvent::Rnd)?;
        Ok(self.ctx().insecure_random.sample(Standard))
    }
}

impl<T> insecure_seed::Host for WasiImpl<T>
where
    T: WasiView,
{
    fn insecure_seed(&mut self) -> anyhow::Result<(u64, u64)> {
        self.ctx().logger.log(LogLevel::DEBUG, "calling insecure seed (insecure_seed) function.".into());
        self.ctx().event_handler.accepts(&RndEvent::Rnd)?;
        let seed: u128 = self.ctx().insecure_random_seed;
        Ok((seed as u64, (seed >> 64) as u64))
    }
}
