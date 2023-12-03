// Import Crates
use yew::prelude::*;
use web_sys::{wasm_bindgen::JsCast, EventTarget, HtmlInputElement};

fn main() {
    yew::Renderer::<App>::new().render();
}

// Entire code for calculator
#[function_component]
pub fn App() -> Html {
    #[derive(PartialEq, Clone, Copy)]
    enum EquationInputs {
        InputOne,
        InputTwo
    }

    // Layout for the equation and determine types
    #[derive(Clone)]
    struct Equation {
        inputone: String,
        inputtwo: String,
        operation: Option<Operation>
    }
    impl Default for Equation {
        fn default() -> Self{
            Self { inputone: String::new(), inputtwo: String::new(), operation: None }
        }
    }

    // Find out what operation it is
    #[derive(Clone)]
    enum Operation {
        Addition,
        Subtraction,
        Division,
        Multiplication
    }
    impl TryFrom<char> for Operation {
        type Error = ();
        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '+' => Ok(Self::Addition),
                '-' => Ok(Self::Subtraction),
                '/' => Ok(Self::Division),
                '*' => Ok(Self::Multiplication),
                _ => Err(()),
            }
        }
    }

    #[derive(PartialEq, Properties)]
    struct CalculatorProps;
    #[function_component(Calculator)]
    fn calculator_component(props: &CalculatorProps) -> Html {
        let equation = use_state(|| Equation::default());
        let equation_inputs = use_state_eq(|| EquationInputs::InputOne);

        let button_pressed = {
            let equation = equation.clone();
            let equation_inputs = equation_inputs.clone();
            Callback::from(move |event: HtmlInputElement| {
                // Find out what button was pressed
                let value: char = unimplemented!();
                let is_operation: bool = unimplemented!();

                let mut new_equation = (*equation).clone();
                if is_operation {
                    new_equation.operation = Some(Operation::try_from(value).unwrap());
                    equation_inputs.set(EquationInputs::InputTwo);
                } else {
                    match *equation_inputs {
                        EquationInputs::InputOne => { new_equation.inputone.push(value); },
                        EquationInputs::InputTwo => { new_equation.inputtwo.push(value); },
                    }
                }
                equation.set(new_equation);
            })
        };
        todo!()
    }

    html! { 
        <div class="dark:bg-black container m-10 mx-auto flex h-screen w-screen rounded-xl bg-gray-600 p-10 drop-shadow-lg">
            <div class="grid gap-2 grid-cols-4 grid-rows-5 w-full h-full">
                <input type="text" id="result" class="w-full bg-black text-white rounded-xl col-span-4" />
                <button class="bg-gray-400 rounded-lg">{"AC"}</button>
                <button class="bg-gray-200 rounded-lg">{"()"}</button>
                <button class="bg-gray-200 rounded-lg">{"%"}</button>
                <button class="bg-gray-200 rounded-lg">{"÷"}</button>
                <button class="bg-gray-200 rounded-lg" value="7">{"7"}</button>
                <button class="bg-gray-200 rounded-lg">{"8"}</button>
                <button class="bg-gray-200 rounded-lg">{"9"}</button>
                <button class="bg-gray-200 rounded-lg">{"X"}</button>
                <button class="bg-gray-200 rounded-lg">{"4"}</button>
                <button class="bg-gray-200 rounded-lg">{"5"}</button>
                <button class="bg-gray-200 rounded-lg">{"6"}</button>
                <button class="bg-gray-200 rounded-lg">{"-"}</button>
                <button class="bg-gray-200 rounded-lg">{"1"}</button>
                <button class="bg-gray-200 rounded-lg">{"2"}</button>
                <button class="bg-gray-200 rounded-lg">{"3"}</button>
                <button class="bg-gray-200 rounded-lg">{"+"}</button>
                <button class="bg-gray-200 rounded-lg">{"0"}</button>
                <button class="bg-gray-200 rounded-lg">{"."}</button>
                <button class="bg-gray-200 rounded-lg">{"⌫"}</button>
                <button class="bg-gray-200 rounded-lg">{"="}</button>
            </div>
        </div>
    }
}
