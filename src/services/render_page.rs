use chrono::Datelike;

pub async fn render_page_service() -> String {
    let current_year = chrono::Local::now().year();
    let template = r##"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Git Garden</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Charm:wght@400;700&family=Noto+Sans&display=swap" rel="stylesheet">
    <meta name="color-scheme" content="dark light" />
    <style>
        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }
        html {
            height: 100%;
            font-family: "Noto Sans", sans-serif;
        }
        body {
            display: flex;
            min-height: 100%;
            margin: 0;
            padding: 40px 20px;
            justify-content: center;
            align-items: center;
            flex-direction: column;
            background-color: #f5f5f5;
        }
        header {
            margin-bottom: 2rem;
        }
        footer {
            margin-top: auto;
        }
        .container {
            width: 100%;
            max-width: 800px;
            text-align: center;
        }
        h1 {
            font-size: 2rem;
            margin-bottom: 1rem;
            font-family: 'Charm', cursive;
        }
        p {
            font-size: 1.2rem;
        }
        p:not(:last-child) {
            margin-bottom: 1rem;
        }
        .small {
            font-size: 0.8rem;
        }
        a {
            color: #0366d6;
            text-decoration: none;
        }
        #form {
            display: flex;
            justify-content: center;
            margin-bottom: 1rem;
            flex-direction: column;
        }
        #form .input {
            display: flex;
            margin-bottom: 1rem;
            justify-content: center;
            flex-direction: column;
            gap: 1rem;
        }
        #form label {
            display: flex;
            width: 100%;
            margin-right: 1rem;
            flex-direction: column;
            text-align: left;
        }
        #form label span {
            margin-bottom: 0.5rem;
            font-size: 0.8rem;
        }
        #form input {
            width: 100%;
            padding: 0.5rem;
            font-size: 1rem;
            border: 1px solid #ccc;
            border-radius: 0.25rem;
        }
        #form button {
            padding: 0.5rem 1rem;
            font-size: 1rem;
            border: none;
            border-radius: 0.25rem;
            background-color: #0366d6;
            color: white;
            cursor: pointer;
        }
        .image-container {
            position: relative;
            width: 100%;
            height: 0;
            padding-bottom: 41.9675%;
        }
        @media (prefers-color-scheme: dark) {
            body {
                background-color: #1a1a1a;
                color: #fff;
            }
            input, button {
                background-color: #333;
                color: #fff;
            }
            input::placeholder {
                color: #ccc;
            }
        }
        @media screen and (min-width: 680px) {
            header {
                margin-bottom: auto;
            }
            #form .input {
                flex-direction: row;
            }
        }
    </style>
</head>
<body>
    <header class="container">
        <h1>Git Garden</h1>
        <p>Enter your GitHub username to generate your Git Garden.</p>
        <p class="small">If you click Generate, the image URL will be copied to your clipboard.</p>
    </header>
    <div class="container">
        <form action="/" method="get" id="form">
            <div class="input">
                <label>
                    <span>GitHub Username</span>
                    <input type="text" name="user_name" placeholder="GitHub Username" required>
                </label>
                <label>
                    <span>Year</span>
                    <input type="number" name="year" placeholder="Year" min="1900" max="{{year}}" value="{{year}}" required>
                </label>
            </div>
            <button type="submit">Generate</button>
        </form>
        <div class="image-container"></div>
    </div>
    <footer class="container">
        <p class="small">By <a href="https://github.com/marshallku">marshallku</a></p>
    </footer>
    <script>
        const copy = async (value) => {
            try {
                await navigator.clipboard.writeText(value);
            } catch {
                const textarea = document.createElement("textarea");

                textarea.value = value;
                textarea.style.position = "fixed";
                textarea.style.width = "1px";
                textarea.style.height = "1px";
                textarea.style.top = "0";
                textarea.style.left = "-1px";
                document.body.append(textarea);
                textarea.focus();
                textarea.select();
                document.execCommand("copy");
                textarea.remove();
            }
        }

        const form = document.getElementById('form');
        const imageContainer = document.querySelector('.image-container');

        form.addEventListener('submit', (e) => {
            e.preventDefault();
            const queries = new URLSearchParams();
            const user_name = form.user_name.value;
            const year = form.year.value;

            queries.set('user_name', user_name);

            if (year && year !== '{{year}}') {
                queries.set('year', year);
            }

            const imagePath = `${location.origin}/?${queries.toString()}`;

            copy(imagePath);

            imageContainer.innerHTML = `<img src="${imagePath}" alt="Git Garden for ${user_name} in ${year}">`;
        });
    </script>
</body>
</html>
"##;
    let html = template.replace("{{year}}", &current_year.to_string());

    html.to_string()
}
