//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

// TODO: something probably that expands similar to:
//
// ```
// #[derive(VisitorAccept)] // Our custom macro
// enum Shape {
//     Circle(Circle),
//     Rectangle(Rectangle),
//     Triangle(Triangle),
// }
//
// trait ShapeVisitor {
//     // Generated by macro for known types
//     fn visit_circle(&mut self, circle: &Circle);
//     fn visit_rectangle(&mut self, rect: &Rectangle);
//     fn visit_triangle(&mut self, tri: &Triangle);
//
//     // Default implementation for unknown types
//     fn visit_unknown(&mut self, _: &dyn Shape) {}
// }
//
// trait Shape {
//     fn accept(&self, visitor: &mut dyn ShapeVisitor);
// }
// ```
//
// NOTE:
