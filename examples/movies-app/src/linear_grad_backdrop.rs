// use floem::view::ViewData;
//
// pub struct LinearGradBackdrop {
//     data: ViewData,
//     gradient: Option<LinearGradient>,
//     content_node: Option<Node>,
// }
//
// pub fn img(image: impl Fn() -> Vec<u8> + 'static) -> Img {
//     img_dynamic(move || image::load_from_memory(&image()).ok().map(Rc::new))
// }
//
// pub(crate) fn img_dynamic(image: impl Fn() -> Option<Rc<DynamicImage>> + 'static) -> Img {
//     let id = Id::next();
//     create_effect(move |_| {
//         id.update_state(image(), false);
//     });
//     LinearGradBackdrop {
//         data: ViewData::new(id),
//         img: None,
//         img_hash: None,
//         img_dimensions: None,
//         content_node: None,
//     }
// }
//
// impl View for LinearGradBackdrop {
//     fn view_data(&self) -> &ViewData {
//         &self.data
//     }
//
//     fn view_data_mut(&mut self) -> &mut ViewData {
//         &mut self.data
//     }
//
//     fn debug_name(&self) -> std::borrow::Cow<'static, str> {
//         "Img".into()
//     }
//
//     fn update(&mut self, cx: &mut crate::context::UpdateCx, state: Box<dyn std::any::Any>) {
//         if let Ok(img) = state.downcast::<Option<Rc<DynamicImage>>>() {
//             self.img_hash = (*img).as_ref().map(|img| {
//                 let mut hasher = Sha256::new();
//                 hasher.update(img.as_bytes());
//                 hasher.finalize().to_vec()
//             });
//             self.img = *img;
//             self.img_dimensions = self.img.as_ref().map(|img| img.dimensions());
//             cx.request_layout(self.id());
//         }
//     }
//
//     fn layout(&mut self, cx: &mut crate::context::LayoutCx) -> taffy::prelude::Node {
//         cx.layout_node(self.id(), true, |cx| {
//             if self.content_node.is_none() {
//                 self.content_node = Some(
//                     cx.app_state_mut()
//                         .taffy
//                         .new_leaf(taffy::style::Style::DEFAULT)
//                         .unwrap(),
//                 );
//             }
//             let content_node = self.content_node.unwrap();
//
//             let (width, height) = self.img_dimensions.unwrap_or((0, 0));
//
//             let style = Style::new()
//                 .width((width as f64).px())
//                 .height((height as f64).px())
//                 .to_taffy_style();
//             let _ = cx.app_state_mut().taffy.set_style(content_node, style);
//
//             vec![content_node]
//         })
//     }
//
//     fn paint(&mut self, cx: &mut crate::context::PaintCx) {
//         if let Some(img) = self.img.as_ref() {
//             let rect = cx.get_content_rect(self.id());
//             cx.draw_img(
//                 floem_renderer::Img {
//                     img,
//                     data: img.as_bytes(),
//                     hash: self.img_hash.as_ref().unwrap(),
//                 },
//                 rect,
//             );
//         }
//     }
// }
