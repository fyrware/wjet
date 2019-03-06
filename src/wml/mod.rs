macro_rules! wml_impl {

    // opening tag: <WidgetType { key: value }>
    (@wml($tree:ident) += [ <$widget_type:ty { $($prop_key:ident: $prop_value:expr),* }> $($wml:tt)* ]) => {
        $(println!("OPENING TAG : tag=({}) key=({}) value=({})", stringify!($widget_type), stringify!($prop_key), $prop_value);)*

        wml_impl! {
            @wml($tree) += [ $($wml)* ]
        }
    };

    // closing tag: <;WidgetType>
    (@wml($tree:ident) += [ <;$widget_type:ident> $($wml:tt)* ]) => {
        println!("CLOSING TAG : tag=({})", stringify!($widget_type));

        wml_impl! {
            @wml($tree) += [ $($wml)* ]
        }
    };

    // self closing tag: <WidgetType { key: value };>
    (@wml($tree:ident) += [ <$widget_type:ty { $($prop_key:ident: $prop_value:expr),* };> $($wml:tt)* ]) => {
        $(println!("SELF CLOSING TAG : tag=({}) key=({}) value=({})", stringify!($widget_type), stringify!($prop_key), $prop_value);)*

        wml_impl! {
            @wml($tree) += [ $($wml)* ]
        }
    };

    // end of macro:
    (@wml($tree:ident) += [ ]) => {
        println!("END OF MACRO");
    };

    (@wml($tree:ident) += [ @wml($tree:ident) += [ $($wml:tt)* ] ]) => {
        println!("this should never happen; if it does, you're doing something wrong.");
    }
}

#[macro_export]
macro_rules! wml {

    // entry point:
    ($($wml:tt)*) => {
        {
            let mut tree = "eff";

            wml_impl! {
                @wml(tree) += [ $($wml)* ]
            }
        }
    };
}

#[test]
fn oof() {
    wml! {
        <FlexBox { direction: "vertical", justify: "center" }>
            <TabController { active: 0 }>
                <Tab { icon: "home", label: "Home" };>
                <Tab { icon: "about_us", label: "About Us" };>
            <;TabController>
            <Pane { grow: true }>
                <Form { on_submit: false }>
                    <FormField { data_type: "string", id: "username", label: "Username" };>
                    <FormField { data_type: "string", id: "password", label: "Password", mask: '•' };>
                    <Button { emit: "submit" }>
                        <Text { content: "Login" };>
                    <;Button>
                <;Form>
            <;Pane>
        <;FlexBox>
    }
}