use napi::{bindgen_prelude::*, Env};
use napi_derive::napi;
use pdfium_render::prelude::PdfDocument;

use crate::{Pages, Viewer};

#[napi]
pub struct Document {
  inner: SharedReference<Viewer, PdfDocument<'static>>,
}

#[napi]
impl Document {
  pub fn new(inner: SharedReference<Viewer, PdfDocument<'_>>) -> Self {
    Self { inner }
  }

  pub fn get_ref(&self) -> &PdfDocument<'static> {
    &*self.inner
  }

  #[napi]
  pub fn pages(&self, reference: Reference<Document>, env: Env) -> Result<Pages> {
    Pages::new(reference, env)
  }

  pub fn clone(&self, env: Env) -> Result<Self> {
    self.inner.clone(env).map(Self::new)
  }
}
