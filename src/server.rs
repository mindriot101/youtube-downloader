use config::Config;
use download_thread::DownloadThread;
use failure::Error;
use job::Job;
use slog::Logger;
use std::path::PathBuf;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

pub struct Server {
    config: PathBuf,
    sleep_time: u64,
    tx: Sender<Job>,
    log: Logger,
}

impl Server {
    pub fn new(config: PathBuf, sleep_time: u64, log: Logger) -> Self {
        let (tx, rx) = mpsc::channel();

        let dl_log = log.clone();
        let dl_log = dl_log.new(o!("name" => "downloader"));
        debug!(log, "spawning download thread");
        thread::spawn(move || {
            let downloader = DownloadThread::new(rx, dl_log);
            downloader.run()
        });

        let log = log.new(o!("name" => "server"));

        info!(log, "starting server");
        Server {
            config,
            sleep_time,
            tx,
            log,
        }
    }

    fn update_configs(&self) -> Result<Vec<Job>, Error> {
        let log = self.log.new(o!("stage" => "update_configs"));
        info!(log, "updating jobs from config file"; "filename" => format!("{:?}", self.config));

        debug!(log, "parsing config");
        let config = Config::from_file(&self.config)?;
        Ok(config.jobs.iter().map(|j| Job::from(j)).collect())
    }

    fn enqueue_job(&self, job: &Job) -> Result<(), Error> {
        let log = self.log.new(o!("stage" => "enqueue_job"));
        info!(log, "submitting job"; "job" => format!("{:?}", job));
        self.tx.send(job.clone())?;
        Ok(())
    }

    fn sleep(&self) {
        let log = self.log.new(o!("stage" => "sleep"));
        info!(log, "sleeping"; "sleep_time" => self.sleep_time);
        thread::sleep(Duration::from_secs(self.sleep_time));
    }

    pub fn run(&self) -> Result<(), Error> {
        loop {
            let jobs = self.update_configs()?;
            for job in &jobs {
                self.enqueue_job(job)?;
            }
            self.sleep();
        }
    }
}
