macro_rules! impl_properties {
    ($name: ident) => {
        impl ImplProperties for $name {
            fn get_style_mut(&mut self) -> &mut Style {
                &mut self.properties.style
            }

            fn get_id(&self) -> Option<&str> {
                self.properties.id.as_deref()
            }

            fn id(mut self, _id: &str) -> Self
            where
                Self: Sized,
            {
                self.properties.id = Some(_id.to_string());
                self
            }

            fn remove_id(&mut self) {
                self.properties.id = None;
            }

            fn get_link(&self) -> Option<&Link> {
                Option::from(&self.properties.link)
            }

            fn link(mut self, _link: &str, _new_tab: bool) -> Self
            where
                Self: Sized,
            {
                self.properties.link = Some(Link::from(_link, _new_tab));
                self
            }

            fn remove_link(&mut self) {
                self.properties.link = None;
            }

            fn get_clip(&self) -> bool {
                self.properties.clip
            }

            fn clip(mut self) -> Self {
                self.properties.clip = true;
                self
            }

            fn remove_clip(&mut self) {
                self.properties.clip = false;
            }

            fn clip_path(mut self, element: &dyn Element) -> Self
            where
                Self: Sized,
            {
                if let Some(id) = element.get_id() {
                    self.properties.clip_path = Some(id.to_string());
                }
                self
            }

            fn get_transforms(&self) -> &[Transform] {
                &self.properties.transforms
            }

            fn transform(mut self, transform: Transform) -> Self {
                self.properties.transforms.push(transform);
                self
            }

            fn remove_transforms(&mut self) {
                self.properties.transforms = Vec::new();
            }
        }
    };
}
