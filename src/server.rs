use config::Config;
use download_thread::DownloadThread;
use failure::Error;
use job::Job;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub struct Server {
    config: PathBuf,
    sleep_time: u64,
    downloader_handle: JoinHandle<Result<(), Error>>,
    tx: Sender<Job>,
}

impl Server {
    pub fn new(config: PathBuf, sleep_time: u64) -> Self {
        let (tx, rx) = mpsc::channel();

        let downloader_handle = thread::spawn(move || {
            let downloader = DownloadThread::new(rx);
            downloader.run()
        });

        Server {
            config,
            sleep_time,
            downloader_handle,
            tx,
        }
    }

    fn update_configs(&self) -> Result<Vec<Job>, Error> {
        let config = Config::from_file(&self.config)?;
        Ok(config.jobs.iter().map(|j| Job::from(j)).collect())
    }

    fn enqueue_job(&self, job: &Job) -> Result<(), Error> {
        self.tx.send(job.clone())?;
        Ok(())
    }

    fn sleep(&self) {
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
        Ok(())
    }
}
