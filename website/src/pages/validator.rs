use axum::response::Html;
use rsx::rsx;
use rsxui::components::Validator;

use crate::layout;

pub async fn page() -> Html<String> {
    let email_example = rsx! {
        <Validator hint="Enter valid email address">
            <input type="email" class="input validator" required placeholder="mail@site.com" />
        </Validator>
    };

    let password_hint = r#"Must be more than 8 characters, including
At least one number
At least one lowercase letter
At least one uppercase letter"#;
    let password_pattern = r"(?=.*\d)(?=.*[a-z])(?=.*[A-Z]).{8,}";

    let password_example = rsx! {
        <Validator hint=password_hint>
            <input
                type="password"
                class="input validator"
                required
                placeholder="Password"
                minlength="8"
                pattern={password_pattern}
                title="Must be more than 8 characters, including number, lowercase letter, uppercase letter"
            />
        </Validator>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Validator"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Validator class changes the color of form elements to error or success based on input's validation rules."
            </p>

            <div class="divider">"Email Validator"</div>
            <p class="text-sm text-gray-600 mb-4">"Writing an invalid email address applies error color to the input. Valid email applies success color."</p>
            {email_example}

            <div class="divider">"Password Requirement Validator"</div>
            <p class="text-sm text-gray-600 mb-4">"A multi-line hint with pattern validation."</p>
            {password_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::Validator;",
                "use rsx::rsx;",
                "",
                "// Email validator",
                "let html = rsx! {",
                "    <Validator hint=\"Enter valid email address\">",
                "        <input",
                "            type=\"email\"",
                "            class=\"input validator\"",
                "            required",
                "            placeholder=\"mail@site.com\"",
                "        />",
                "    </Validator>",
                "};",
                "",
                "// Password validator with pattern",
                "let html = rsx! {",
                "    <Validator hint=\"Must be more than 8 characters...\">",
                "        <input",
                "            type=\"password\"",
                "            class=\"input validator\"",
                "            required",
                "            placeholder=\"Password\"",
                "            minlength=\"8\"",
                "            pattern=\"(?=.*\\d)(?=.*[a-z])(?=.*[A-Z]).{8,}\"",
                "        />",
                "    </Validator>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/validator").await)
}
