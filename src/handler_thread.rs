use failure::Error;
use job::Job;
use slog::Logger;
use std::sync::mpsc::Sender;
use zmq;

pub(crate) struct HandlerThread {
    tx: Sender<Job>,
    log: Logger,
    socket: zmq::Socket,
}

impl HandlerThread {
    pub(crate) fn new(tx: Sender<Job>, log: Logger) -> Result<Self, Error> {
        let context = zmq::Context::new();
        let socket = context.socket(zmq::SocketType::REP)?;
        let log = log.new(o!("program" => "handler_thread"));

        info!(log, "binding listener socket");
        socket.bind("tcp://*:5505")?;

        Ok(HandlerThread { tx, log, socket })
    }

    pub(crate) fn run(&self) -> Result<(), Error> {
        let log = self.log.new(o!("stage" => "run"));
        loop {
            info!(log, "waiting for message");
            let msg = self.socket.recv_string(0)?;
            match msg {
                Ok(msg) => self.handle_message(msg)?,
                Err(e) => bail!("handling error: {:?}", e),
            }
            self.socket.send_str("ok", 0)?;
        }
    }

    fn handle_message(&self, msg: String) -> Result<(), Error> {
        let log = self.log.new(o!("stage" => "handle_message"));

        debug!(log, "deserialising message");
        let job: Job = msg.into();

        debug!(log, "sending message"; "message" => format!("{:?}", job));
        self.tx.send(job)?;
        Ok(())
    }
}
