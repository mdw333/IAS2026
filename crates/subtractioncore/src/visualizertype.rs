//use crate::{newutilities::{gcd, gcd3}, point::Point};
//use eframe::egui;
//use egui::Color32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VisualizerType {
    MainRegionHexagon,
    BoundaryHexagon,
    Triangle,
}

impl std::fmt::Display for VisualizerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           VisualizerType::MainRegionHexagon => write!(f, "Main Region Hexagon"),
           VisualizerType::BoundaryHexagon => write!(f, "Boundary Hexagon"),
           VisualizerType::Triangle => write!(f, "Triangle"),
       }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TriangleDirection {
    WedgeStartingAtTop,
    WedgeStartingAtLL,
    WedgeStartingAtLR,
    VeeStartingAtBottom,
    VeeStartingAtUL,
    VeeStartingAtUR,
}

impl std::fmt::Display for TriangleDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           TriangleDirection::WedgeStartingAtTop => write!(f, "Wedge Starting At Top"),
           TriangleDirection::WedgeStartingAtLL => write!(f, "Wedge Starting At LL"),
           TriangleDirection::WedgeStartingAtLR => write!(f, "Wedge Starting At LR"),
           TriangleDirection::VeeStartingAtBottom => write!(f, "Vee Starting At Bottom"),
           TriangleDirection::VeeStartingAtUL => write!(f, "Vee Starting At UL"),
           TriangleDirection::VeeStartingAtUR => write!(f, "Vee Starting At UR"),
       }
    }
}

