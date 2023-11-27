// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

// TODO: Rewrite, support all req types
// TODO: Remove logging maybe
#[macro_export]
macro_rules! api {
  ($base:literal, {$(
    fn $fun:ident($endpoint:literal) -> $ty:ty $({
      $($pn:ident:$pt:ty),*$(,)?
    })?;
  )*}) => {
    $(pub async fn $fun($($($pn: $pt),*)?) -> neko_core::Res<$ty> {
      let req = format!(concat!($base, $endpoint, $("?",$(stringify!($pn), "={", stringify!($pn), "}&"),*)?), $($($pn=$pn),*)?);
      neko_core::log::trace!("Sending req to {req}");
      let res = $crate::get_reqwest().get(req).send().await?;
      neko_core::log::trace!("Received status: {}", res.status());
      Ok(res.json::<$ty>().await?)
    })*
  };
}
