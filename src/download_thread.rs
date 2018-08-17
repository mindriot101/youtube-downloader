use failure::Error;
use job::Job;
use slog::Logger;
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::Receiver;

pub struct DownloadThread {
    rx: Receiver<Job>,
    log: Logger,
}

impl DownloadThread {
    pub fn new(rx: Receiver<Job>, log: Logger) -> Self {
        DownloadThread { rx, log }
    }

    pub fn run(&self) -> Result<(), Error> {
        let log = self.log.new(o!("stage" => "run"));
        loop {
            info!(log, "waiting for new download jobs");
            let next_job = self.rx.recv()?;
            match self.perform_work(next_job) {
                Err(e) => crit!(log, "failure to download"; "error" => format!("{:?}", e)),
                Ok(_) => {}
            }
        }
        Ok(())
    }

    fn perform_work(&self, job: Job) -> Result<(), Error> {
        let log = self.log.new(o!("stage" => "perform_work"));

        let output_template = self.compute_output_template(&job.dest)?;
        info!(log, "downloading");
        let mut cmd = Command::new("youtube-dl")
            .args(&[
                &job.url,
                "--format",
                "best",
                "--output",
                &output_template,
                "--continue",
            ]).spawn()?;
        let ecode = cmd.wait()?;
        if !ecode.success() {
            bail!("error running download command");
        }
        Ok(())
    }

    fn compute_output_template(&self, dest: &str) -> Result<String, Error> {
        let path = Path::new(dest);
        let template = path.join("%(upload_date)s-%(title)s.%(ext)s");
        Ok(template
            .to_str()
            .ok_or_else(|| format_err!("error constructing valid template"))?
            .to_string())
    }
}
