use crate::core::canvas::location::Location;
use crate::core::simulation::component::ComponentIdx;

pub struct CanvasComponent {
    pub component: ComponentIdx,
    pub loc: Location,
}