/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Resolved color values.

use super::{Context, ToResolvedValue};

use crate::properties::{PropertyId, ShorthandId};
use crate::values::computed::color as computed;
use crate::values::generics::color as generics;

impl ToResolvedValue for computed::Color {
    // A resolved color value is (almost always) a rgba color, with currentcolor resolved.
    type ResolvedValue = Self;

    #[inline]
    fn to_resolved_value(self, context: &Context) -> Self::ResolvedValue {
        if context.for_property == PropertyId::NonCustom(ShorthandId::TextDecoration.into())
            && matches!(self, Self::CurrentColor)
        {
            return self;
        }
        generics::Color::Absolute(context.style.resolve_color(&self))
    }

    #[inline]
    fn from_resolved_value(resolved: Self::ResolvedValue) -> Self {
        resolved
    }
}

impl ToResolvedValue for computed::CaretColor {
    // A resolved caret-color value is an rgba color, with auto resolving to
    // currentcolor.
    type ResolvedValue = computed::Color;

    #[inline]
    fn to_resolved_value(self, context: &Context) -> Self::ResolvedValue {
        let color = match self.0 {
            generics::ColorOrAuto::Color(color) => color,
            generics::ColorOrAuto::Auto => generics::Color::currentcolor(),
        };
        color.to_resolved_value(context)
    }

    #[inline]
    fn from_resolved_value(resolved: Self::ResolvedValue) -> Self {
        generics::CaretColor(generics::ColorOrAuto::Color(
            computed::Color::from_resolved_value(resolved),
        ))
    }
}

impl ToResolvedValue for computed::OutlineColor {
    // The resolved value of `outline-color: auto` depends on `outline-style`:
    // it stays `auto` (serialized as `auto`) when `outline-style` is `auto`,
    // and resolves to currentColor otherwise.
    type ResolvedValue = computed::ColorOrAuto;

    #[inline]
    fn to_resolved_value(self, context: &Context) -> Self::ResolvedValue {
        match self.0 {
            generics::ColorOrAuto::Auto if context.style.get_outline().outline_style.is_auto() => {
                generics::ColorOrAuto::Auto
            },
            generics::ColorOrAuto::Auto => generics::ColorOrAuto::Color(
                computed::Color::currentcolor().to_resolved_value(context),
            ),
            generics::ColorOrAuto::Color(color) => {
                generics::ColorOrAuto::Color(color.to_resolved_value(context))
            },
        }
    }

    #[inline]
    fn from_resolved_value(resolved: Self::ResolvedValue) -> Self {
        match resolved {
            generics::ColorOrAuto::Auto => generics::OutlineColor(generics::ColorOrAuto::Auto),
            generics::ColorOrAuto::Color(color) => generics::OutlineColor(
                generics::ColorOrAuto::Color(computed::Color::from_resolved_value(color)),
            ),
        }
    }
}
