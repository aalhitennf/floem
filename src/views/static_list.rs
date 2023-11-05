use crate::{
    context::{EventCx, UpdateCx},
    id::Id,
    style::Style,
    view::{ChangeFlags, View},
};
use kurbo::Rect;
use taffy::style::FlexDirection;

pub struct StaticList {
    id: Id,
    children: Vec<Box<dyn View>>,
}

pub fn static_list<V>(iterator: impl IntoIterator<Item = V>) -> StaticList
where
    V: View + 'static,
{
    StaticList {
        id: Id::next(),
        children: iterator
            .into_iter()
            .map(|v| -> Box<dyn View> { Box::new(v) })
            .collect(),
    }
}

impl View for StaticList {
    fn id(&self) -> Id {
        self.id
    }

    fn view_style(&self) -> Option<crate::style::Style> {
        Some(Style::new().flex_direction(FlexDirection::Column))
    }

    fn child(&self, id: Id) -> Option<&dyn View> {
        self.children
            .iter()
            .find(|v| v.id() == id)
            .map(|child| child as &dyn View)
    }

    fn child_mut(&mut self, id: Id) -> Option<&mut dyn View> {
        self.children
            .iter_mut()
            .find(|v| v.id() == id)
            .map(|child| child as &mut dyn View)
    }

    fn children(&self) -> Vec<&dyn View> {
        self.children
            .iter()
            .map(|child| child as &dyn View)
            .collect()
    }

    fn children_mut(&mut self) -> Vec<&mut dyn View> {
        self.children
            .iter_mut()
            .map(|child| child as &mut dyn View)
            .collect()
    }

    fn debug_name(&self) -> std::borrow::Cow<'static, str> {
        "StaticList".into()
    }

    fn update(
        &mut self,
        _cx: &mut UpdateCx,
        _state: Box<dyn std::any::Any>,
    ) -> crate::view::ChangeFlags {
        ChangeFlags::empty()
    }

    fn style(&mut self, cx: &mut crate::context::StyleCx) {
        for child in &mut self.children {
            cx.style_view(child);
        }
    }

    fn layout(&mut self, cx: &mut crate::context::LayoutCx) -> taffy::prelude::Node {
        cx.layout_node(self.id, true, |cx| {
            let nodes = self
                .children
                .iter_mut()
                .map(|child| cx.layout_view(child))
                .collect::<Vec<_>>();
            nodes
        })
    }

    fn compute_layout(&mut self, cx: &mut crate::context::LayoutCx) -> Option<Rect> {
        let mut layout_rect = Rect::ZERO;
        for child in &mut self.children {
            layout_rect = layout_rect.union(cx.compute_view_layout(child));
        }
        Some(layout_rect)
    }

    fn event(
        &mut self,
        cx: &mut EventCx,
        id_path: Option<&[Id]>,
        event: crate::event::Event,
    ) -> bool {
        for child in self.children.iter_mut() {
            if cx.view_event(child, id_path, event.clone()) {
                return true;
            }
        }
        false
    }

    fn paint(&mut self, cx: &mut crate::context::PaintCx) {
        for child in self.children.iter_mut() {
            cx.paint_view(child);
        }
    }
}