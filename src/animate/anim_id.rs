use std::{rc::Rc, sync::atomic::AtomicUsize};

use crate::{style::StyleProp, update::ANIM_UPDATE_MESSAGES};

use super::{anim_val::AnimValue, AnimPropKind, AnimUpdateMsg};

static ANIM_ID_GEN: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnimId(usize);

impl AnimId {
    pub fn next() -> Self {
        AnimId(ANIM_ID_GEN.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }

    pub fn from(id: usize) -> Self {
        AnimId(id)
    }

    pub(crate) fn update_style_prop<P: StyleProp>(&self, _prop: P, val: P::Type) {
        ANIM_UPDATE_MESSAGES.with(|msgs| {
            let mut msgs = msgs.borrow_mut();

            msgs.entry(*self)
                .or_insert_with(|| vec![])
                .push(AnimUpdateMsg::Prop {
                    kind: AnimPropKind::Prop {
                        prop: P::prop_ref(),
                    },
                    val: AnimValue::DynProp(Rc::new(val)),
                });
        });
    }
}
