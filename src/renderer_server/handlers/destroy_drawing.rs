use super::super::event_loop_notifier::EventLoopNotifier;
use super::HandlerError;

pub(crate) fn destroy_drawing(event_loop: &EventLoopNotifier) -> Result<(), HandlerError> {
    event_loop.exit()?;
    Ok(())
}
