error_chain! {
    errors {
        Mojang(code: u32, e: ::objects::Error) {
            description("Mojang API error")
            display("Mojang API error: {}", e.error_message)
        }
    }

    foreign_links {
        Net(::tokio_curl::PerformError);
        Curl(::curl::Error);
        Json(::serde_json::error::Error);
    }
}
