//
// pub fn suspense<V>(
//     /// Returns a fallback UI that will be shown while `async` [`Resource`](leptos_reactive::Resource)s are still loading. By default this is the empty view.
//     fallback: ViewFn,
//     /// Children will be displayed once all `async` [`Resource`](leptos_reactive::Resource)s have resolved.
//     children: Rc<dyn Fn() -> V>,
// ) -> impl IntoView
// where
//     V: IntoView + 'static,
// {
//     let orig_children = children;
//     let context = SuspenseContext::new();
//
//     #[cfg(not(any(feature = "csr", feature = "hydrate")))]
//     let owner =
//         Owner::current().expect("<Suspense/> created with no reactive owner");
//
//     let current_id = HydrationCtx::next_component();
//
//     // provide this SuspenseContext to any resources below it
//     // run in a memo so the children are children of this parent
//     #[cfg(not(feature = "hydrate"))]
//     let children = create_memo({
//         let orig_children = Rc::clone(&orig_children);
//         move |_| {
//             provide_context(context);
//             orig_children().into_view()
//         }
//     });
//     // #[cfg(feature = "hydrate")]
//     // let children = create_memo({
//     //     let orig_children = Rc::clone(&orig_children);
//     //     move |_| {
//     //         provide_context(context);
//     //         if SharedContext::fragment_has_local_resources(
//     //             &current_id.to_string(),
//     //         ) {
//     //             HydrationCtx::with_hydration_off({
//     //                 let orig_children = Rc::clone(&orig_children);
//     //                 move || orig_children().into_view()
//     //             })
//     //         } else {
//     //             orig_children().into_view()
//     //         }
//     //     }
//     // });
//
//     // likewise for the fallback
//     let fallback = create_memo({
//         move |_| {
//             provide_context(context);
//             fallback.run()
//         }
//     });
//
//     #[cfg(any(feature = "csr", feature = "hydrate"))]
//     let ready = context.ready();
//
//     // let child = DynChild::new({
//     //     move || {
//     //         // pull lazy memo before checking if context is ready
//     //         let children_rendered = children.get_untracked();
//     //
//     //         #[cfg(any(feature = "csr", feature = "hydrate"))]
//     //         {
//     //             if ready.get() {
//     //                 children_rendered
//     //             } else {
//     //                 fallback.get_untracked()
//     //             }
//     //         }
//     //     }
//     // })
//     // .into_view();
//     let core_component = match child {
//         leptos_dom::View::CoreComponent(repr) => repr,
//         _ => unreachable!(),
//     };
//
//     HydrationCtx::continue_from(current_id);
//     HydrationCtx::next_component();
//
//     leptos_dom::View::Suspense(current_id, core_component)
// }


pub fn create_local_resource_with_initial_value<S, T, Fu>(
    source: impl Fn() -> S + 'static,
    fetcher: impl Fn(S) -> Fu + 'static,
    initial_value: Option<T>,
) -> Resource<S, T>
where
    S: PartialEq + Clone + 'static,
    T: 'static,
    Fu: Future<Output = T> + 'static,
{
    let resolved = initial_value.is_some();
    let (value, set_value) = create_signal(initial_value);

    let (loading, set_loading) = create_signal(false);

    let fetcher = Rc::new(move |s| {
        Box::pin(fetcher(s)) as Pin<Box<dyn Future<Output = T>>>
    });
    let source = create_memo(move |_| source());

    let r = Rc::new(ResourceState {
        value,
        set_value,
        loading,
        set_loading,
        source,
        fetcher,
        resolved: Rc::new(Cell::new(resolved)),
        scheduled: Rc::new(Cell::new(false)),
        version: Rc::new(Cell::new(0)),
        suspense_contexts: Default::default(),
    });

    let id = with_runtime(|runtime| {
        let r = Rc::clone(&r) as Rc<dyn UnserializableResource>;
        let id = runtime.create_unserializable_resource(r);
        runtime.push_scope_property(ScopeProperty::Resource(id));
        id
    })
    .expect("tried to create a Resource in a runtime that has been disposed.");

    // This is a local resource, so we're always going to handle it on the
    // client
    create_render_effect({
        let r = Rc::clone(&r);
        move |_| r.load(false, id)
    });

    Resource {
        id,
        source_ty: PhantomData,
        out_ty: PhantomData,
    }
}
