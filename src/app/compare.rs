use eframe::egui::Response;

pub fn response_compare(variable: Response, temp_response: &mut Option<Response>) {
    if let Some(response) = temp_response {
        *response = response.union(variable);
    } else {
        *temp_response = Some(variable);
    }
}
