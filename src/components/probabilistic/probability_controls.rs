use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};

use crate::{
    components::input_items::*,
    pages::probabilistic_algorithms::{get_probabilistic_algorithms, ProbabilisticConfig, ProbabilisticAlgorithmsRoute},
    utils::{gen_u32_vec, knuth_shuffle},
};

#[derive(Properties, Clone, PartialEq)]
pub struct ProbabilisticControlsProps {
    pub config: ProbabilisticConfig,
    pub update_input: Callback<Vec<u32>>,
    pub update_config: Callback<(ProbabilisticConfig, bool)>,
}

#[function_component(SortControls)]
pub fn probabilistic_controls(props: &ProbabilisticControlsProps) -> Html {
    let ProbabilisticControlsProps {
        config,
        update_input,
        update_config,
    } = props.clone();

    let history = use_history().unwrap();

    let algorithm_names = use_state_eq(|| {
        get_probabilistic_algorithms()
            .values()
            .map(|algorithm| algorithm.name.to_string())
            .collect::<Vec<String>>()
    });

    let gen_input = {
        let config = config.clone();

        Callback::from(move |_e: MouseEvent| {
            update_input.emit(knuth_shuffle(gen_u32_vec(config.input_len)));
        })
    };
    let change_input_len = {
        let config = config.clone();
        let update_config = update_config.clone();

        Callback::from(move |input_len| {
            if input_len > 1 && input_len != config.input_len {
                update_config.emit((
                    SortConfig {
                        input_len,
                        ..config.clone()
                    },
                    true,
                ));
            }
        })
    };
    let change_playback_time = {
        let config = config.clone();

        Callback::from(move |playback_time| {
            update_config.emit((
                SortConfig {
                    playback_time,
                    ..config.clone()
                },
                true,
            ));
        })
    };
    let change_algorithm = Callback::from(move |algorithm: String| {
        history.push(ProbabilisticAlgorithmsRoute::ProbabilisticAlgorithm {
            algorithm: algorithm.replace(' ', "-").to_lowercase(),
        });
    });

    html! {
        <div class="probabilistic-controls">
            <Button title="Generate input" onclick={gen_input} />
            <IntInput<usize>
                title="Input length"
                value={props.config.input_len}
                oninput={change_input_len}
                min={2}
            />
            <FloatInput<f32>
                title="Playback time (seconds)"
                value={props.config.playback_time}
                oninput={change_playback_time}
                min={0.0}
            />
            <SelectInput
                title="Algorithm"
                options={(*algorithm_names).clone()}
                selected_value={config.probabilistic_algorithm.name}
                onchange={change_algorithm}
            />
        </div>
    }
}
