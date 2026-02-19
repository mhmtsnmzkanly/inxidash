// Responsibility: Provide theme constants and helper markup used by front-end renderer.
// Design reasoning: Centralizing theme metadata keeps the same list in sync between server and client renderers.
// Extension guidance: Add theme descriptors or expose palette metadata as the UI matures.
// Security considerations: Theme names are static, so there is no risk of injecting user data.

pub const THEME_OPTIONS: &[(&str, &str)] = &[
    ("default", "Balanced"),
    ("dark", "Night"),
    ("royal", "Royal"),
    ("glass", "Glass"),
];
