use rama_http::headers::x_robots_tag::DirectiveDateTime;
use rama_http::layer::har::spec::Page;

#[test]
fn har_preserves_utc_and_fractional_timestamp_spelling() {
    for (stored, expected) in [
        (
            "1969-12-31T21:59:59.999999999-02:00",
            "1969-12-31T23:59:59.999999999+00:00",
        ),
        (
            "2026-11-25T15:20:00.500-02:00",
            "2026-11-25T17:20:00.500+00:00",
        ),
    ] {
        let page: Page = serde_json::from_value(serde_json::json!({
            "startedDateTime": stored,
            "id": "fixture",
            "title": "fixture",
            "pageTimings": {}
        }))
        .unwrap();

        assert_eq!(serde_json::to_value(page).unwrap()["startedDateTime"], expected);
    }
}

#[test]
fn robots_directive_preserves_utc_and_fractional_timestamp_spelling() {
    let date: DirectiveDateTime = "2026-11-25T15:20:00.500-02:00".parse().unwrap();

    assert_eq!(date.to_string(), "2026-11-25T17:20:00.500+00:00");
}
