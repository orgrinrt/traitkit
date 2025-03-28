//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

// TODO: something that does not require Any, and is also thread safe and much more performant
//       if we can't top Any then I suppose this is something we will skip
//
// NOTE: there are several ways to try and approach this:
//       1. generate type ID enums with autogenerated downcast methods
//       2. structural matching / impl injection
//       3. custom type registry with ids and downcast methods

// TODO: not sure which way to go, but here are some ideas:
// ```
// type id enums:
// enum ShapeType { Circle, Rectangle, Triangle }
//
// trait Shape {
//     fn shape_type(&self) -> ShapeType;
//
//     fn as_circle(&self) -> Option<&Circle> {
//         if self.shape_type() == ShapeType::Circle {
//             // Safety: We've verified the type
//             unsafe { Some(std::mem::transmute(self)) }
//         } else {
//             None
//         }
//     }
//     // Similar methods for other types
// }
// ```
// structural:
// ```
// trait Shape {
//     // Each shape implements only relevant method
//     fn as_circle(&self) -> Option<&Circle> { None }
//     fn as_rectangle(&self) -> Option<&Rectangle> { None }
// }
//
// impl Shape for Circle {
//     fn as_circle(&self) -> Option<&Circle> { Some(self) }
// }
// ```
// custom type registry:
// ```
// trait Downcastable {
//     fn type_id(&self) -> TypeId;
//
//     // Implement once in the trait
//     fn downcast_ref<T: 'static + Downcastable>(&self) -> Option<&T> {
//         if self.type_id() == T::static_type_id() {
//             unsafe { Some(std::mem::transmute(self)) }
//         } else {
//             None
//         }
//     }
// }
// ```
