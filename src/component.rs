//! A glyph embedded in another glyph.

use druid::kurbo::Affine;
use druid::Data;
use norad::GlyphName;

use crate::path::EntityId;

#[derive(Debug, Data, Clone)]
pub struct Component {
    pub base: GlyphName,
    #[data(same_fn = "affine_eq")]
    pub transform: Affine,
    pub id: EntityId,
}

fn affine_eq(left: &Affine, right: &Affine) -> bool {
    left.as_coeffs() == right.as_coeffs()
}

impl Component {
    pub fn from_norad(src: &norad::glyph::Component) -> Self {
        let base = src.base.clone();
        let transform = src.transform.into();
        let id = EntityId::new_with_parent(0);
        Component {
            base,
            transform,
            id,
        }
    }

    pub fn to_norad(&self) -> norad::glyph::Component {
        let base = self.base.clone();
        let transform = self.transform.into();
        let identifier = None;
        norad::glyph::Component {
            base,
            transform,
            identifier,
        }
    }
}
