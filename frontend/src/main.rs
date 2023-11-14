use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
pub fn App() -> Html {
    html! {
        <div class="dark:bg-black container m-10 mx-auto flex h-screen w-screen rounded-xl bg-gray-600 p-10 drop-shadow-lg">
            <table class="w-full">
                <tr>
                    <td colspan="4"><input type="text" id="result" class="w-full bg-black text-white rounded-xl" /></td>
                </tr>
                <tr>
                    <td><input type="button" value="AC"/></td>
                    <td><input type="button" value="()"/></td>
                    <td><input type="button" value="%"/></td>
                    <td><input type="button" value="/"/></td>
                </tr>
                <tr>
                    <td><input type="button" value="7"/></td>
                    <td><input type="button" value="8"/></td>
                    <td><input type="button" value="9"/></td>
                    <td><input type="button" value="*"/></td>
                </tr>
                <tr>
                    <td><input type="button" value="4"/></td>
                    <td><input type="button" value="5"/></td>
                    <td><input type="button" value="6"/></td>
                    <td><input type="button" value="-"/></td>
                </tr>
                <tr>
                    <td><input type="button" value="1"/></td>
                    <td><input type="button" value="2"/></td>
                    <td><input type="button" value="3"/></td>
                    <td><input type="button" value="+"/></td>
                </tr>
                <tr>
                    <td><input type="button" value="0"/></td>
                    <td><input type="button" value="."/></td>
                    <td><input type="button" value="back"/></td>
                    <td><input type="button" value="="/></td>
                </tr>
            </table>
        </div>
    }
}
