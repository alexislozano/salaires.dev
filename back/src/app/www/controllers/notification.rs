use maud::Markup;

use crate::app::www::components::notification;

pub async fn delete() -> Markup {
    notification::view(None)
}