// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::clone::Clone;
use std::cmp::PartialEq;
use std::ops::Drop;
use std::ffi::{CStr, CString};
use ffi;

pub use ffi::enums::{
    Antialias,
    SubpixelOrder,
    HintStyle,
    HintMetrics,

    FontType,
    FontWeight,
    FontSlant,
};
use ::matrices::Matrix;
use ffi::{
    cairo_font_options_t,
    cairo_font_face_t,
    cairo_scaled_font_t
};

pub use ffi::{
    FontExtents,
    Glyph,
    TextCluster,
    TextExtents
};

/* TODO
 Allocates an array of cairo_glyph_t's. This function is only useful in
 implementations of cairo_user_scaled_font_text_to_glyphs_func_t where the user
 needs to allocate an array of glyphs that cairo will free. For all other uses,
 user can use their own allocation method for glyphs.


impl Glyph {

    //pub fn cairo_glyph_allocate(num_glyphs: c_int) -> *Glyph;

    //pub fn cairo_glyph_free(glyphs: *Glyph);
}

 Allocates an array of cairo_glyph_t's. This function is only useful in
 implementations of cairo_user_scaled_font_text_to_glyphs_func_t where the user
 needs to allocate an array of glyphs that cairo will free. For all other uses,
 user can use their own allocation method for glyphs.

impl TextCluster {
    //pub fn cairo_text_cluster_allocate(num_clusters: c_int) -> *TextCluster;

    //pub fn cairo_text_cluster_free(clusters: *TextCluster);
}
*/


pub struct FontOptions(*mut cairo_font_options_t);

impl FontOptions {
    pub fn new() -> FontOptions {
        let font_options = unsafe {
            FontOptions(ffi::cairo_font_options_create())
        };
        font_options.ensure_status();
        font_options
    }

    #[doc(hidden)]
    pub fn get_ptr(&self) -> *mut cairo_font_options_t {
        let FontOptions(ptr) = *self;
        ptr
    }

    pub fn ensure_status(&self) {
        let status = unsafe {
            ffi::cairo_font_options_status(self.get_ptr())
        };
        status.ensure_valid()
    }

    pub fn merge(&mut self, other: &mut FontOptions) {
        unsafe {
            ffi::cairo_font_options_merge(self.get_ptr(), other.get_ptr())
        }
    }

    pub fn hash(&self) -> u64{
        unsafe {
            ffi::cairo_font_options_hash(self.get_ptr()) as u64
        }
    }

    pub fn set_antialias(&self, antialias: Antialias) {
        unsafe {
            ffi::cairo_font_options_set_antialias(self.get_ptr(), antialias)
        }
    }

    pub fn get_antialias(&self) -> Antialias {
        unsafe {
            ffi::cairo_font_options_get_antialias(self.get_ptr())
        }
    }

    pub fn set_subpixel_order(&self, order: SubpixelOrder) {
        unsafe {
            ffi::cairo_font_options_set_subpixel_order(self.get_ptr(), order)
        }
    }

    pub fn get_subpixel_order(&self) -> SubpixelOrder {
        unsafe {
            ffi::cairo_font_options_get_subpixel_order(self.get_ptr())
        }
    }

    pub fn set_hint_style(&self, hint_style: HintStyle) {
        unsafe {
            ffi::cairo_font_options_set_hint_style(self.get_ptr(), hint_style)
        }
    }

    pub fn get_hint_style(&self) -> HintStyle {
        unsafe {
            ffi::cairo_font_options_get_hint_style(self.get_ptr())
        }
    }

    pub fn set_hint_metrics(&self, hint_metrics: HintMetrics) {
        unsafe {
            ffi::cairo_font_options_set_hint_metrics(self.get_ptr(), hint_metrics)
        }
    }

    pub fn get_hint_metrics(&self) -> HintMetrics {
        unsafe {
            ffi::cairo_font_options_get_hint_metrics(self.get_ptr())
        }
    }
}

impl PartialEq for FontOptions {
    fn eq(&self, other: &FontOptions) -> bool {
        unsafe {
            ffi::cairo_font_options_equal(self.get_ptr(), other.get_ptr()).as_bool()
        }
    }
}

impl Clone for FontOptions {
    fn clone(&self) -> FontOptions {
        unsafe {
            FontOptions(ffi::cairo_font_options_copy(self.get_ptr()))
        }
    }
}

impl Drop for FontOptions {
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_font_options_destroy(self.get_ptr())
        }
    }
}

pub struct FontFace(pub *mut cairo_font_face_t);

impl FontFace {
    #[doc(hidden)]
    pub fn get_ptr(&self) -> *mut cairo_font_face_t {
        let FontFace(ptr) = *self;
        ptr
    }

    pub fn toy_create(family: &str, slant: FontSlant, weight: FontWeight) -> FontFace {
        let font_face = FontFace(
            unsafe {
                let family_cstring = CString::new(family).unwrap();
                ffi::cairo_toy_font_face_create(family_cstring.as_ptr(), slant, weight)
            }
        );
        font_face.ensure_status();
        font_face
    }

    pub fn toy_get_family(&self) -> Option<String> {
        unsafe {
            let family = ffi::cairo_toy_font_face_get_family(self.get_ptr());
            if family.is_null() {
                None
            } else {
                Some(CStr::from_ptr(family).to_string_lossy().into_owned())
            }
        }
    }

    pub fn toy_get_slant(&self) -> FontSlant {
        unsafe {
            ffi::cairo_toy_font_face_get_slant(self.get_ptr())
        }
    }

    pub fn toy_get_weight(&self) -> FontWeight {
        unsafe {
            ffi::cairo_toy_font_face_get_weight(self.get_ptr())
        }
    }

    pub fn ensure_status(&self) {
        let status = unsafe {
            ffi::cairo_font_face_status(self.get_ptr())
        };
        status.ensure_valid()
    }

    pub fn get_type(&self) -> FontType {
        unsafe {
            ffi::cairo_font_face_get_type(self.get_ptr())
        }
    }

    pub fn get_reference_count(&self) -> usize {
        unsafe {
            ffi::cairo_font_face_get_reference_count(self.get_ptr()) as usize
        }
    }

    pub fn reference(&self) -> FontFace {
        unsafe {
            FontFace(ffi::cairo_font_face_reference(self.get_ptr()))
        }
    }
}

impl Drop for FontFace {
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_font_face_destroy(self.get_ptr())
        }
    }
}

pub struct ScaledFont(pub *mut cairo_scaled_font_t);

impl ScaledFont {
    #[doc(hidden)]
    pub fn get_ptr(&self) -> *mut cairo_scaled_font_t {
        let ScaledFont(ptr) = *self;
        ptr
    }

    pub fn new(font_face: FontFace, font_matrix: &mut Matrix, ctm: &mut Matrix, options: FontOptions) -> ScaledFont {
        let scaled_font = unsafe {
            ScaledFont(ffi::cairo_scaled_font_create(font_face.get_ptr(), font_matrix, ctm, options.get_ptr()))
        };
        scaled_font.ensure_status();
        scaled_font
    }

    pub fn ensure_status(&self) {
        let status = unsafe {
            ffi::cairo_scaled_font_status(self.get_ptr())
        };
        status.ensure_valid()
    }

    pub fn get_type(&self) -> FontType {
        unsafe {
            ffi::cairo_scaled_font_get_type(self.get_ptr())
        }
    }

    pub fn get_reference_count(&self) -> usize {
        unsafe {
            ffi::cairo_scaled_font_get_reference_count(self.get_ptr()) as usize
        }
    }

    //pub fn cairo_scaled_font_extents(scaled_font: *mut cairo_scaled_font_t, extents: *mut cairo_font_extents_t);

    //                    cairo_text_extents_t;
    //pub fn cairo_scaled_font_text_extents(scaled_font: *mut cairo_scaled_font_t, utf8: *mut char, extents: *mut cairo_text_extents_t);

    //pub fn cairo_scaled_font_glyph_extents(scaled_font: *mut cairo_scaled_font_t, glyphs: *mut Glyph, num_glyphs: c_int, extents: *mut cairo_text_extents_t);

    //pub fn cairo_scaled_font_text_to_glyphs(scaled_font: *mut cairo_scaled_font_t, x: c_double, y: c_double, utf8: *mut char, utf8_len: c_int, glyphs: **mut Glyph, num_glyphs: *mut c_int, clusters: **mut TextCluster, num_clusters: *mut c_int, cluster_flags: *mut TextClusterFlags) -> Status;

    //pub fn cairo_scaled_font_get_font_face(scaled_font: *mut cairo_scaled_font_t) -> *mut cairo_font_face_t;

    //pub fn cairo_scaled_font_get_font_options(scaled_font: *mut cairo_scaled_font_t, options: *mut cairo_font_options_t);

    //pub fn cairo_scaled_font_get_font_matrix(scaled_font: *mut cairo_scaled_font_t, font_matrix: *mut cairo_matrix_t);

    //pub fn cairo_scaled_font_get_ctm(scaled_font: *mut cairo_scaled_font_t, ctm: *mut cairo_matrix_t);

    //pub fn cairo_scaled_font_get_scale_matrix(scaled_font: *mut cairo_scaled_font_t, scale_matrix: *cairo_matrix_t);

    pub fn reference(&self) -> ScaledFont {
        unsafe {
            ScaledFont(ffi::cairo_scaled_font_reference(self.get_ptr()))
        }
    }
}

impl Drop for ScaledFont {
    fn drop(&mut self) {
        unsafe {
            ffi::cairo_scaled_font_destroy(self.get_ptr())
        }
    }
}
