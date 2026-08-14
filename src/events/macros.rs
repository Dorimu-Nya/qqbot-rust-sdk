// `event_kind!` 负责解析一份事件枚举声明，再把解析结果交给专用生成宏：
// 1. 原事件枚举（如 `GuildEvent`）；
// 2. `event_kind_output!` 生成不携带数据的 Kind 及其转换；
// 3. `event_spec_output!` 生成每个变体的强类型标记及 `EventSpec`；
// 4. 主宏自身生成原枚举和读取事件数据的方法。
//
// 宏内部使用 “TT muncher（逐项吞 token）” 模式：每次解析一个变体，
// 把生成结果存入几个累加器，再递归处理剩余变体。
//
// 冒号后的 `meta`、`vis`、`ident`、`ty`、`tt` 是 Rust 固定的片段说明符，不能改名：
// - `:meta`：属性内部的元信息，如 `derive(Debug)`；
// - `:vis`：可见性语法，如 `pub`、`pub(crate)`，也可以为空；
// - `:ident`：一个标识符，如类型名或变体名；
// - `:ty`：一个 Rust 类型；
// - `:tt`：一个 token tree，即单个 token 或一组由括号包围的 token；
// - `$(...)*`：将同一种结构连续匹配零次或多次。
//
// 主要变量速查：
// - `$enum_attributes`：原枚举上方的全部属性，如 `#[derive(Debug)]`；
// - `$visibility`：原枚举的可见性，如 `pub` 或 `pub(crate)`；
// - `$event_name`：原事件枚举的名称，如 `GuildEvent`；
// - `$unparsed_variants` / `$remaining_variants`：入口收到的全部 / 递归中尚未处理的变体；
// - `$event_variant_output`：累积生成的原事件枚举变体代码；
// - `$kind_variant_output`：累积生成的无载荷 Kind 枚举变体代码；
// - `$kind_match_arm_output`：累积生成的 `Event -> EventKind` 匹配臂；
// - `$data_match_arm_output`：累积生成的 `data()` 取载荷匹配臂。
// - `$event_spec_input`：累积传给 EventSpec 生成宏的“标记名 + 载荷类型”；
macro_rules! event_kind_output {
    (
        [$visibility:vis]
        [$event_name:ident]
        [$($kind_variant_output:tt)*]
        [$($kind_match_arm_output:tt)*]
    ) => {
        ::paste::paste! {
            /// 与事件类型列表对应、但不携带事件载荷的枚举。
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumIter)]
            $visibility enum [<$event_name Kind>] {
                $($kind_variant_output)*
            }

            impl $event_name {
                /// 返回当前事件对应的不携带数据的 Kind。
                pub fn to_kind(&self) -> [<$event_name Kind>] {
                    self.into()
                }
            }

            impl From<&$event_name> for [<$event_name Kind>] {
                fn from(event: &$event_name) -> Self {
                    match event {
                        $($kind_match_arm_output)*
                    }
                }
            }

            impl From<$event_name> for [<$event_name Kind>] {
                fn from(event: $event_name) -> Self {
                    Self::from(&event)
                }
            }
        }
    };
}

macro_rules! event_spec_output {
    (
        [$visibility:vis]
        [$event_name:ident]
        [$(($marker_name:ident, $payload_type:ty);)*]
    ) => {
        ::paste::paste! {
            #[doc = concat!("`", stringify!($event_name), "` 的强类型事件标记。")]
            $visibility mod [<$event_name:snake _markers>] {
                use super::*;

                $(
                    #[doc = concat!("`", stringify!($marker_name), "` 事件的类型标记。")]
                    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
                    pub struct $marker_name;

                    impl $crate::events::EventSpec for $marker_name {
                        type Payload = $payload_type;
                    }
                )*
            }
        }
    };
}

macro_rules! event_kind {
    // ===== 对外入口 =====
    // 接收形如 `pub enum Event { A, B(Data) }` 的枚举声明。
    // 这里不会立即解析每个变体，只把原始输入粗分为属性、可见性、枚举名和变体列表。
    (
        // 保留枚举上的 `#[derive(...)]`、文档注释等属性。
        $(#[$enum_attributes:meta])*
        // `$visibility` 匹配可见性，`$event_name` 匹配枚举名。
        $visibility:vis enum $event_name:ident {
            // 此处先把变体整体收为 token，交给下面的 `@parse` 逐个解析。
            $($unparsed_variants:tt)*
        }
        // 允许调用宏时在整个枚举声明后多写一个逗号。
        $(,)?
    ) => {
        // `@parse` 是仅供宏内部递归使用的标记，并不会出现在生成代码中。
        // 此处把刚捕获的 token 重排成“固定信息 + 累加器 + 待处理变体”的内部格式；
        // 后续 `@parse` 每次取出一个变体，将其生成结果追加到对应累加器。
        event_kind! {
            @parse
            // 这三项传给每一轮 `@parse`，最终用于生成类型定义。
            [$(#[$enum_attributes])*]
            [$visibility]
            [$event_name]
            // 以下五个空 `[]` 是五个待填充的累加器。
            // 顺序：原枚举变体、Kind 变体、to_kind 匹配臂、data 匹配臂、
            // EventSpec 生成所需的“标记名 + 载荷类型”。
            []
            []
            []
            []
            []
            // 把尚未解析的全部变体放在最后，开始递归“吞取”。
            $($unparsed_variants)*
        }
    };

    // ===== 逐个变体递归解析 =====
    // 下方每条规则都只描述“当前第一个变体”的完整语法，并以逗号结束；
    // 因此本轮只会吃掉一个变体，逗号之后的全部 token 由
    // `$($remaining_variants:tt)*` 接住，留给下一轮处理。
    //
    // 规则右侧会再次调用整个 `event_kind!` 宏，把更新后的累加器和
    // `$remaining_variants` 一起传入。编译器会重新从上到下检查所有 `@parse` 分支，
    // 选择能匹配下一个变体形状的规则，并非固定跳转到代码位置上的“下一条规则”。
    // 这是编译期间的宏递归，不是运行时函数递归。

    // ===== 解析空元组变体：`Variant()` =====
    // 它没有载荷，但需要保留括号形式，匹配时也写成 `Variant()`。
    (@parse
        // 前八组 `[]` 是上一轮传入的固定信息和五个累加器。
        [$($enum_attributes:tt)*]
        [$visibility:vis]
        [$event_name:ident]
        [$($event_variant_output:tt)*]
        [$($kind_variant_output:tt)*]
        [$($kind_match_arm_output:tt)*]
        [$($data_match_arm_output:tt)*]
        [$($event_spec_input:tt)*]
        // `$variant_attributes` 是当前变体自己的属性；`$variant_name` 是当前变体名。
        $(#[$variant_attributes:meta])*
        $variant_name:ident (),
        // 前面的 `(),` 到逗号为止只吃掉当前变体；这里接住逗号后的全部 token。
        $($remaining_variants:tt)*
    ) => {
        event_kind! {
            @parse
            // 前三个参数（属性、可见性、枚举名）在递归中保持不变。
            [$($enum_attributes)*]
            [$visibility]
            [$event_name]
            // 每个累加器都在旧内容后追加本变体对应的代码。
            [$($event_variant_output)* $(#[$variant_attributes])* $variant_name(),]
            [$($kind_variant_output)* $variant_name,]
            [$($kind_match_arm_output)* $event_name::$variant_name() => Self::$variant_name,]
            // 空变体没有数据，用 `&()` 统一表示“无载荷”。
            [$($data_match_arm_output)* $event_name::$variant_name() => &(),]
            [$($event_spec_input)* ($variant_name, ());]
            // 再次调用整个宏：编译器会从头检查各个 `@parse` 分支，
            // 根据 `$remaining_variants` 开头的形状选择空元组、单字段或单元变体规则。
            // 若它已为空，所有要求存在当前变体的分支都会匹配失败，最终命中递归终点。
            $($remaining_variants)*
        }
    };

    // ===== 解析单字段元组变体：`Variant(Data)` =====
    // 这是有载荷事件唯一允许的形式。
    (@parse
        // 固定信息与已有生成结果，均从上一轮原样接入。
        [$($enum_attributes:tt)*]
        [$visibility:vis]
        [$event_name:ident]
        [$($event_variant_output:tt)*]
        [$($kind_variant_output:tt)*]
        [$($kind_match_arm_output:tt)*]
        [$($data_match_arm_output:tt)*]
        [$($event_spec_input:tt)*]
        // 当前变体：`$variant_name` 是名字，`$payload_type` 是括号中的载荷类型。
        $(#[$variant_attributes:meta])*
        $variant_name:ident ($payload_type:ty),
        // 当前变体之后的内容留给下一轮解析。
        $($remaining_variants:tt)*
    ) => {
        event_kind! {
            @parse
            [$($enum_attributes)*]
            [$visibility]
            [$event_name]
            [$($event_variant_output)* $(#[$variant_attributes])* $variant_name($payload_type),]
            [$($kind_variant_output)* $variant_name,]
            // 转 Kind 时只关心变体名，`..` 忽略其中的数据。
            [$($kind_match_arm_output)* $event_name::$variant_name(..) => Self::$variant_name,]
            // `self` 是引用，match 的默认绑定模式会让 `data` 也是载荷引用。
            [$($data_match_arm_output)* $event_name::$variant_name(data) => data,]
            [$($event_spec_input)* ($variant_name, $payload_type);]
            // 带着更新后的五个累加器，再次从 `$remaining_variants` 的第一个变体开始匹配。
            $($remaining_variants)*
        }
    };

    // ===== 拒绝多字段元组变体：`Variant(A, B, ...)` =====
    // 单独匹配这种错误，可以给调用者比“宏匹配失败”更明确的提示。
    (@parse
        [$($enum_attributes:tt)*]
        [$visibility:vis]
        [$event_name:ident]
        [$($event_variant_output:tt)*]
        [$($kind_variant_output:tt)*]
        [$($kind_match_arm_output:tt)*]
        [$($data_match_arm_output:tt)*]
        [$($event_spec_input:tt)*]
        $(#[$variant_attributes:meta])*
        // `$first_payload_type` 匹配第一个类型，`$remaining_payload_types` 匹配至少一个后续类型；
        // 这样可确定当前变体确实携带两个或更多字段。
        $variant_name:ident ($first_payload_type:ty, $($remaining_payload_types:ty),+ $(,)?),
        $($remaining_variants:tt)*
    ) => {
        // 错误在本轮直接产生，不再把 `$remaining_variants` 传给下一轮。
        compile_error!("event_kind! event variants may carry at most one data field");
    };

    // ===== 拒绝结构体变体：`Variant { field: Type }` =====
    // `data()` 约定每个事件最多对应一个整体载荷，因此不支持具名字段。
    (@parse
        [$($enum_attributes:tt)*]
        [$visibility:vis]
        [$event_name:ident]
        [$($event_variant_output:tt)*]
        [$($kind_variant_output:tt)*]
        [$($kind_match_arm_output:tt)*]
        [$($data_match_arm_output:tt)*]
        [$($event_spec_input:tt)*]
        $(#[$variant_attributes:meta])*
        // `$named_fields` 整体接住 `{ ... }` 中的 token；这里只需识别形状，无须解析字段细节。
        $variant_name:ident { $($named_fields:tt)* },
        $($remaining_variants:tt)*
    ) => {
        // 错误在本轮直接产生，不再继续递归。
        compile_error!("event_kind! event variants must be unit, empty tuple, or single-field tuple variants");
    };

    // ===== 解析单元变体：`Variant` =====
    // 与 `Variant()` 一样没有载荷，但 Rust 的声明和匹配语法都不带括号。
    (@parse
        // 固定信息与五个累加器来自上一轮 `@parse` 调用。
        [$($enum_attributes:tt)*]
        [$visibility:vis]
        [$event_name:ident]
        [$($event_variant_output:tt)*]
        [$($kind_variant_output:tt)*]
        [$($kind_match_arm_output:tt)*]
        [$($data_match_arm_output:tt)*]
        [$($event_spec_input:tt)*]
        // 当前变体只有属性和名字，没有 `()`、载荷类型或 `{}` 字段。
        $(#[$variant_attributes:meta])*
        $variant_name:ident,
        // 后续变体统一收进 `$remaining_variants`，本轮不关心它们的具体形状。
        $($remaining_variants:tt)*
    ) => {
        event_kind! {
            @parse
            [$($enum_attributes)*]
            [$visibility]
            [$event_name]
            [$($event_variant_output)* $(#[$variant_attributes])* $variant_name,]
            [$($kind_variant_output)* $variant_name,]
            [$($kind_match_arm_output)* $event_name::$variant_name => Self::$variant_name,]
            // 无载荷仍返回 `&()`，让 `data()` 对所有变体拥有统一返回类型。
            [$($data_match_arm_output)* $event_name::$variant_name => &(),]
            [$($event_spec_input)* ($variant_name, ());]
            // 更新累加器后递归：有剩余变体就继续分类解析，否则进入终点生成代码。
            $($remaining_variants)*
        }
    };

    // ===== 递归终点：所有变体处理完毕后分派给各生成宏 =====
    // 当末尾已没有待解析的变体时，五个累加器就是完整的生成材料。
    // 注意本规则没有 `$remaining_variants`：只有上一轮传来的待处理 token 已清空时才会命中。
    // 此时前面那些要求匹配 `$variant_name` 的分支均无法命中，编译器才会匹配到本规则。
    // 虽然它写在最后，但 `macro_rules!` 的递归调用仍然可以命中后方规则。
    (@parse
        // 固定信息：一路原样传递到这里，用于恢复原枚举的外壳。
        [$($enum_attributes:tt)*]
        [$visibility:vis]
        [$event_name:ident]
        // 生成信息：前面各轮解析出的片段，分别填入下方对应位置。
        [$($event_variant_output:tt)*]
        [$($kind_variant_output:tt)*]
        [$($kind_match_arm_output:tt)*]
        [$($data_match_arm_output:tt)*]
        [$($event_spec_input:tt)*]
    ) => {
        // 主宏只恢复事件本体和依赖 data 匹配臂的方法。
        $($enum_attributes)*
        $visibility enum $event_name {
            $($event_variant_output)*
        }

        impl $event_name {
            /// 返回当前事件变体携带的数据。
            ///
            /// 不携带数据的事件返回 `()`。
            pub fn data(&self) -> &(dyn ::std::any::Any + Send + Sync) {
                match self {
                    $($data_match_arm_output)*
                }
            }
        }

        // 两个专用宏只接收各自生成代码所必需的解析结果。
        $crate::events::macros::event_kind_output! {
            [$visibility]
            [$event_name]
            [$($kind_variant_output)*]
            [$($kind_match_arm_output)*]
        }

        $crate::events::macros::event_spec_output! {
            [$visibility]
            [$event_name]
            [$($event_spec_input)*]
        }
    };
}

// 将宏限制为 crate 内可见，供各事件模块通过普通路径导入使用。
pub(crate) use {event_kind, event_kind_output, event_spec_output};
