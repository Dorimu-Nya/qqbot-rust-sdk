macro_rules! event_kind {
    (
        $(#[$enum_attr:meta])*
        $vis:vis enum $event:ident {
            $($variants:tt)*
        }
        $(,)?
    ) => {
        event_kind! {
            @parse
            [$(#[$enum_attr])*]
            [$vis]
            [$event]
            []
            []
            []
            $($variants)*
        }
    };

    (@parse
        [$($enum_attr:tt)*]
        [$vis:vis]
        [$event:ident]
        [$($enum_variants:tt)*]
        [$($kind_variants:tt)*]
        [$($match_arms:tt)*]
    ) => {
        ::paste::paste! {
            $($enum_attr)*
            $vis enum $event {
                $($enum_variants)*
            }

            /// 与事件类型列表对应、但不携带事件载荷的枚举。
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumIter)]
            $vis enum [<$event Kind>] {
                $($kind_variants)*
            }

            impl $event {
                pub fn to_kind(&self) -> [<$event Kind>] {
                    self.into()
                }
            }

            impl From<&$event> for [<$event Kind>] {
                fn from(event: &$event) -> Self {
                    match event {
                        $($match_arms)*
                    }
                }
            }

            impl From<$event> for [<$event Kind>] {
                fn from(event: $event) -> Self {
                    Self::from(&event)
                }
            }
        }
    };

    (@parse
        [$($enum_attr:tt)*]
        [$vis:vis]
        [$event:ident]
        [$($enum_variants:tt)*]
        [$($kind_variants:tt)*]
        [$($match_arms:tt)*]
        $(#[$variant_attr:meta])*
        $variant:ident (),
        $($rest:tt)*
    ) => {
        event_kind! {
            @parse
            [$($enum_attr)*]
            [$vis]
            [$event]
            [$($enum_variants)* $(#[$variant_attr])* $variant(),]
            [$($kind_variants)* $variant,]
            [$($match_arms)* $event::$variant() => Self::$variant,]
            $($rest)*
        }
    };

    (@parse
        [$($enum_attr:tt)*]
        [$vis:vis]
        [$event:ident]
        [$($enum_variants:tt)*]
        [$($kind_variants:tt)*]
        [$($match_arms:tt)*]
        $(#[$variant_attr:meta])*
        $variant:ident ($($fields:ty),+ $(,)?),
        $($rest:tt)*
    ) => {
        event_kind! {
            @parse
            [$($enum_attr)*]
            [$vis]
            [$event]
            [$($enum_variants)* $(#[$variant_attr])* $variant($($fields),+),]
            [$($kind_variants)* $variant,]
            [$($match_arms)* $event::$variant(..) => Self::$variant,]
            $($rest)*
        }
    };

    (@parse
        [$($enum_attr:tt)*]
        [$vis:vis]
        [$event:ident]
        [$($enum_variants:tt)*]
        [$($kind_variants:tt)*]
        [$($match_arms:tt)*]
        $(#[$variant_attr:meta])*
        $variant:ident { $($fields:tt)* },
        $($rest:tt)*
    ) => {
        event_kind! {
            @parse
            [$($enum_attr)*]
            [$vis]
            [$event]
            [$($enum_variants)* $(#[$variant_attr])* $variant { $($fields)* },]
            [$($kind_variants)* $variant,]
            [$($match_arms)* $event::$variant { .. } => Self::$variant,]
            $($rest)*
        }
    };

    (@parse
        [$($enum_attr:tt)*]
        [$vis:vis]
        [$event:ident]
        [$($enum_variants:tt)*]
        [$($kind_variants:tt)*]
        [$($match_arms:tt)*]
        $(#[$variant_attr:meta])*
        $variant:ident,
        $($rest:tt)*
    ) => {
        event_kind! {
            @parse
            [$($enum_attr)*]
            [$vis]
            [$event]
            [$($enum_variants)* $(#[$variant_attr])* $variant,]
            [$($kind_variants)* $variant,]
            [$($match_arms)* $event::$variant => Self::$variant,]
            $($rest)*
        }
    };
}

pub(crate) use event_kind;
