use std::{ops::RangeInclusive, rc::Rc};

use crate::{
    statements::ComponentList,
    model::{BoundingBox, ComponentSet, CoordinateIndex, Quality, Segment},
};

#[allow(clippy::module_name_repetitions)]
pub struct ComponentSetBuilder {
    component_counter: i32,

    name: Rc<str>,
    // Coordinates are stored with their raw time values for analysis later
    coordinates: Vec<BuilderCoordinate>,
    components: Vec<BuilderComponent>,
    segments: Vec<Segment>,
    bounding_boxes: Vec<BoundingBox>,

    segment_statements: Vec<BuilderSegment>,
}

impl Default for ComponentSetBuilder {
    fn default() -> Self {
        Self {
            component_counter: 0,
            name: String::new().into(),
            coordinates: Vec::default(),
            components: Vec::default(),
            segments: Vec::default(),
            bounding_boxes: Vec::default(),
            segment_statements: Vec::default(),
        }
    }
}

struct BuilderSegment {
    hierarchy: Rc<str>,
    component_list: ComponentList,
    quality: Option<Quality>,
    label: Option<Rc<str>>,
}

struct BuilderCoordinate {
    pub x_position: f64,
    pub y_position: f64,
    pub time: f64,
    pub pressure: Option<f64>,
    pub z_position: Option<f64>,
    pub button: Option<f64>,
    pub rho: Option<f64>,
    pub theta: Option<f64>,
    pub phi: Option<f64>,
}

enum BuilderComponent {
    PenDown(RangeInclusive<CoordinateIndex>),
    PenUp(RangeInclusive<CoordinateIndex>),
    Dt(f64),
}

impl ComponentSetBuilder {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.coordinates.is_empty()
    }

    #[must_use]
    pub fn name(mut self, name: Rc<str>) -> Self {
        self.name = name;
        self
    }

    fn add_coordinates(
        mut self,
        mut new_coordinates: Vec<BuilderCoordinate>,
        component: fn(RangeInclusive<CoordinateIndex>) -> BuilderComponent,
    ) -> Self {
        let start_idx = self.coordinates.len();
        let component_size = new_coordinates.len();

        self.coordinates.append(&mut new_coordinates);

        let end_idx = start_idx + component_size - 1;
        self.components.push(component(start_idx..=end_idx));

        // If the component is empty, don't increment the component counter
        if component_size == 0 {
            return self;
        }

        self.component_counter += 1;
        self
    }

    #[must_use]
    pub fn pen_down(self, new_coordinates: Vec<BuilderCoordinate>) -> Self {
        self.add_coordinates(new_coordinates, BuilderComponent::PenDown)
    }

    #[must_use]
    pub fn pen_up(self, new_coordinates: Vec<BuilderCoordinate>) -> Self {
        self.add_coordinates(new_coordinates, BuilderComponent::PenUp)
    }

    #[must_use]
    pub fn dt(mut self, dt: f64) -> Self {
        // Because Dt is an empty component, we don't increment the component counter
        self.components.push(BuilderComponent::Dt(dt));
        self
    }

    #[must_use]
    pub fn segment(
        mut self,
        hierarchy: Rc<str>,
        component_list: ComponentList,
        quality: Option<Quality>,
        label: Option<Rc<str>>,
    ) -> Self {
        self.segment_statements.push(BuilderSegment {
            hierarchy,
            component_list,
            quality,
            label,
        });
        self
    }

    #[must_use]
    pub fn build(self) -> ComponentSet {
        todo!()
    }
}
