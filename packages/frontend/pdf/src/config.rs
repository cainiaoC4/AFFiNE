use napi_derive::napi;
use pdfium_render::prelude::PdfPageRenderRotation;

#[napi]
pub enum Rotation {
  Zero,
  Quarter,
  Half,
  ThreeQuarters,
}

impl Into<PdfPageRenderRotation> for Rotation {
  fn into(self) -> PdfPageRenderRotation {
    match self {
      Self::Zero => PdfPageRenderRotation::None,
      Self::Quarter => PdfPageRenderRotation::Degrees90,
      Self::Half => PdfPageRenderRotation::Degrees180,
      Self::ThreeQuarters => PdfPageRenderRotation::Degrees270,
    }
  }
}
