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
        [$($data_arms:tt)*]
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

                /// 返回当前事件变体携带的数据。
                ///
                /// 不携带数据的事件返回 `()`。
                pub fn data(&self) -> &(dyn ::std::any::Any + Send + Sync) {
                    match self {
                        $($data_arms)*
                    }
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
        [$($data_arms:tt)*]
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
            [$($data_arms)* $event::$variant() => &(),]
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
        [$($data_arms:tt)*]
        $(#[$variant_attr:meta])*
        $variant:ident ($field:ty),
        $($rest:tt)*
    ) => {
        event_kind! {
            @parse
            [$($enum_attr)*]
            [$vis]
            [$event]
            [$($enum_variants)* $(#[$variant_attr])* $variant($field),]
            [$($kind_variants)* $variant,]
            [$($match_arms)* $event::$variant(..) => Self::$variant,]
            [$($data_arms)* $event::$variant(data) => data,]
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
        [$($data_arms:tt)*]
        $(#[$variant_attr:meta])*
        $variant:ident ($first_field:ty, $($remaining_fields:ty),+ $(,)?),
        $($rest:tt)*
    ) => {
        compile_error!("event_kind! event variants may carry at most one data field");
    };

    (@parse
        [$($enum_attr:tt)*]
        [$vis:vis]
        [$event:ident]
        [$($enum_variants:tt)*]
        [$($kind_variants:tt)*]
        [$($match_arms:tt)*]
        [$($data_arms:tt)*]
        $(#[$variant_attr:meta])*
        $variant:ident { $($fields:tt)* },
        $($rest:tt)*
    ) => {
        compile_error!("event_kind! event variants must be unit, empty tuple, or single-field tuple variants");
    };

    (@parse
        [$($enum_attr:tt)*]
        [$vis:vis]
        [$event:ident]
        [$($enum_variants:tt)*]
        [$($kind_variants:tt)*]
        [$($match_arms:tt)*]
        [$($data_arms:tt)*]
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
            [$($data_arms)* $event::$variant => &(),]
            $($rest)*
        }
    };
}

pub(crate) use event_kind;
