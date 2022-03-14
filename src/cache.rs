use crate::types::{GeometryChanged, Axis};
use crate::Node;

/// The Cache stores the result of layout as well as intermediate values for each node
pub trait Cache {
    type Item: for<'w> Node<'w>;

    // Getters

    /// Get the geometry changed bitflag of the node
    fn geometry_changed(&self, node: Self::Item) -> GeometryChanged;

    /// Get the visibility flag of the node
    fn visible(&self, node: Self::Item) -> bool;

    /// Get the computed width of a node
    fn width(&self, node: Self::Item) -> f32;

    /// Get the computed height of a node
    fn height(&self, node: Self::Item) -> f32;

    /// Get the computed x position of a node
    fn posx(&self, node: Self::Item) -> f32;

    /// Get the computed y position of a node
    fn posy(&self, node: Self::Item) -> f32;

    /// Get the computed space to the left of a node
    fn left(&self, node: Self::Item) -> f32;
    /// Get the computed space to the right of a node
    fn right(&self, node: Self::Item) -> f32;
    /// Get the computed space above a node
    fn top(&self, node: Self::Item) -> f32;
    /// Get the computed space below a node
    fn bottom(&self, node: Self::Item) -> f32;

    fn new_width(&self, node: Self::Item) -> f32;
    fn new_height(&self, node: Self::Item) -> f32;

    /// Get the computed maximum width of the child nodes
    fn child_width_max(&self, node: Self::Item) -> f32;

    /// Get the computed sum of the widths of the child nodes
    fn child_width_sum(&self, node: Self::Item) -> f32;

    /// Get the computed maximum width of the child nodes
    fn child_height_max(&self, node: Self::Item) -> f32;

    /// Get the computed sum of the widths of the child nodes
    fn child_height_sum(&self, node: Self::Item) -> f32;

    /// Get the computed maximum grid row
    fn grid_row_max(&self, node: Self::Item) -> f32;

    /// Set the computed maximum grid row size
    fn set_grid_row_max(&mut self, node: Self::Item, value: f32);

    /// Get the computed maximum grid column
    fn grid_col_max(&self, node: Self::Item) -> f32;

    /// Set the computed maximum grid column size
    fn set_grid_col_max(&mut self, node: Self::Item, value: f32);

    // Setters

    fn set_visible(&mut self, node: Self::Item, value: bool);

    fn set_geo_changed(&mut self, node: Self::Item, flag: GeometryChanged, value: bool);

    fn set_child_width_sum(&mut self, node: Self::Item, value: f32);
    fn set_child_height_sum(&mut self, node: Self::Item, value: f32);
    fn set_child_width_max(&mut self, node: Self::Item, value: f32);
    fn set_child_height_max(&mut self, node: Self::Item, value: f32);

    fn horizontal_free_space(&self, node: Self::Item) -> f32;
    fn set_horizontal_free_space(&mut self, node: Self::Item, value: f32);
    fn vertical_free_space(&self, node: Self::Item) -> f32;
    fn set_vertical_free_space(&mut self, node: Self::Item, value: f32);

    fn horizontal_stretch_sum(&self, node: Self::Item) -> f32;
    fn set_horizontal_stretch_sum(&mut self, node: Self::Item, value: f32);
    fn vertical_stretch_sum(&self, node: Self::Item) -> f32;
    fn set_vertical_stretch_sum(&mut self, node: Self::Item, value: f32);

    fn set_width(&mut self, node: Self::Item, value: f32);
    fn set_height(&mut self, node: Self::Item, value: f32);
    fn set_posx(&mut self, node: Self::Item, value: f32);
    fn set_posy(&mut self, node: Self::Item, value: f32);

    fn set_left(&mut self, node: Self::Item, value: f32);
    fn set_right(&mut self, node: Self::Item, value: f32);
    fn set_top(&mut self, node: Self::Item, value: f32);
    fn set_bottom(&mut self, node: Self::Item, value: f32);

    fn set_new_width(&mut self, node: Self::Item, value: f32);
    fn set_new_height(&mut self, node: Self::Item, value: f32);

    fn stack_first_child(&self, node: Self::Item) -> bool;
    fn set_stack_first_child(&mut self, node: Self::Item, value: bool);
    fn stack_last_child(&self, node: Self::Item) -> bool;
    fn set_stack_last_child(&mut self, node: Self::Item, value: bool);

    // generic getters
    fn width_height(&self, node: Self::Item, axis: Axis) -> f32 {
        match axis {
            Axis::X => self.width(node),
            Axis::Y => self.height(node),
        }
    }
    fn posx_posy(&self, node: Self::Item, axis: Axis) -> f32 {
        match axis {
            Axis::X => self.posx(node),
            Axis::Y => self.posy(node),
        }
    }
    fn left_top(&self, node: Self::Item, axis: Axis) -> f32 {
        match axis {
            Axis::X => self.left(node),
            Axis::Y => self.top(node),
        }
    }
    fn right_bottom(&self, node: Self::Item, axis: Axis) -> f32 {
        match axis {
            Axis::X => self.right(node),
            Axis::Y => self.bottom(node),
        }
    }
    fn new_width_height(&self, node: Self::Item, axis: Axis) -> f32 {
        match axis {
            Axis::X => self.new_width(node),
            Axis::Y => self.new_height(node),
        }
    }
    fn child_width_height_max(&self, node: Self::Item, axis: Axis) -> f32 {
        match axis {
            Axis::X => self.child_width_max(node),
            Axis::Y => self.child_height_max(node),
        }
    }
    fn child_width_height_sum(&self, node: Self::Item, axis: Axis) -> f32 {
        match axis {
            Axis::X => self.child_width_sum(node),
            Axis::Y => self.child_height_sum(node),
        }
    }
    fn grid_row_col_max(&self, node: Self::Item, axis: Axis) -> f32 {
        match axis {
            Axis::X => self.grid_row_max(node),
            Axis::Y => self.grid_col_max(node),
        }
    }
    fn set_width_height(&mut self, node: Self::Item, value: f32, axis: Axis) {
        match axis {
            Axis::X => self.set_width(node, value),
            Axis::Y => self.set_height(node, value),
        }
    }
    fn set_posx_posy(&mut self, node: Self::Item, value: f32, axis: Axis) {
        match axis {
            Axis::X => self.set_posx(node, value),
            Axis::Y => self.set_posy(node, value),
        }
    }
    fn set_left_top(&mut self, node: Self::Item, value: f32, axis: Axis) {
        match axis {
            Axis::X => self.set_left(node, value),
            Axis::Y => self.set_top(node, value),
        }
    }
    fn set_right_bottom(&mut self, node: Self::Item, value: f32, axis: Axis) {
        match axis {
            Axis::X => self.set_right(node, value),
            Axis::Y => self.set_bottom(node, value),
        }
    }
    fn set_new_width_height(&mut self, node: Self::Item, value: f32, axis: Axis) {
        match axis {
            Axis::X => self.set_new_width(node, value),
            Axis::Y => self.set_new_height(node, value),
        }
    }
    fn set_child_width_height_max(&mut self, node: Self::Item, value: f32, axis: Axis) {
        match axis {
            Axis::X => self.set_child_width_max(node, value),
            Axis::Y => self.set_child_height_max(node, value),
        }
    }
    fn set_child_width_height_sum(&mut self, node: Self::Item, value: f32, axis: Axis) {
        match axis {
            Axis::X => self.set_child_width_sum(node, value),
            Axis::Y => self.set_child_height_sum(node, value),
        }
    }
    fn set_grid_row_col_max(&mut self, node: Self::Item, value: f32, axis: Axis) {
        match axis {
            Axis::X => self.set_grid_row_max(node, value),
            Axis::Y => self.set_grid_col_max(node, value),
        }
    }
}
