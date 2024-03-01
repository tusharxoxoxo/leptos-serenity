use leptos::*;
use leptos_use::{
    use_color_mode_with_options, ColorMode, UseColorModeOptions,
    UseColorModeReturn
};
use leptos::html::html;



#[component]
pub fn InfoPage(

) -> impl IntoView {

    let UseColorModeReturn { .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .initial_value(ColorMode::from(html().class_name())),
    );

    let content: Vec<(&'static str)> = vec![
        ("1: The \"What Have I Gotten Myself Into?\" Phase"),
        ("2: The \"This Might Actually (Sort Of) Work\" Phase"),
        ("3: The \"Accidental Brilliance (Maybe?)\" Phase"),
        ("4: The \"World, Prepare Yourself\" Phase"),
        ("5: \"Accidental Tech Mogul\" Phase"),
        ("6: The \"I'm Going to Disneyland!\" Phase"),
        ("7: The \"I'm Going to Disneyland!\" Phase"),
        ("8: The \"I'm Going to Disneyland!\" Phase"),
        ("9: The \"I'm Going to Disneyland!\" Phase"),
        ("10: The \"I'm Going to Disneyland!\" Phase"),

    ];

    view! {
        <div class="dark:bg-[#1a1a1a] dark:text-white flex w-full justify-center">
            <div class="w-10/12 mt-12 font-robotomono">
                <div class="">
                    <p class="text-2xl font-semibold">Roadmap</p>
                    <p>leptos serenity is a fully extensible interface for your leptos applications.</p>
                    <a href="https://github.com/tusharxoxoxo/rust-ganbanzo" class="underline">Come build!</a>
                    <br/><br/>
                    <p>Here are a couple of things that I had in mind:</p>
                    {
                        let cmap = content.into_iter().map(|(title)| {
                        view! {
                            <div class="py-2 w-8/12">
                                <strong class="text-lg">- {title}</strong>
                            </div>
                        }
                    }).collect::<Vec<_>>();

                        cmap
                    }
                </div>
            <p class="my-20">"Although I plan on working on this, it's always fun to work with other people, so if this excites you, feel free to message me on discord or solve an issue on Github!"</p>

            </div>
        </div>
    }
}
