pub fn compute() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch::style::Style {
                flex_basis: stretch::style::Dimension::Points(50f32),
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch::style::Style { flex_direction: stretch::style::FlexDirection::Column, ..Default::default() },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
}
