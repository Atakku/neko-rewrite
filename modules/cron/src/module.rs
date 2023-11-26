// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use neko_core::*;
use tokio_cron_scheduler::{Job, JobScheduler};

pub mod macros;

pub use tokio_cron_scheduler;

module! {
  Cron {
    jobs: Vec<Job>
  }

  impl on_runtime(cron) {
    let sched = JobScheduler::new().await?;
    for job in cron.jobs {
      sched.add(job).await?;
    }
    sched.start().await?;
    Ok(None)
  }

  fn add_job(&mut self, job: Job) {
    self.jobs.push(job);
  }
}
