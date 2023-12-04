// Import Crates
use yew::prelude::*;
use web_sys::{wasm_bindgen::JsCast, HtmlButtonElement};

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
    #[derive(Clone, Debug)]
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
    #[derive(Clone, Debug)]
    enum Operation {
        Addition,
        Subtraction,
        Division,
        Multiplication,
        Modulus
    }
    impl Operation {
        fn operators(value: char) -> Result<Self, ()> {
            match value {
                '+' => Ok(Self::Addition),
                '-' => Ok(Self::Subtraction),
                '/' => Ok(Self::Division),
                '*' => Ok(Self::Multiplication),
                '%' => Ok(Self::Modulus),
                _ => Err(()),
            }
        }
        fn maths(&self, inputone:f64, inputtwo:f64) -> f64 {
            use Operation::*;
            match self {
                Addition => inputone + inputtwo,
                Subtraction => inputone - inputtwo,
                Division => inputone / inputtwo,
                Multiplication => inputone * inputtwo,
                Modulus => inputone % inputtwo,
            }
        }
    }
    
    #[derive(PartialEq, Properties)]
    struct CalculatorProps;

    let equation = use_state(|| Equation::default());
    let equation_inputs = use_state_eq(|| EquationInputs::InputOne);
    let result = use_state(String::new);
    let button_pressed = {
        let equation = equation.clone();
        let equation_inputs = equation_inputs.clone();
        let result = result.clone();
        Callback::from(move |event: MouseEvent| {
            // Find out what button was pressed
            let value: String = event.target().unwrap().dyn_into::<HtmlButtonElement>().unwrap().value();
            let is_operation: bool = match format!("{value}").as_str() {
                "+" | "-" | "/" | "*" | "%" => true,
                _ => false
            };
            let is_equal: bool = match format!("{value}").as_str() {
                "=" => true,
                _ => false
            };

            let mut new_equation = (*equation).clone();
            if is_operation {
                new_equation.operation = Some(Operation::operators(value.chars().next().unwrap()).unwrap());
                equation_inputs.set(EquationInputs::InputTwo);
            } else {
                match *equation_inputs {
                    EquationInputs::InputOne => { new_equation.inputone.push_str(&value); },
                    EquationInputs::InputTwo => { new_equation.inputtwo.push_str(&value); },
                }
            }

            equation.set(new_equation);

            if is_equal {
               result.set(equation.operation.clone().unwrap().maths(equation.inputone.parse().unwrap(), equation.inputtwo.parse().unwrap()).to_string()) 
            }

        })
    };

    html! { 
        <div class="dark:bg-black container m-10 mx-auto flex h-screen w-screen rounded-xl bg-gray-600 p-10 drop-shadow-lg">
            <div class="grid gap-2 grid-cols-4 grid-rows-5 w-full h-full">
                <input type="text" value={ format!("{:?}", *result.clone()) } id="result" class="w-full bg-black text-white rounded-xl col-span-4" />
                <button class="bg-gray-400 rounded-lg">{"C"}</button>
                <button class="bg-gray-200 rounded-lg">{"()"}</button>
                <button class="bg-gray-200 rounded-lg" onclick={ button_pressed.clone() }  value="%">{"%"}</button>
                <button class="bg-gray-200 rounded-lg" onclick={ button_pressed.clone() }  value="/">{"÷"}</button>
                <button class="bg-gray-200 rounded-lg" onclick={ button_pressed.clone() } value="7">{"7"}</button>
                <button class="bg-gray-200 rounded-lg" onclick={ button_pressed.clone() }  value="8">{"8"}</button>
                <button class="bg-gray-200 rounded-lg" onclick={ button_pressed.clone() }  value="9">{"9"}</button>
                <button class="bg-gray-200 rounded-lg" onclick={ button_pressed.clone() }  value="*">{"×"}</button>
                <button class="bg-gray-200 rounded-lg" onclick={ button_pressed.clone() }  value="4">{"4"}</button>
                <button class="bg-gray-200 rounded-lg" onclick={ button_pressed.clone() }  value="5">{"5"}</button>
                <button class="bg-gray-200 rounded-lg" onclick={ button_pressed.clone() }  value="6">{"6"}</button>
                <button class="bg-gray-200 rounded-lg" onclick={ button_pressed.clone() }  value="-">{"-"}</button>
                <button class="bg-gray-200 rounded-lg" onclick={ button_pressed.clone() }  value="1">{"1"}</button>
                <button class="bg-gray-200 rounded-lg" onclick={ button_pressed.clone() }  value="2">{"2"}</button>
                <button class="bg-gray-200 rounded-lg" onclick={ button_pressed.clone() }  value="3">{"3"}</button>
                <button class="bg-gray-200 rounded-lg" onclick={ button_pressed.clone() }  value="+">{"+"}</button>
                <button class="bg-gray-200 rounded-lg" onclick={ button_pressed.clone() }  value="0">{"0"}</button>
                <button class="bg-gray-200 rounded-lg" onclick={ button_pressed.clone() }  value=".">{"."}</button>
                <button class="bg-gray-200 rounded-lg">{"⌫"}</button>
                <button class="bg-gray-200 rounded-lg" onclick={ button_pressed.clone() } value="=">{"="}</button>
            </div>
        </div>
    }
}
