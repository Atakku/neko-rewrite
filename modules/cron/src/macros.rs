// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#[macro_export]
macro_rules! job {
  ($fw:ident, $time:literal, $block:block) => {
    $fw.req::<$crate::Cron>()?.add_job($crate::tokio_cron_scheduler::Job::new_async($time, |_id, _jsl| Box::pin(async move $block))?)
  };
}
