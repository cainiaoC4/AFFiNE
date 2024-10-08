use napi::bindgen_prelude::*;
use napi_derive::napi;
use pdfium_render::prelude::PdfPage;

use crate::{Pages, Rect, Rotation};

#[napi]
pub struct Page {
  inner: SharedReference<Pages, PdfPage<'static>>,
}

#[napi]
impl Page {
  pub fn new(inner: SharedReference<Pages, PdfPage<'static>>) -> Self {
    Self { inner }
  }

  #[napi]
  pub fn text(&self) -> Result<String> {
    self
      .inner
      .text()
      .map(|t| t.all())
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn rect(&self) -> Rect {
    self.inner.page_size().into()
  }

  #[napi]
  pub fn render_as_bytes(
    &self,
    width: i32,
    height: i32,
    rotation: Option<Rotation>,
  ) -> Option<&[u8]> {
    self
      .inner
      .render(width, height, rotation.map(Into::into))
      .map(|bitmap| bitmap.as_raw_bytes())
      .ok()
  }

  #[napi]
  pub fn render(
    &self,
    width: i32,
    height: i32,
    rotation: Option<Rotation>,
  ) -> Option<Uint8ClampedArray> {
    self
      .inner
      .render(width, height, rotation.map(Into::into))
      .map(|bitmap| Uint8ClampedArray::from(bitmap.as_rgba_bytes()))
      .ok()
  }
}
