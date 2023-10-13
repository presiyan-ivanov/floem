use kurbo::Rect;

use crate::{
    context::{EventCx, UpdateCx},
    id::Id,
    view::{ChangeFlags, View},
    view_tuple::ViewTuple,
};

pub struct Th<V> {
    id: Id,
    child: V,
}

pub fn th<V: View + 'static>(children: V) -> Th<V> {
    let id = Id::next();
    Th {
        id,
        child: children,
    }
}

impl<V: View + 'static> View for Th<V> {
    fn id(&self) -> Id {
        self.id
    }

    fn child(&self, id: Id) -> Option<&dyn View> {
        self.child.child(id)
    }

    fn child_mut(&mut self, id: Id) -> Option<&mut dyn View> {
        self.child.child_mut(id)
    }

    fn children(&self) -> Vec<&dyn View> {
        self.child.children()
    }

    fn children_mut(&mut self) -> Vec<&mut dyn View> {
        self.child.children_mut()
    }

    fn debug_name(&self) -> std::borrow::Cow<'static, str> {
        "Th".into()
    }

    fn update(&mut self, cx: &mut UpdateCx, state: Box<dyn std::any::Any>) -> ChangeFlags {
        if let Ok(state) = state.downcast() {
            self.child = *state;
            cx.request_layout(self.id);
            ChangeFlags::LAYOUT
        } else {
            ChangeFlags::empty()
        }
    }
    fn layout(&mut self, cx: &mut crate::context::LayoutCx) -> taffy::prelude::Node {
        cx.layout_node(self.id, true, |cx| vec![self.child.layout_main(cx)])
    }

    fn compute_layout(&mut self, cx: &mut crate::context::LayoutCx) -> Option<Rect> {
        Some(self.child.compute_layout_main(cx))
    }

    fn event(
        &mut self,
        cx: &mut crate::context::EventCx,
        id_path: Option<&[Id]>,
        event: crate::event::Event,
    ) -> bool {
        if cx.should_send(self.child.id(), &event) {
            self.child.event_main(cx, id_path, event)
        } else {
            false
        }
    }

    fn paint(&mut self, cx: &mut crate::context::PaintCx) {
        self.child.paint_main(cx);
    }

}
