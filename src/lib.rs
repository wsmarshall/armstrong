pub mod configuration;
pub mod routes;
pub mod startup;

// ///a trait summing up operations required of a logger
// pub trait Log: Sync + Send {
//     ///Determines if a log message with the specified metadata
//     /// will be logged
//     ///
//     /// this is used by the 'log_enabled!' macro to allow
//     /// callers to avoid expensive computation of
//     /// log message arguments if the message
//     /// would be discarded anyway
//     fn enabled(&self, metadata: &Metadata) -> bool;

//     ///Logs the 'Record
//     ///
//     /// note that 'enabled' is NOT necessarily called before this
//     /// implementations of log should perform all required
//     /// filtering internally
//     fn log(&self, record: &Record);

//     ///Flushes (clears) any buffered records
//     fn flush(&self);
// }
