use super::api::Tree;

macro_rules! wml_impl {

    // opening tag: <WidgetType { key: value }>
    ([ $tree:ident ] += [ <$widget_type:ty { $($prop_key:ident: $prop_value:expr),* }> $($wml:tt)* ]) => {
        println!("OPENING TAG: {} {{", stringify!($widget_type));
        $(println!("    {} => {}", stringify!($prop_key), $prop_value);)*
        println!("}}\n");

        wml_impl! {
            [ $tree ] += [ $($wml)* ]
        }
    };

    // closing tag: <;WidgetType>
    ([ $tree:ident ] += [ <;$widget_type:ty> $($wml:tt)* ]) => {
        println!("CLOSING TAG: {}\n", stringify!($widget_type));

        wml_impl! {
            [ $tree ] += [ $($wml)* ]
        }
    };

    // self closing tag: <WidgetType { key: value };>
    ([ $tree:ident ] += [ <$widget_type:ty { $($prop_key:ident: $prop_value:expr),* };> $($wml:tt)* ]) => {
        println!("SELF CLOSING TAG: {} {{", stringify!($widget_type));
        $(println!("    {} => {}", stringify!($prop_key), $prop_value);)*
        println!("}}\n");

        wml_impl! {
            [ $tree ] += [ $($wml)* ]
        }
    };

    // for loop: for { x in y } { <WidgetType {};> }
    ([ $tree:ident ] += [ for { $var:ident in $iter:expr } { $($repeat:tt)* } $($wml:tt)* ]) => {
        println!("FOR LOOP: {} in {}\n", stringify!($var), stringify!($iter));

        for $var in $iter {
            wml_impl! {
                [ $tree ] += [ $($repeat)* ]
            }
        }

        wml_impl! {
            [ $tree ] += [ $($wml)* ]
        }
    };

    // if statement: if { condition } { <WidgetType {};>
    ([ $tree:ident ] += [ if { $cond:expr } { $($then:tt)* } $($wml:tt)* ]) => {
        println!("IF STMT: {}\n", stringify!($cond));

        if $cond {
            wml_impl! {
                [ $tree ] += [ $($then)* ]
            }
        }

        wml_impl! {
            [ $tree ] += [ $($wml)* ]
        }
    };

    // end of statement: <;>
    ([ $tree:ident ] += [ <;> $($wml:tt)* ]) => {
        println!("END OF STMT\n");

        wml_impl! {
            [ $tree ] += [ $($wml)* ]
        }
    };

    // end of macro:
    ([ $tree:ident ] += [ ]) => {
        println!("END OF MACRO");
    };
}

#[macro_export]
macro_rules! wml {

    // beginning of macro:
    ($($wml:tt)*) => {
        {
            let mut tree = Tree::new();

            wml_impl! {
                [ tree ] += [ $($wml)* ]
            };

            tree
        }
    };
}

#[test]
fn oof() {
    let about_us_enabled = true;

    wml! {
        <FlexBox { direction: "vertical", justify: "center" }>
            <TabController { active: 0 }>
                <Tab { icon: "home", label: "Home" };>

                if { about_us_enabled } {
                    <Tab { icon: "about_us", label: "About Us" };>
                }

            <;TabController>
            <List { title: "Title Here" }>

                for { i in 0..10 } {
                    <ListItem { x: 0 };>
                }

            <;List>
        <;FlexBox>
    };
}