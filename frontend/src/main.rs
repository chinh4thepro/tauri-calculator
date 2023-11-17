use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
pub fn App() -> Html {
    html! { 
        <div class="dark:bg-black container m-10 mx-auto flex h-screen w-screen rounded-xl bg-gray-600 p-10 drop-shadow-lg">
            <div class="grid gap-2 grid-cols-4 grid-rows-5 w-full h-full">
                <input type="text" id="result" class="w-full bg-black text-white rounded-xl col-span-4" />
                <button class="bg-gray-400 rounded-lg">{"AC"}</button>
                <button class="bg-gray-200 rounded-lg">{"()"}</button>
                <button class="bg-gray-200 rounded-lg">{"%"}</button>
                <button class="bg-gray-200 rounded-lg">{"÷"}</button>
                <button class="bg-gray-200 rounded-lg">{"7"}</button>
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
                <button class="bg-gray-200 rounded-lg">{"B"}</button>
                <button class="bg-gray-200 rounded-lg">{"="}</button>
            </div>
        </div>
    }
}
