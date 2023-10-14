use floem_reactive::as_child_of_current_scope;
use kurbo::{Rect, Shape};

use std::{hash::Hash};

use crate::{
    context::{EventCx, UpdateCx},
    id::Id,
    view::{ChangeFlags, View},
    view_tuple::ViewTuple,
};

use floem_reactive::Scope;

use super::{scroll, virtual_list, VirtualListItemSize};

// trait ValidTable {}
//
// impl ValidTable for Tbl<Head<Row<impl TableCell>>, Body<Row<impl TableCell>>>{}

pub trait TableCell {}
// impl TableCell for Td<impl View> {}

pub struct Tbl<Head: View, Body: View> {
    id: Id,
    head: Head,
    body: Body,
}

pub fn tbl<V: View + 'static, R>(
    head: Head<Row<impl ViewTuple + 'static>>,
    body: Body<Row<impl ViewTuple + 'static>, R>,
) -> Tbl<Head<Row<impl ViewTuple>>, Body<Row<impl ViewTuple>, R>> {
    Tbl {
        id: Id::next(),
        head,
        body,
    }
}

impl<TC: TableCell + ViewTuple + 'static, R> View for Tbl<Head<Row<TC>>, Body<Row<TC>, R>> {
    fn id(&self) -> Id {
        self.id
    }

    fn child(&self, id: Id) -> Option<&dyn View> {
        None
    }

    fn child_mut(&mut self, id: Id) -> Option<&mut dyn View> {
        None
    }

    fn children(&self) -> Vec<&dyn View> {
        vec![&self.head, &self.body]
    }

    fn children_mut(&mut self) -> Vec<&mut dyn View> {
        vec![&mut self.head, &mut self.body]
    }

    fn debug_name(&self) -> std::borrow::Cow<'static, str> {
        "Tbl".into()
    }

    fn update(
        &mut self,
        _cx: &mut crate::context::UpdateCx,
        _state: Box<dyn std::any::Any>,
    ) -> crate::view::ChangeFlags {
        ChangeFlags::empty()
    }

    fn layout(&mut self, cx: &mut crate::context::LayoutCx) -> taffy::prelude::Node {
        cx.layout_node(self.id, true, |cx| {
            vec![self.head.layout_main(cx), self.body.layout_main(cx)]
        })
    }

    fn compute_layout(&mut self, cx: &mut crate::context::LayoutCx) -> Option<Rect> {
        let head = self.head.compute_layout_main(cx);
        let body = self.body.compute_layout_main(cx);
        Some(head.as_rect().unwrap().union(body.as_rect().unwrap()))
    }

    fn event(
        &mut self,
        cx: &mut crate::context::EventCx,
        id_path: Option<&[Id]>,
        event: crate::event::Event,
    ) -> bool {
        if cx.should_send(self.head.id(), &event) {
            self.head.event_main(cx, id_path, event)
        } else {
            false
        }
    }

    fn paint(&mut self, cx: &mut crate::context::PaintCx) {
        self.head.paint_main(cx);
        self.body.paint_main(cx);
    }
}

pub struct Head<V: View> {
    id: Id,
    child: V,
}

pub fn head<V: ViewTuple + 'static>(child: Row<V>) -> Head<Row<V>> {
    Head {
        id: Id::next(),
        child,
    }
}

impl<V: ViewTuple + 'static> View for Head<Row<V>> {
    fn id(&self) -> Id {
        self.id
    }

    fn child(&self, id: Id) -> Option<&dyn View> {
        if self.child.id() == id {
            Some(&self.child)
        } else {
            None
        }
    }

    fn child_mut(&mut self, id: Id) -> Option<&mut dyn View> {
        if self.child.id() == id {
            Some(&mut self.child)
        } else {
            None
        }
    }

    fn children(&self) -> Vec<&dyn View> {
        vec![&self.child]
    }

    fn children_mut(&mut self) -> Vec<&mut dyn View> {
        vec![&mut self.child]
    }

    fn debug_name(&self) -> std::borrow::Cow<'static, str> {
        "Head".into()
    }

    fn update(
        &mut self,
        _cx: &mut crate::context::UpdateCx,
        _state: Box<dyn std::any::Any>,
    ) -> crate::view::ChangeFlags {
        ChangeFlags::empty()
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

pub struct Body<V: View, ROW>
where
    V: 'static,
    ROW: 'static,
{
    id: Id,
    view_fn: Box<dyn Fn(ROW) -> (V, Scope)>,
}

pub fn body<V, ROWS, ERF, ROW, KF, K, VF, RV>(
    // child: V,
    each_row_fn: ERF,
    key_fn: KF,
    view_fn: VF,
) -> Body<V, ROW>
where
    V: View,
    ERF: Fn() -> ROW + 'static,
    KF: Fn(&ROW) -> K + 'static,
    K: Hash + Eq + 'static,
    ROWS: VirtualListVector<ROW>,
    VF: Fn(ROW) -> V + 'static + Clone,
{
    let mut items_vector = each_row_fn();

    let view_fn = Box::new(as_child_of_current_scope(view_fn));
    scroll(
        virtual_list(
            super::VirtualListDirection::Vertical,
            VirtualListItemSize::Fixed(Box::new(move || 40.0)),
            move || each_row_fn(),
            move |x| key_fn(x),
            move |x| view_fn(x)
        )
        // .style(|s| s.flex_col()),
    )
    .style(|s| {
        s.width(100.pct())
            .height(97.pct())
            .margin_bottom(50.px())
            .padding_bottom(20.px())
    });

    Body {
        id: Id::next(),
        view_fn,
    }
}

impl<V: View, R> View for Body<V, R> {
    fn id(&self) -> Id {
        self.id
    }

    fn child(&self, id: Id) -> Option<&dyn View> {
        let child = self
            .children
            .iter()
            .find(|v| v.as_ref().map(|(v, _)| v.id() == id).unwrap_or(false));
        if let Some(child) = child {
            child.as_ref().map(|(view, _)| view as &dyn View)
        } else {
            None
        }
    }

    fn child_mut(&mut self, id: Id) -> Option<&mut dyn View> {
        let child = self
            .children
            .iter_mut()
            .find(|v| v.as_ref().map(|(v, _)| v.id() == id).unwrap_or(false));
        if let Some(child) = child {
            child.as_mut().map(|(view, _)| view as &mut dyn View)
        } else {
            None
        }
    }

    fn children(&self) -> Vec<&dyn View> {
        self.children
            .iter()
            .filter_map(|child| child.as_ref())
            .map(|child| &child.0 as &dyn View)
            .collect()
    }

    fn children_mut(&mut self) -> Vec<&mut dyn View> {
        self.children
            .iter_mut()
            .filter_map(|child| child.as_mut())
            .map(|child| &mut child.0 as &mut dyn View)
            .collect()
    }

    fn debug_name(&self) -> std::borrow::Cow<'static, str> {
        "Body".into()
    }

    fn update(
        &mut self,
        _cx: &mut crate::context::UpdateCx,
        _state: Box<dyn std::any::Any>,
    ) -> crate::view::ChangeFlags {
        ChangeFlags::empty()
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

pub struct Row<VT> {
    id: Id,
    children: VT,
}

pub fn tr<VT: ViewTuple + 'static>(children: VT) -> Row<VT> {
    let id = Id::next();
    Row { id, children }
}

impl<VT: ViewTuple + 'static> View for Row<VT> {
    fn id(&self) -> Id {
        self.id
    }

    fn child(&self, id: Id) -> Option<&dyn View> {
        self.children.child(id)
    }

    fn child_mut(&mut self, id: Id) -> Option<&mut dyn View> {
        self.children.child_mut(id)
    }

    fn children(&self) -> Vec<&dyn View> {
        self.children.children()
    }

    fn children_mut(&mut self) -> Vec<&mut dyn View> {
        self.children.children_mut()
    }

    fn debug_name(&self) -> std::borrow::Cow<'static, str> {
        "Row".into()
    }

    fn update(&mut self, cx: &mut UpdateCx, state: Box<dyn std::any::Any>) -> ChangeFlags {
        if let Ok(state) = state.downcast() {
            self.children = *state;
            cx.request_layout(self.id);
            ChangeFlags::LAYOUT
        } else {
            ChangeFlags::empty()
        }
    }

    fn event(
        &mut self,
        cx: &mut EventCx,
        id_path: Option<&[Id]>,
        event: crate::event::Event,
    ) -> bool {
        let mut handled = false;
        self.children.foreach_rev(&mut |view| {
            let id = view.id();
            if cx.should_send(id, &event) {
                handled = view.event_main(cx, id_path, event.clone());
                if handled {
                    return true;
                }
            }
            false
        });
        handled
    }

    fn layout(&mut self, cx: &mut crate::context::LayoutCx) -> taffy::prelude::Node {
        cx.layout_node(self.id, true, |cx| {
            let mut nodes = Vec::new();
            self.children.foreach_mut(&mut |view| {
                let node = view.layout_main(cx);
                nodes.push(node);
                false
            });
            nodes
        })
    }

    fn compute_layout(&mut self, cx: &mut crate::context::LayoutCx) -> Option<Rect> {
        let mut layout_rect = Rect::ZERO;
        self.children.foreach_mut(&mut |view| {
            layout_rect = layout_rect.union(view.compute_layout_main(cx));
            false
        });
        Some(layout_rect)
    }

    fn paint(&mut self, cx: &mut crate::context::PaintCx) {
        self.children.foreach_mut(&mut |view| {
            view.paint_main(cx);
            false
        });
    }
}
