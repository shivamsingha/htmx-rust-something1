use crate::templates::HelloTemplate;

pub async fn hello() -> HelloTemplate<'static> {
    HelloTemplate { name: "world" }
}
