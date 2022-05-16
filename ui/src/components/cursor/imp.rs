use std::cell::RefCell;

use gtk::subclass::prelude::*;
use gtk::{glib, gsk, prelude::*};

use crate::font::Font;
use crate::SCALE;

#[derive(Default)]
pub struct Cursor {
    pub pos: RefCell<(i64, i64)>,
    pub text: RefCell<String>,
    pub double_width: RefCell<bool>,

    pub node: RefCell<Option<gsk::RenderNode>>,

    pub width_percentage: RefCell<f32>,
    pub attr_id: RefCell<i64>,

    pub hide: RefCell<bool>,

    // TODO(ville): bind property
    pub font: RefCell<Font>,
}

#[glib::object_subclass]
impl ObjectSubclass for Cursor {
    const NAME: &'static str = "Cursor";
    type Type = super::Cursor;
    type ParentType = gtk::Widget;
}

impl ObjectImpl for Cursor {
    fn properties() -> &'static [glib::ParamSpec] {
        use once_cell::sync::Lazy;
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![glib::ParamSpecObject::new(
                "font",
                "font",
                "Font",
                Font::static_type(),
                glib::ParamFlags::READWRITE,
            )]
        });

        PROPERTIES.as_ref()
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "font" => self.font.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

    fn set_property(
        &self,
        _obj: &Self::Type,
        _id: usize,
        value: &glib::Value,
        pspec: &glib::ParamSpec,
    ) {
        match pspec.name() {
            "font" => self
                .font
                .replace(value.get().expect("font value must be object Font")),
            _ => unimplemented!(),
        };
    }
}

impl WidgetImpl for Cursor {
    fn snapshot(&self, _widget: &Self::Type, snapshot: &gtk::Snapshot) {
        if *self.hide.borrow() {
            return;
        }

        if let Some(ref node) = *self.node.borrow() {
            snapshot.append_node(node);
        }
    }

    fn measure(
        &self,
        widget: &Self::Type,
        orientation: gtk::Orientation,
        for_size: i32,
    ) -> (i32, i32, i32, i32) {
        match orientation {
            gtk::Orientation::Horizontal => {
                // width
                let len = self.double_width.borrow().then(|| 2.0).unwrap_or(1.0);
                let font = self.font.borrow();
                let w = len * (font.char_width() / SCALE);
                let w = w.ceil() as i32;

                (w, w, -1, -1)
            }
            gtk::Orientation::Vertical => {
                // height
                let font = self.font.borrow();
                let h = font.height() / SCALE;
                let h = h.ceil() as i32;

                return (h, h, -1, -1);
            }
            _ => self.parent_measure(widget, orientation, for_size),
        }
    }
}
