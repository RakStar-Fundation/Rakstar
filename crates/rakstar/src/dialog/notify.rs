use super::handler::notify_dialog;

pub fn send(
    dialog_id: u32,
    player_id: u32,
    button_response: u8,
    selected_item: i8,
    answer: String,
) {
    notify_dialog(
        dialog_id,
        player_id,
        button_response,
        selected_item,
        answer.to_string(),
    );
}
