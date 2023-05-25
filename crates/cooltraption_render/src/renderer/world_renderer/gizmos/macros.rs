#[macro_export]
macro_rules! unique_id {
    () => {{
        use lazy_static::lazy_static;
        use uuid::Uuid;
        lazy_static! {
            static ref UNIQUE_ID: Uuid = Uuid::new_v4();
        }
        *UNIQUE_ID
    }};
}

#[macro_export]
macro_rules! rect {
    ($bounding_box:expr, $color:expr) => {
        $crate::world_renderer::gizmos::shape(unique_id!(), Shape::Rect, $color, $bounding_box)
    };
}

#[macro_export]
macro_rules! ellipse {
    ($bounding_box:expr, $color:expr) => {
        $crate::world_renderer::gizmos::shape(unique_id!(), Shape::Ellipse, $color, $bounding_box)
    };
}
