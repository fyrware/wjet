use super::api::State;
use super::api::Style;
use super::api::Tree;
use super::api::Widget;

#[test]
fn it_works() {
    struct Foo;

    impl Widget for Foo {

        fn attach(&self) -> State {
            State {

            }
        }

        fn design(&self) -> Style {
            Style {

            }
        }

        fn render(&self, children: Tree) -> Tree {
            Tree {

            }
        }
    }

    assert_eq!(2 + 2, 4);
}