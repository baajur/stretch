#[test]
fn align_items_flex_end_child_without_margin_bigger_than_parent() {
    let mut stretch = stretch::Stretch::new();
    let node00 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(70f32),
                    height: stretch::style::Dimension::Points(70f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = stretch
        .new_node(
            stretch::style::Style { align_items: stretch::style::AlignItems::FlexEnd, ..Default::default() },
            &[node00],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch::style::Style {
                align_items: stretch::style::AlignItems::Center,
                justify_content: stretch::style::JustifyContent::Center,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(50f32),
                    height: stretch::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 50f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 70f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 70f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, -10f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, -10f32);
    assert_eq!(stretch.layout(node00).unwrap().size.width, 70f32);
    assert_eq!(stretch.layout(node00).unwrap().size.height, 70f32);
    assert_eq!(stretch.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node00).unwrap().location.y, 0f32);
}
