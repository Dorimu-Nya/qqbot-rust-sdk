macro_rules! event_kind {
    (
        $event:ident => $kind:ident {
            $(
                $variant:ident: $pattern:pat
            ),+ $(,)?
        }
    ) => {
        /// 与事件枚举一一对应、不携带事件载荷的注册键。
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumIter)]
        pub enum $kind {
            $($variant),+
        }

        impl $event {
            pub fn to_kind(&self) -> $kind {
                self.into()
            }
        }

        impl From<&$event> for $kind {
            fn from(event: &$event) -> Self {
                use $event::*;

                match event {
                    $($pattern => Self::$variant),+
                }
            }
        }

        impl From<$event> for $kind {
            fn from(event: $event) -> Self {
                Self::from(&event)
            }
        }
    };
}

pub(crate) use event_kind;
